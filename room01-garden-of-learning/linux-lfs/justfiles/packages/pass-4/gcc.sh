#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
gcc_file="$(find . -name "gcc-*.tar.xz" | head -n1)"
gcc_folder="$(echo "$gcc_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"
gcc_version="$(echo "$gcc_folder" | cut -d'-' -f2)"

if [ ! -d "$gcc_folder" ]; then
    mkdir -vp "$gcc_folder"
    tar -xvf "$gcc_file" -C "$gcc_folder" --strip-component 1
fi
pushd "$gcc_folder"

case "$(uname -m)" in
  x86_64)
    sed -e '/m64=/s/lib64/lib/' \
        -i.orig gcc/config/i386/t-linux64
  ;;
esac

mkdir -vp build
pushd build

build_n_install() {
    set -x

    ../configure \
        --prefix=/usr \
        LD=ld \
        --enable-languages=c,c++ \
        --enable-default-pie \
        --enable-default-ssp \
        --enable-host-pie \
        --disable-multilib \
        --disable-bootstrap \
        --disable-fixincludes \
        --with-system-zlib

    make
    ulimit -s -H unlimited

    sed -e '/cpython/d' -i ../gcc/testsuite/gcc.dg/plugin/plugin.exp
    chown -R tester .
    su tester -c "PATH=$PATH make -k check"

    ../contrib/test_summary | grep -A7 Summ

    make install
}

time build_n_install

chown -v -R root:root \
    /usr/lib/gcc/"$(gcc -dumpmachine)/$gcc_version"/include{,-fixed}

ln -svr /usr/bin/cpp /usr/lib
ln -sv gcc.1 /usr/share/man/man1/cc.1
ln -sfv ../../libexec/gcc/"$(gcc -dumpmachine)/$gcc_version"/liblto_plugin.so \
    /usr/lib/bfd-plugins/

echo 'int main(){}' | cc -x c - -v -Wl,--verbose &> dummy.log
readelf -l a.out | grep ': /lib'

grep -E -o '/usr/lib.*/S?crt[1in].*succeeded' dummy.log
grep -B4 '^ /usr/include' dummy.log
grep 'SEARCH.*/usr/lib' dummy.log |sed 's|; |\n|g'
grep "/lib.*/libc.so.6 " dummy.log
grep found dummy.log

rm -v a.out dummy.log

mkdir -pv /usr/share/gdb/auto-load/usr/lib
mv -v /usr/lib/*gdb.py /usr/share/gdb/auto-load/usr/lib
