#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
gettext_file="$(find . -name "gettext-*.tar.xz" | head -n1)"
gettext_folder="$(echo "$gettext_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-3"

if [ ! -d "$gettext_folder" ]; then
    mkdir -vp "$gettext_folder"
    tar -xvf "$gettext_file" -C "$gettext_folder" --strip-component 1
fi
pushd "$gettext_folder"

build_n_install() {
    set -x
    ./configure --disable-shared
    make
}

time build_n_install
cp -v gettext-tools/src/{msgfmt,msgmerge,xgettext} /usr/bin
