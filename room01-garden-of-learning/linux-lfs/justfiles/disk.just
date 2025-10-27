set working-directory := ".."

LFS := "/mnt/lfs"

# Display this help screen
help:
    @just --list disk --list-submodules

# Create a qcow2 virtual disk
create-virtual-disk size="10G" file="lfs.qcow2":
    qemu-img create -f qcow2 "{{ file }}" "{{ size }}"

# Enable nbd module
enable-nbd max_part="4":
    modprobe nbd max_part="{{ max_part }}"

# Disable nbd module
disable-nbd:
    rmmod nbd

# Connect nbd virtual disk
connect-virtual-disk file="lfs.qcow2" dev="/dev/nbd1":
    qemu-nbd --connect="{{ dev }}" "{{ file }}"

# Disconnect nbd virtual disk
disconnect-virtual-disk dev="/dev/nbd1":
    qemu-nbd --disconnect "{{ dev }}"

# Format lfs device and create partitions
format-disk dev="/dev/nbd1":
    #!/usr/bin/env bash
    set -euxo pipefail

    sgdisk --clear --mbrtogpt "{{ dev }}"
    # Get the codes using "sgdisk -L"
    sgdisk -n 1:2048:4095 -c 1:"BIOS Boot Partition" -t 1:ef02 "{{ dev }}"
    sgdisk -n 2:0:+100M -c 2:"EFI System Partition" -t 2:ef00 "{{ dev }}"
    sgdisk -n 3:0:+200M -c 3:"Linux /boot" -t 3:8300 "{{ dev }}"
    ENDSECTOR="$(sgdisk --end-of-largest "{{ dev }}")"
    sgdisk -n "4:0:$ENDSECTOR" -c 4:"Linux" -t 4:8304 "{{ dev }}"
    sgdisk -p "{{ dev }}"

    mkfs.vfat "{{ dev }}p1"
    mkfs.vfat "{{ dev }}p2"
    mkfs.ext4 "{{ dev }}p3"
    mkfs.ext4 "{{ dev }}p4"

# Mount lfs partition
mount-parts dev="/dev/nbd1" root_part="p4":
    mkdir -pv "{{ LFS }}"

    mount -v -t ext4 "{{ dev }}{{ root_part }}" "{{ LFS }}"
    chown root:root "{{ LFS }}"
    chmod 755 "{{ LFS }}"

# Unmount lfs partition
unmount-parts:
    if [ ! -d "{{ LFS }}" ]; then exit 1; fi
    umount --recursive -v "{{ LFS }}"
    rm -r "{{ LFS }}"
