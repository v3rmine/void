terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox"
  }
}

module "openmediavault" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-vm"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_token        = var.proxmox_token
  ssh_private_key_file = "~/.ssh/id_ed25519"

  vm_name = "openmediavault"

  qemu_agent_enabled       = true
  stop_on_destroy          = false
  local_installation_media = "iso/openmediavault-7.4.17-x86_64-linux.iso"

  disk_size        = 20
  cpu_cores        = 2
  cpu_vcpus        = 2
  dedicated_memory = 2048
  floating_memory  = 1024
}
