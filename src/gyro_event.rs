//! Implements the [`GyroEvent`] and its parsing from the protocol.

use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a gyro event from the protocol.
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)] // the names are already obvious enough
pub struct GyroEvent {
    x: f32,
    y: f32,
    z: f32,
}

impl TryFrom<&[u8]> for GyroEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a gyro event.
    ///
    /// The full command is not validated here, identifying the command as a gyro event and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 3 * super::BYTES_PER_FLOAT;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(GyroEvent {
                x: try_f32_from_le_bytes(&input[0..4])?,
                y: try_f32_from_le_bytes(&input[4..8])?,
                z: try_f32_from_le_bytes(&input[8..12])?,
            })
        }
    }
}

#[allow(missing_docs)] // the names are already obvious enough
impl GyroEvent {
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

#[cfg(test)]
mod tests {
    use crate::gyro_event::GyroEvent;

    #[test]
    fn test_parse_gyro_event() {
        let input: &[u8] = b"H.\xd7;\x0c\xb2\xe8<z\xe62\xbd";
        let expected = GyroEvent {
            x: 0.0065667965,
            y: 0.028405212,
            z: -0.04367683,
        };

        assert_eq!(GyroEvent::try_from(input), Ok(expected));
    }
}
