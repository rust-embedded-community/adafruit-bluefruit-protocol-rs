//! Implements the [`ButtonEvent`] and its parsing from the protocol.

use super::ProtocolParseError;
use core::error::Error;
use core::fmt::{Display, Formatter};

/// Errors which can be raised while parsing a button event.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ButtonParseError {
    /// The message contained an unknown button. For the known buttons see [`Button`].
    UnknownButton(u8),
    /// The message contained an unknown button state. For the known button states see [`ButtonState`].
    UnknownButtonState(u8),
}

impl Display for ButtonParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use ButtonParseError::*;
        match self {
            UnknownButton(button) => write!(f, "Unknown button: {:#x}", button),
            UnknownButtonState(state) => write!(f, "Unknown button state: {:#x}", state),
        }
    }
}

impl Error for ButtonParseError {}

/// Lists all possible buttons which can be sent in the event.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub enum Button {
    Button1,
    Button2,
    Button3,
    Button4,
    Up,
    Down,
    Left,
    Right,
}

impl Button {
    /// Maps the ID in the protocol to the [`Button`].
    pub fn from_id(input: &u8) -> Result<Button, ButtonParseError> {
        match input {
            b'1' => Ok(Button::Button1),
            b'2' => Ok(Button::Button2),
            b'3' => Ok(Button::Button3),
            b'4' => Ok(Button::Button4),
            b'5' => Ok(Button::Up),
            b'6' => Ok(Button::Down),
            b'7' => Ok(Button::Left),
            b'8' => Ok(Button::Right),
            _ => Err(ButtonParseError::UnknownButton(*input)),
        }
    }
}

/// The state of the button.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)] // the names are already obvious enough
pub enum ButtonState {
    Released,
    Pressed,
}

impl ButtonState {
    /// Maps the ID in the protocol to the [`ButtonState`].
    pub fn from_id(input: &u8) -> Result<ButtonState, ButtonParseError> {
        match input {
            b'0' => Ok(ButtonState::Released),
            b'1' => Ok(ButtonState::Pressed),
            _ => Err(ButtonParseError::UnknownButtonState(*input)),
        }
    }
}

/// Represents a button event from the protocol.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub struct ButtonEvent {
    button: Button,
    state: ButtonState,
}

impl TryFrom<&[u8]> for ButtonEvent {
    type Error = ProtocolParseError;

    /// Parse the data section of a button command.
    ///
    /// The full command is not validated here, identifying the command as a button command and CRC validation is the responsibility of the caller!
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let expected_len = 2;
        if input.len() != expected_len {
            Err(ProtocolParseError::InvalidLength(expected_len, input.len()))
        } else {
            Ok(ButtonEvent {
                button: Button::from_id(&input[0]).map_err(ProtocolParseError::ButtonParseError)?,
                state: ButtonState::from_id(&input[1])
                    .map_err(ProtocolParseError::ButtonParseError)?,
            })
        }
    }
}

#[allow(missing_docs)] // the names are already obvious enough
impl ButtonEvent {
    pub fn button(&self) -> &Button {
        &self.button
    }
    pub fn state(&self) -> &ButtonState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use crate::button_event::{Button, ButtonEvent, ButtonParseError, ButtonState};
    use crate::ProtocolParseError;

    fn assert_is_button_event(
        result: &Result<ButtonEvent, ProtocolParseError>,
        button: Button,
        button_state: ButtonState,
    ) {
        match result {
            Ok(event) => {
                assert_eq!(event.button(), &button);
                assert_eq!(event.state(), &button_state)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_button1_pressed_event() {
        let input: &[u8] = b"11";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Button1,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button1_released_event() {
        let input: &[u8] = b"11";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Button1,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button2_pressed_event() {
        let input: &[u8] = b"21";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Button2,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button3_pressed_event() {
        let input: &[u8] = b"31";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Button3,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button4_pressed_event() {
        let input: &[u8] = b"41";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Button4,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button_up_pressed_event() {
        let input: &[u8] = b"51";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Up,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button_down_pressed_event() {
        let input: &[u8] = b"61";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Down,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button_left_pressed_event() {
        let input: &[u8] = b"71";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Left,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_button_right_pressed_event() {
        let input: &[u8] = b"81";
        assert_is_button_event(
            &ButtonEvent::try_from(input),
            Button::Right,
            ButtonState::Pressed,
        );
    }

    #[test]
    fn test_parse_invalid_button() {
        let input: &[u8] = b"01";
        assert_eq!(
            ButtonEvent::try_from(input),
            Err(ProtocolParseError::ButtonParseError(
                ButtonParseError::UnknownButton(b'0')
            ))
        );
    }

    #[test]
    fn test_parse_invalid_button_state() {
        let input: &[u8] = b"13";
        assert_eq!(
            ButtonEvent::try_from(input),
            Err(ProtocolParseError::ButtonParseError(
                ButtonParseError::UnknownButtonState(b'3')
            ))
        );
    }
}
