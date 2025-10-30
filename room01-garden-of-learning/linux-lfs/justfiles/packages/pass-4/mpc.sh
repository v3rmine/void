#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
mpc_file="$(find . -name "mpc-*.tar.gz" | head -n1)"
mpc_folder="$(echo "$mpc_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"
mpc_version="$(echo "$mpc_folder" | cut -d'-' -f2)"

if [ ! -d "$mpc_folder" ]; then
    mkdir -vp "$mpc_folder"
    tar -xvf "$mpc_file" -C "$mpc_folder" --strip-component 1
fi
pushd "$mpc_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --disable-static \
        --docdir="/usr/share/doc/mpc-$mpc_version"
    make
    make html

    make check

    make install
    make install-html
}

time build_n_install
