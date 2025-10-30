#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
pkgconf_file="$(find . -name "pkgconf-*.tar.xz" | head -n1)"
pkgconf_folder="$(echo "$pkgconf_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"
pkgconf_version="$(echo "$pkgconf_folder" | cut -d'-' -f2)"

if [ ! -d "$pkgconf_folder" ]; then
    mkdir -vp "$pkgconf_folder"
    tar -xvf "$pkgconf_file" -C "$pkgconf_folder" --strip-component 1
fi
pushd "$pkgconf_folder"

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --disable-static \
        --docdir="/usr/share/doc/pkgconf-$pkgconf_version"
    make
    make install
}

time build_n_install

ln -sv pkgconf /usr/bin/pkg-config
ln -sv pkgconf.1 /usr/share/man/man1/pkg-config.1
