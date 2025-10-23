set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc
    set -euo pipefail

    pushd $LFS/sources
    coreutils_file="$(find -name "coreutils-*.tar.xz" | head -n1)"
    coreutils_folder="$(echo "$coreutils_file" | sed -E "s/(^\.\/|\.tar\.xz)//g")"

    if [ ! -d "$coreutils_folder" ]; then
        tar -xvf "$coreutils_file"
    fi
    pushd "$coreutils_folder"

    build_n_install() {
        set -x
        ./configure --prefix=/usr                     \
                    --host=$LFS_TGT                   \
                    --build=$(build-aux/config.guess) \
                    --enable-install-program=hostname \
                    --enable-no-install-program=kill,uptime
        make
        make DESTDIR=$LFS install
    }

    time build_n_install
    mv -v $LFS/usr/bin/chroot              $LFS/usr/sbin
    mkdir -pv $LFS/usr/share/man/man8
    mv -v $LFS/usr/share/man/man1/chroot.1 $LFS/usr/share/man/man8/chroot.8
    sed -i 's/"1"/"8"/'                    $LFS/usr/share/man/man8/chroot.8
