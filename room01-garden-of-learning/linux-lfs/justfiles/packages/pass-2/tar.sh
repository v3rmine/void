#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
tar_file="$(find . -name "tar-*.tar.xz" | head -n1)"
tar_folder="$(echo "$tar_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$tar_folder" ]; then
    mkdir -vp "$tar_folder"
    tar -xvf "$tar_file" -C "$tar_folder" --strip-component 1
fi
pushd "$tar_folder"

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
