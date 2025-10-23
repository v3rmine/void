#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
gcc_file="$(find . -name "gcc-*.tar.xz" | head -n1)"
gcc_folder="$(echo "$gcc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-1"
gcc_version="$(echo "$gcc_folder" | cut -d'-' -f2)"

if [ ! -d "$gcc_folder" ]; then
    mkdir -vp "$gcc_folder"
    tar -xvf "$gcc_file" -C "$gcc_folder" --strip-component 1
fi
pushd "$gcc_folder"

mkdir -vp libstdcpp_build
pushd libstdcpp_build

build_n_install() {
    set -x
    ../libstdc++-v3/configure \
        --host="$LFS_TGT" \
        --build="$(../config.guess)" \
        --prefix=/usr \
        --disable-multilib \
        --disable-nls \
        --disable-libstdcxx-pch \
        --with-gxx-include-dir="/tools/$LFS_TGT/include/c++/$gcc_version"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
rm -v "$LFS"/usr/lib/lib{stdc++{,exp,fs},supc++}.la
