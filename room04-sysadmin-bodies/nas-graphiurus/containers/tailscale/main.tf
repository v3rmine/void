terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox"
  }
}

module "tailscale" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-lxc"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"
}
