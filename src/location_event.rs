//! Implements the [`LocationEvent`] and its parsing from the protocol.

use super::{try_f32_from_le_bytes, ProtocolParseError};

/// Represents a location event from the protocol.
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod tests {
    use crate::location_event::LocationEvent;

    #[test]
    fn test_parse_gyro_event() {
        // as an exception this is not done using a real-world string received via bluetooth as I don't have a mock GPS app installed and don't want my real location in the source code :)
        let expected_lat = 1.2f32;
        let expected_lon = 2.3f32;
        let expected_alt = 3.4f32;
        let input = [
            expected_lat.to_le_bytes(),
            expected_lon.to_le_bytes(),
            expected_alt.to_le_bytes(),
        ]
        .concat();

        let expected = LocationEvent {
            latitude: expected_lat,
            longitude: expected_lon,
            altitude: expected_alt,
        };

        assert_eq!(LocationEvent::try_from(input.as_slice()), Ok(expected));
    }
}
