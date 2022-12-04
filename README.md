# Adafruit Bluefruit LE Connect Controller Protocol Parser
[![CI](https://github.com/rursprung/adafruit-bluefruit-protocol-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/rursprung/adafruit-bluefruit-protocol-rs/actions/workflows/CI.yml)

This implements the [Adafruit Bluefruit LE Connect controller protocol](https://learn.adafruit.com/bluefruit-le-connect/controller)
which is e.g. used by the [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend).

Note that this work is not affiliated with Adafruit.

## Optional features
* `defmt`: you can enable the [`defmt`](https://defmt.ferrous-systems.com/) feature to get a `defmt::Format` implementation for all structs & enums and a `defmt::debug!` call for each command being parsed.
* `rgb`: if enabled, the `ColorEvent` implements `Into<RGB8>` for the [RGB crate](https://crates.io/crates/rgb).
