#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
gzip_file="$(find . -name "gzip-*.tar.xz" | head -n1)"
gzip_folder="$(echo "$gzip_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$gzip_folder" ]; then
    mkdir -vp "$gzip_folder"
    tar -xvf "$gzip_file" -C "$gzip_folder" --strip-component 1
fi
pushd "$gzip_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
