#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
util_linux_file="$(find . -name "util-linux-*.tar.xz" | head -n1)"
util_linux_folder="$(echo "$util_linux_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-3"
util_linux_version="$(echo "$util_linux_folder" | cut -d'-' -f3)"

if [ ! -d "$util_linux_folder" ]; then
    mkdir -vp "$util_linux_folder"
    tar -xvf "$util_linux_file" -C "$util_linux_folder" --strip-component 1
fi
pushd "$util_linux_folder"

build_n_install() {
    set -x

    mkdir -pv /var/lib/hwclock

    ./configure \
        --libdir=/usr/lib \
        --runstatedir=/run \
        --disable-chfn-chsh \
        --disable-login \
        --disable-nologin \
        --disable-su \
        --disable-setpriv \
        --disable-runuser \
        --disable-pylibmount \
        --disable-static \
        --disable-liblastlog2 \
        --without-python \
        ADJTIME_PATH=/var/lib/hwclock/adjtime \
        --docdir="/usr/share/doc/util-linux-$util_linux_version"

    make
    make install
}

time build_n_install
