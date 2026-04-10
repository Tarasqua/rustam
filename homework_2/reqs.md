# Документ требований

## Введение

Данная функциональность реализует систему «Умный дом» на языке Rust с использованием асинхронного
рантайма Tokio. Система включает два типа умных устройств: умную розетку (управление по TCP) и умный
термометр (получение данных по UDP). Для каждого устройства предусмотрен симулятор, позволяющий
тестировать логику без реального оборудования. Архитектура построена на обобщённых трейтах, что
обеспечивает взаимозаменяемость реальных и тестовых реализаций.

## Глоссарий

- **SmartSocket** — умная розетка; устройство, поддерживающее включение/выключение и запрос потребляемой мощности через TCP.
- **SmartThermometer** — умный термометр; устройство, получающее значения температуры по UDP и возвращающее последнее полученное значение.
- **SocketSimulator** — симулятор умной розетки; TCP-сервер, хранящий состояние розетки и обрабатывающий команды от клиентов.
- **ThermometerSimulator** — симулятор умного термометра; UDP-клиент, периодически отправляющий значения температуры на заданный адрес.
- **SmartHome** — агрегатор умных устройств; структура, хранящая коллекцию устройств и формирующая отчёт об их состоянии.
- **DeviceReport** — текстовый отчёт о состоянии одного устройства.
- **HouseReport** — сводный текстовый отчёт о состоянии всех устройств в доме.
- **TcpCommand** — бинарный или текстовый протокол команд, передаваемых умной розетке по TCP.
- **UdpTemperaturePacket** — UDP-пакет, содержащий значение температуры в виде числа с плавающей точкой (f32).
- **Tokio** — асинхронный рантайм для Rust, используемый во всём проекте.

---

## Требования

### Требование 1: Трейт умного устройства

**User Story:** Как разработчик, я хочу единый обобщённый трейт для умных устройств, чтобы реальные и тестовые реализации были взаимозаменяемы.

#### Критерии приёмки

1. THE SmartDevice_Trait SHALL define an async method `report() -> DeviceReport` that returns a textual status of the device.
2. THE SmartDevice_Trait SHALL be object-safe or usable with generics so that `SmartHome` can store heterogeneous collections of devices.
3. WHEN a device fails to retrieve its status, THE SmartDevice_Trait SHALL allow `report()` to return a `DeviceReport` containing a human-readable error description.

---

### Требование 2: Умная розетка — клиентская логика

**User Story:** Как пользователь системы, я хочу управлять умной розеткой через TCP, чтобы включать/выключать её и узнавать потребляемую мощность.

#### Критерии приёмки

1. THE SmartSocket SHALL connect to a remote TCP address provided at construction time.
2. WHEN a caller invokes `turn_on()`, THE SmartSocket SHALL send a TcpCommand to the remote device and await acknowledgement.
3. WHEN a caller invokes `turn_off()`, THE SmartSocket SHALL send a TcpCommand to the remote device and await acknowledgement.
4. WHEN a caller invokes `power_consumption()`, THE SmartSocket SHALL send a TcpCommand to the remote device and return the power value (watts, f32) received in the response.
5. IF the TCP connection cannot be established or a response is not received within the timeout, THEN THE SmartSocket SHALL return an error of type `SmartHomeError`.
6. THE SmartSocket SHALL implement `SmartDevice_Trait` so that `report()` returns the current on/off state and power consumption, or an error description if the device is unreachable.

---

### Требование 3: Симулятор умной розетки

**User Story:** Как разработчик, я хочу симулятор умной розетки, чтобы тестировать клиентскую логику без реального устройства.

#### Критерии приёмки

1. THE SocketSimulator SHALL read the TCP listen address from CLI arguments at startup.
2. THE SocketSimulator SHALL accept multiple simultaneous TCP client connections using non-blocking I/O via Tokio.
3. THE SocketSimulator SHALL maintain an internal boolean state representing whether the socket is on or off.
4. WHEN a `turn_on` TcpCommand is received, THE SocketSimulator SHALL update its state to on and send an acknowledgement response.
5. WHEN a `turn_off` TcpCommand is received, THE SocketSimulator SHALL update its state to off and send an acknowledgement response.
6. WHEN a `power_query` TcpCommand is received, THE SocketSimulator SHALL respond with a simulated power value (watts, f32).
7. IF an unrecognised TcpCommand is received, THEN THE SocketSimulator SHALL respond with an error message and keep its current state unchanged.
8. THE SocketSimulator SHALL be runnable as a standalone binary (`cargo run --bin socket_simulator`).

