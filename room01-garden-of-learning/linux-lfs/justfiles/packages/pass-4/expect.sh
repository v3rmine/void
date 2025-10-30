#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
expect_file="$(find . -name "expect*.tar.gz" | head -n1)"
expect_folder="$(echo "$expect_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$expect_folder" ]; then
    mkdir -vp "$expect_folder"
    tar -xvf "$expect_file" -C "$expect_folder" --strip-component 1
fi
pushd "$expect_folder"

if python3 -c 'from pty import spawn; spawn(["echo", "ok"])' | grep -vq "ok"; then
    echo "[ERROR] Expect needs PTYs to work"
    exit 1
fi

patch -Np1 -i ../expect-*-gcc15-1.patch

build_n_install() {
    set -x
    ./configure \
        --prefix=/usr \
        --with-tcl=/usr/lib \
        --enable-shared \
        --disable-rpath \
        --mandir=/usr/share/man \
        --with-tclinclude=/usr/include
    make
    make test
    make install
}

time build_n_install

ln -svf expect5.45.4/libexpect5.45.4.so /usr/lib
