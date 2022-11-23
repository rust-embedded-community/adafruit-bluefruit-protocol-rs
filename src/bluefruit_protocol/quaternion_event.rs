use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a quaternion event from the protocol.
#[derive(Debug, defmt::Format)]
pub struct QuaternionEvent {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl TryFrom<&[u8]> for QuaternionEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a quaternion event.
    ///
    /// The full command is not validated here, identifying the command as a quaternion event and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 4 * super::BYTES_PER_FLOAT;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(QuaternionEvent {
                x: try_f32_from_le_bytes(&input[0..4])?,
                y: try_f32_from_le_bytes(&input[4..8])?,
                z: try_f32_from_le_bytes(&input[8..12])?,
                w: try_f32_from_le_bytes(&input[12..16])?,
            })
        }
    }
}
