use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a location event from the protocol.
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub struct LocationEvent {
    latitude: f32,
    longitude: f32,
    altitude: f32,
}

impl TryFrom<&[u8]> for LocationEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a location event.
    ///
    /// The full command is not validated here, identifying the command as a location event and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 3 * super::BYTES_PER_FLOAT;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(LocationEvent {
                latitude: try_f32_from_le_bytes(&input[0..4])?,
                longitude: try_f32_from_le_bytes(&input[4..8])?,
                altitude: try_f32_from_le_bytes(&input[8..12])?,
            })
        }
    }
}

#[allow(missing_docs)] // the names are already obvious enough
impl LocationEvent {
    pub fn latitude(&self) -> f32 {
        self.latitude
    }

    pub fn longitude(&self) -> f32 {
        self.longitude
    }

    pub fn altitude(&self) -> f32 {
        self.altitude
    }
}
