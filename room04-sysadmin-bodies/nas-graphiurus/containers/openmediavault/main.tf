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
  cpu_cores              = 4
  dedicated_memory       = 2048
  swap_memory            = 1024
  disk_size              = 10
  unprivileged_container = true

  proxmox_cloud_init   = true
  cloud_init_user_data = file("./cloud-init.yml")

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

  files_text = {
    "/etc/crypttab" = <<EOF
parity1 UUID=2580faaf-42b6-4a45-a97c-3b8482e6bcb6 /root/hdd_key luks
disk1 UUID=dbb1e30c-44f3-4541-8941-2452a544b93b /root/hdd_key luks
EOF
  }

  files_path = {
    "/root/hdd_key" = "../../../../.secrets/files/hdd_keyfile"
  }

  passthrough_devices = [
    {
      path = "/dev/sda"
    },
    {
      path = "/dev/sdb"
    }
  ]
}
