//! Implements the [`MagnetometerEvent`] and its parsing from the protocol.

use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a magnetometer event from the protocol.
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub struct MagnetometerEvent {
    x: f32,
    y: f32,
    z: f32,
}

impl TryFrom<&[u8]> for MagnetometerEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a magnetometer event.
    ///
    /// The full command is not validated here, identifying the command as a magnetometer event and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 3 * super::BYTES_PER_FLOAT;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(MagnetometerEvent {
                x: try_f32_from_le_bytes(&input[0..4])?,
                y: try_f32_from_le_bytes(&input[4..8])?,
                z: try_f32_from_le_bytes(&input[8..12])?,
            })
        }
    }
}

#[allow(missing_docs)] // the names are already obvious enough
impl MagnetometerEvent {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}
