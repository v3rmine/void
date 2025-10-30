#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
man_pages_file="$(find . -name "man-pages-*.tar.xz" | head -n1)"
man_pages_folder="$(echo "$man_pages_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$man_pages_folder" ]; then
    mkdir -vp "$man_pages_folder"
    tar -xvf "$man_pages_file" -C "$man_pages_folder" --strip-component 1
fi
pushd "$man_pages_folder"

rm -vf man3/crypt*

build_n_install() {
    set -x
    make -R \
        GIT=false \
        prefix=/usr \
        install
}

time build_n_install
