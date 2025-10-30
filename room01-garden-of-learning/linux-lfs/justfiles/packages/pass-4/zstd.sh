#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
zstd_file="$(find . -name "zstd-*.tar.gz" | head -n1)"
zstd_folder="$(echo "$zstd_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$zstd_folder" ]; then
    mkdir -vp "$zstd_folder"
    tar -xvf "$zstd_file" -C "$zstd_folder" --strip-component 1
fi
pushd "$zstd_folder"

build_n_install() {
    set -x
    make prefix=/usr
    make check
    make prefix=/usr install
}

time build_n_install

rm -v /usr/lib/libzstd.a
