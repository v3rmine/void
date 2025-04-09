variable "tenancy_ocid" {
  description = "Tenancy ocid where to create the sources"
  type        = string
}

variable "user_ocid" {
  description = "Ocid of user that terraform will use to create the resources"
  type        = string
}

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
  default     = "eu-paris-1"
}

variable "compartment_ocid" {
  description = "Compartment ocid where to create all resources"
  type        = string
}

variable "ssh_public_key" {
  description = "Public key to be used for ssh access"
  type        = string
}
