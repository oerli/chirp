chirp! moist soil sensor with Rust (WIP)
========================================

This is my first attempt to write a I2C Rust driver for the chirp! the plant watering alarm https://wemakethings.net/chirp/

## TODO - WIP
- Capacitance not working yet properly
- Negative Temperature not tested (negative numbers)

## Setup Environment
Get and install the latest ARM tools from https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads
Get rustup from https://rustup.rs/ and install at least rust 1.31 and add the microbit target

    rustup install nightly
    rustup target add thumbv6m-none-eabi

## Build Example
Does work at the moment *only* in *release* mode and capacitance doesn't seem to work properly, debug hangs after temperature read:

    cargo build --example microbit --release
    arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/release/examples/microbit out.hex
    cp out.hex /Volumes/MICROBIT/

On macOS open the terminal and run the screen command with the microbit serial device

    screen /dev/cu.usbmodem14202 115200
