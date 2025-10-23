set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    make_file="$(find -name "make-*.tar.gz" | head -n1)"
    make_folder="$(echo "$make_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")"

    if [ ! -d "$make_folder" ]; then
        tar -xvf "$make_file"
    fi
    pushd "$make_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    --build=$(build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
