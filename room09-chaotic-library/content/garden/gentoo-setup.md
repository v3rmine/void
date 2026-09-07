---
date: 2026-05-01
title: 'Gentoo: Setup'
description: ""
hidden: true
in_search_index: false
include_in_feeds: false
taxonomies:
  tags:
    - en
extra:
  public_draft: true
  guid: ''
  lang: en
  toc: false
---

i have a really flaky USB key so i booted the live image in RAM (cached version). i also had issues because the iso was visually written in like 10s on the USB key but it easily needed 5min do really write it and empty the caches using `sync` command {{ customoji(name="SmileBecauseThisIsFine") }}

Gentoo doesn't have a graphical setup so i'll keep the [handbook](https://wiki.gentoo.org/wiki/Handbook:AMD64) open on my phone and on a tab in the browser!

it's a trial setup on a [framework storage expansion card](https://frame.work/fr/en/products/storage-expansion-card-2nd-gen) (i really like them to try new systems {{ customoji(name="CatBlush") }}) so we keep it simple with only 3 partitions and nothing out of the ordinary
- `/dev/sda1`: EFI 1GiB `vfat`
- `/dev/sda2`: swap 16GiB `linux-swap`
- `/dev/sda3`: root 100GiB `ext4`

Gentoo works with stages to choose different configurations options for the installation, multilib support (support 32 and 64bits programs) and openrc or systemd (i chose openrc because why not)

i changed `/mnt/gentoo/etc/portage/make.conf` (edited things get a triple comment before)
```cmake
# These settings were set by the catalyst build script that automatically
# built this stage.
# Please consult /usr/share/portage/config/make.conf.example for a more
# detailed example.
### we add -march=native
COMMON_FLAGS="-march=native -O2 -pipe"
CFLAGS="${COMMON_FLAGS}"
CXXFLAGS="${COMMON_FLAGS}"
FCFLAGS="${COMMON_FLAGS}"
FFLAGS="${COMMON_FLAGS}"

### we add this line to target the current cpu
RUSTFLAGS="${RUSTFLAGS} -C target-cpu=native"

# NOTE: This stage was built with the bindist USE flag enabled

# This sets the language of build output to English.
# Please keep this setting intact when reporting bugs.
LC_MESSAGES=C.UTF-8
```

i didn't touch `MAKEOPTS` to limit the number of parallel compilation because my laptop has 16 threads and 32GiB of ram (so i'm fine with the max RAM/2 rule) {{ customoji(name="FemFingerGun") }}

---

we are now chrooted in the new system {{ customoji(name="personas/astrid/Sparkling 1") }}

i'm adding `ACCEPT\_LICENSE\="-\* @FREE @BINARY-REDISTRIBUTABLE"` to `/etc/portage/make.conf` to allow binary distributed softwares

and it's now time for a looooooong `emerge --ask --verbose --update --deep --changed-use @world` {{ customoji(name="CatLoading") }}

finiiiished i'm sidetracking a bit because i want to also try niri so:
- i need to setup [GURU](https://wiki.gentoo.org/wiki/Project:GURU/Information_for_End_Users)
  - that need git `emerge --ask dev-vcs/git`
  - and `emerge --ask app-eselect/eselect-repository`
  - i can now do a `eselect repository enable guru`
  - and then sync it with `emerge --sync guru`
- i need to unmask niri with `echo 'gui-wm/niri ~amd64' >> /etc/portage/package.accept\_keywords/niri`
- then i can install niri `emerge --ask gui-wm/niri`
- we can unmask fuzzel with `echo 'gui-apps/fuzzel ~amd64' >> /etc/portage/package.accept\_keywords/fuzzel`
- and install it `emerge --ask gui-apps/fuzzel` (fuzzel is an app launcher)

it's time for some kernel installation with grub, initramfs and the prebuilt `sys-kernel/gentoo-kernel-bin` (because yes it's a test of the distro so i keep it simple)

we keep it simple for the system logging using `emerge --ask app-admin/sysklogd`

we keep the cron daemon classic with cronie `emerge --ask sys-process/cronie`