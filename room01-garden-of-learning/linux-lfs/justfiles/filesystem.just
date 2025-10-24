set working-directory := ".."

LFS := "$PWD/lfs"

# Display this help screen
help:
    @just --list filesystem --list-submodules

# Create base LFS filesystem
create-base-filesystem:
    #!/usr/bin/env bash
    set -euxo pipefail
    umask 022

    mkdir -pv "{{ LFS }}"/{etc,var} "{{ LFS }}"/usr/{bin,lib,sbin}

    for i in bin lib sbin; do
        ln -sv "usr/$i" "{{ LFS }}/$i"
    done

    case $(uname -m) in
        x86_64) mkdir -pv "{{ LFS }}/lib64" ;;
    esac

    mkdir -pv "{{ LFS }}/tools"

    chown -v lfs "{{ LFS }}"/{usr{,/*},var,etc,tools}
    case $(uname -m) in
      x86_64) chown -v lfs "{{ LFS }}/lib64" ;;
    esac

# Purge LFS filesystem beside sources
purge-filesystem:
    find "{{ LFS }}" -maxdepth 1 -mindepth 1 -print | grep -v "sources" | xargs rm -r

# Patch ownership before chrooting (because lfs user doesn't exist in chroot)
patch-ownership-for-chroot:
    #!/usr/bin/env bash
    set -euxo pipefail

    chown --from lfs -R root:root "{{ LFS }}/{usr,var,etc,tools}"
    case $(uname -m) in
      x86_64) chown --from lfs -R root:root "{{ LFS }}/lib64" ;;
    esac

# Mount filesystem before chrooting
mount-filesystem-for-chroot:
    #!/usr/bin/env bash
    set -euxo pipefail

    mkdir -pv "{{ LFS }}"/{dev,proc,sys,run}

    mount -v --bind /dev "{{ LFS }}/dev"

    mount -vt devpts devpts -o gid=5,mode=0620 "{{ LFS }}/dev/pts"
    mount -vt proc proc "{{ LFS }}/proc"
    mount -vt sysfs sysfs "{{ LFS }}/sys"
    mount -vt tmpfs tmpfs "{{ LFS }}/run"

    if [ -h "{{ LFS }}/dev/shm" ]; then
        install -v -d -m 1777 "{{ LFS }}$(realpath /dev/shm)"
    else
        mount -vt tmpfs -o nosuid,nodev tmpfs "{{ LFS }}/dev/shm"
    fi

# Create chroot filesystem
create-chroot-filesystem:
    #!/usr/bin/env bash
    set -euxo pipefail
    umask 022

    # Ensure we are chrooted
    if [ "$(ls -di /)" != "2 /" ]; then
        echo "[ERROR]: we are not in the chroot"
        exit 1
    fi

    mkdir -pv /{boot,home,mnt,opt,srv}

    mkdir -pv /etc/{opt,sysconfig}
    mkdir -pv /lib/firmware
    mkdir -pv /media/{floppy,cdrom}
    mkdir -pv /usr/{,local/}{include,src}
    mkdir -pv /usr/lib/locale
    mkdir -pv /usr/local/{bin,lib,sbin}
    mkdir -pv /usr/{,local/}share/{color,dict,doc,info,locale,man}
    mkdir -pv /usr/{,local/}share/{misc,terminfo,zoneinfo}
    mkdir -pv /usr/{,local/}share/man/man{1..8}
    mkdir -pv /var/{cache,local,log,mail,opt,spool}
    mkdir -pv /var/lib/{color,misc,locate}

    ln -sfv /run /var/run
    ln -sfv /run/lock /var/lock

    install -dv -m 0750 /root
    install -dv -m 1777 /tmp /var/tmp
