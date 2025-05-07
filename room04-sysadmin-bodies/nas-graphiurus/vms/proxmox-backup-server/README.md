# OpenMediaVault

## Installation using cloud-init

## SATA Controller Passthrough
https://pve.proxmox.com/wiki/PCI(e)_Passthrough
https://forum.proxmox.com/threads/passthrough-of-onboard-sata-controller-locks-up-system.115569/

## OMV fstab Config
```xml
      <mntent>
        <uuid>04340e3b-3dd4-4d87-8b49-28beeace205d</uuid>
        <fsname>/dev/mapper/parity1</fsname>
        <dir>/media/parity1</dir>
        <type>xfs</type>
        <opts>defaults,noauto,nofail,x-systemd.automount,x-systemd.idle-timeout=15min,x-systemd.device-timeout=5</opts>
        <freq>0</freq>
        <passno>2</passno>
        <hidden>0</hidden>
      </mntent>
      <mntent>
        <uuid>4ab51432-fc50-4dc3-8f42-201db7fb052d</uuid>
        <fsname>/dev/mapper/disk1</fsname>
        <dir>/media/disk1</dir>
        <type>xfs</type>
        <opts>defaults,noauto,nofail,x-systemd.automount,x-systemd.idle-timeout=15min,x-systemd.device-timeout=5</opts>
        <freq>0</freq>
        <passno>2</passno>
        <hidden>0</hidden>
      </mntent>
```
