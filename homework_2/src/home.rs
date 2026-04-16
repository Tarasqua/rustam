//! Smart home aggregator — stores rooms and devices, generates house reports.
//!
//! Also provides:
//! - [`SmartHomeBuilder`] — a typestate builder that prevents adding devices
//!   before any room has been added (enforced at compile time).
//! - Observer support on [`Room`] — callbacks fired when a device is added.

use std::collections::HashMap;

use futures::future::join_all;

use crate::{DeviceReport, HouseReport, SmartDevice};

// ---------------------------------------------------------------------------
// NOTE: Observer trait
// ---------------------------------------------------------------------------

/// Callback invoked whenever a device is added to a [`Room`].
///
/// Implement this trait on a struct, or use a plain closure — both work
/// because of the blanket `impl` below.
pub trait DeviceAddedObserver: Send + Sync {
    /// Called after `device_name` has been inserted into the room.
    fn on_device_added(&self, room_name: &str, device_name: &str);
}

/// Blanket impl so that any `Fn(&str, &str) + Send + Sync` can be used
/// directly as an observer without a wrapper struct.
impl<F: Fn(&str, &str) + Send + Sync> DeviceAddedObserver for F {
    fn on_device_added(&self, room_name: &str, device_name: &str) {
        self(room_name, device_name);
    }
}

// ---------------------------------------------------------------------------
// Room
// ---------------------------------------------------------------------------

/// A named room containing a collection of smart devices.
///
/// Supports observer callbacks (see [`Room::add_observer`]).
pub struct Room {
    /// The display name of this room.
    pub name: String,
    /// Named devices in this room.
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
    /// Observers notified on every `add_device` call.
    observers: Vec<Box<dyn DeviceAddedObserver>>,
}

impl Room {
    /// Creates a new empty room with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
            observers: Vec::new(),
        }
    }

    /// Registers an observer (struct or closure) that is called whenever
    /// a device is added to this room.
    ///
    /// Uses dynamic dispatch (`dyn DeviceAddedObserver`) so heterogeneous
    /// observer types can be mixed freely.
    pub fn add_observer(&mut self, observer: impl DeviceAddedObserver + 'static) {
        self.observers.push(Box::new(observer));
    }

    /// Inserts a device and notifies all registered observers.
    pub fn add_device(&mut self, name: impl Into<String>, device: Box<dyn SmartDevice>) {
        let name = name.into();
        self.devices.insert(name.clone(), device);
        for obs in &self.observers {
            obs.on_device_added(&self.name, &name);
        }
    }
}

// ---------------------------------------------------------------------------
// Typestate marker types for SmartHomeBuilder
// ---------------------------------------------------------------------------

/// Typestate: no room has been added yet — devices cannot be added.
pub struct NoRoom;
/// Typestate: at least one room exists — devices can be added.
pub struct HasRoom;

// ---------------------------------------------------------------------------
// SmartHomeBuilder
// ---------------------------------------------------------------------------

/// A builder for [`SmartHome`].
///
/// The type parameter `S` is a typestate that tracks whether at least one
/// room has been added. The compiler rejects calls to `add_device` when
/// `S = NoRoom`, making the constraint entirely static.
///
/// # Example
/// ```rust,ignore
/// let home = SmartHomeBuilder::new("My Home")
///     .add_room("Living Room")
///     .add_device("Living Room", "socket1", Box::new(my_socket))
///     .build();
/// ```
pub struct SmartHomeBuilder<S> {
    name: String,
    rooms: HashMap<String, Room>,
    _state: std::marker::PhantomData<S>,
}

impl SmartHomeBuilder<NoRoom> {
    /// Creates a new builder. No rooms or devices can be added yet.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
            _state: std::marker::PhantomData,
        }
    }

    /// Adds the first room, transitioning the builder to [`HasRoom`] state.
    pub fn add_room(mut self, room_name: impl Into<String>) -> SmartHomeBuilder<HasRoom> {
        let name = room_name.into();
        self.rooms.insert(name.clone(), Room::new(name));
        SmartHomeBuilder {
            name: self.name,
            rooms: self.rooms,
            _state: std::marker::PhantomData,
        }
    }
}

