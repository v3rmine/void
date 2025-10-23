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

clear_unpacked_sources:
    pushd {{ LFS }}/sources && find . -maxdepth 1 -type d -delete && popd

mod binutils "packages/binutils.just"
mod gcc "packages/gcc.just"
mod linux "packages/linux.just"
mod glibc "packages/glibc.just"
mod libstdcpp "packages/libstdc++.just"
mod m4 "packages/m4.just"
mod ncurses "packages/ncurses.just"
mod bash "packages/bash.just"
mod coreutils "packages/coreutils.just"
mod diffutils "packages/diffutils.just"
mod file "packages/file.just"
mod findutils "packages/findutils.just"
mod gawk "packages/gawk.just"
mod grep "packages/grep.just"
mod gzip "packages/gzip.just"
mod make "packages/make.just"
mod patch "packages/patch.just"
mod sed "packages/sed.just"
mod tar "packages/tar.just"
mod xz "packages/xz.just"

build-pass-1:
    just packages::binutils::build-pass-1
    just packages::gcc::build-pass-1
    just packages::linux::build-pass-1
    just packages::glibc::build-pass-1
    just packages::libstdcpp::build-pass-1

build-pass-2:
    just packages::m4::build-pass-2
    just packages::ncurses::build-pass-2
    just packages::bash::build-pass-2
    just packages::coreutils::build-pass-2
    just packages::diffutils::build-pass-2
    just packages::file::build-pass-2
    just packages::findutils::build-pass-2
    just packages::gawk::build-pass-2
    just packages::grep::build-pass-2
    just packages::gzip::build-pass-2
    just packages::make::build-pass-2
    just packages::patch::build-pass-2
    just packages::sed::build-pass-2
    just packages::tar::build-pass-2
    just packages::xz::build-pass-2
    just packages::binutils::build-pass-2
