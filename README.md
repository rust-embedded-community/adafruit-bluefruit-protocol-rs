# Adafruit Bluefruit LE Connect Controller Protocol Parser
[![CI](https://github.com/rust-embedded-community/adafruit-bluefruit-protocol-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/rust-embedded-community/adafruit-bluefruit-protocol-rs/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/adafruit-bluefruit-protocol)](https://crates.io/crates/adafruit-bluefruit-protocol)
![Licenses](https://img.shields.io/crates/l/adafruit-bluefruit-protocol)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

This implements the [Adafruit Bluefruit LE Connect controller protocol](https://learn.adafruit.com/bluefruit-le-connect/controller)
which is e.g. used by the [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend).

Note that this work is not affiliated with Adafruit.

## Mandatory Features
This crate is `no_std` and you can choose whether you want to use
[`heapless::Vec`](https://docs.rs/heapless/0.8.0/heapless/struct.Vec.html) by selecting the feature `heapless` or
[`alloc::vec::Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html) by selecting the feature `alloc`.
If you select neither or both you'll get a compile error.

## Optional Features
* `defmt`: you can enable the [`defmt`](https://defmt.ferrous-systems.com/) feature to get a `defmt::Format` implementation for all structs & enums and a `defmt::debug!` call for each command being parsed.
* `rgb`: if enabled, `From<ColorEvent> for RGB8` is implemented to support the [RGB crate](https://crates.io/crates/rgb).
* `serde`: if enabled, all events implement the [serde](https://serde.rs/) `#[derive(Serialize, Deserialize)]`.
* All events can be selected as individual features. By default, they are all selected,
  but you can opt to only select the event(s) you are interested in which will result in a small binary size.
  If other events are received, a `ProtocolParseError::DisabledControllerDataPackageType` will be returned.

## Usage
The entry point to use this crate is the `parse` function. Note that this is a [sans I/O](https://sans-io.readthedocs.io/)
crate, i.e. you have to talk to the Adafruit device, the `parse` function just expects a byte sequence.

## Examples
A simple example for the STM32F4 microcontrollers is [available](examples/stm32f4-event-printer/README.md).

## Changelog
For the changelog please see the dedicated [CHANGELOG.md](CHANGELOG.md).

## Minimum Supported Rust Version (MSRV)
This crate is guaranteed to compile on stable Rust 1.81 and up. It *might*
compile with older versions but that may change in any new patch release.
