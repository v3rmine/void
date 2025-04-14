terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox"
  }
}

module "openmediavault" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox"

  proxmox_endpoint = "https://proxmox:8006/"
  proxmox_token = var.proxmox_token
  ssh_private_key_file = "~/.ssh/id_ed25519"
}
