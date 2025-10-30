#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
acl_file="$(find . -name "acl-*.tar.xz" | head -n1)"
acl_folder="$(echo "$acl_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"
acl_version="$(echo "$acl_folder" | cut -d'-' -f2)"

if [ ! -d "$acl_folder" ]; then
    mkdir -vp "$acl_folder"
    tar -xvf "$acl_file" -C "$acl_folder" --strip-component 1
fi
pushd "$acl_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --disable-static \
        --docdir="/usr/share/doc/acl-$acl_version"
    make
    make check || true
    make install
}

time build_n_install
