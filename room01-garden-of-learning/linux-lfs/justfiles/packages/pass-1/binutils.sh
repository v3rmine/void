#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
binutils_file="$(find . -name "binutils-*.tar.xz" | head -n1)"
binutils_folder="$(echo "$binutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-1"

if [ ! -d "$binutils_folder" ]; then
    mkdir -vp "$binutils_folder"
    tar -xvf "$binutils_file" -C "$binutils_folder" --strip-component 1
fi
pushd "$binutils_folder"

mkdir -vp build
pushd build

build_n_install() {
    set -x
    ../configure \
        --prefix="$LFS/tools" \
        --with-sysroot="$LFS" \
        --target="$LFS_TGT" \
        --disable-nls \
        --enable-gprofng=no \
        --disable-werror \
        --enable-new-dtags \
        --enable-default-hash-style=gnu
    make
    make install
}

time build_n_install
