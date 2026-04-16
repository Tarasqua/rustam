# Паттерны в умном доме — как это работает

---

## 1. Builder (Строитель) с Typestate

### Идея

Обычный Builder просто накапливает параметры и в конце вызывает `build()`.
Typestate Builder идёт дальше: **состояние объекта закодировано в его типе**,
и компилятор запрещает вызывать методы, которые ещё недоступны.

### Схема состояний

```
SmartHomeBuilder<NoRoom>          SmartHomeBuilder<HasRoom>
        │                                  │
        │  .add_room("Kitchen")            │  .add_room(...)      ← можно добавлять ещё комнаты
        └─────────────────────────────────►│
                                           │  .add_device(...)    ← только здесь доступен!
                                           │  .add_observer(...)
                                           │  .build()
                                           │
                                           ▼
                                       SmartHome
```

### Как это реализовано

```rust
// Маркерные типы — не содержат данных, только несут информацию для компилятора
pub struct NoRoom;
pub struct HasRoom;

pub struct SmartHomeBuilder<S> {
    name: String,
    rooms: HashMap<String, Room>,
    _state: PhantomData<S>,   // <-- хранит тип S, не занимая памяти
}
```

`impl SmartHomeBuilder<NoRoom>` содержит только `new()` и `add_room()`.
`impl SmartHomeBuilder<HasRoom>` содержит `add_room()`, `add_device()`, `add_observer()`, `build()`.

Попытка вызвать `add_device` до `add_room` — **ошибка компиляции**:

```
error[E0599]: no method named `add_device` found for struct
              `SmartHomeBuilder<NoRoom>`
```

### Пример использования

```rust
let home = SmartHomeBuilder::new("Мой дом")
    // Здесь тип: SmartHomeBuilder<NoRoom>
    .add_room("Кухня")
    // Здесь тип: SmartHomeBuilder<HasRoom>  ← переход!
    .add_device("Кухня", "Чайник", Box::new(kettle))
    .add_room("Спальня")
    .add_device("Спальня", "Лампа", Box::new(lamp))
    .build();
    // Возвращает SmartHome
```

---

## 2. Observer (Наблюдатель)

### Идея

Объект `Room` не знает, кто хочет получать уведомления. Он просто хранит
список подписчиков и вызывает их при добавлении устройства.
Подписчики могут быть любыми — структурами или замыканиями.

### Схема взаимодействия

```
  Кто-то вызывает room.add_device("лампа", device)
                        │
                        ▼
              ┌─────────────────┐
              │      Room       │
              │                 │
              │  devices: {...} │  ← устройство добавляется сюда
              │                 │
              │  observers: [   │
              │    Observer1,   │──► on_device_added("Кухня", "лампа")
              │    Observer2,   │──► on_device_added("Кухня", "лампа")
              │    Closure,     │──► on_device_added("Кухня", "лампа")
              │  ]              │
              └─────────────────┘
```

### Как это реализовано

```rust
// Трейт — контракт для любого подписчика
pub trait DeviceAddedObserver: Send + Sync {
    fn on_device_added(&self, room_name: &str, device_name: &str);
}

// Blanket impl — любое замыкание Fn(&str, &str) автоматически
// становится DeviceAddedObserver без написания обёртки
impl<F: Fn(&str, &str) + Send + Sync> DeviceAddedObserver for F {
    fn on_device_added(&self, room_name: &str, device_name: &str) {
        self(room_name, device_name)
    }
}

pub struct Room {
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
    observers: Vec<Box<dyn DeviceAddedObserver>>,  // ← динамическая диспетчеризация
}
```

### Два способа подписаться

```rust
// 1. Структура-подписчик
struct Logger { prefix: String }

impl DeviceAddedObserver for Logger {
    fn on_device_added(&self, room: &str, device: &str) {
        println!("[{}] {} добавлен в {}", self.prefix, device, room);
    }
}

room.add_observer(Logger { prefix: "LOG".into() });

// 2. Замыкание — работает благодаря blanket impl
room.add_observer(|room: &str, device: &str| {
    println!("Новое устройство: {} в {}", device, room);
});
```