impl SmartHomeBuilder<HasRoom> {
    /// Adds another room. Only available once at least one room exists.
    pub fn add_room(mut self, room_name: impl Into<String>) -> Self {
        let name = room_name.into();
        self.rooms
            .entry(name.clone())
            .or_insert_with(|| Room::new(name));
        self
    }

    /// Adds a device to an existing room.
    ///
    /// If `room_name` does not exist it is created automatically (same
    /// behaviour as [`SmartHome::add_device`]).
    pub fn add_device(
        mut self,
        room_name: impl Into<String>,
        device_name: impl Into<String>,
        device: Box<dyn SmartDevice>,
    ) -> Self {
        let rn = room_name.into();
        let entry = self
            .rooms
            .entry(rn.clone())
            .or_insert_with(|| Room::new(rn));
        entry.add_device(device_name, device);
        self
    }

    /// Registers an observer on a room. The room is created if it does not exist.
    pub fn add_observer(
        mut self,
        room_name: impl Into<String>,
        observer: impl DeviceAddedObserver + 'static,
    ) -> Self {
        let rn = room_name.into();
        let entry = self
            .rooms
            .entry(rn.clone())
            .or_insert_with(|| Room::new(rn));
        entry.add_observer(observer);
        self
    }

    /// Consumes the builder and returns a [`SmartHome`].
    pub fn build(self) -> SmartHome {
        SmartHome {
            name: self.name,
            rooms: self.rooms,
        }
    }
}

// ---------------------------------------------------------------------------
// SmartHome
// ---------------------------------------------------------------------------

/// The top-level smart home aggregator.
///
/// Stores a named collection of [`Room`]s, each containing
/// named [`SmartDevice`] instances. Generates concurrent house reports.
pub struct SmartHome {
    /// The display name of this home.
    pub name: String,
    /// Rooms keyed by name.
    pub(crate) rooms: HashMap<String, Room>,
}

impl SmartHome {
    /// Creates a new empty [`SmartHome`] with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }

    /// Adds a device to the named room, creating the room if it does not exist.
    ///
    /// If a device with the same name already exists in the room, it is replaced.
    pub fn add_device(
        &mut self,
        room: impl Into<String>,
        name: impl Into<String>,
        device: Box<dyn SmartDevice>,
    ) {
        let room_name = room.into();
        let entry = self
            .rooms
            .entry(room_name.clone())
            .or_insert_with(|| Room::new(room_name));
        entry.add_device(name, device);
    }

