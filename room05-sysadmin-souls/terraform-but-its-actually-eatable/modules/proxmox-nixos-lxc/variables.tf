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

variable "proxmox_node_name" {
  description = "Proxmox node name"
  type        = string
  default     = "proxmox"
}

# =============================================================================
# Instance
# =============================================================================

variable "hostname" {
  description = "Container's hostname"
  type        = string
}

variable "network_bridge" {
  description = "Container's network's bridge interface"
  type        = string
  default     = "vnet0"
}

variable "unprivileged_container" {
  description = "If true, the container will be unprivileged"
  type        = bool
  default     = true
}

variable "firewall_enabled" {
  description = "Enable firewall on container's network interface"
  type        = bool
  default     = true
}

variable "disk_size" {
  description = "Size of the container disk in GB"
  type        = number
}

variable "cpu_cores" {
  description = "Number of CPU cores for the container"
  type        = number
  default     = 1
}

variable "dedicated_memory" {
  description = "Amount of dedicated memory for the container in MB"
  type        = number
  default     = 1024
}

variable "swap_memory" {
  description = "Amount of swap memory for the container in MB"
  type        = number
  default     = 512
}

variable "security_group" {
  description = "Container's security group"
  type        = string
  default     = "internal_dmz"
}

variable "passthrough_devices" {
  description = "List of devices to passthrough to the container"
  type = list(object({
    deny_write = optional(bool)
    gid        = optional(number)
    uid        = optional(number)
    mode       = optional(string)
    path       = string
  }))
  default = []
}

variable "extra_conf" {
  description = "Proxmox container config extra lines"
  type        = list(string)
  default     = []
}

variable "files" {
  description = "Files in the container"
  type        = map(string)
  default     = {}
}

variable "nixos_env" {
  description = "NixOS env variables to be passed during build"
  type        = map(string)
  default     = {}
}
