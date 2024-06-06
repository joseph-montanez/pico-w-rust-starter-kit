$env:OPENOCD_SCRIPTS = "C:\Program Files\Raspberry Pi\Pico SDK v1.5.1\openocd\scripts"
& "C:\Program Files\Raspberry Pi\Pico SDK v1.5.1\openocd\openocd.exe" --% -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000"
