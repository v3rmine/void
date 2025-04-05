variable "hcloud_token" {
  description = "Hetzner Cloud API Token"
  type = string
}

variable "ssh_public_key_file" {
  description = "Local path to your public key"
  type = string
  default = "~/.ssh/id_rsa.pub"
}

variable "ssh_private_key_file" {
  description = "Local path to your private key"
  type = string
  default = "~/.ssh/id_rsa"
}

variable "ssh_public_key_name" {
  description = "Name of your public key to identify at Hetzner Cloud portal"
  type = string
  default = "My-SSH-Key"
}

variable "hcloud_server_type" {
  description = "vServer type name, lookup via `hcloud server-type list`"
  type = string
  default = "cx22"
}

variable "hcloud_server_datacenter" {
  description = "Desired datacenter location name, lookup via `hcloud datacenter list`"
  type = string
  default = "fsn1-dc14"
}

variable "hcloud_server_name" {
  description = "Name of the server"
  type = string
  default = "serv1"
}

