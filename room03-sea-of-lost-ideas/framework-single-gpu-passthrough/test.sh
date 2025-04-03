#!/bin/bash
set -x
# Stop display manager (Gnome specific)
systemctl stop gdm.service

while systemctl is-active --quiet gdm.service; do
   sleep 5
done
# Unbind VTconsoles
# Unbind VTconsoles if currently bound (adapted from https://www.kernel.org/doc/Documentation/fb/fbcon.txt)
if test -e "/tmp/vfio-bound-consoles" ; then
    rm -f /tmp/vfio-bound-consoles
fi
for (( i = 0; i < 16; i++))
do
  if test -x /sys/class/vtconsole/vtcon${i}; then
      if cat /sys/class/vtconsole/vtcon${i}/name | grep -q "frame buffer"; then
	      echo 0 > /sys/class/vtconsole/vtcon${i}/bind
         echo "Unbinding console ${i}"
      fi
  fi
done

modprobe -r amdgpu

devs="0000_c1_00_1 0000_c1_00_0"
for iommu_dev in $devs; do
   virsh nodedev-detach "pci_$iommu_dev"
   #echo "vfio-pci" > "/sys/bus/pci/devices/$iommu_dev/driver_override"
   #echo "$iommu_dev" > "/sys/bus/pci/devices/$iommu_dev/driver/unbind"
   sleep 1
done

modprobe vfio-pci
modprobe vfio
modprobe vfio_iommu_type1
