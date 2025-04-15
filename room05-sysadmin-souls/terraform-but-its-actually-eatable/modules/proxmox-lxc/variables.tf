# =============================================================================
# Provider
# =============================================================================

variable "proxmox_endpoint" {
  description = "Proxmox API URL"
  type        = string
}

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

variable "proxmox_insecure_cert" {
  description = "Whether to ignore self-signed TLS certificate"
  type        = bool
  default     = true
}

variable "ssh_private_key_file" {
  description = "Path to SSH private key file"
  type        = string
}
