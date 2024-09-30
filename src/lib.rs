//! This implements the [Adafruit Bluefruit LE Connect controller protocol](https://learn.adafruit.com/bluefruit-le-connect/controller)
//! which is e.g. used by the [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend).
//!
//! The entry point to use this crate is the [`parse`] function.
//!
//! ## Optional features
//! * `defmt`: you can enable the `defmt` feature to get a `defmt::Format` implementation for all structs & enums and a `defmt::debug!` call for each command being parsed.
//! * `rgb`: if enabled, `From<ColorEvent> for RGB8` is implemented to support the [RGB crate](https://crates.io/crates/rgb).
//! * `serde`: if enabled, all events implement the [serde](https://serde.rs/) `#[derive(Serialize, Deserialize)]`.
//! * All events can be selected as individual features. By default, they are all selected,
//!   but you can opt to only select the event(s) you are interested in which will result in a small binary size.
//!   If other events are received, a [`ProtocolParseError::DisabledControllerDataPackageType`] will be returned.

#![forbid(unsafe_code)]
// use deny instead of forbid due to bogus warnings, see also https://github.com/rust-lang/rust/issues/81670
#![deny(warnings)]
#![deny(missing_docs)]
#![forbid(missing_debug_implementations)]
// use deny instead of forbid because try_f32_from_le_bytes might be unused depending on the selected features
#![deny(unused)]
#![no_std]

#[cfg(not(any(
    feature = "accelerometer_event",
    feature = "button_event",
    feature = "color_event",
    feature = "gyro_event",
    feature = "location_event",
    feature = "magnetometer_event",
    feature = "quaternion_event"
)))]
compile_error!("at least one event type must be selected in the features!");

#[cfg(not(any(feature = "alloc", feature = "heapless")))]
compile_error!("you must choose either 'alloc' or 'heapless' as a feature!");

#[cfg(all(feature = "alloc", feature = "heapless"))]
compile_error!("you must choose either 'alloc' or 'heapless' as a feature but not both!");

#[cfg(feature = "accelerometer_event")]
pub mod accelerometer_event;
#[cfg(feature = "button_event")]
pub mod button_event;
#[cfg(feature = "color_event")]
pub mod color_event;
#[cfg(feature = "gyro_event")]
pub mod gyro_event;
#[cfg(feature = "location_event")]
pub mod location_event;
#[cfg(feature = "magnetometer_event")]
pub mod magnetometer_event;
#[cfg(feature = "quaternion_event")]
pub mod quaternion_event;

#[cfg(feature = "accelerometer_event")]
use accelerometer_event::AccelerometerEvent;
#[cfg(feature = "button_event")]
use button_event::{ButtonEvent, ButtonParseError};
#[cfg(feature = "color_event")]
use color_event::ColorEvent;
use core::cmp::min;
#[cfg(feature = "gyro_event")]
use gyro_event::GyroEvent;
#[cfg(feature = "heapless")]
use heapless::Vec;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{Display, Formatter};
#[cfg(feature = "location_event")]
use location_event::LocationEvent;
#[cfg(feature = "magnetometer_event")]
use magnetometer_event::MagnetometerEvent;
#[cfg(feature = "quaternion_event")]
use quaternion_event::QuaternionEvent;

/// Lists all (supported) events which can be sent by the controller. These come with the parsed event data and are the result of a [`parse`] call.
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub enum ControllerEvent {
    #[cfg(feature = "button_event")]
    ButtonEvent(ButtonEvent),
    #[cfg(feature = "color_event")]
    ColorEvent(ColorEvent),
    #[cfg(feature = "quaternion_event")]
    QuaternionEvent(QuaternionEvent),
    #[cfg(feature = "accelerometer_event")]
    AccelerometerEvent(AccelerometerEvent),
    #[cfg(feature = "gyro_event")]
    GyroEvent(GyroEvent),
    #[cfg(feature = "magnetometer_event")]
    MagnetometerEvent(MagnetometerEvent),
    #[cfg(feature = "location_event")]
    LocationEvent(LocationEvent),
}

