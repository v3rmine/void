#!/usr/bin/bash
set -euo pipefail
# Ensure we are chrooted
if [ "$(ls -di /)" != "2 /" ]; then
    echo "[ERROR]: we are not in the chroot"
    exit 1
fi

pushd "/sources"
shadow_file="$(find . -name "shadow-*.tar.xz" | head -n1)"
shadow_folder="$(echo "$shadow_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")-pass-4"

if [ ! -d "$shadow_folder" ]; then
    mkdir -vp "$shadow_folder"
    tar -xvf "$shadow_file" -C "$shadow_folder" --strip-component 1
fi
pushd "$shadow_folder"

sed -i 's/groups$(EXEEXT) //' src/Makefile.in
find man -name Makefile.in -exec sed -i 's/groups\.1 / /' {} \;
find man -name Makefile.in -exec sed -i 's/getspnam\.3 / /' {} \;
find man -name Makefile.in -exec sed -i 's/passwd\.5 / /' {} \;

sed -e 's:#ENCRYPT_METHOD DES:ENCRYPT_METHOD YESCRYPT:' \
    -e 's:/var/spool/mail:/var/mail:' \
    -e '/PATH=/{s@/sbin:@@;s@/bin:@@}' \
    -i etc/login.defs

build_n_install() {
    set -x

    touch /usr/bin/passwd
    ./configure \
        --sysconfdir=/etc \
        --disable-static \
        --with-{b,yes}crypt \
        --without-libbsd \
        --with-group-name-max-length=32

    make
    make exec_prefix=/usr install
    make -C man install-man
}

time build_n_install

pwconv
grpconv

mkdir -pv /etc/default
useradd -D --gid 999

echo 'root' | passwd --stdin
