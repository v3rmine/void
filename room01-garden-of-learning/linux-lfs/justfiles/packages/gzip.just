set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    gzip_file="$(find -name "gzip-*.tar.xz" | head -n1)"
    gzip_folder="$(echo "$gzip_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$gzip_folder" ]; then
        tar -xvf "$gzip_file"
    fi
    pushd "$gzip_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr --host=$LFS_TGT
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
