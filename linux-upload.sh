#!/bin/bash

echo "Compiling"
cargo build --release

# Mounting
./linux-mount-pico.sh

# Convert to
echo "Converting ELF to UF2"
elf2uf2-rs ./target/thumbv6m-none-eabi/release/pico-webapp /media/joseph/RPI-RP2/pico-webapp
echo "Done!"