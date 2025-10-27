#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
texinfo_file="$(find . -name "texinfo-*.tar.xz" | head -n1)"
texinfo_folder="$(echo "$texinfo_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-3"

if [ ! -d "$texinfo_folder" ]; then
    mkdir -vp "$texinfo_folder"
    tar -xvf "$texinfo_file" -C "$texinfo_folder" --strip-component 1
fi
pushd "$texinfo_folder"

build_n_install() {
    set -x
    ./configure --prefix=/usr
    make
    make install
}

time build_n_install
