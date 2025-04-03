#!/bin/bash
# Helpful to read output when debugging
set -x
echo "=== $(date) ==="

VIDEO="c1_00_0"
VIDEO1="c1:00.0"
AUDIO="c1_00_1"
# AUDIO1="c1:00.1"
#RTC Wake Timer
TIME="+8sec"

# Stop display manager (Gnome specific)
systemctl stop gdm.service

# Unbind VTconsoles
echo 0 > /sys/class/vtconsole/vtcon0/bind
echo 0 > /sys/class/vtconsole/vtcon1/bind
# echo efi-framebuffer.0 > /sys/bus/platform/drivers/efi-framebuffer/unbind

# Syncing Disk and clearing The Caches(RAM)
# sync; echo 1 > /proc/sys/vm/drop_caches

# Un-Binding GPU From driver
sleep 2
for iommu_dev in "/sys/bus/pci/devices/0000:$VIDEO1/iommu_group/devices"/*; do
    basename "$iommu_dev" > "$iommu_dev/driver/unbind"
    echo "vfio-pci" > "$iommu_dev/driver_override"
done

modprobe -i vfio-pci

# Waiting for AMD GPU To Finish
# Loop Variables
declare -i Loop
Loop=1
declare -i TimeOut
TimeOut=5
while ! (dmesg | grep "amdgpu 0000:$VIDEO1" | tail -5 | grep "amdgpu: finishing device."); do 
    echo "Loop-1"; 
    if [ "$Loop" -le "$TimeOut" ]; then
        echo "Waiting";
        TimeOut+=1; 
        echo "Try: $TimeOut"; 
        sleep 1; 
    else break;
    fi; 
done

# Unbind the GPU from display driver
virsh nodedev-detach "pci_0000_$VIDEO"
sleep 1
virsh nodedev-detach "pci_0000_$AUDIO"

# Unload AMD drivers
modprobe -r --remove-dependencies amdgpu

# Reseting The Loop Counter
Loop=1
# Making Sure that AMD GPU is Un-Loaded
while (lsmod | grep amdgpu); do 
    echo "Loop-3"; 
    if [ "$Loop" -le "$TimeOut" ]; then
        echo "AMD GPU in use"; 
        lsmod | grep amdgpu | awk '{print $1}' | while read -r AM; do 
            modprobe -r "$AM"; 
        done;
        TimeOut+=1; 
        echo "AMDGPU try: $TimeOut"; 
        sleep 1; 
    else 
        echo "Fail To Remove AMD GPU";
        rmmod amdgpu; 
        break;
    fi;
done

# Garbage collection
unset Loop
unset TimeOut

#Putting System To a quick sleep cycle to make sure that amd graphic card is Properly reset 
rtcwake -m mem --date $TIME

echo "=== === ==="
