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

variable "ssh_username" {
  description = "Proxmox host SSH username"
  type        = string
  default     = "root"
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

variable "vm_name" {
  description = "VM name"
  type        = string
}

variable "network_bridge" {
  description = "VM's network's bridge interface"
  type        = string
  default     = "vnet0"
}

variable "local_installation_media" {
  description = "Identifier of the installation media file"
  type        = string
  default     = null
}

variable "qemu_agent_enabled" {
  description = "Whether the QEMU agent is enabled on the VM"
  type        = bool
  default     = false
}

variable "stop_on_destroy" {
  description = "Force stop the VM on destroy, should be true if qemu agent is not installed / enabled on the VM"
  type        = bool
  default     = true
}

variable "firewall_enabled" {
  description = "Enable firewall on VM's network interface"
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

variable "cpu_sockets" {
  description = "Number of CPU sockets for the VM (number of physical CPU sockets)"
  type        = number
  default     = 1
}

variable "cpu_vcpus" {
  description = "Number of virtual CPU cores for the VM (should be at most cpu_cores * cpu_sockets)"
  type        = number
  default     = 1
}

variable "dedicated_memory" {
  description = "Amount of dedicated memory for the VM in MB"
  type        = number
  default     = 1024
}

variable "floating_memory" {
  description = "Amount of floating memory for the VM in MB (set equal to dedicated to enable ballooning)"
  type        = number
  default     = 512
}

variable "security_group" {
  description = "VM's security group"
  type        = string
  default     = "internal_dmz"
}
