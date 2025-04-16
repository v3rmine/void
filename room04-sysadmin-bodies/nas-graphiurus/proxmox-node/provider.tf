provider "proxmox" {
  endpoint  = "https://proxmox:8006"
  insecure  = true
  api_token = var.proxmox_token
  username  = var.proxmox_username
  password  = var.proxmox_password

  ssh {
    agent       = true
    username    = "root"
    private_key = file("~/.ssh/id_ed25519")
  }
}

terraform {
  required_providers {
    proxmox = {
      source  = "bpg/proxmox"
      version = ">= 0.75"
    }
  }
}
