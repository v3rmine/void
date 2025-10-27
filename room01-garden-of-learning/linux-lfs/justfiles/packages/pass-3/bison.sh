#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
bison_file="$(find . -name "bison-*.tar.xz" | head -n1)"
bison_folder="$(echo "$bison_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-3"
bison_version="$(echo "$bison_folder" | cut -d'-' -f2)"

if [ ! -d "$bison_folder" ]; then
    mkdir -vp "$bison_folder"
    tar -xvf "$bison_file" -C "$bison_folder" --strip-component 1
fi
pushd "$bison_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --docdir="/usr/share/doc/bison-$bison_version"
    make
    make install
}

time build_n_install
