terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-openmediavault"
  }
}

module "openmediavault" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-lxc"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  hostname               = "openmediavault"
  os_type                = "debian"
  os_template            = "vztmpl/debian-12-standard_12.7-1_amd64.tar.zst"
  cpu_cores              = 2
  dedicated_memory       = 1024
  swap_memory            = 1024
  disk_size              = 10
  unprivileged_container = true

  files_path = {
    "/etc/cloud/cloud.cfg.d/cloud.cfg" = "./cloud-init.yml"
  }

  extra_setup_commands = [
    "apt-get update",
    "apt-get install -y cloud-init gnupg curl",
    "curl -s https://packages.openmediavault.org/public/archive.key | gpg --dearmor -o /usr/share/keyrings/openmediavault-archive-keyring.gpg",
    # Run cloud-init which applies all the configuration specified in the YAML file
    "cloud-init modules --mode=config",
  ]

  # passthrough_devices = [
  #   {
  #     path = "/dev/sda"
  #   },
  #   {
  #     path = "/dev/sdb"
  #   }
  # ]
}
