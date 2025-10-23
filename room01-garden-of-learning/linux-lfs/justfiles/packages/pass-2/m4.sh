#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
m4_file="$(find . -name "m4-*.tar.xz" | head -n1)"
m4_folder="$(echo "$m4_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$m4_folder" ]; then
    mkdir -vp "$m4_folder"
    tar -xvf "$m4_file" -C "$m4_folder" --strip-component 1
fi
pushd "$m4_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(build-aux/config.guess)"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
