set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    gawk_file="$(find -name "gawk-*.tar.xz" | head -n1)"
    gawk_folder="$(echo "$gawk_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$gawk_folder" ]; then
        tar -xvf "$gawk_file"
    fi
    pushd "$gawk_folder"

    sed -i 's/extras//' Makefile.in

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    --build=$(build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
