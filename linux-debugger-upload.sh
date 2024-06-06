#!/bin/bash

echo "Compiling"
cargo build --target=thumbv6m-none-eabi

sudo openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000" -c "program target/thumbv6m-none-eabi/debug/pico-webapp verify reset exit"