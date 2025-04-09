# =============================================================================
# Provider
# =============================================================================

variable "fingerprint" {
  description = "Fingerprint of oci api private key"
  type        = string
}

variable "private_key_path" {
  description = "Path to oci api private key used"
  type        = string
}

variable "region" {
  description = "The oci region where resources will be created"
  type        = string
  default      = "eu-paris-1"
}

variable "tenancy_ocid" {
  description = "Tenancy ocid where to create the sources"
  type        = string
}

variable "user_ocid" {
  description = "Ocid of user that terraform will use to create the resources"
  type        = string
}

variable "compartment_ocid" {
  description = "Compartment ocid where to create all resources"
  type        = string
}

# =============================================================================
# Instance
# =============================================================================

variable "instance_name" {
  description = "Name of the instance."
  type        = string
}

variable "instance_ad_number" {
  description = "The availability domain number of the instance. If none is provided, it will start with AD-1 and continue in round-robin."
  type        = number
  default     = 1
}

variable "instance_state" {
  description = "(Updatable) The target state for the instance. Could be set to RUNNING or STOPPED."
  type        = string
  default     = "RUNNING"

  validation {
    condition     = contains(["RUNNING", "STOPPED"], var.instance_state)
    error_message = "Accepted values are RUNNING or STOPPED."
  }
}

variable "ssh_public_key" {
  description = "Public SSH key to be included in the ~/.ssh/authorized_keys file for the default user on the instance."
  type        = string
  default     = null
}

variable "availability_domain" {
  description = "The availability domain number of the instance."
  type        = number
  default     = 1
}

variable "auto_iptables" {
  description = "Automatically configure iptables to allow inbound traffic."
  type        = bool
  default     = false
}

variable "instance_shape" {
  description = "The shape of an instance. It must be compatible with the Always Free Tier."
  type        = string
  default     = "VM.Standard.A1.Flex"

  validation {
    condition = contains(["VM.Standard.A1.Flex", "VM.Standard.E2.1.Micro"], var.instance_shape)
    error_message = "Shape can only be VM.Standard.A1.Flex or VM.Standard.E2.1.Micro for Always Free Tier."
  }
}

variable "instance_ocpus" {
  description = "Number of OCPUs"
  type        = number
  default     = 1
}

variable "instance_memory_in_gbs" {
  description = "Amount of Memory (GB)"
  type        = number
  default     = 6
}

variable "instance_source_type" {
  description = "The source type for the instance."
  type        = string
  default     = "image"
}

variable "boot_volume_size_in_gbs" {
  description = "Boot volume size in GBs"
  type        = number
  default     = 200

  validation {
    condition     = var.boot_volume_size_in_gbs >= 50 && var.boot_volume_size_in_gbs <= 200
    error_message = "Boot volume size must be between 50 and 200 GBs."
  }
}

variable "instance_image_ocid" {
  description = "OCID of the image to use for the instance."
  type	      = string
  # eu-marseille-1 | Canonical-Ubuntu-24.04-Minimal-aarch64-2024.10.08-0
  # eu-paris-1     | Canonical-Ubuntu-24.04-Minimal-aarch64-2025.01.31-1
  # See https://docs.oracle.com/en-us/iaas/images/ for more information
  default     = "ocid1.image.oc1.eu-paris-1.aaaaaaaal7gsuv6pdtalp3zjhufyat46kbzmooy7rpxi3nh4js2hcjmezrha"
}

variable "instance_inital_cmds" {
  description = "Initial commands to run on the instance."
  type        = list(string)
  default     = null
}