### Динамический полиморфизм

`Vec<Box<dyn DeviceAddedObserver>>` — вектор трейт-объектов.
Конкретный тип стирается, вызов идёт через vtable (таблицу виртуальных методов).
Это позволяет хранить разные типы подписчиков в одном списке.

```
Vec<Box<dyn DeviceAddedObserver>>
  ┌──────────────────────────────────┐
  │  Box ──► Logger { prefix: "A" }  │  vtable → Logger::on_device_added
  │  Box ──► Counter(Arc<Mutex<u32>>)│  vtable → Counter::on_device_added
  │  Box ──► |r, d| println!(...)    │  vtable → <closure>::on_device_added
  └──────────────────────────────────┘
```

---

## 3. Compositor (Компоновщик)

### Идея

Собрать разнородные объекты в один список и единым вызовом получить
отчёт по всем. Каждый объект знает, как описать себя (`describe()`),
а Compositor просто обходит список.

### Схема

```
ReportCompositor
  title: "Мой дом"
  items: [
    Box<dyn Reportable> ──► Socket::describe()      → "Socket 'Кухня': 150 W"
    Box<dyn Reportable> ──► Thermometer::describe() → "Термометр: 21.5 °C"
    Box<dyn Reportable> ──► Camera::describe()      → "Камера: запись"
  ]
        │
        │  compositor.report()
        ▼
  === Мой дом ===
    - Socket 'Кухня': 150 W
    - Термометр: 21.5 °C
    - Камера: запись
```

### Как это реализовано

```rust
pub trait Reportable {
    fn describe(&self) -> String;
}

pub struct ReportCompositor {
    title: String,
    items: Vec<Box<dyn Reportable>>,
}

impl ReportCompositor {
    // Статический полиморфизм на входе: T: Reportable + 'static
    // Внутри всё равно хранится как Box<dyn Reportable>
    pub fn add<T: Reportable + 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }

    pub fn report(&self) {
        println!("=== {} ===", self.title);
        for item in &self.items {
            println!("  - {}", item.describe());
        }
    }
}
```

### Статический vs динамический полиморфизм

| - | Статический (`<T: Trait>`) | Динамический (`dyn Trait`) |
|---|---|---|
| Когда решается | Компиляция | Выполнение |
| Скорость | Быстрее (нет vtable) | Чуть медленнее |
| Гибкость | Один тип за раз | Разные типы в одном контейнере |
| В коде | `fn add<T: Reportable>` | `Vec<Box<dyn Reportable>>` |

В `ReportCompositor` используются **оба**:
- `add<T>` — статический полиморфизм на входе (удобный API, нет ручного `Box::new`)
- `Vec<Box<dyn Reportable>>` — динамический внутри (чтобы хранить разные типы)

---

## Общая картина проекта

```
homework_2 (lib)
│
├── SmartDevice (trait)          ← общий интерфейс устройств
│     └── report() -> String
│
├── home
│     ├── Room                   ← комната с устройствами + Observer
│     │     ├── devices: HashMap<String, Box<dyn SmartDevice>>
│     │     └── observers: Vec<Box<dyn DeviceAddedObserver>>
│     │
│     ├── SmartHome              ← агрегатор комнат
│     │     └── house_report()  ← async, параллельный сбор отчётов
│     │
│     └── SmartHomeBuilder<S>   ← typestate builder
│           ├── <NoRoom>: new(), add_room()
│           └── <HasRoom>: add_room(), add_device(), build()
│
├── compositor
│     ├── Reportable (trait)
│     └── ReportCompositor       ← компоновщик отчётов
│
├── socket
│     ├── SmartSocket            ← TCP-клиент розетки
│     └── protocol               ← TcpCommand / TcpResponse
│
└── thermometer
      ├── SmartThermometer       ← UDP-приёмник температуры
      └── packet                 ← UdpTemperaturePacket
```
