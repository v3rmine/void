# FreeBSD on Ionos (or others).
Source: https://smallhacks.wordpress.com/2024/03/26/installing-freebsd-14-on-ionos-vps-hosting/

## Install FreeBSD
1. Buy VPS, with any OS
2. Insert DVD: Grml
3. Reboot
4. `wget https://mfsbsd.vx.sk/files/images/14/amd64/mfsbsd-14.1-RELEASE-amd64.img`
5. `fdisk -l` => find the `/dev/<disk>` to use
6. `dd if=mfsbsd-14.1-RELEASE-amd64.img of=/dev/<disk>`
7. Eject DVD & reboot
8. ssh using mfsbsd default creds root/mfsroot
9. `gpart recover vtbd0` to fix the partition table
10. `bsdinstall`
11. Reboot

## Reduce boot time
1. `fetch http://ftp.freebsd.org/pub/FreeBSD/releases/amd64/14.1-RELEASE/src.txz`
2. `tar -C / -xvf src.txz`
3. `freebsd-update fetch && freebsd-update install`
4. `cd /usr/src/`
5. ```make buildkernel -j `sysctl -n hw.ncpu` KERNCONF=MINIMAL && make installkernel KERNCONF=MINIMAL```
6. Update `/boot/loader.conf`
```conf
...
kern.boottrace.enabled=1
autoboot_delay=1
ufs_load="YES"
...
```

## Enabling IPv6 and fixing “Bogus Host Name” warning
> IONOS uses DHCPv6+rtsol to assign IP address and routing configuration for the IPv6.

1. Add ipv6 in the Ionos panel
2. `pkg install dual-dhclient-daemon`
3. Update `/etc/rc.conf`:
```conf
...
ifconfig_vtnet0="DHCP"
dhclient_program="/usr/local/sbin/dual-dhclient"
ipv6_activate_all_interfaces="YES"
ipv6_defaultrouter="fe80::1%vtnet0"
...
```
4. Update `/etc/dhclient.conf`:
```conf
...
interface "vtnet0" {
  ignore host-name;
}
...
```
5. `service dhclient restart vtnet0`
