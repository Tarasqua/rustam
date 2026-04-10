//! UDP packet encoding/decoding for temperature values.
//!
//! A temperature packet is exactly 4 bytes: a big-endian IEEE 754 `f32`.

use crate::SmartHomeError;

/// A UDP packet carrying a single temperature reading (°C).
///
/// Wire format: 4 bytes, big-endian IEEE 754 `f32`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UdpTemperaturePacket(pub f32);

impl UdpTemperaturePacket {
    /// Encodes the temperature into a 4-byte big-endian array.
    pub fn encode(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Decodes a temperature from a byte slice.
    ///
    /// # Errors
    /// Returns [`SmartHomeError::Protocol`] if `bytes` is shorter than 4 bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, SmartHomeError> {
        if bytes.len() < 4 {
            return Err(SmartHomeError::Protocol(format!(
                "UDP packet too short: expected 4 bytes, got {}",
                bytes.len()
            )));
        }
        let arr: [u8; 4] = bytes[..4].try_into().unwrap();
        Ok(UdpTemperaturePacket(f32::from_be_bytes(arr)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: smart-home-tcp-udp, Property 8: UDP packet encode/decode round-trip
    // Validates: Requirements 4.2
    proptest! {
        #[test]
        fn prop_udp_packet_round_trip(bits in any::<u32>()) {
            let value = f32::from_bits(bits);
            // NaN does not equal itself, so skip NaN values
            prop_assume!(!value.is_nan());
            let packet = UdpTemperaturePacket(value);
            let encoded = packet.encode();
            let decoded = UdpTemperaturePacket::decode(&encoded).unwrap();
            prop_assert_eq!(packet.0.to_bits(), decoded.0.to_bits());
        }
    }

    // Validates: Requirements 4.2 — short packet returns Protocol error
    #[test]
    fn test_short_packet_returns_protocol_error() {
        let result = UdpTemperaturePacket::decode(&[0x01, 0x02]);
        assert!(matches!(result, Err(SmartHomeError::Protocol(_))));
    }
}
