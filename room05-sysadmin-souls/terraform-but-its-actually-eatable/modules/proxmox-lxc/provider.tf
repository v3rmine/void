provider "proxmox" {
  endpoint  = var.proxmox_endpoint
  insecure  = var.proxmox_insecure_cert
  api_token = var.proxmox_token

  ssh {
    agent    = true
    username = "root"
    private_key = file(var.ssh_private_key_file)
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
