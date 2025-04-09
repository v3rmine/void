resource "oci_core_vcn" "default" {
  cidr_block     = "10.1.0.0/16"
  compartment_id = var.compartment_ocid
  display_name   = "${local.instance_name}-vcn"
  dns_label      = "vcn"
}

resource "oci_core_security_list" "default" {
  compartment_id = var.compartment_ocid
  vcn_id         = oci_core_vcn.default.id
  display_name   = "${local.instance_name}-security-list"

  # Allow outbound traffic on all ports for all protocols
  egress_security_rules {
    protocol    = "all"
    destination = "0.0.0.0/0"
    stateless   = false
  }

  # Allow inbound traffic on all ports for all protocols
  ingress_security_rules {
    protocol  = "all"
    source    = "0.0.0.0/0"
    stateless = false
  }

  # Allow inbound icmp traffic of a specific type
  ingress_security_rules {
    protocol  = 1
    source    = "0.0.0.0/0"
    stateless = false

    icmp_options {
      type = 3
      code = 4
    }
  }
}

resource "oci_core_subnet" "default" {
  availability_domain = data.oci_identity_availability_domain.default.name
  cidr_block          = "10.1.20.0/24"
  display_name        = "${local.instance_name}-subnet"
  dns_label           = "subnet"
  security_list_ids   = [
    oci_core_security_list.default.id
	]
  compartment_id      = var.compartment_ocid
  vcn_id              = oci_core_vcn.default.id
  route_table_id      = oci_core_vcn.default.default_route_table_id
  dhcp_options_id     = oci_core_vcn.default.default_dhcp_options_id
}

# TODO: migrate to initial_cmds variable
# resource "null_resource" "remote-exec" {

#   provisioner "remote-exec" {
#     inline = [
#       "export DATE=$(date +%Y%m%d); sudo iptables -L > \"/home/ubuntu/iptables-$DATE.bak\"",
#       "sudo sh -c 'iptables -D INPUT -j REJECT --reject-with icmp-host-prohibited 2> /dev/null; iptables-save > /etc/iptables/rules.v4;'",
#       "sudo sh -c 'iptables -D FORWARD -j REJECT --reject-with icmp-host-prohibited 2> /dev/null; iptables-save > /etc/iptables/rules.v4;'",
#     ]
#   }
# }

resource "oci_core_internet_gateway" "default" {
  compartment_id = var.compartment_ocid
  display_name   = "${local.instance_name}-igw"
  vcn_id         = oci_core_vcn.default.id
}

resource "oci_core_default_route_table" "default" {
  manage_default_resource_id = oci_core_vcn.default.default_route_table_id
  display_name               = "${local.instance_name}-drt"

  route_rules {
    destination       = "0.0.0.0/0"
    destination_type  = "CIDR_BLOCK"
    network_entity_id = oci_core_internet_gateway.default.id
  }
}
