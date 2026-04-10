//! Smart thermometer client — receives temperature readings via UDP.

pub mod packet;

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use tokio::net::UdpSocket;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{DeviceReport, SmartDevice, SmartHomeError};
use packet::UdpTemperaturePacket;

/// A UDP-based smart thermometer.
///
/// On creation, spawns a background Tokio task that listens for
/// [`UdpTemperaturePacket`] datagrams and stores the latest value.
/// The background task is cancelled when this struct is dropped.
pub struct SmartThermometer {
    /// The bound local address of the UDP socket.
    pub addr: SocketAddr,
    /// The most recently received temperature (°C), or `None` if no packet yet.
    last_temp: Arc<Mutex<Option<f32>>>,
    /// Token used to signal the background task to stop.
    cancel: CancellationToken,
    /// Handle to the background listener task (kept to avoid detaching).
    _task: JoinHandle<()>,
}

impl SmartThermometer {
    /// Creates a new [`SmartThermometer`] bound to `bind_addr`.
    ///
    /// Spawns a background task that reads UDP datagrams and updates
    /// the stored temperature. The task runs until the thermometer is dropped.
    ///
    /// # Errors
    /// Returns [`SmartHomeError::Io`] if the UDP socket cannot be bound.
    pub async fn new(bind_addr: impl ToSocketAddrs) -> Result<Self, SmartHomeError> {
        // Resolve address
        let addr = bind_addr
            .to_socket_addrs()
            .map_err(SmartHomeError::Io)?
            .next()
            .ok_or_else(|| SmartHomeError::Protocol("empty address list".into()))?;

        let socket = UdpSocket::bind(addr).await.map_err(SmartHomeError::Io)?;
        let bound_addr = socket.local_addr().map_err(SmartHomeError::Io)?;

        let last_temp: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(None));
        let cancel = CancellationToken::new();

        let last_temp_clone = last_temp.clone();
        let cancel_clone = cancel.clone();

        let task = tokio::spawn(async move {
            let mut buf = [0u8; 64];
            loop {
                tokio::select! {
                    _ = cancel_clone.cancelled() => break,
                    result = socket.recv(&mut buf) => {
                        match result {
                            Ok(n) => {
                                if let Ok(packet) = UdpTemperaturePacket::decode(&buf[..n]) {
                                    *last_temp_clone.lock().unwrap() = Some(packet.0);
                                }
                                // Malformed packets are silently ignored
                            }
                            Err(_) => break,
                        }
                    }
                }
            }
        });

        Ok(Self {
            addr: bound_addr,
            last_temp,
            cancel,
            _task: task,
        })
    }

    /// Returns the last received temperature in °C.
    ///
    /// # Errors
    /// Returns [`SmartHomeError::NoData`] if no packet has been received yet.
    pub async fn temperature(&self) -> Result<f32, SmartHomeError> {
        self.last_temp.lock().unwrap().ok_or(SmartHomeError::NoData)
    }
}

impl Drop for SmartThermometer {
    /// Cancels the background UDP listener task.
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

#[async_trait::async_trait]
impl SmartDevice for SmartThermometer {
    /// Returns a report with the current temperature,
    /// or an error description if no data has been received.
    async fn report(&self) -> DeviceReport {
        match self.temperature().await {
            Ok(t) => format!("SmartThermometer: {:.1} °C", t),
            Err(e) => format!("SmartThermometer: ERROR — {}", e),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet::UdpTemperaturePacket;
    use tokio::net::UdpSocket;

    // Feature: smart-home-tcp-udp, Property 4: temperature UDP round-trip
    // Validates: Requirements 4.1, 4.2, 4.3
    #[tokio::test]
    async fn test_temperature_udp_round_trip() {
        let thermo = SmartThermometer::new("127.0.0.1:0").await.unwrap();
        let thermo_addr = thermo.addr;

        let sender = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let expected = 23.5_f32;
        let packet = UdpTemperaturePacket(expected);
        sender.send_to(&packet.encode(), thermo_addr).await.unwrap();

        // Give the background task time to process
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let temp = thermo.temperature().await.unwrap();
        assert!(
            (temp - expected).abs() < 0.001,
            "temperature should match sent value"
        );
    }

    // Validates: Requirements 4.5 — NoData before first packet
    #[tokio::test]
    async fn test_no_data_before_first_packet() {
        let thermo = SmartThermometer::new("127.0.0.1:0").await.unwrap();
        let result = thermo.temperature().await;
        assert!(
            matches!(result, Err(SmartHomeError::NoData)),
            "should return NoData before any packet is received"
        );
    }

    // Validates: Requirements 4.4 — drop does not panic
    #[tokio::test]
    async fn test_drop_does_not_panic() {
        let thermo = SmartThermometer::new("127.0.0.1:0").await.unwrap();
        drop(thermo);
        // If we reach here without panic, the test passes
    }
}
