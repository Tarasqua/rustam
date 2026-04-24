//! Example: Observer pattern on Room — callbacks on device addition.
//!
//! Demonstrates:
//! - Registering a struct-based subscriber (implements [`DeviceAddedObserver`]).
//! - Registering a closure-based subscriber.
//! - Both fire when a device is added to the room.
//!
//! Run with:
//! ```
//! cargo run --example observer_example
//! ```

use homework_2::home::{DeviceAddedObserver, Room};
use homework_2::{DeviceReport, SmartDevice};

// ---------------------------------------------------------------------------
// A simple device stub
// ---------------------------------------------------------------------------

struct Lamp {
    name: String,
}

#[async_trait::async_trait]
impl SmartDevice for Lamp {
    async fn report(&self) -> DeviceReport {
        format!("Lamp '{}': ON", self.name)
    }
}

// ---------------------------------------------------------------------------
// Struct-based observer
// ---------------------------------------------------------------------------

/// Logs every device addition to an internal list.
struct DeviceLogger {
    prefix: String,
}

impl DeviceAddedObserver for DeviceLogger {
    fn on_device_added(&self, room_name: &str, device_name: &str) {
        println!(
            "[{}] Device added — room: '{}', device: '{}'",
            self.prefix, room_name, device_name
        );
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let mut room = Room::new("Living Room");

    // 1. Struct-based observer
    room.add_observer(DeviceLogger {
        prefix: "Logger".into(),
    });

    // 2. Closure-based observer — works because of the blanket impl on Fn(&str, &str)
    room.add_observer(|room_name: &str, device_name: &str| {
        println!("[Closure] '{}' was added to '{}'", device_name, room_name);
    });

    println!("--- Adding devices ---");
    room.add_device(
        "Floor Lamp",
        Box::new(Lamp {
            name: "Floor Lamp".into(),
        }),
    );
    room.add_device(
        "Desk Lamp",
        Box::new(Lamp {
            name: "Desk Lamp".into(),
        }),
    );

    println!();
    println!(
        "Room '{}' now has {} device(s).",
        room.name,
        room.devices.len()
    );
}
