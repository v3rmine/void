output "instance" {
  value = {
    id             = proxmox_virtual_environment_vm.default.id
    name           = proxmox_virtual_environment_vm.default.name
    vm_id          = proxmox_virtual_environment_vm.default.vm_id
    ipv4_addresses = proxmox_virtual_environment_vm.default.ipv4_addresses
    ipv6_addresses = proxmox_virtual_environment_vm.default.ipv6_addresses
    started        = proxmox_virtual_environment_vm.default.started
  }
}