---

### Требование 4: Умный термометр — клиентская логика

**User Story:** Как пользователь системы, я хочу получать актуальную температуру от умного термометра, чтобы отслеживать условия в помещении.

#### Критерии приёмки

1. WHEN a `SmartThermometer` object is created, THE SmartThermometer SHALL start a background Tokio task that listens on a UDP socket for incoming `UdpTemperaturePacket` values.
2. WHEN a `UdpTemperaturePacket` is received, THE SmartThermometer SHALL store the contained f32 temperature value as the last known temperature.
3. WHEN a caller invokes `temperature()`, THE SmartThermometer SHALL return the last received temperature value (f32).
4. WHEN the `SmartThermometer` object is dropped, THE SmartThermometer SHALL stop the background UDP listener task.
5. IF no temperature packet has been received yet, THEN THE SmartThermometer SHALL return a `SmartHomeError` indicating no data is available.
6. THE SmartThermometer SHALL implement `SmartDevice_Trait` so that `report()` returns the current temperature or an error description if no data has been received.

---

### Требование 5: Симулятор умного термометра

**User Story:** Как разработчик, я хочу симулятор умного термометра, чтобы тестировать клиентскую логику без реального датчика.

#### Критерии приёмки

1. THE ThermometerSimulator SHALL read the UDP destination address and send interval from a configuration file at startup.
2. THE ThermometerSimulator SHALL send `UdpTemperaturePacket` values to the configured address at the configured interval using non-blocking I/O via Tokio.
3. THE ThermometerSimulator SHALL send arbitrary (e.g., randomly generated) temperature values within a realistic range (−50.0 to +150.0 °C).
4. THE ThermometerSimulator SHALL be runnable as a standalone binary (`cargo run --bin thermometer_simulator`).
5. IF the configuration file is missing or malformed, THEN THE ThermometerSimulator SHALL print a descriptive error and exit with a non-zero status code.

---

### Требование 6: Умный дом — агрегатор устройств

**User Story:** Как пользователь системы, я хочу получать сводный отчёт о состоянии всех устройств в доме, чтобы видеть общую картину.

#### Критерии приёмки

1. THE SmartHome SHALL store a named collection of rooms, each containing a named collection of devices implementing `SmartDevice_Trait`.
2. WHEN `house_report()` is called, THE SmartHome SHALL asynchronously query each device's `report()` method and aggregate the results into a `HouseReport`.
3. WHEN a device returns an error in its `report()`, THE SmartHome SHALL include the error description in the `HouseReport` without aborting the overall report generation.
4. THE SmartHome SHALL allow devices to be added to a room by name at runtime.
5. IF a room with the given name does not exist when adding a device, THEN THE SmartHome SHALL create the room automatically.

---

### Требование 7: Пример приложения

**User Story:** Как проверяющий, я хочу запустить пример приложения, чтобы убедиться в корректной совместной работе всех компонентов.

#### Критерии приёмки

1. THE Example_App SHALL create a `SmartHome` instance containing at least one `SmartSocket` and one `SmartThermometer`.
2. WHEN the simulators are running, THE Example_App SHALL successfully call `house_report()` and print the result to stdout.
3. WHEN a device simulator is not running, THE Example_App SHALL print an error description for that device in the report without panicking.
4. THE Example_App SHALL be runnable via `cargo run` or as a binary example (`cargo run --example smart_home`).

---

### Требование 8: Качество кода и тестирование

**User Story:** Как разработчик, я хочу, чтобы код соответствовал стандартам качества Rust, чтобы проект был поддерживаемым и проверяемым.

#### Критерии приёмки

1. THE Codebase SHALL pass `cargo clippy -- -D warnings` without any warnings or errors.
2. THE Codebase SHALL pass `cargo fmt --check` without any formatting issues.
3. THE Codebase SHALL include unit tests covering the core logic of `SmartSocket`, `SmartThermometer`, and `SmartHome` using mock or in-process implementations.
4. WHEN unit tests are run via `cargo test`, THE Codebase SHALL report all tests as passing.
5. THE Codebase SHALL use English doc-comments (`///` or `//!`) on all public items.
6. THE Codebase SHALL organise source code into separate Rust modules: `socket`, `thermometer`, `home`, and `error`.
