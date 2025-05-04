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
  base_image       = "iso/debian-12-genericcloud-amd64.img"
  cpu_cores        = 4
  cpu_vcpus        = 4
  dedicated_memory = 2048
  floating_memory  = 1024
  disk_size        = 10

  qemu_agent_enabled   = false
  cloud_init_user_data = file("./cloud-init.yml")

  machine = "q35"
  # pci_passthrough = [
  #   { id = "06:00.0" },
  #   { id = "06:00.1" }
  # ]

  # passthrough_devices = [
  #   "/dev/disk/by-id/ata-ST4000DM004-2U9104_ZFN5L7XV",
  #   "/dev/disk/by-id/ata-ST4000DM004-2U9104_ZFN5LLMA"
  # ]


  #   files_path = {
  #     "/root/hdd_key" = "../../../../.secrets/files/hdd_keyfile"
  #   }
}
