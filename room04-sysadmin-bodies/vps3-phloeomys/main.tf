terraform {
  backend "local" {
    path = "../../.secrets/terraform/.state/hetzner"
  }
}

module "vps3-phloeomys" {
  source = "../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/hetzner"

  hcloud_token = var.hcloud_token

  hcloud_server_name = "vps3-phloeomys"
  hcloud_server_type = "cax11"
  default_user = "v3rmine"

  ssh_public_key_name = var.hcloud_ssh_public_key_name
}