/// Represents the different kinds of errors which can happen when the protocol is being parsed.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProtocolParseError {
    /// The message contained an event which is not known to the current implementation.
    /// This can mean that:
    /// * the message was malformed or
    /// * that a newer protocol version has been used.
    UnknownEvent(Option<u8>),
    /// The message contained an event which is known to the library but has not been selected as a feature and can thus not be parsed. Select the feature when compiling the library to handle this message.
    DisabledControllerDataPackageType(ControllerDataPackageType),
    /// An error occurred while parsing a [`ButtonEvent`].
    #[cfg(feature = "button_event")]
    ButtonParseError(ButtonParseError),
    /// The event in the message did not have the expected length. The first value is the expected length, the second the actual length.
    InvalidLength(usize, usize),
    /// The event in the message did not have the expected CRC. The first value is the expected CRC, the second the actual CRC.
    InvalidCrc(u8, u16),
    /// There was a problem parsing a float from a message. The parameter gives the length of the received input.
    InvalidFloatSize(usize),
}

impl Display for ProtocolParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use ProtocolParseError::*;
        match self {
            UnknownEvent(event) => write!(f, "Unknown event type: {:?}", event),
            DisabledControllerDataPackageType(event) => {
                write!(f, "Disabled event type: {:?}", event)
            }
            ButtonParseError(_) => write!(f, "Error while parsing button event"),
            InvalidLength(expected, actual) => write!(
                f,
                "Invalid message length: expected {} but received {}",
                expected, actual
            ),
            InvalidCrc(expected, actual) => write!(
                f,
                "Invalid CRC: expected {:#x} but calculated {:#x}",
                expected, actual
            ),
            InvalidFloatSize(length) => write!(
                f,
                "Failed to parse float from a message with size {}",
                length
            ),
        }
    }
}

impl Error for ProtocolParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use ProtocolParseError::*;
        match self {
            ButtonParseError(e) => Some(e),
            _ => None,
        }
    }
}

/// Lists all data packages which can be sent by the controller. Internal state used during parsing. Use [`ControllerEvent`] to return the actual event.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub enum ControllerDataPackageType {
    ButtonCommand,
    Color,
    Quaternion,
    Accelerometer,
    Gyro,
    Magnetometer,
    Location,
}

const BYTES_PER_FLOAT: usize = 4;

/// Maximum length of a controller message which must be tolerated by any caller.
pub const MAX_CONTROLLER_MESSAGE_LENGTH: usize = 32; // give slightly more than necessary to avoid cutting off an unexpected message

impl ControllerDataPackageType {
    /// Returns the length of the data section of the command.
    fn data_len(&self) -> usize {
        match self {
            ControllerDataPackageType::ButtonCommand => 2,
            ControllerDataPackageType::Color => 3,
            ControllerDataPackageType::Quaternion => 4 * BYTES_PER_FLOAT,
            ControllerDataPackageType::Accelerometer => 3 * BYTES_PER_FLOAT,
            ControllerDataPackageType::Gyro => 3 * BYTES_PER_FLOAT,
            ControllerDataPackageType::Magnetometer => 3 * BYTES_PER_FLOAT,
            ControllerDataPackageType::Location => 3 * BYTES_PER_FLOAT,
        }
    }
}

impl TryFrom<u8> for ControllerDataPackageType {
    type Error = ProtocolParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'B' => Ok(ControllerDataPackageType::ButtonCommand),
            b'C' => Ok(ControllerDataPackageType::Color),
            b'Q' => Ok(ControllerDataPackageType::Quaternion),
            b'A' => Ok(ControllerDataPackageType::Accelerometer),
            b'G' => Ok(ControllerDataPackageType::Gyro),
            b'M' => Ok(ControllerDataPackageType::Magnetometer),
            b'L' => Ok(ControllerDataPackageType::Location),
            _ => Err(ProtocolParseError::UnknownEvent(Some(value))),
        }
    }
}

#[cfg(feature = "heapless")]
type ParseResult<const MAX_RESULTS: usize> =
    Vec<Result<ControllerEvent, ProtocolParseError>, MAX_RESULTS>;

