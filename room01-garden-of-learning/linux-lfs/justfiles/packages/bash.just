set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    bash_file="$(find -name "bash-*.tar.gz" | head -n1)"
    bash_folder="$(echo "$bash_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")"

    if [ ! -d "$bash_folder" ]; then
        tar -xvf "$bash_file"
    fi
    pushd "$bash_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr                      \
            --build=$(sh support/config.guess) \
            --host=$LFS_TGT                    \
            --without-bash-malloc
        make
        make DESTDIR=$LFS install
        ln -sv bash $LFS/bin/sh
    }

    time build_n_install
