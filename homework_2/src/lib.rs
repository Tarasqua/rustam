//! Smart Home library — public API re-exports.
//!
//! Provides the [`SmartDevice`] trait and type aliases used throughout
//! the crate, plus re-exports of all public sub-modules.

pub mod error;
pub mod home;
pub mod socket;
pub mod thermometer;

pub use error::SmartHomeError;

/// A human-readable status report for a single device.
pub type DeviceReport = String;

/// A formatted multi-line report for the entire house.
pub type HouseReport = String;

/// Common interface for all smart devices.
///
/// Implemented by [`socket::SmartSocket`], [`thermometer::SmartThermometer`],
/// and test doubles such as `MockSmartDevice`.
///
/// # Object safety and downcasting
///
/// The trait is object-safe via `async_trait`, allowing `Box<dyn SmartDevice>`
/// storage in [`home::SmartHome`]. The `as_any` method enables downcasting
/// back to the concrete type when you need device-specific operations
/// (e.g. calling `turn_on()` on a `SmartSocket`):
///
/// ```rust,ignore
/// if let Some(socket) = home.get_device::<SmartSocket>("Living Room", "socket1") {
///     socket.turn_on().await?;
/// }
/// ```
#[async_trait::async_trait]
pub trait SmartDevice: Send + Sync {
    /// Returns a human-readable status report for this device.
    ///
    /// On error the implementation MUST return a non-empty [`DeviceReport`]
    /// containing a description of the error rather than propagating it.
    async fn report(&self) -> DeviceReport;

    /// Returns `self` as `&dyn std::any::Any` to enable downcasting.
    ///
    /// Every implementor should provide the one-liner body:
    /// `fn as_any(&self) -> &dyn std::any::Any { self }`
    ///
    /// This is the standard Rust pattern for recovering a concrete type
    /// from a trait object stored as `Box<dyn SmartDevice>`.
    fn as_any(&self) -> &dyn std::any::Any;
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// A test double that returns a fixed report string.
    struct MockSmartDevice {
        report: String,
    }

    #[async_trait::async_trait]
    impl SmartDevice for MockSmartDevice {
        async fn report(&self) -> DeviceReport {
            self.report.clone()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    // Feature: smart-home-tcp-udp, Property 1: report() always returns non-empty string
    // Validates: Requirements 1.1, 1.3
    proptest! {
        #[test]
        fn prop_report_non_empty(s in "[a-zA-Z0-9 ]{1,64}") {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let device = MockSmartDevice { report: s.clone() };
            let result = rt.block_on(device.report());
            prop_assert!(!result.is_empty());
        }
    }
}