    /// Returns a shared reference to a device, downcast to its concrete type `T`.
    pub fn get_device<T: 'static>(&self, room: &str, name: &str) -> Option<&T> {
        self.rooms
            .get(room)?
            .devices
            .get(name)?
            .as_any()
            .downcast_ref::<T>()
    }

    /// Generates a formatted multi-line report for all devices in the house.
    ///
    /// Device reports are collected concurrently via [`join_all`].
    pub async fn house_report(&self) -> HouseReport {
        let mut lines = vec![format!("=== {} ===", self.name)];

        let mut labels: Vec<(String, String)> = Vec::new();
        let mut futs: Vec<_> = Vec::new();

        for (room_name, room) in &self.rooms {
            for (dev_name, device) in &room.devices {
                labels.push((room_name.clone(), dev_name.clone()));
                futs.push(device.report());
            }
        }

        let reports: Vec<DeviceReport> = join_all(futs).await;

        for ((room_name, dev_name), report) in labels.into_iter().zip(reports) {
            lines.push(format!("  [{room_name}] {dev_name}: {report}"));
        }

        lines.join("\n")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceReport, SmartDevice};
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct MockDevice {
        report: String,
    }

    #[async_trait]
    impl SmartDevice for MockDevice {
        async fn report(&self) -> DeviceReport {
            self.report.clone()
        }
    }

    struct ErrorDevice;

    #[async_trait]
    impl SmartDevice for ErrorDevice {
        async fn report(&self) -> DeviceReport {
            "ERROR — simulated failure".to_string()
        }
    }

    // Feature: smart-home-tcp-udp, Property 6: device storage round-trip
    // Validates: Requirements 6.1, 6.2, 6.4
    #[tokio::test]
    async fn test_device_appears_in_report() {
        let mut home = SmartHome::new("Test Home");
        home.add_device(
            "Living Room",
            "socket1",
            Box::new(MockDevice {
                report: "100 W".to_string(),
            }),
        );
        let report = home.house_report().await;
        assert!(report.contains("socket1"));
        assert!(report.contains("100 W"));
    }

    // Feature: smart-home-tcp-udp, Property 7: error resilience
    // Validates: Requirements 6.3, 7.3
    #[tokio::test]
    async fn test_error_device_does_not_abort_report() {
        let mut home = SmartHome::new("Test Home");
        home.add_device(
            "Kitchen",
            "good_device",
            Box::new(MockDevice {
                report: "OK".to_string(),
            }),
        );
        home.add_device("Kitchen", "bad_device", Box::new(ErrorDevice));
        let report = home.house_report().await;
        assert!(report.contains("good_device"));
        assert!(report.contains("bad_device"));
        assert!(report.contains("ERROR"));
    }

    // Validates: Requirements 6.5 — adding to non-existent room creates it
    #[tokio::test]
    async fn test_add_device_creates_room() {
        let mut home = SmartHome::new("Test Home");
        home.add_device(
            "New Room",
            "device1",
            Box::new(MockDevice {
                report: "OK".to_string(),
            }),
        );
        assert!(home.rooms.contains_key("New Room"));
    }

    // Builder: cannot call add_device before add_room (compile-time check).
    // This test verifies the happy path — builder produces a working SmartHome.
    #[tokio::test]
    async fn test_builder_happy_path() {
        let home = SmartHomeBuilder::new("Builder Home")
            .add_room("Kitchen")
            .add_device(
                "Kitchen",
                "kettle",
                Box::new(MockDevice {
                    report: "2000 W".to_string(),
                }),
            )
            .build();

        let report = home.house_report().await;
        assert!(report.contains("kettle"));
        assert!(report.contains("2000 W"));
    }

    // Observer: callback is fired when a device is added.
    #[test]
    fn test_observer_called_on_add_device() {
        let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let log_clone = log.clone();

        let mut room = Room::new("Living Room");
        room.add_observer(move |room_name: &str, device_name: &str| {
            log_clone
                .lock()
                .unwrap()
                .push(format!("{room_name}/{device_name}"));
        });

        room.add_device(
            "socket1",
            Box::new(MockDevice {
                report: "OK".to_string(),
            }),
        );
        room.add_device(
            "socket2",
            Box::new(MockDevice {
                report: "OK".to_string(),
            }),
        );

        let entries = log.lock().unwrap();
        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&"Living Room/socket1".to_string()));
        assert!(entries.contains(&"Living Room/socket2".to_string()));
    }

    // Observer: struct-based subscriber works alongside closure-based one.
    #[test]
    fn test_struct_observer() {
        struct Counter(Arc<Mutex<u32>>);
        impl DeviceAddedObserver for Counter {
            fn on_device_added(&self, _room: &str, _device: &str) {
                *self.0.lock().unwrap() += 1;
            }
        }

        let count = Arc::new(Mutex::new(0u32));
        let mut room = Room::new("Hall");
        room.add_observer(Counter(count.clone()));

        room.add_device(
            "d1",
            Box::new(MockDevice {
                report: "x".to_string(),
            }),
        );
        room.add_device(
            "d2",
            Box::new(MockDevice {
                report: "x".to_string(),
            }),
        );

        assert_eq!(*count.lock().unwrap(), 2);
    }
}
