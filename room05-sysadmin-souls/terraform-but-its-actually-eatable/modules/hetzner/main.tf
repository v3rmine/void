data "hcloud_ssh_key" "default" {
  name = var.ssh_public_key_name
}

resource "hcloud_server" "default" {
  name = var.hcloud_server_name
  labels = { "os" = "nixos" }

  server_type = var.hcloud_server_type
  datacenter = var.hcloud_server_datacenter

  # Image is ignored, as we boot into rescue mode, but is a required field
  image = "fedora-41"
  rescue = "linux64"
  ssh_keys = [data.hcloud_ssh_key.default.id]

  public_net {
    ipv4_enabled = var.ipv4_enabled
    ipv6_enabled = var.ipv6_enabled
  }

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

  # Install NixOS in rescue mode using nixos-anywhere
  provisioner "remote-exec" {
    inline = [
      "set -x",
      # TODO: Install NixOS using nixos-anywhere
      # Exit rescue mode and boot into coreos
      # "reboot"
    ]
  }

  # Wait for the server to be available
  # provisioner "local-exec" {
  #   command = "until nc -zv ${self.ipv6_address} 22; do sleep 15; done"
  # }

  # Deploy config using deploy-rs
}
