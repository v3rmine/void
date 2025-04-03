#!/bin/bash
set -xe
echo "=== $(date) ==="

VIDEO1="c1:00.0"
#AUDIO1="c1:00.1"
#RTC Wake Timer
TIME="+8sec"

# Re-Bind GPU to AMD Driver
for iommu_dev in "/sys/bus/pci/devices/0000:$VIDEO1/iommu_group/devices"/*; do
    basename "$iommu_dev" > "$iommu_dev/remove"
done

#Load amd driver
modprobe amdgpu 

#Putting System To a quick sleep cycle to make sure that amd graphic card is Properly reset 
rtcwake -m mem --date $TIME

# Unload stub driver
modprobe -r --remove-dependencies vfio-pci

sleep 3
echo "1" | tee -a /sys/bus/pci/rescan

# Rebind VT consoles
echo 1 > /sys/class/vtconsole/vtcon0/bind
echo 1 > /sys/class/vtconsole/vtcon1/bind
echo efi-framebuffer.0 > /sys/bus/platform/drivers/efi-framebuffer/bind

# Restart Display Manager
systemctl start gdm.service

echo "=== === ==="
