#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
lz4_file="$(find . -name "lz4-*.tar.gz" | head -n1)"
lz4_folder="$(echo "$lz4_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$lz4_folder" ]; then
    mkdir -vp "$lz4_folder"
    tar -xvf "$lz4_file" -C "$lz4_folder" --strip-component 1
fi
pushd "$lz4_folder"

build_n_install() {
    set -x
    make BUILD_STATIC=no PREFIX=/usr
    make -j1 check
    make BUILD_STATIC=no PREFIX=/usr install
}

time build_n_install
