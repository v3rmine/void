resource "proxmox_virtual_environment_container" "default" {
  node_name    = var.node_name
  tags         = ["terraform"]
  unprivileged = var.unprivileged_container

  features {
    nesting = true
  }

  initialization {
    hostname = "tailscale"

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
    name   = "veth0"
    bridge = "vmbr0"
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
          "nixos-rebuild switch --upgrade",
          "nix-collect-garbage",
        ]),
        "'",
      ]),
      "rm -r /tmp/${self.vm_id}"
    ]
  }
}
