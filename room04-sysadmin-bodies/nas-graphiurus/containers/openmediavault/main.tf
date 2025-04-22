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
    "/etc/cloud/datasource/user-data"        = "./cloud-init.yml"
    "/etc/cloud/cloud.cfg.d/10_instance.cfg" = "./cloud-init-proxmox.yml"
  }
  files_text = {
    #     "/etc/cloud/datasource/meta-data" = <<EOF
    # instance-id: iid-local
    # dsmode: local
    #     EOF
    "/etc/cloud/datasource/meta-data" = "instance-id: iid-local"
  }

  extra_setup_commands = [
    "passwd -d root",
    "apt-get update",
    # Need ifupdown2 for network configuration in Debian LXC
    "apt-get install -y cloud-init gnupg curl ifupdown2",
    # Get openmediavault signing key
    "curl -s https://packages.openmediavault.org/public/archive.key | gpg --dearmor -o /usr/share/keyrings/openmediavault-archive-keyring.gpg",
    # We don't need systemd-networkd.service in LXC
    "systemctl disable systemd-networkd.service",
    # Enable cloud-init for next reboot
    "systemctl enable cloud-init-local.service",
    "systemctl enable cloud-init.service",
    "systemctl enable cloud-config.service",
    "systemctl enable cloud-final.service",
    "cloud-init clean --logs"
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
