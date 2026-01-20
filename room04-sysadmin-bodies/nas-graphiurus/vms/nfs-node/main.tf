terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-nfs-nas"
  }
}

module "nfs-nas" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-vm"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  vm_name          = "nfs-nas"
  base_image       = "iso/debian-12-generic-amd64.img"
  cpu_cores        = 4
  dedicated_memory = 4096
  floating_memory  = 2048
  disk_size        = 10

  qemu_agent_enabled = true
  cloud_init_user_data = templatefile("./cloud-init.yml", {
    ssh_authorized_key = trimspace(file("~/.ssh/id_ed25519.pub"))
    # Encode file in base64 to escape newlines and characters
    configuration_nix = base64encode(file("configuration.nix"))
  })

  pci_passthrough = [
    { id : "06:00.0" },
    { id : "06:00.1" },
  ]
}