#[cfg(feature = "alloc")]
type ParseResult<const MAX_RESULTS: usize> = Vec<Result<ControllerEvent, ProtocolParseError>>;
#[cfg(feature = "alloc")]
const MAX_RESULTS: usize = 0;

/// Parse the input string for commands. Unexpected content will be ignored if it's not formatted like a command!
pub fn parse<#[cfg(feature = "heapless")] const MAX_RESULTS: usize>(
    input: &[u8],
) -> ParseResult<MAX_RESULTS> {
    /// Simple state machine for the parser, represents whether the parser is seeking a start or has found it.
    enum ParserState {
        SeekStart,
        ParseCommand,
    }
    let mut state = ParserState::SeekStart;

    let mut result = Vec::new();

    for pos in 0..input.len() {
        let byte = input[pos];
        match state {
            ParserState::SeekStart => {
                if byte == b'!' {
                    state = ParserState::ParseCommand
                }
            }
            ParserState::ParseCommand => {
                let data_package = extract_and_parse_command(&input[(pos - 1)..]);
                #[cfg(feature = "alloc")]
                result.push(data_package);
                #[cfg(feature = "heapless")]
                result.push(data_package).ok();
                #[cfg(feature = "heapless")]
                if result.len() == MAX_RESULTS {
                    return result;
                }
                state = ParserState::SeekStart;
            }
        };
    }

    result
}

/// Extract a command and then try to parse it.
fn extract_and_parse_command(input: &[u8]) -> Result<ControllerEvent, ProtocolParseError> {
    let command = ControllerDataPackageType::try_from(input[1])?;
    let command_end = min(command.data_len() + 2, input.len() - 1);
    parse_command(command, &input[..=command_end])
}

