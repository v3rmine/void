output "instance" {
  value = {
    id = oci_core_instance.default.id
    name = oci_core_instance.default.display_name
    public_ip = oci_core_instance.default.public_ip
    private_ip = oci_core_instance.default.private_ip
    shape = oci_core_instance.default.shape
    state = oci_core_instance.default.state
  }
}
