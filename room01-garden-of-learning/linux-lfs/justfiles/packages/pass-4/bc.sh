#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
bc_file="$(find . -name "bc-*.tar.xz" | head -n1)"
bc_folder="$(echo "$bc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$bc_folder" ]; then
    mkdir -vp "$bc_folder"
    tar -xvf "$bc_file" -C "$bc_folder" --strip-component 1
fi
pushd "$bc_folder"

build_n_install() {
    set -x

    CC='gcc -std=c99' ./configure --prefix=/usr -G -O3 -r

    make
    make test
    make install
}

time build_n_install
