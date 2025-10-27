set working-directory := ".."

LFS := "/mnt/lfs"

# Enter the chroot
enter-chroot:
    #!/usr/bin/env bash
    just chroot::_unchecked_enter-chroot <<"EOF"
    if [ "$(ls -di /)" != "2 /" ]; then
        echo "[ERROR]: we are not in the chroot"
        exit 1
    fi
    EOF
    just chroot::_unchecked_enter-chroot


# Unchecked enter the chroot
[private]
_unchecked_enter-chroot:
    chroot "{{ LFS }}" /usr/bin/env -i \
        HOME=/root \
        TERM="$TERM" \
        PS1='(lfs chroot) \u:\w\$ ' \
        PATH=/usr/bin:/usr/sbin \
        MAKEFLAGS="-j$(nproc)" \
        TESTSUITEFLAGS="-j$(nproc)" \
        /bin/bash --login
