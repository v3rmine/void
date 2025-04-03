#!/bin/bash
set -x

if test -e /etc/libvirt/ && ! test -e /etc/libvirt/hooks;
then
   mkdir -p /etc/libvirt/hooks;
fi
if test -e /etc/libvirt/hooks/qemu;
then
    mv /etc/libvirt/hooks/qemu /etc/libvirt/hooks/qemu.back
fi

cp -v fw-vfio-up.sh /bin/fw-vfio-up.sh
cp -v fw-vfio-down.sh /bin/fw-vfio-down.sh
cp -v qemu /etc/libvirt/hooks/qemu

chmod +x /bin/fw-vfio-up.sh
chmod +x /bin/fw-vfio-down.sh
chmod +x /etc/libvirt/hooks/qemu
