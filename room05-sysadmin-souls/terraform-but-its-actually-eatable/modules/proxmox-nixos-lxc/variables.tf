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

# =============================================================================
# Instance
# =============================================================================

variable "node_name" {
  description = "Container node name"
  type        = string
}

variable "unprivileged_container" {
  description = "If true, the container will be unprivileged"
  type        = bool
  default     = true
}

variable "disk_size" {
  description = "Size of the VM disk in GB"
  type        = number
}

variable "cpu_cores" {
  description = "Number of CPU cores for the VM (number of CPU threads)"
  type        = number
  default     = 1
}

variable "dedicated_memory" {
  description = "Amount of dedicated memory for the VM in MB"
  type        = number
  default     = 1024
}

variable "swap_memory" {
  description = "Amount of swap memory for the VM in MB"
  type        = number
  default     = 512
}
