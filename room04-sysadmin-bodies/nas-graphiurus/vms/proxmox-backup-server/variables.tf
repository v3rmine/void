variable "proxmox_token" {
  description = "Proxmox API Token"
  type        = string
  default     = null
}

variable "proxmox_username" {
  description = "Proxmox account username"
  type        = string
  default     = null
}

variable "proxmox_password" {
  description = "Proxmox account password"
  type        = string
  default     = null
}

variable "tailscale_auth_key" {
  description = "TailScale auth key"
  type        = string
}

variable "ssh_public_key" {
  description = "SSH public key for the container"
  type        = string
}
