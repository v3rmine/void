resource "proxmox_virtual_environment_container" "default" {
  node_name = var.proxmox_node_name
  tags      = ["terraform"]

  unprivileged = var.unprivileged_container

  initialization {
    hostname = var.hostname

    ip_config {
      ipv4 {
        address = "dhcp"
      }
      ipv6 {
        address = "dhcp"
      }
    }
  }

  cpu {
    cores        = var.cpu_cores
    architecture = "amd64"
  }

  memory {
    dedicated = var.dedicated_memory
    swap      = var.swap_memory
  }

  disk {
    datastore_id = "local-lvm"
    size         = var.disk_size
  }

  operating_system {
    template_file_id = "local:${var.os_template}"
    type             = var.os_type
  }

  network_interface {
    name     = "veth0"
    bridge   = var.network_bridge
    firewall = var.firewall_enabled
  }

  dynamic "device_passthrough" {
    for_each = var.passthrough_devices

    content {
      deny_write = device_passthrough.value.deny_write
      gid        = device_passthrough.value.gid
      uid        = device_passthrough.value.uid
      mode       = device_passthrough.value.mode
      path       = device_passthrough.value.path
    }
  }

  connection {
    type = "ssh"
    user = "root"
    host = "proxmox"
  }
  // Add extra config lines in container config
  provisioner "remote-exec" {
    inline = flatten([
      ["set -o errexit"],
      ["pct stop ${self.vm_id}"],
      [
        for line in var.extra_conf : "echo '${base64encode(line)}' | base64 -d >> /etc/pve/lxc/${self.vm_id}.conf"
      ],
      ["pct start ${self.vm_id}"]
    ])
  }


  // Create var.files_text
  provisioner "remote-exec" {
    inline = flatten([
      ["set -o errexit"],
      [for path, content in var.files_text : join("", [
        "echo '${base64encode(content)}' | base64 -d",
        "| pct exec ${self.vm_id} -- sh -c '",
        join("; ", [
          "mkdir -p $(dirname ${path})",
          "cat > ${path}",
        ]),
        "'"
      ])]
    ])
  }
  // Create var.files_path
  provisioner "remote-exec" {
    inline = flatten([
      ["set -o errexit"],
      [for path, file_path in var.files_path : join("", [
        "echo '${filebase64(file_path)}' | base64 -d",
        "| pct exec ${self.vm_id} -- sh -c '",
        join("; ", [
          "mkdir -p $(dirname ${path})",
          "cat > ${path}",
        ]),
        "'"
      ])]
    ])
  }

  // Run extra setup commands
  provisioner "remote-exec" {
    inline = flatten([
      ["set -o errexit"],
      [for command in var.extra_setup_commands : "echo '${base64encode(command)}' | base64 -d | pct exec ${self.vm_id} -- sh"]
    ])
  }

  // Restart to be in a fresh state
  provisioner "remote-exec" {
    inline = [
      "set -o errexit",
      "pct stop ${self.vm_id}",
      "pct start ${self.vm_id}"
    ]
  }
}

resource "proxmox_virtual_environment_firewall_rules" "default" {
  node_name = var.proxmox_node_name
  vm_id     = proxmox_virtual_environment_container.default.vm_id

  dynamic "rule" {
    for_each = var.security_group != null ? [var.security_group] : []

    content {
      security_group = rule.value
    }
  }
}
