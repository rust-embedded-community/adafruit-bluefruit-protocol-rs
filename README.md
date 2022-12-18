# Adafruit Bluefruit LE Connect Controller Protocol Parser
[![CI](https://github.com/rursprung/adafruit-bluefruit-protocol-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/rursprung/adafruit-bluefruit-protocol-rs/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/adafruit-bluefruit-protocol)](https://crates.io/crates/adafruit-bluefruit-protocol)
![Licenses](https://img.shields.io/crates/l/adafruit-bluefruit-protocol)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

This implements the [Adafruit Bluefruit LE Connect controller protocol](https://learn.adafruit.com/bluefruit-le-connect/controller)
which is e.g. used by the [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend).

Note that this work is not affiliated with Adafruit.

## Optional features
* `defmt`: you can enable the [`defmt`](https://defmt.ferrous-systems.com/) feature to get a `defmt::Format` implementation for all structs & enums and a `defmt::debug!` call for each command being parsed.
* `rgb`: if enabled, the `ColorEvent` implements `Into<RGB8>` for the [RGB crate](https://crates.io/crates/rgb).
* `serde`: if enabled, all events implement the [serde](https://serde.rs/) `#[derive(Serialize, Deserialize)]`.
* All events can be selected as individual features. By default, they are all selected,
  but you can opt to only select the event(s) you are interested in which will result in a small binary size.
  If other events are received, a `ProtocolParseError::DisabledControllerDataPackageType` will be returned.