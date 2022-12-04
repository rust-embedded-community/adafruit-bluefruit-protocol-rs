//! Implements the [Controller Protocol](https://learn.adafruit.com/bluefruit-le-connect/controller) from Adafruit
//! which is e.g. used by the [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend).

#![forbid(unsafe_code)]
// use deny instead of forbid due to bogus warnings, see also https://github.com/rust-lang/rust/issues/81670
#![deny(warnings)]
#![deny(missing_docs)]
#![forbid(missing_debug_implementations)]
// use deny instead of forbid due to bogus warnings, see also https://github.com/rust-lang/rust/issues/81670
#![deny(unused)]
#![no_std]

pub mod accelerometer_event;
pub mod button_event;
pub mod color_event;
pub mod gyro_event;
pub mod location_event;
pub mod magnetometer_event;
pub mod quaternion_event;

use accelerometer_event::AccelerometerEvent;
use button_event::{ButtonEvent, ButtonParseError};
use color_event::ColorEvent;
use core::cmp::min;
use gyro_event::GyroEvent;
use heapless::Vec;
use location_event::LocationEvent;
use magnetometer_event::MagnetometerEvent;
use quaternion_event::QuaternionEvent;

/// Lists all (supported) events which can be sent by the controller. These come with the parsed event data and are the result of a [`parse`] call.
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
pub enum ControllerEvent {
    ButtonEvent(ButtonEvent),
    ColorEvent(ColorEvent),
    QuaternionEvent(QuaternionEvent),
    AccelerometerEvent(AccelerometerEvent),
    GyroEvent(GyroEvent),
    MagnetometerEvent(MagnetometerEvent),
    LocationEvent(LocationEvent),
}

/// Represents the different kinds of errors which can happen when the protocol is being parsed.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProtocolParseError {
    /// The message contained an event which is not known to the current implementation. This can either mean that the message was malformed or that a newer protocol version has been used.
    UnknownEvent(Option<u8>),
    /// An error occurred while parsing a [`ButtonEvent`].
    ButtonParseError(ButtonParseError),
    /// The event in the message did not have the expected length.
    InvalidLength(usize, usize),
    /// The event in the message did not have the expected CRC.
    InvalidCrc(u8, u16),
    /// There was a problem parsing a float from a message.
    InvalidFloatSize(usize),
}

/// Lists all data packages which can be sent by the controller. Internal state used during parsing. Use [`ControllerEvent`] to return the actual event.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)] // the names are already obvious enough
enum ControllerDataPackageType {
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

/// Parse the input string for commands. Unexpected content will be ignored if it's not formatted like a command!
pub fn parse<const MAX_RESULTS: usize>(
    input: &[u8],
) -> Vec<Result<ControllerEvent, ProtocolParseError>, MAX_RESULTS> {
    /// Simple state machine for the parser, represents whether the parser is seeking a start or has found it.
    enum ParserState {
        SeekStart,
        ParseCommand,
    }
    let mut state = ParserState::SeekStart;

    let mut result: Vec<Result<ControllerEvent, ProtocolParseError>, MAX_RESULTS> = Vec::new();

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
                result.push(data_package).ok();
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
            ButtonEvent::try_from(data).map(ControllerEvent::ButtonEvent)
        }
        ControllerDataPackageType::Color => {
            ColorEvent::try_from(data).map(ControllerEvent::ColorEvent)
        }
        ControllerDataPackageType::Quaternion => {
            QuaternionEvent::try_from(data).map(ControllerEvent::QuaternionEvent)
        }
        ControllerDataPackageType::Accelerometer => {
            AccelerometerEvent::try_from(data).map(ControllerEvent::AccelerometerEvent)
        }
        ControllerDataPackageType::Gyro => {
            GyroEvent::try_from(data).map(ControllerEvent::GyroEvent)
        }
        ControllerDataPackageType::Magnetometer => {
            MagnetometerEvent::try_from(data).map(ControllerEvent::MagnetometerEvent)
        }
        ControllerDataPackageType::Location => {
            LocationEvent::try_from(data).map(ControllerEvent::LocationEvent)
        }
    }
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
fn try_f32_from_le_bytes(input: &[u8]) -> Result<f32, ProtocolParseError> {
    Ok(f32::from_le_bytes(<[u8; 4]>::try_from(input).map_err(
        |_| ProtocolParseError::InvalidFloatSize(input.len()),
    )?))
}
