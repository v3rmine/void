set working-directory := ".."

LFS := "$PWD/lfs"

# Display this help screen
help:
    @just --list disk --list-submodules

create-virtual-disk size="10G" file="lfs.qcow2":
    qemu-img create -f qcow2 {{ file }} {{ size }}

# --- nbd module management ---
enable-nbd max_part="4":
    modprobe nbd max_part={{ max_part }}

disable-nbd:
    rmmod nbd

# --- --- ---

# --- virtual disk connection ---
connect-virtual-disk file="lfs.qcow2" dev="/dev/nbd1":
    qemu-nbd --connect={{ dev }} {{ file }}

disconnect-virtual-disk dev="/dev/nbd1":
    qemu-nbd --disconnect {{ dev }}

# --- --- ---

# --- format disk ---
format-disk dev="/dev/nbd1":
    #!/usr/bin/env bash
    set -euxo pipefail

    sgdisk --clear --mbrtogpt {{ dev }}
    # Get the codes using "sgdisk -L"
    sgdisk -n 1:2048:4095 -c 1:"BIOS Boot Partition" -t 1:ef02 {{ dev }}
    sgdisk -n 2:0:+100M -c 2:"EFI System Partition" -t 2:ef00 {{ dev }}
    sgdisk -n 3:0:+200M -c 3:"Linux /boot" -t 3:8300 {{ dev }}
    ENDSECTOR=`sgdisk --end-of-largest {{ dev }}`
    sgdisk -n 4:0:$ENDSECTOR -c 4:"Linux" -t 4:8304 {{ dev }}
    sgdisk -p {{ dev }}

    mkfs.vfat {{ dev }}p1
    mkfs.vfat {{ dev }}p2
    mkfs.ext4 {{ dev }}p3
    mkfs.ext4 {{ dev }}p4

# --- --- ---

# --- mount partitions ---
mount-parts dev="/dev/nbd1" root_part="p4":
    mkdir -pv {{ LFS }}

    mount -v -t ext4 {{ dev }}{{ root_part }} {{ LFS }}
    chown root:root {{ LFS }}
    chmod 755 {{ LFS }}

unmount-parts:
    if [ ! -d "{{ LFS }}" ]; then exit 1; fi
    umount -v {{ LFS }}
    rm -r {{ LFS }}

# --- --- ---
