locals {
  nixos_env = join(" ", [
    for key, value in var.nixos_env : "${key}=${value}"
  ])
}

resource "proxmox_virtual_environment_container" "default" {
  node_name = var.proxmox_node_name
  tags      = ["terraform"]

  unprivileged = var.unprivileged_container

  features {
    nesting = true
  }

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

  console {
    type = "console"
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
    template_file_id = "local:vztmpl/nixos-24.11-lxc-x86_64-linux.tar.xz"
    type             = "nixos"
  }

  network_interface {
    name     = "veth0"
    bridge   = var.network_bridge
    firewall = var.firewall_enabled
  }

  connection {
    type = "ssh"
    user = "root"
    host = "proxmox"
  }
  provisioner "remote-exec" {
    inline = [
      "mkdir -p /tmp/${self.vm_id}"
    ]
  }
  provisioner "file" {
    source      = "configuration.nix"
    destination = "/tmp/${self.vm_id}/configuration.nix"
  }
  // Add extra config lines in container config
  provisioner "remote-exec" {
    inline = flatten([
      ["pct stop ${self.vm_id}"],
      [
        for line in var.extra_conf : "echo '${line}' >> /etc/pve/lxc/${self.vm_id}.conf"
      ],
      ["pct start ${self.vm_id}"]
    ])
  }
  // Create var.files
  provisioner "remote-exec" {
    inline = flatten([
      ["echo 'Provisioning var.files'"],
      [for path, content in var.files : join("", [
        "echo '${content}'",
        "| pct exec ${self.vm_id} -- sh -c '",
        join("; ", [
          "source /etc/set-environment",
          "mkdir -p $(dirname ${path})",
          "cat > ${path}",
        ]),
        "'"
      ])]
    ])
  }

  // Setup NixOS
  provisioner "remote-exec" {
    inline = [
      join("", [
        "cat /tmp/${self.vm_id}/configuration.nix",
        "| pct exec ${self.vm_id} -- sh -c '",
        join("; ", [
          "source /etc/set-environment",
          "until ping -c 1 1.1.1.1 >/dev/null 2>&1; do echo \"Waiting for internet connection...\"; sleep 2; done",
          "passwd --delete root",
          "cat > /etc/nixos/configuration.nix",
          "nix-channel --update",
          "env ${local.nixos_env} nixos-rebuild switch --impure --upgrade",
          "nix-collect-garbage",
        ]),
        "'",
      ]),
      "rm -r /tmp/${self.vm_id}"
    ]
  }

  // Restart to be in a fresh state
  provisioner "remote-exec" {
    inline = [
      "pct stop ${self.vm_id}",
      "pct start ${self.vm_id}"
    ]
  }
}
