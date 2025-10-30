#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
xz_file="$(find . -name "xz-*.tar.gz" | head -n1)"
xz_folder="$(echo "$xz_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"
xz_version="$(echo "$xz_folder" | cut -d'-' -f2)"

if [ ! -d "$xz_folder" ]; then
    mkdir -vp "$xz_folder"
    tar -xvf "$xz_file" -C "$xz_folder" --strip-component 1
fi
pushd "$xz_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --disable-static \
        --docdir="/usr/share/doc/xz-$xz_version"

    make
    make check
    make install
}

time build_n_install
