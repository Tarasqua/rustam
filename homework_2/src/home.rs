//! Smart home aggregator — stores rooms and devices, generates house reports.

use std::collections::HashMap;

use futures::future::join_all;

use crate::{DeviceReport, HouseReport, SmartDevice};

/// A named room containing a collection of smart devices.
pub struct Room {
    /// The display name of this room.
    pub name: String,
    /// Named devices in this room.
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl Room {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
        }
    }
}

/// The top-level smart home aggregator.
///
/// Stores a named collection of [`Room`]s, each containing
/// named [`SmartDevice`] instances. Generates concurrent house reports.
pub struct SmartHome {
    /// The display name of this home.
    pub name: String,
    /// Rooms keyed by name.
    rooms: HashMap<String, Room>,
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
        entry.devices.insert(name.into(), device);
    }

    /// Returns a shared reference to a device, downcast to its concrete type `T`.
    ///
    /// This is how you reach device-specific methods (e.g. `turn_on()` on a
    /// [`SmartSocket`][crate::socket::SmartSocket]) after the device has been
    /// stored as `Box<dyn SmartDevice>`.
    ///
    /// # How it works
    /// Devices are stored as trait objects (`Box<dyn SmartDevice>`), which erases
    /// the concrete type. `as_any()` hands back `&dyn Any`, and `downcast_ref::<T>`
    /// checks the runtime type tag — returning `Some(&T)` on a match, `None` otherwise.
    ///
    /// # Example
    /// ```rust,ignore
    /// use std::time::Duration;
    /// use homework_2::socket::SmartSocket;
    ///
    /// if let Some(socket) = home.get_device::<SmartSocket>("Living Room", "socket1") {
    ///     socket.turn_on().await.unwrap();
    /// }
    /// ```
    pub fn get_device<T: 'static>(&self, room: &str, name: &str) -> Option<&T> {
        // Walk: home → room → device (as trait object) → downcast to T
        self.rooms
            .get(room)? // find the room by name
            .devices
            .get(name)? // find the device by name
            .as_any() // erase to &dyn Any (via SmartDevice::as_any)
            .downcast_ref::<T>() // attempt runtime type check
    }

    /// Generates a formatted multi-line report for all devices in the house.
    ///
    /// Device reports are collected concurrently via [`join_all`].
    /// Devices that return errors are included in the report rather than
    /// causing the whole report to fail.
    pub async fn house_report(&self) -> HouseReport {
        let mut lines = vec![format!("=== {} ===", self.name)];

        // Collect labels and futures separately to avoid lifetime issues
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceReport, SmartDevice};
    use async_trait::async_trait;

    /// Test double: always returns a fixed report string.
    struct MockDevice {
        report: String,
    }

    #[async_trait]
    impl SmartDevice for MockDevice {
        async fn report(&self) -> DeviceReport {
            self.report.clone()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    /// Test double: always returns an error description.
    struct ErrorDevice;

    #[async_trait]
    impl SmartDevice for ErrorDevice {
        async fn report(&self) -> DeviceReport {
            "ERROR — simulated failure".to_string()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
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
        assert!(
            report.contains("socket1"),
            "report should contain device name"
        );
        assert!(
            report.contains("100 W"),
            "report should contain device report"
        );
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
        assert!(report.contains("good_device"), "good device should appear");
        assert!(report.contains("bad_device"), "bad device should appear");
        assert!(report.contains("ERROR"), "error description should appear");
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
        assert!(
            home.rooms.contains_key("New Room"),
            "room should be created automatically"
        );
    }
}
