//! Example: ReportCompositor — compositor pattern with static polymorphism.
//!
//! Demonstrates adding heterogeneous [`Reportable`] items to a compositor
//! and printing a combined report via `compositor.report()`.
//!
//! Run with:
//! ```text
//! cargo run --example compositor_example
//! ```

use homework_2::compositor::{ReportCompositor, Reportable};

struct Socket {
    name: String,
    watts: f32,
}

impl Reportable for Socket {
    fn describe(&self) -> String {
        format!("Socket '{}': {:.1} W", self.name, self.watts)
    }
}

struct Thermometer {
    name: String,
    celsius: f32,
}

impl Reportable for Thermometer {
    fn describe(&self) -> String {
        format!("Thermometer '{}': {:.1} °C", self.name, self.celsius)
    }
}

struct SecurityCamera {
    location: String,
    recording: bool,
}

impl Reportable for SecurityCamera {
    fn describe(&self) -> String {
        format!(
            "Camera '{}': {}",
            self.location,
            if self.recording { "recording" } else { "idle" }
        )
    }
}

fn main() {
    let mut compositor = ReportCompositor::new("My Smart Home");

    // add() accepts any T: Reportable — static dispatch, no manual boxing needed
    compositor.add(Socket {
        name: "Living Room Socket".into(),
        watts: 150.0,
    });
    compositor.add(Thermometer {
        name: "Bedroom Thermometer".into(),
        celsius: 21.5,
    });
    compositor.add(SecurityCamera {
        location: "Front Door".into(),
        recording: true,
    });
    compositor.add(Socket {
        name: "Kitchen Socket".into(),
        watts: 2000.0,
    });

    // Prints the combined report for all items
    compositor.report();
}
