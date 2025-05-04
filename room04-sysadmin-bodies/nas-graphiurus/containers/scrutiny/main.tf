terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox-scrutiny"
  }
}

module "scrutiny" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-nixos-lxc"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  hostname               = "scrutiny"
  cpu_cores              = 1
  dedicated_memory       = 512
  swap_memory            = 512
  disk_size              = 10
  unprivileged_container = true
  protection             = false

  extra_conf = [
    "lxc.mount.entry: /run/udev run/udev none bind,ro",
    "lxc.mount.entry: /dev/sda dev/sda none bind,create=file",
    "lxc.mount.entry: /dev/sdb dev/sdb none bind,create=file",
    "lxc.cap.drop:",
    "lxc.cap.drop: mac_admin mac_override sys_time sys_module",
  ]
  files = {
    "/etc/sshd/authorized_keys" = var.ssh_public_key
  }
}
