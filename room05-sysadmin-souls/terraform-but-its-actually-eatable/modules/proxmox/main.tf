resource "proxmox_virtual_environment_vm" "default" {
  name      = "openmediavault"
  node_name = "proxmox"
  tags      = ["terraform"]

  # should be true if qemu agent is not installed / enabled on the VM
  stop_on_destroy = false

  boot_order = [ "scsi0", "ide3" ]
  scsi_hardware = "virtio-scsi-single"

  agent {
    enabled = true
  }

  cdrom {
    file_id = "local:iso/openmediavault-7.4.17-x86_64-linux.iso"
    interface = "ide3"
  }

  disk {
    datastore_id = "local-lvm"
    interface    = "scsi0"
    iothread     = true
    size         = 20
  }

  cpu {
    cores        = 2
    sockets      = 1
    hotplugged   = 2
    type         = "x86-64-v2-AES"  # recommended for modern CPUs
  }

  memory {
    dedicated = 2048
    floating  = 1024 # set equal to dedicated to enable ballooning
  }

  network_device {
    model = "virtio"
    bridge = "vmbr0"
  }
}
