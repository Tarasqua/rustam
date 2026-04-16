//! Compositor pattern for building smart home reports.
//!
//! [`ReportCompositor`] collects heterogeneous reportable items via static
//! polymorphism (generics + trait bounds) and prints a combined report to
//! stdout when [`ReportCompositor::report`] is called.
//!
//! # Example
//! ```rust,ignore
//! let mut compositor = ReportCompositor::new("My Home");
//! compositor.add(socket);
//! compositor.add(thermometer);
//! compositor.report(); // prints all items to stdout
//! ```

/// Any type that can produce a one-line text description of itself.
pub trait Reportable {
    /// Returns a human-readable description of this item.
    fn describe(&self) -> String;
}

/// Collects [`Reportable`] items and prints a combined report.
///
/// Items are stored as `Box<dyn Reportable>` so that different concrete types
/// can be mixed. The `add` method uses a generic bound `<T: Reportable>` for
/// ergonomic call sites while still using dynamic dispatch internally.
pub struct ReportCompositor {
    title: String,
    items: Vec<Box<dyn Reportable>>,
}

impl ReportCompositor {
    /// Creates a new compositor with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    /// Adds a reportable item to the compositor.
    ///
    /// The generic bound `T: Reportable + 'static` allows passing any
    /// concrete type without the caller needing to box it manually.
    pub fn add<T: Reportable + 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }

    /// Prints the combined report for all added items to stdout.
    pub fn report(&self) {
        println!("=== {} ===", self.title);
        for item in &self.items {
            println!("  - {}", item.describe());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeDevice {
        name: String,
        status: String,
    }

    impl Reportable for FakeDevice {
        fn describe(&self) -> String {
            format!("{}: {}", self.name, self.status)
        }
    }

    #[test]
    fn test_compositor_collects_items() {
        let mut c = ReportCompositor::new("Test");
        c.add(FakeDevice {
            name: "socket".into(),
            status: "ON".into(),
        });
        c.add(FakeDevice {
            name: "thermo".into(),
            status: "22 °C".into(),
        });
        // Verify items are stored (report() prints to stdout, hard to assert in unit test)
        assert_eq!(c.items.len(), 2);
        assert_eq!(c.items[0].describe(), "socket: ON");
        assert_eq!(c.items[1].describe(), "thermo: 22 °C");
    }

    #[test]
    fn test_compositor_empty() {
        let c = ReportCompositor::new("Empty");
        assert_eq!(c.items.len(), 0);
        // report() on empty compositor should not panic
        c.report();
    }
}
