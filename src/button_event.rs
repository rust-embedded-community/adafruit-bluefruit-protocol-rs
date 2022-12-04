use super::ProtocolParseError;

/// Errors which can be raised while parsing a button event.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ButtonParseError {
    UnknownButton(u8),
    UnknownButtonState(u8),
}

/// Lists all possible buttons which can be sent in the event.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

impl ButtonEvent {
    pub fn button(&self) -> &Button {
        &self.button
    }
    pub fn state(&self) -> &ButtonState {
        &self.state
    }
}
