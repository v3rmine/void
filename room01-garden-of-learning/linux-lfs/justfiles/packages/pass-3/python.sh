#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
python_file="$(find . -name "Python-*.tar.xz" | head -n1)"
python_folder="$(echo "$python_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-3"

if [ ! -d "$python_folder" ]; then
    mkdir -vp "$python_folder"
    tar -xvf "$python_file" -C "$python_folder" --strip-component 1
fi
pushd "$python_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --enable-shared \
        --without-ensurepip \
        --without-static-libpython
    make
    make install
}

time build_n_install
