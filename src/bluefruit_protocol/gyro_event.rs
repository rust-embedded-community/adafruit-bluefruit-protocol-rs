use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a gyro event from the protocol.
#[derive(Debug, defmt::Format)]
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
