#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
m4_file="$(find . -name "m4-*.tar.xz" | head -n1)"
m4_folder="$(echo "$m4_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$m4_folder" ]; then
    mkdir -vp "$m4_folder"
    tar -xvf "$m4_file" -C "$m4_folder" --strip-component 1
fi
pushd "$m4_folder"

build_n_install() {
    set -x

    ./configure --prefix=/usr

    make
    make check
    make install
}

time build_n_install
