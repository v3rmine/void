#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
libcap_file="$(find . -name "libcap-*.tar.xz" | head -n1)"
libcap_folder="$(echo "$libcap_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$libcap_folder" ]; then
    mkdir -vp "$libcap_folder"
    tar -xvf "$libcap_file" -C "$libcap_folder" --strip-component 1
fi
pushd "$libcap_folder"

sed -i '/install -m.*STA/d' libcap/Makefile

build_n_install() {
    set -x
    make prefix=/usr lib=lib
    make test
    make prefix=/usr lib=lib install
}

time build_n_install
