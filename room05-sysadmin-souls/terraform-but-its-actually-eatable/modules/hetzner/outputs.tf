output "instance" {
  value = {
    id = hcloud_server.default.id
    name = hcloud_server.default.name
    ipv4 = hcloud_server.default.ipv4_address
    ipv6 = hcloud_server.default.ipv6_address
    status = hcloud_server.default.status
  }
}
