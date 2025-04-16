terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox"
  }
}

module "tailscale" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-nixos-lxc"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  node_name              = "tailscale"
  cpu_cores              = 1
  dedicated_memory       = 1024
  swap_memory            = 512
  disk_size              = 4
  unprivileged_container = true
}
