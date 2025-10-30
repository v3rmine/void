#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
attr_file="$(find . -name "attr-*.tar.gz" | head -n1)"
attr_folder="$(echo "$attr_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"
attr_version="$(echo "$attr_folder" | cut -d'-' -f2)"

if [ ! -d "$attr_folder" ]; then
    mkdir -vp "$attr_folder"
    tar -xvf "$attr_file" -C "$attr_folder" --strip-component 1
fi
pushd "$attr_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --disable-static \
        --sysconfdir=/etc \
        --docdir="/usr/share/doc/attr-$attr_version"
    make
    make check || true
    make install
}

time build_n_install
