#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
bash_file="$(find . -name "bash-*.tar.gz" | head -n1)"
bash_folder="$(echo "$bash_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-2"

if [ ! -d "$bash_folder" ]; then
    mkdir -vp "$bash_folder"
    tar -xvf "$bash_file" -C "$bash_folder" --strip-component 1
fi
pushd "$bash_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --build="$(sh support/config.guess)" \
        --host="$LFS_TGT" \
        --without-bash-malloc
    make
    make DESTDIR="$LFS" install
}

time build_n_install

ln -sv bash "$LFS/bin/sh"
