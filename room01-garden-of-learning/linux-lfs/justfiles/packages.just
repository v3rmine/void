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

clear-unpacked-sources:
    pushd {{ LFS }}/sources && find . -maxdepth 1 -type d -exec rm -rv {} ';' && popd

build-pass-1:
    #!/usr/bin/env bash
    set -euo pipefail
    build { bash "./justfiles/packages/pass-1/$1.sh" }

    build binutils
    build gcc
    build linux
    build glibc
    build libstdc++

build-pass-2:
    #!/usr/bin/env bash
    set -euo pipefail
    build { bash "./justfiles/packages/pass-2/$1.sh" }

    build m4
    build ncurses
    build bash
    build coreutils
    build diffutils
    build file
    build findutils
    build gawk
    build grep
    build gzip
    build make
    build patch
    build sed
    build tar
    build xz
    build binutils
    build gcc
