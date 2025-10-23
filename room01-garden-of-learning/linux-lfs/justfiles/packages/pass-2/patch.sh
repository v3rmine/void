#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
patch_file="$(find . -name "patch-*.tar.xz" | head -n1)"
patch_folder="$(echo "$patch_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$patch_folder" ]; then
    mkdir -vp "$patch_folder"
    tar -xvf "$patch_file" -C "$patch_folder" --strip-component 1
fi
pushd "$patch_folder"

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
