#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
sed_file="$(find . -name "sed-*.tar.xz" | head -n1)"
sed_folder="$(echo "$sed_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$sed_folder" ]; then
    mkdir -vp "$sed_folder"
    tar -xvf "$sed_file" -C "$sed_folder" --strip-component 1
fi
pushd "$sed_folder"

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
