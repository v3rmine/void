set working-directory := "../.."

build-pass-2:
    #!/usr/bin/env -S sudo -u lfs -- bash
    source ~/.bashrc

    pushd $LFS/sources
    file_file="$(find -name "file-*.tar.gz" | head -n1)"
    file_folder="$(echo "$file_file" | sed -E "s/(^\.\/|\.tar\.gz)//g")"

    if [ ! -d "$file_folder" ]; then
        tar -xvf "$file_file"
    fi
    pushd "$file_folder"

    mkdir -vp build

    build_n_install() {
        set -x
        pushd build
            ../configure --disable-bzlib      \
                        --disable-libseccomp \
                        --disable-xzlib      \
                        --disable-zlib
            make
        popd

        ./configure --prefix=/usr --host=$LFS_TGT --build=$(./config.guess)
        make FILE_COMPILE=$(pwd)/build/src/file
        make DESTDIR=$LFS install
    }

    time build_n_install
    rm -v $LFS/usr/lib/libmagic.la
