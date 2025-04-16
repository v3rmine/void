resource "proxmox_virtual_environment_vm" "default" {
  name      = var.vm_name
  node_name = var.proxmox_node_name
  tags      = ["terraform"]

  # should be true if qemu agent is not installed / enabled on the VM
  stop_on_destroy = var.stop_on_destroy

  boot_order    = ["scsi0", "ide3"]
  scsi_hardware = "virtio-scsi-single"

  agent {
    enabled = var.qemu_agent_enabled
  }

  dynamic "cdrom" {
    for_each = var.local_installation_media != null && var.local_installation_media != "" ? [1] : []
    content {
      file_id   = "local:${var.local_installation_media}"
      interface = "ide3"
    }
  }

  disk {
    datastore_id = "local-lvm"
    interface    = "scsi0"
    iothread     = true
    size         = var.disk_size
  }

  cpu {
    cores      = var.cpu_cores
    sockets    = var.cpu_sockets
    hotplugged = var.cpu_vcpus
    type       = "x86-64-v2-AES" # recommended for modern CPUs
  }

  memory {
    dedicated = var.dedicated_memory
    # set equal to dedicated to enable ballooning
    floating = var.floating_memory
  }

  network_device {
    model    = "virtio"
    bridge   = var.network_bridge
    firewall = var.firewall
  }
}
