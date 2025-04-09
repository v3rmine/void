output "instance" {
  value = {
    id = hetzner_server.default.id
    name = hetzner_server.default.name
    ipv4 = oci_core_instance.default.ipv4_address
    ipv6 = oci_core_instance.default.ipv6_address
    status = oci_core_instance.default.status
  }
}
