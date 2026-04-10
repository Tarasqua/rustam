//! TCP text protocol for the smart socket.
//!
//! Each message is a UTF-8 line terminated with `\n`.
//!
//! | Client → Server | Server → Client      |
//! |-----------------|----------------------|
//! | `TURN_ON\n`     | `OK\n`               |
//! | `TURN_OFF\n`    | `OK\n`               |
//! | `POWER\n`       | `POWER <f32>\n`      |
//! | anything else   | `ERROR <msg>\n`      |

use crate::SmartHomeError;

/// Commands that a client can send to the socket simulator.
#[derive(Debug, Clone, PartialEq)]
pub enum TcpCommand {
    /// Turn the socket on.
    TurnOn,
    /// Turn the socket off.
    TurnOff,
    /// Request the current power consumption in watts.
    PowerQuery,
}

impl TcpCommand {
    /// Encodes the command as a newline-terminated string ready to send over TCP.
    pub fn encode(&self) -> String {
        match self {
            TcpCommand::TurnOn => "TURN_ON\n".to_string(),
            TcpCommand::TurnOff => "TURN_OFF\n".to_string(),
            TcpCommand::PowerQuery => "POWER\n".to_string(),
        }
    }

    /// Decodes a trimmed line received from the network into a [`TcpCommand`].
    ///
    /// Returns [`SmartHomeError::Protocol`] for unrecognised input.
    pub fn decode(s: &str) -> Result<Self, SmartHomeError> {
        match s.trim() {
            "TURN_ON" => Ok(TcpCommand::TurnOn),
            "TURN_OFF" => Ok(TcpCommand::TurnOff),
            "POWER" => Ok(TcpCommand::PowerQuery),
            other => Err(SmartHomeError::Protocol(format!(
                "unknown command: {other:?}"
            ))),
        }
    }
}

/// Responses that the socket simulator sends back to the client.
#[derive(Debug, Clone, PartialEq)]
pub enum TcpResponse {
    /// Acknowledgement for TURN_ON / TURN_OFF.
    Ok,
    /// Power reading in watts.
    Power(f32),
    /// Error message from the server.
    Error(String),
}

impl TcpResponse {
    /// Encodes the response as a newline-terminated string.
    pub fn encode(&self) -> String {
        match self {
            TcpResponse::Ok => "OK\n".to_string(),
            TcpResponse::Power(w) => format!("POWER {w}\n"),
            TcpResponse::Error(msg) => format!("ERROR {msg}\n"),
        }
    }

    /// Decodes a trimmed line received from the network into a [`TcpResponse`].
    ///
    /// Returns [`SmartHomeError::Protocol`] for unrecognised input.
    pub fn decode(s: &str) -> Result<Self, SmartHomeError> {
        let s = s.trim();
        if s == "OK" {
            return Ok(TcpResponse::Ok);
        }
        if let Some(rest) = s.strip_prefix("POWER ") {
            let watts: f32 = rest
                .parse()
                .map_err(|_| SmartHomeError::Protocol(format!("invalid power value: {rest:?}")))?;
            return Ok(TcpResponse::Power(watts));
        }
        if let Some(msg) = s.strip_prefix("ERROR ") {
            return Ok(TcpResponse::Error(msg.to_string()));
        }
        Err(SmartHomeError::Protocol(format!(
            "unrecognised response: {s:?}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: smart-home-tcp-udp, Property 9: TCP protocol encode/decode round-trip
    // Validates: Requirements 2.2, 2.3, 2.4
    proptest! {
        #[test]
        fn prop_command_round_trip(idx in 0usize..3) {
            let cmds = [TcpCommand::TurnOn, TcpCommand::TurnOff, TcpCommand::PowerQuery];
            let cmd = cmds[idx].clone();
            let encoded = cmd.encode();
            let decoded = TcpCommand::decode(&encoded).unwrap();
            prop_assert_eq!(cmd, decoded);
        }

        #[test]
        fn prop_response_ok_round_trip(_x in 0u8..1) {
            let resp = TcpResponse::Ok;
            let encoded = resp.encode();
            let decoded = TcpResponse::decode(&encoded).unwrap();
            prop_assert_eq!(resp, decoded);
        }

        #[test]
        fn prop_response_power_round_trip(w in -1e6f32..1e6f32) {
            // Skip NaN/Inf which don't round-trip through string formatting
            prop_assume!(w.is_finite());
            let resp = TcpResponse::Power(w);
            let encoded = resp.encode();
            let decoded = TcpResponse::decode(&encoded).unwrap();
            prop_assert_eq!(resp, decoded);
        }
    }
}