/// Parse a command (which has previously been extracted by [`parse`]).
fn parse_command(
    command: ControllerDataPackageType,
    command_input: &[u8],
) -> Result<ControllerEvent, ProtocolParseError> {
    #[cfg(feature = "defmt")]
    defmt::debug!(
        "parsing the command of type {} from message {:a}",
        command,
        command_input
    );

    // validate the length of the received command
    let len = command_input.len();
    let expected_len = command.data_len() + 3; // ! + command + data + CRC
    if len != expected_len {
        return Err(ProtocolParseError::InvalidLength(expected_len, len));
    }

    let data_start: usize = 2; // skip ! + command
    let data_end = len - 2;

    // CRC validation (done before dealing with the command as this prevents wasting time if it's invalid)
    let crc = &command_input[len - 1];
    check_crc(&command_input[..=data_end], crc)?;

    // parse the actual command based on its type
    let data = &command_input[data_start..=data_end];
    match command {
        ControllerDataPackageType::ButtonCommand => {
            #[cfg(feature = "button_event")]
            return ButtonEvent::try_from(data).map(ControllerEvent::ButtonEvent);
            #[cfg(not(feature = "button_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
        ControllerDataPackageType::Color => {
            #[cfg(feature = "color_event")]
            return ColorEvent::try_from(data).map(ControllerEvent::ColorEvent);
            #[cfg(not(feature = "color_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
        ControllerDataPackageType::Quaternion => {
            #[cfg(feature = "quaternion_event")]
            return QuaternionEvent::try_from(data).map(ControllerEvent::QuaternionEvent);
            #[cfg(not(feature = "quaternion_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
        ControllerDataPackageType::Accelerometer => {
            #[cfg(feature = "accelerometer_event")]
            return AccelerometerEvent::try_from(data).map(ControllerEvent::AccelerometerEvent);
            #[cfg(not(feature = "accelerometer_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
        ControllerDataPackageType::Gyro => {
            #[cfg(feature = "gyro_event")]
            return GyroEvent::try_from(data).map(ControllerEvent::GyroEvent);
            #[cfg(not(feature = "gyro_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
        ControllerDataPackageType::Magnetometer => {
            #[cfg(feature = "magnetometer_event")]
            return MagnetometerEvent::try_from(data).map(ControllerEvent::MagnetometerEvent);
            #[cfg(not(feature = "magnetometer_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
        ControllerDataPackageType::Location => {
            #[cfg(feature = "location_event")]
            return LocationEvent::try_from(data).map(ControllerEvent::LocationEvent);
            #[cfg(not(feature = "location_event"))]
            return Err(ProtocolParseError::DisabledControllerDataPackageType(
                command,
            ));
        }
    };
}

/// Check the CRC of a command
fn check_crc(data: &[u8], crc: &u8) -> Result<(), ProtocolParseError> {
    #[cfg(feature = "defmt")]
    defmt::trace!("calculating CRC for {:a}, expecting {}", data, crc);

    let mut sum: u16 = 0;
    for byte in data {
        sum += *byte as u16;
    }

    let calculated_crc = !sum & 0xff;

    if *crc as u16 == calculated_crc {
        Ok(())
    } else {
        Err(ProtocolParseError::InvalidCrc(*crc, calculated_crc))
    }
}

/// Small wrapper to convert the 4-byte value to an `f32` and handle the error.
#[allow(unused)] // can be unused if no event which needs this has been selected as a feature.
fn try_f32_from_le_bytes(input: &[u8]) -> Result<f32, ProtocolParseError> {
    Ok(f32::from_le_bytes(<[u8; 4]>::try_from(input).map_err(
        |_| ProtocolParseError::InvalidFloatSize(input.len()),
    )?))
}

#[cfg(test)]
mod tests {
    use crate::button_event::{Button, ButtonParseError, ButtonState};
    use crate::{check_crc, parse, try_f32_from_le_bytes, ControllerEvent, ProtocolParseError};

    fn assert_is_button_event(
        event: &Result<ControllerEvent, ProtocolParseError>,
        button: Button,
        button_state: ButtonState,
    ) {
        match event {
            Ok(ControllerEvent::ButtonEvent(event)) => {
                assert_eq!(event.button(), &button);
                assert_eq!(event.state(), &button_state)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse() {
        let input = b"\x00!B11:!B10;\x00\x00!\x00\x00\x00\x00!B138";
        #[cfg(feature = "heapless")]
        let result = parse::<4>(input);
        #[cfg(feature = "alloc")]
        let result = parse(input);

        assert_eq!(result.len(), 4);
        assert_is_button_event(&result[0], Button::Button1, ButtonState::Pressed);
        assert_is_button_event(&result[1], Button::Button1, ButtonState::Released);
        assert_eq!(result[2], Err(ProtocolParseError::UnknownEvent(Some(0))));
        if let Err(e) = &result[3] {
            assert_eq!(
                e,
                &ProtocolParseError::ButtonParseError(ButtonParseError::UnknownButtonState(b'3'))
            );
            #[cfg(feature = "alloc")]
            {
                use alloc::string::ToString;
                use core::error::Error;
                assert_eq!(
                    e.source().unwrap().to_string(),
                    "Unknown button state: 0x33"
                );
            }
        } else {
            assert!(false, "expected an error");
        }
    }

    #[test]
    fn test_check_crc_ok() {
        let input = b"!B11:";
        let data = &input[0..input.len() - 1];
        let crc = input.last().unwrap();

        assert!(check_crc(data, &crc).is_ok());
    }

    #[test]
    fn test_check_crc_err() {
        let input = b"!B11;"; // should either be "!B11:" or "!B10;"
        let correct_crc = b':';
        let data = &input[0..input.len() - 1];
        let crc = input.last().unwrap();

        assert_eq!(
            check_crc(data, &crc),
            Err(ProtocolParseError::InvalidCrc(*crc, correct_crc as u16))
        );
    }

    #[test]
    fn test_try_f32_from_le_bytes_ok() {
        let input = b"9\x1e\x0c\xc0";
        let expected: f32 = -2.1893446;

        assert_eq!(try_f32_from_le_bytes(input), Ok(expected));
    }

    #[test]
    fn test_try_f32_from_le_bytes_err() {
        let input = b"\x1e\x0c\xc0";

        assert_eq!(
            try_f32_from_le_bytes(input),
            Err(ProtocolParseError::InvalidFloatSize(3))
        );
    }
}
