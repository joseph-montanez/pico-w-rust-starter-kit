sudo socat /dev/ttyACM0,rawer,b115200 STDOUT | defmt-print -e target/thumbv6m-none-eabi/debug/pico-webapp