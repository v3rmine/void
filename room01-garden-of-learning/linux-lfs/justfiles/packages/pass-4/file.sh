#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
file_file="$(find . -name "file-*.tar.gz" | head -n1)"
file_folder="$(echo "$file_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$file_folder" ]; then
    mkdir -vp "$file_folder"
    tar -xvf "$file_file" -C "$file_folder" --strip-component 1
fi
pushd "$file_folder"

build_n_install() {
    set -x

    ./configure --prefix=/usr
    make
    make check
    make install
}

time build_n_install
