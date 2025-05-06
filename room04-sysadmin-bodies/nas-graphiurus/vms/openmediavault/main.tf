terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-openmediavault"
  }
}

module "openmediavault" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-vm"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  vm_name          = "openmediavault"
  base_image       = "iso/debian-12-generic-amd64.img"
  cpu_cores        = 4
  dedicated_memory = 4096
  floating_memory  = 1024
  disk_size        = 20

  qemu_agent_enabled   = true
  cloud_init_user_data = file("./cloud-init.yml")

  pci_passthrough = [
    { id : "06:00.0" },
    { id : "06:00.1" },
  ]
}
