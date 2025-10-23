#!/usr/bin/bash
set -euo pipefail
# Ensure script is runned by lfs
if [ "$UID" != "$(id -u lfs)" ]; then
  exec su "lfs" "$0" -- "$@"
fi
# Source LFS variables
source "$HOME/.bashrc"

pushd "$LFS/sources"
glibc_file="$(find . -name "glibc-*.tar.xz" | head -n1)"
glibc_folder="$(echo "$glibc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-1"
glibc_patch="$(find . -name "glibc-*.patch" | head -n1)"

if [ ! -d "$glibc_folder" ]; then
    mkdir -vp "$glibc_folder"
    tar -xvf "$glibc_file" -C "$glibc_folder" --strip-component 1
fi
pushd "$glibc_folder"

case $(uname -m) in
    i?86)
        ln -sfv ld-linux.so.2 "$LFS/lib/ld-lsb.so.3"
    ;;
    x86_64)
        ln -sfv ../lib/ld-linux-x86-64.so.2 "$LFS/lib64"
        ln -sfv ../lib/ld-linux-x86-64.so.2 "$LFS/lib64/ld-lsb-x86-64.so.3"
    ;;
esac

patch -Np1 -i "../$glibc_patch"

mkdir -vp build
pushd build

build_n_install() {
    set -x

    echo "rootsbindir=/usr/sbin" > configparms
    ../configure \
        --prefix=/usr \
        --host="$LFS_TGT" \
        --build="$(../scripts/config.guess)" \
        --disable-nscd \
        libc_cv_slibdir=/usr/lib \
        --enable-kernel=5.4
    make
    make DESTDIR="$LFS" install
}

time build_n_install

sed '/RTLDLIST=/s@/usr@@g' -i "$LFS/usr/bin/ldd"

echo 'int main(){}' | "$LFS_TGT-gcc" -x c - -v -Wl,--verbose &> dummy.log
readelf -l a.out | grep ': /lib'

grep -E -o "$LFS/lib.*/S?crt[1in].*succeeded" dummy.log
grep -B3 "^ $LFS/usr/include" dummy.log
grep 'SEARCH.*/usr/lib' dummy.log | sed 's|; |\n|g'
grep "/lib.*/libc.so.6 " dummy.log
grep found dummy.log
rm -v a.out dummy.log
