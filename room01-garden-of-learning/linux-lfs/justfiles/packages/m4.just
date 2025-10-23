set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    m4_file="$(find -name "m4-*.tar.xz" | head -n1)"
    m4_folder="$(echo "$m4_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$m4_folder" ]; then
        tar -xvf "$m4_file"
    fi
    pushd "$m4_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr   \
                    --host=$LFS_TGT \
                    --build=$(build-aux/config.guess)
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
