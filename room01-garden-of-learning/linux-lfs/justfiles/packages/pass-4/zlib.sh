#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
zlib_file="$(find . -name "zlib-*.tar.gz" | head -n1)"
zlib_folder="$(echo "$zlib_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$zlib_folder" ]; then
    mkdir -vp "$zlib_folder"
    tar -xvf "$zlib_file" -C "$zlib_folder" --strip-component 1
fi
pushd "$zlib_folder"

build_n_install() {
    set -x
    ./configure --prefix=/usr
    make
    make check
    make install
}

time build_n_install

rm -fv /usr/lib/libz.a
