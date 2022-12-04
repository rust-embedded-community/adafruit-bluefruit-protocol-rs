use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a [quaternion](https://en.wikipedia.org/wiki/Quaternion) event from the protocol.
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
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

#[allow(missing_docs)] // the names are already obvious enough
impl QuaternionEvent {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }
}
