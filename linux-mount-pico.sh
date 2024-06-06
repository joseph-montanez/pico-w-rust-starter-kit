#!/bin/bash

MOUNT_POINT="/media/joseph/RPI-RP2"
DEVICE_LABEL="RPI-RP2"

# Create mount point if it doesn't exist
if [ ! -d "$MOUNT_POINT" ]; then
    sudo mkdir -p "$MOUNT_POINT"
fi

# Loop through all /dev/sd* devices
for DEVICE in /dev/sd*; do
    # Skip devices that are not partitions (e.g., /dev/sda but not /dev/sda1)
    if [[ ! $DEVICE =~ [0-9]$ ]]; then
        continue
    fi

    # Get the label of the device
    LABEL=$(sudo blkid -o value -s LABEL "$DEVICE")

    # Check if the label matches RPI-RP2
    if [ "$LABEL" == "$DEVICE_LABEL" ]; then
        echo "Found device $DEVICE with label $DEVICE_LABEL"

        # Mount the device
        sudo mount -t vfat -o uid=1000,gid=1000 "$DEVICE" "$MOUNT_POINT"

        if [ $? -eq 0 ]; then
            echo "Successfully mounted $DEVICE to $MOUNT_POINT"
        else
            echo "Failed to mount $DEVICE"
        fi

        # Exit after mounting the first matching device
        exit 0
    fi
done

echo "No device with label $DEVICE_LABEL found"
exit 1
