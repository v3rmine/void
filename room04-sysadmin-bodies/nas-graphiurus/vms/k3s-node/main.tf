terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-k3s-nas"
  }
}

module "k3s-nas" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-vm"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  vm_name          = "k3s-nas"
  base_image       = "iso/debian-12-generic-amd64.img"
  cpu_cores        = 8
  dedicated_memory = 8192
  # floating_memory  = 4096
  disk_size = 64

  qemu_agent_enabled = true
  cloud_init_user_data = templatefile("./cloud-init.yml", {
    ssh_authorized_key = trimspace(file("~/.ssh/id_ed25519.pub"))
    # Encode file in base64 to escape newlines and characters
    configuration_nix = base64encode(file("configuration.nix"))
  })
}
