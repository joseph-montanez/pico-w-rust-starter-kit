cmd /c "plink.exe -serial COM5 -sercfg 115200,8,n,1,N | defmt-print.exe -e target\thumbv6m-none-eabi\debug\pico-webapp"