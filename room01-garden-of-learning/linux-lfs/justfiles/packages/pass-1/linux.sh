#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
    exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
linux_file="$(find . -name "linux-*.tar.xz" | head -n1)"
linux_folder="$(echo "$linux_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-1"

if [ ! -d "$linux_folder" ]; then
    mkdir -vp "$linux_folder"
    tar -xvf "$linux_file" -C "$linux_folder" --strip-component 1
fi
pushd "$linux_folder"

build_n_install() {
    set -x
    make mrproper
    make headers
    find usr/include -type f ! -name '*.h' -delete
    cp -rv usr/include "$LFS/usr"
}

time build_n_install
