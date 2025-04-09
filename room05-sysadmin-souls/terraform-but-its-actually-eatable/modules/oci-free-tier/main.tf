resource "oci_core_instance" "default" {
  availability_domain = data.oci_identity_availability_domain.default.name
  compartment_id      = var.compartment_ocid
  display_name        = local.instance_name
  shape               = var.instance_shape

  shape_config {
    ocpus         = var.instance_ocpus
    memory_in_gbs = var.instance_memory_in_gbs
  }

  create_vnic_details {
    subnet_id                 = oci_core_subnet.default.id
    display_name              = format("%sVNIC", replace(title(var.instance_name), "/\\s/", ""))
    assign_public_ip          = true
    assign_private_dns_record = true
    hostname_label            = local.instance_name
  }

  source_details {
    source_type             = var.instance_source_type
    source_id               = var.instance_image_ocid
    boot_volume_size_in_gbs = var.boot_volume_size_in_gbs
  }

  metadata = {
    ssh_authorized_keys = var.ssh_public_key
    # user_data           = {
    #     runcmd: var.instance_inital_cmds
    # }
  }

  timeouts {
    create = "60m"
  }
}

data "oci_identity_availability_domain" "default" {
  compartment_id = var.tenancy_ocid
  ad_number      = var.availability_domain
}
