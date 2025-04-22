terraform {
  backend "local" {
    path = "../../../../.secrets/terraform/.state/proxmox"
  }
}

module "tailscale" {
  source = "../../../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/proxmox-nixos-lxc"

  proxmox_endpoint     = "https://proxmox:8006/"
  proxmox_username     = var.proxmox_username
  proxmox_password     = var.proxmox_password
  ssh_private_key_file = "~/.ssh/id_ed25519"

  hostname               = "tailscale"
  cpu_cores              = 1
  dedicated_memory       = 512
  swap_memory            = 512
  disk_size              = 4
  unprivileged_container = true
  protection             = true
  tags                   = ["do_not_delete"]

  extra_conf = [
    "lxc.cgroup2.devices.allow: c 10:200 rwm",
    "lxc.mount.entry: /dev/net/tun dev/net/tun none bind,create=file"
  ]
  files = {
    "/run/secrets/tailscale_key" = var.tailscale_auth_key
    "/etc/sshd/authorized_keys"  = var.ssh_public_key
  }
}
