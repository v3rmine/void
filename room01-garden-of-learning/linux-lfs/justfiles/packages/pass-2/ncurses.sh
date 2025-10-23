#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
ncurses_file="$(find . -name "ncurses-*.tgz" | head -n1)"
ncurses_folder="$(echo "$ncurses_file" | sed -E "s/(^\.\/|\.tgz)//g")-pass-2"

if [ ! -d "$ncurses_folder" ]; then
    mkdir -vp "$ncurses_folder"
    tar -xvf "$ncurses_file" -C "$ncurses_folder" --strip-component 1
fi
pushd "$ncurses_folder"

mkdir -vp build

build_n_install() {
    set -x
    pushd build
        ../configure \
            --prefix="$LFS/tools" \
            AWK=gawk
        make -C include
        make -C progs tic
        install progs/tic "$LFS/tools/bin"
    popd

    ./configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(./config.guess)" \
        --mandir=/usr/share/man \
        --with-manpage-format=normal \
        --with-shared \
        --without-normal \
        --with-cxx-shared \
        --without-debug \
        --without-ada \
        --disable-stripping \
        AWK=gawk
    make
    make DESTDIR="$LFS" install
}

time build_n_install
ln -sv libncursesw.so "$LFS/usr/lib/libncurses.so"
sed -e 's/^#if.*XOPEN.*$/#if 1/' \
    -i "$LFS/usr/include/curses.h"
