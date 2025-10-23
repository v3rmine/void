set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    xz_file="$(find -name "tar-*.tar.xz" | head -n1)"
    xz_folder="$(echo "$xz_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$xz_folder" ]; then
        tar -xvf "$xz_file"
    fi
    pushd "$xz_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr                     \
                    --host=$LFS_TGT                   \
                    --build=$(build-aux/config.guess) \
                    --disable-static                  \
                    --docdir=/usr/share/doc/xz-5.8.1
        make
        make DESTDIR=$LFS install
    }

    time build_n_install

    rm -vf $LFS/usr/lib/liblzma.la
