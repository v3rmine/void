terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-zimaos"
  }
}

module "zimaos" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-vm"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  vm_name          = "zimaos"
  base_image       = "iso/zimaos-1.4.0.img"
  cpu_cores        = 4
  cpu_vcpus        = 4
  dedicated_memory = 2048
  floating_memory  = 1024
  disk_size        = 24
  firewall_enabled = false

  bios = "ovmf"

  # passthrough_devices = [
  #   "/dev/disk/by-id/ata-ST4000DM004-2U9104_ZFN5L7XV",
  #   "/dev/disk/by-id/ata-ST4000DM004-2U9104_ZFN5LLMA"
  # ]
}
