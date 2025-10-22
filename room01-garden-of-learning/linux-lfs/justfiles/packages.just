set working-directory := ".."

LFS := "$PWD/lfs"

# Display this help screen
help:
    @just --list packages --list-submodules

get-packages packages-source-url="https://mirror.koddos.net/lfs/lfs-packages/lfs-packages-12.4.tar":
    #!/usr/bin/env bash
    check_packages() {
        pushd {{ LFS }}/sources
            md5sum -c md5sums
        popd
    }

    if ! check_packages >/dev/null 2>&1; then
        umask 022

        mkdir -pv {{ LFS }}/sources
        chmod -v a+wt {{ LFS }}/sources

        wget --no-check-certificate -qO- {{ packages-source-url }} | tar xvf - -C {{ LFS }}/sources --strip-component 1

        check_packages

        chown root:root {{ LFS }}/sources/*
    fi

mod binutils "packages/binutils.just"
mod gcc "packages/gcc.just"
mod linux "packages/linux.just"
mod glibc "packages/glibc.just"
mod libstdcpp "packages/libstdc++.just"
