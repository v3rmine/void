terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-pbs"
  }
}

module "pbs" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-vm"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  vm_name          = "proxmox-backup-server"
  base_image       = "iso/debian-12-generic-amd64.img"
  cpu_cores        = 2
  dedicated_memory = 2048
  floating_memory  = 1024
  disk_size        = 10

  qemu_agent_enabled   = true
  cloud_init_user_data = file("./cloud-init.yml")
}
