#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
grep_file="$(find . -name "grep-*.tar.xz" | head -n1)"
grep_folder="$(echo "$grep_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$grep_folder" ]; then
    mkdir -vp "$grep_folder"
    tar -xvf "$grep_file" -C "$grep_folder" --strip-component 1
fi
pushd "$grep_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(./build-aux/config.guess)"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
