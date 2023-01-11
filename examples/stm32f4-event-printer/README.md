# Bluetooth Receiver Using Rust on NUCLEO-F401RE ARM32 Board
This example showcases how the [adafruit-bluefruit-protocol](https://crates.io/crates/adafruit-bluefruit-protocol) crate can be used
with an [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend) on an STM32F4 chip.
It uses [RTIC](https://rtic.rs/) (Real-Time Interrupt-driven Concurrency) underneath.

The example logs all events (and other messages) using [`defmt`](https://defmt.ferrous-systems.com/).
After the application has started you can connect using the Adafruit smartphone app and send any data packages and they will be printed using `defmt`.

The example has been tested on a [ST Nucleo-F401RE](https://www.st.com/en/evaluation-tools/nucleo-f401re.html) development
board but should work on any STM32F4xx family microcontroller as long as the bluetooth module is connected on `USART1` using the following pins (or the code adapted accordingly):
* `TX` (of the microcontroller - this is RX for the bluetooth module!) on `PB6`
* `RX` (of the microcontroller - this is TX for the bluetooth module!) on `PB10`

## Prerequisites
1. Optional: ensure that the rust toolchain is up-to-date: `rustup update`
1. Install `probe-run`: `cargo install probe-run`
1. Install `flip-link`: `cargo install flip-link`
    * Note: `flip-link` is not strictly necessary for this example (it doesn't need
      stack protection), however it can be considered best practices to include it.
1. Install the cross-compile target: `rustup target add thumbv7em-none-eabihf`
1. Optional: install the LLVM tools: `rustup component add llvm-tools-preview`
1. Install the STLink drivers

## Build & Download to Board
1. Connect the board via USB
2. Run `cargo run` (the correct chip & target is already defined in `Cargo.toml` and `.cargo/config`)
3. Enjoy your running program :)
