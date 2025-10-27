set working-directory := ".."

LFS := "/mnt/lfs"

# Display this help screen
help:
    @just --list filesystem --list-submodules

# Create base LFS filesystem
create-base-filesystem:
    #!/usr/bin/env bash
    set -euxo pipefail
    umask 022

    mkdir -pv "{{ LFS }}"/{etc,var} "{{ LFS }}"/usr/{bin,lib,sbin}

    for i in bin lib sbin; do
        ln -sv "usr/$i" "{{ LFS }}/$i"
    done

    case $(uname -m) in
        x86_64) mkdir -pv "{{ LFS }}/lib64" ;;
    esac

    mkdir -pv "{{ LFS }}/tools"

    chown -v lfs "{{ LFS }}"/{usr{,/*},var,etc,tools}
    case $(uname -m) in
      x86_64) chown -v lfs "{{ LFS }}/lib64" ;;
    esac

# Purge LFS filesystem beside sources
purge-filesystem:
    find "{{ LFS }}" -maxdepth 1 -mindepth 1 -print | grep -v "sources" | xargs rm -r

# Patch ownership before chrooting (because lfs user doesn't exist in chroot)
patch-ownership-for-chroot:
    #!/usr/bin/env bash
    set -euxo pipefail

    chown --from lfs -R root:root "{{ LFS }}"/{usr,var,etc,tools}
    case $(uname -m) in
      x86_64) chown --from lfs -R root:root "{{ LFS }}/lib64" ;;
    esac

# Mount filesystem before chrooting
mount-filesystem-for-chroot:
    #!/usr/bin/env bash
    set -euxo pipefail

    mkdir -pv "{{ LFS }}"/{dev,proc,sys,run}

    mount -v --bind /dev "{{ LFS }}/dev"

    mount -vt devpts devpts -o gid=5,mode=0620 "{{ LFS }}/dev/pts"
    mount -vt proc proc "{{ LFS }}/proc"
    mount -vt sysfs sysfs "{{ LFS }}/sys"
    mount -vt tmpfs tmpfs "{{ LFS }}/run"

    if [ -h "{{ LFS }}/dev/shm" ]; then
        install -v -d -m 1777 "{{ LFS }}$(realpath /dev/shm)"
    else
        mount -vt tmpfs -o nosuid,nodev tmpfs "{{ LFS }}/dev/shm"
    fi

# Create chroot filesystem
create-chroot-filesystem:
    #!/bin/env bash
    set -euxo pipefail

    mkdir -vp "{{ LFS }}/tools/scripts/lfs"
    cp -v ./justfiles/chroot/create-chroot-filesystem.sh "{{ LFS }}/tools/scripts/lfs/create-chroot-filesystem.sh"
    chown -v root:root "{{ LFS }}/tools/scripts/lfs/create-chroot-filesystem.sh"
    chmod -v u+x "{{ LFS }}/tools/scripts/lfs/create-chroot-filesystem.sh"
    just chroot::enter-chroot <<"EOF"
    /tools/scripts/lfs/create-chroot-filesystem.sh
    EOF

# Create chroot essential files
create-chroot-essentials:
    #!/bin/env bash
    set -euxo pipefail

    mkdir -vp "{{ LFS }}/tools/scripts/lfs"
    cp -v ./justfiles/chroot/create-chroot-essentials.sh "{{ LFS }}/tools/scripts/lfs/create-chroot-essentials.sh"
    chown -v root:root "{{ LFS }}/tools/scripts/lfs/create-chroot-essentials.sh"
    chmod -v u+x "{{ LFS }}/tools/scripts/lfs/create-chroot-essentials.sh"
    just chroot::enter-chroot <<"EOF"
    /tools/scripts/lfs/create-chroot-essentials.sh
    EOF

# Cleanup chroot after build
cleanup-chroot-after-build:
    #!/bin/env bash
    set -euxo pipefail

    just chroot::enter-chroot <<"EOF"
    rm -rf /usr/share/{info,man,doc}/*
    find /usr/{lib,libexec} -name \*.la -delete
    rm -rf /tools
    EOF

    just packages::clear-unpacked-sources

# Backup temporary filesystem
backup-temporary-filesystem file="lfs-temporary-filesystem.tar.xz":
    #!/bin/env bash
    set -euxo pipefail

    unmount() {
        if mountpoint -q "{{ LFS }}$1"; then
            umount "{{ LFS }}$1"
        fi
    }

    unmount /dev/shm
    unmount /dev/pts
    unmount /sys
    unmount /proc
    unmount /run
    unmount /dev

    initial_dir="$(pwd)"

    pushd "{{ LFS }}"
    tar -cJpf "$initial_dir/{{ file }}" .
    popd

# Restore temporary filesystem
restore-temporary-filesystem file="lfs-temporary-filesystem.tar.xz":
    #!/bin/env bash
    set -euxo pipefail

    initial_dir="$(pwd)"

    pushd "{{ LFS }}"
    rm -rf ./*
    tar -xpf "$initial_dir/{{ file }}
