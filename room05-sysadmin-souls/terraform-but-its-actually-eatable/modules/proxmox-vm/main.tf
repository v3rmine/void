resource "proxmox_virtual_environment_file" "cloud_init" {
  count = var.cloud_init_user_data != null ? 1 : 0

  content_type = "snippets"
  datastore_id = "snippets"
  node_name    = var.proxmox_node_name

  source_raw {
    data      = var.cloud_init_user_data
    file_name = "${uuid()}.cloud-config.yml"
  }

  lifecycle {
    ignore_changes = [source_raw[0].file_name]
  }
}

resource "proxmox_virtual_environment_vm" "default" {
  name      = var.vm_name
  node_name = var.proxmox_node_name
  tags      = flatten([var.tags, ["tofu"]])

  # should be true if qemu agent is not installed / enabled on the VM
  stop_on_destroy = var.stop_on_destroy

  boot_order    = ["virtio0", "ide3"]
  scsi_hardware = "virtio-scsi-single"
  bios          = var.bios
  machine       = var.machine

  agent {
    enabled = var.qemu_agent_enabled
  }

  # https://github.com/bpg/terraform-provider-proxmox/issues/1639
  serial_device {
    device = "socket"
  }

  initialization {
    dynamic "user_account" {
      for_each = var.cloud_init_user_data == null ? [1] : []

      content {
        username = "root"
        password = "root"
      }
    }

    ip_config {
      ipv4 {
        address = "dhcp"
      }
      ipv6 {
        address = "dhcp"
      }
    }

    user_data_file_id = var.cloud_init_user_data != null ? proxmox_virtual_environment_file.cloud_init[0].id : null
  }

  dynamic "cdrom" {
    for_each = var.local_installation_media != null ? [1] : []

    content {
      file_id   = "local:${var.local_installation_media}"
      interface = "ide3"
    }
  }

  disk {
    datastore_id = "local-lvm"
    file_id      = var.base_image != null ? "local:${var.base_image}" : null

    interface = "virtio0"
    iothread  = true
    size      = var.disk_size
    cache     = "writeback"
    discard   = "on"
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
    firewall = var.firewall_enabled
  }

  dynamic "disk" {
    for_each = var.passthrough_disks

    content {
      aio          = "native"
      backup       = false
      cache        = "none"
      discard      = "on"
      datastore_id = ""
      file_format  = "raw"
      interface    = "scsi${disk.key + 1}"

      path_in_datastore = disk.value
    }
  }

  dynamic "hostpci" {
    for_each = var.pci_passthrough

    content {
      device = "hostpci${hostpci.key}"
      id     = hostpci.value.id
      pcie   = hostpci.value.pcie == null ? false : hostpci.value.pcie
    }
  }
}

resource "proxmox_virtual_environment_firewall_options" "default" {
  node_name = var.proxmox_node_name
  vm_id     = proxmox_virtual_environment_vm.default.vm_id

  dhcp    = true
  enabled = var.firewall_enabled
}

resource "proxmox_virtual_environment_firewall_rules" "default" {
  node_name = var.proxmox_node_name
  vm_id     = proxmox_virtual_environment_vm.default.vm_id

  dynamic "rule" {
    for_each = var.security_group != null ? [var.security_group] : []

    content {
      security_group = rule.value
    }
  }
}
