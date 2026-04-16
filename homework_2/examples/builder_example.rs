//! Example: SmartHomeBuilder — typestate builder pattern.
//!
//! Demonstrates that:
//! - `add_device` is unavailable before `add_room` (compile-time guarantee).
//! - The builder chains fluently and produces a working [`SmartHome`].
//!
//! Run with:
//! ```text
//! cargo run --example builder_example
//! ```

use homework_2::home::SmartHomeBuilder;
use homework_2::{DeviceReport, SmartDevice};

/// A simple stub device for the example.
struct LightBulb {
    name: String,
    on: bool,
}

#[async_trait::async_trait]
impl SmartDevice for LightBulb {
    async fn report(&self) -> DeviceReport {
        format!(
            "LightBulb '{}': {}",
            self.name,
            if self.on { "ON" } else { "OFF" }
        )
    }
}

#[tokio::main]
async fn main() {
    // The following would NOT compile — add_device is only available after add_room:
    //
    //   SmartHomeBuilder::new("X").add_device("room", "dev", Box::new(...));
    //                              ^^^^^^^^^^^ method not found in `SmartHomeBuilder<NoRoom>`

    let home = SmartHomeBuilder::new("Builder Demo Home")
        .add_room("Living Room")
        // add_device is now available because we are in HasRoom state
        .add_device(
            "Living Room",
            "Ceiling Light",
            Box::new(LightBulb {
                name: "Ceiling Light".into(),
                on: true,
            }),
        )
        .add_room("Kitchen")
        .add_device(
            "Kitchen",
            "Counter Light",
            Box::new(LightBulb {
                name: "Counter Light".into(),
                on: false,
            }),
        )
        .build();

    println!("Home built: {}", home.name);
    println!();
    let report = home.house_report().await;
    println!("{report}");
}
