#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
diffutils_file="$(find . -name "diffutils-*.tar.xz" | head -n1)"
diffutils_folder="$(echo "$diffutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-2"

if [ ! -d "$diffutils_folder" ]; then
    mkdir -vp "$diffutils_folder"
    tar -xvf "$diffutils_file" -C "$diffutils_folder" --strip-component 1
fi
pushd "$diffutils_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        gl_cv_func_strcasecmp_works=y \
        --build="$(./build-aux/config.guess)"
    make
    make DESTDIR="$LFS" install
}

time build_n_install
