//! Implements the [`ColorEvent`] and its parsing from the protocol.

use super::ProtocolParseError;
#[cfg(feature = "rgb")]
use rgb::RGB8;

/// Represents a color event from the protocol.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)] // the names are already obvious enough
pub struct ColorEvent {
    red: u8,
    green: u8,
    blue: u8,
}

impl TryFrom<&[u8]> for ColorEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a color event.
    ///
    /// The full command is not validated here, identifying the command as a color event and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 3;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(ColorEvent {
                red: input[0],
                green: input[1],
                blue: input[2],
            })
        }
    }
}

#[allow(missing_docs)] // the names are already obvious enough
impl ColorEvent {
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}

#[cfg(feature = "rgb")]
impl Into<RGB8> for ColorEvent {
    fn into(self) -> RGB8 {
        RGB8 {
            r: self.red,
            g: self.green,
            b: self.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color_event::ColorEvent;
    #[cfg(feature = "rgb")]
    use rgb::RGB8;

    #[test]
    fn test_parse_color_event() {
        let input: &[u8] = b"\xff-9";
        let expected = ColorEvent {
            red: 255,
            green: 45,
            blue: 57,
        };

        assert_eq!(ColorEvent::try_from(input), Ok(expected));
    }

    #[test]
    #[cfg(feature = "rgb")]
    fn test_into_rgb8() {
        let input = ColorEvent {
            red: 1,
            green: 2,
            blue: 3,
        };
        let expected = RGB8 { r: 1, g: 2, b: 3 };

        let result: RGB8 = input.into();
        assert_eq!(result, expected);
    }
}
