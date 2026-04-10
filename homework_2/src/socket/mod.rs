//! Smart socket client — communicates with the socket simulator over TCP.

pub mod protocol;

use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::{DeviceReport, SmartDevice, SmartHomeError};
use protocol::{TcpCommand, TcpResponse};

/// A TCP client for a smart socket device.
///
/// Each method call opens a fresh TCP connection, sends one command,
/// reads one response, and closes the connection.
/// This keeps the implementation simple and stateless on the client side.
pub struct SmartSocket {
    /// Remote address of the socket simulator.
    addr: SocketAddr,
    /// Per-operation timeout.
    timeout: Duration,
}

impl SmartSocket {
    /// Creates a new [`SmartSocket`] pointing at `addr`.
    ///
    /// # Errors
    /// Returns [`SmartHomeError::Protocol`] if `addr` cannot be resolved.
    pub fn new(addr: impl ToSocketAddrs, timeout: Duration) -> Result<Self, SmartHomeError> {
        let addr = addr
            .to_socket_addrs()
            .map_err(SmartHomeError::Io)?
            .next()
            .ok_or_else(|| SmartHomeError::Protocol("empty address list".into()))?;
        Ok(Self { addr, timeout })
    }

    /// Sends a single command and returns the parsed response.
    async fn send_command(&self, cmd: TcpCommand) -> Result<TcpResponse, SmartHomeError> {
        let stream = timeout(self.timeout, TcpStream::connect(self.addr))
            .await
            .map_err(|_| SmartHomeError::Timeout)?
            .map_err(SmartHomeError::Io)?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Send command
        writer
            .write_all(cmd.encode().as_bytes())
            .await
            .map_err(SmartHomeError::Io)?;

        // Read response line
        let mut line = String::new();
        timeout(self.timeout, reader.read_line(&mut line))
            .await
            .map_err(|_| SmartHomeError::Timeout)?
            .map_err(SmartHomeError::Io)?;

        TcpResponse::decode(&line)
    }

    /// Turns the socket on.
    pub async fn turn_on(&self) -> Result<(), SmartHomeError> {
        match self.send_command(TcpCommand::TurnOn).await? {
            TcpResponse::Ok => Ok(()),
            TcpResponse::Error(msg) => Err(SmartHomeError::Protocol(msg)),
            other => Err(SmartHomeError::Protocol(format!(
                "unexpected response: {other:?}"
            ))),
        }
    }

    /// Turns the socket off.
    pub async fn turn_off(&self) -> Result<(), SmartHomeError> {
        match self.send_command(TcpCommand::TurnOff).await? {
            TcpResponse::Ok => Ok(()),
            TcpResponse::Error(msg) => Err(SmartHomeError::Protocol(msg)),
            other => Err(SmartHomeError::Protocol(format!(
                "unexpected response: {other:?}"
            ))),
        }
    }

    /// Returns the current power consumption in watts.
    pub async fn power_consumption(&self) -> Result<f32, SmartHomeError> {
        match self.send_command(TcpCommand::PowerQuery).await? {
            TcpResponse::Power(w) => Ok(w),
            TcpResponse::Error(msg) => Err(SmartHomeError::Protocol(msg)),
            other => Err(SmartHomeError::Protocol(format!(
                "unexpected response: {other:?}"
            ))),
        }
    }
}

#[async_trait::async_trait]
impl SmartDevice for SmartSocket {
    /// Returns a report with the current power consumption,
    /// or an error description if the device is unreachable.
    async fn report(&self) -> DeviceReport {
        match self.power_consumption().await {
            Ok(w) => format!("SmartSocket [{}]: {:.1} W", self.addr, w),
            Err(e) => format!("SmartSocket [{}]: ERROR — {}", self.addr, e),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::TcpResponse;
    use std::sync::{Arc, Mutex};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;

    /// Minimal in-process socket server for testing.
    /// Tracks on/off state and a fixed power value.
    async fn start_test_server(initial_on: bool, power: f32) -> (SocketAddr, Arc<Mutex<bool>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let state = Arc::new(Mutex::new(initial_on));
        let state_clone = state.clone();

        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    break;
                };
                let state = state_clone.clone();
                tokio::spawn(async move {
                    let (reader, mut writer) = stream.into_split();
                    let mut reader = BufReader::new(reader);
                    let mut line = String::new();
                    while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                        let response = match TcpCommand::decode(&line) {
                            Ok(TcpCommand::TurnOn) => {
                                *state.lock().unwrap() = true;
                                TcpResponse::Ok
                            }
                            Ok(TcpCommand::TurnOff) => {
                                *state.lock().unwrap() = false;
                                TcpResponse::Ok
                            }
                            Ok(TcpCommand::PowerQuery) => TcpResponse::Power(power),
                            Err(_) => TcpResponse::Error("unknown command".into()),
                        };
                        let _ = writer.write_all(response.encode().as_bytes()).await;
                        line.clear();
                    }
                });
            }
        });

        (addr, state)
    }

    // Feature: smart-home-tcp-udp, Property 2: TCP command round-trip
    // Validates: Requirements 2.2, 2.3, 3.3, 3.4, 3.5
    #[tokio::test]
    async fn test_turn_on_off_round_trip() {
        let (addr, state) = start_test_server(false, 100.0).await;
        let socket = SmartSocket::new(addr, Duration::from_secs(2)).unwrap();

        socket.turn_on().await.unwrap();
        assert!(*state.lock().unwrap(), "state should be on after turn_on");

        socket.turn_off().await.unwrap();
        assert!(
            !*state.lock().unwrap(),
            "state should be off after turn_off"
        );
    }

    // Feature: smart-home-tcp-udp, Property 3: power query round-trip
    // Validates: Requirements 2.4, 3.6
    #[tokio::test]
    async fn test_power_consumption_round_trip() {
        let expected_power = 220.0_f32;
        let (addr, _state) = start_test_server(true, expected_power).await;
        let socket = SmartSocket::new(addr, Duration::from_secs(2)).unwrap();

        let power = socket.power_consumption().await.unwrap();
        assert!(
            (power - expected_power).abs() < 0.01,
            "power should match server value"
        );
    }

    // Validates: Requirements 2.5 — unreachable port returns SmartHomeError
    #[tokio::test]
    async fn test_unreachable_port_returns_error() {
        // Port 1 is reserved and should be unreachable
        let socket = SmartSocket::new("127.0.0.1:1", Duration::from_millis(200)).unwrap();
        let result = socket.turn_on().await;
        assert!(result.is_err(), "should fail when server is not running");
    }
}
