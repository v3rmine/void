# =============================================================================
# Provider
# =============================================================================
#
variable "hcloud_token" {
  description = "Hetzner Cloud API Token"
  type = string
}

# =============================================================================
# Instance
# =============================================================================

variable "ssh_private_key_file" {
  description = "Local path to your private key"
  type = string
  default = "~/.ssh/id_ed25519"
}

variable "ssh_public_key_name" {
  description = "Name of your public key to identify at Hetzner Cloud portal"
  type = string
}

variable "hcloud_server_type" {
  description = "vServer type name, lookup via `hcloud server-type list`"
  type = string
  default = "cax11"
}

variable "hcloud_server_datacenter" {
  description = "Desired datacenter location name, lookup via `hcloud datacenter list`"
  type = string
  default = "fsn1-dc14"
}

variable "hcloud_server_name" {
  description = "Name of the server"
  type = string
}

variable "default_user" {
  description = "Default user for the server"
  type = string
}

variable "ipv4_enabled" {
  description = "Enable IPv4 access"
  type = bool
  default = true
}

variable "ipv6_enabled" {
  description = "Enable IPv6 access"
  type = bool
  default = true
}
