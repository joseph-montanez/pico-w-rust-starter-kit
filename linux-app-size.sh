#!/bin/bash

# Check if an argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <path_to_elf_file>"
    exit 1
fi

ELF_FILE=$1

# Check if the file exists
if [ ! -f "$ELF_FILE" ]; then
    echo "Error: File '$ELF_FILE' not found!"
    exit 1
fi

# Get the size information
SIZE_OUTPUT=$(arm-none-eabi-size "$ELF_FILE")

# Extract text, data, and bss sizes
TEXT_SIZE=$(echo "$SIZE_OUTPUT" | awk 'NR==2 {print $1}')
DATA_SIZE=$(echo "$SIZE_OUTPUT" | awk 'NR==2 {print $2}')
BSS_SIZE=$(echo "$SIZE_OUTPUT" | awk 'NR==2 {print $3}')

# Calculate flash and RAM usage
FLASH_USAGE=$((TEXT_SIZE + DATA_SIZE))
RAM_USAGE=$((DATA_SIZE + BSS_SIZE))

# Convert to KB
FLASH_USAGE_KB=$(echo "scale=2; $FLASH_USAGE / 1024" | bc)
RAM_USAGE_KB=$(echo "scale=2; $RAM_USAGE / 1024" | bc)

# Display the results
echo "Flash memory usage: $FLASH_USAGE bytes ($FLASH_USAGE_KB KB)"
echo "RAM usage: $RAM_USAGE bytes ($RAM_USAGE_KB KB)"