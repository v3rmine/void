output "instance" {
  value = {
    id    = proxmox_virtual_environment_container.default.id
    name  = proxmox_virtual_environment_container.default.node_name
    vm_id = proxmox_virtual_environment_container.default.vm_id
    # ipv4_addresses = proxmox_virtual_environment_vm.default.ipv4_addresses
    # ipv6_addresses = proxmox_virtual_environment_vm.default.ipv6_addresses
  }
}
