#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
bzip2_file="$(find . -name "bzip2-*.tar.gz" | head -n1)"
bzip2_folder="$(echo "$bzip2_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")-pass-4"

if [ ! -d "$bzip2_folder" ]; then
    mkdir -vp "$bzip2_folder"
    tar -xvf "$bzip2_file" -C "$bzip2_folder" --strip-component 1
fi
pushd "$bzip2_folder"

patch -Np1 -i ../bzip2-*-install_docs-1.patch

sed -i 's@\(ln -s -f \)$(PREFIX)/bin/@\1@' Makefile
sed -i "s@(PREFIX)/man@(PREFIX)/share/man@g" Makefile

build_n_install() {
    set -x

    make -f Makefile-libbz2_so
    make clean

    make
    make PREFIX=/usr install
}

time build_n_install

cp -av libbz2.so.* /usr/lib
ln -sv libbz2.so.1.0.8 /usr/lib/libbz2.so

cp -v bzip2-shared /usr/bin/bzip2
for i in /usr/bin/{bzcat,bunzip2}; do
  ln -sfv bzip2 "$i"
done

rm -fv /usr/lib/libbz2.a
