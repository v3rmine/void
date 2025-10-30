#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
libxcrypt_file="$(find . -name "libxcrypt-*.tar.xz" | head -n1)"
libxcrypt_folder="$(echo "$libxcrypt_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$libxcrypt_folder" ]; then
    mkdir -vp "$libxcrypt_folder"
    tar -xvf "$libxcrypt_file" -C "$libxcrypt_folder" --strip-component 1
fi
pushd "$libxcrypt_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --enable-hashes=strong,glibc \
        --enable-obsolete-api=no \
        --disable-static \
        --disable-failure-tokens
    make
    make check
    make install
}

time build_n_install
