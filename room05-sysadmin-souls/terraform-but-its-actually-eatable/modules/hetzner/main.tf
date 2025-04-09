
resource "hcloud_ssh_key" "key" {
  name = var.ssh_public_key_name
  public_key = file(var.ssh_public_key_file)
}

resource "hcloud_server" "default" {
  name = var.hcloud_server_name
  labels = { "os" = "nixos" }

  server_type = var.hcloud_server_type
  datacenter = var.hcloud_server_datacenter

  # Image is ignored, as we boot into rescue mode, but is a required field
  image = "fedora-40"
  rescue = "linux64"
  ssh_keys = [hcloud_ssh_key.key.id]

  connection {
    host = self.ipv6_address
    timeout = "5m"
    private_key = file(var.ssh_private_key_file)
    # Root is the available user in rescue mode
    user = "root"
  }

  # Wait for the server to be available
  provisioner "local-exec" {
    command = "until nc -zv ${self.ipv6_address} 22; do sleep 5; done"
  }

  # Copy config.yaml and replace $ssh_public_key variable
  provisioner "file" {
    content = replace(file("config.yaml"), "$ssh_public_key", trimspace(file(var.ssh_public_key_file)))
    destination = "/root/config.yaml"
  }

  # Copy coreos-installer binary, as initramfs has not sufficient space to compile it in rescue mode
  provisioner "file" {
    source = "coreos-installer"
    destination = "/usr/local/bin/coreos-installer"
  }

  # Install Butane in rescue mode
  provisioner "remote-exec" {
    inline = [
      "set -x",
      # TODO: Install Butane
      # Exit rescue mode and boot into coreos
      "reboot"
    ]
  }

  # Wait for the server to be available
  provisioner "local-exec" {
    command = "until nc -zv ${self.ipv6_address} 22; do sleep 15; done"
  }

  # Configure CoreOS after installation
  provisioner "remote-exec" {
    connection {
      host = self.ipv6_address
      timeout = "1m"
      private_key = file(var.ssh_private_key_file)
      user = var.default_user
    }

    inline = [
      "sudo hostnamectl set-hostname ${self.name}"
      # Add additional commands if needed
    ]
  }
}
