terraform {
  backend "local" {
    path = "../../../.secrets/terraform/.state/proxmox-node"
  }
}

resource "proxmox_virtual_environment_firewall_rules" "proxmox_node" {
  node_name = "proxmox"

  // Required on Cluster or Node
  rule {
    enabled = true
    type    = "in"
    action  = "ACCEPT"
    macro   = "DNS"
    iface   = "vnet0"
    dest    = "10.0.0.1"
    log     = "nolog"
    comment = "Allow DNS traffic on vnet0 to the gateway"
  }

  rule {
    enabled = true
    type    = "in"
    action  = "ACCEPT"
    macro   = "DHCPfwd"
    iface   = "vnet0"
    log     = "nolog"
    comment = "Allow DHCP forwarding on vnet0"
  }

  // To set on container/VM
  rule {
    enabled = true
    type    = "out"
    action  = "DROP"
    dest    = "192.168.50.1"
    macro   = "HTTPS"
    log     = "nolog"
    comment = "Block HTTPS traffic to LAN Gateway"
  }

  rule {
    enabled = true
    type    = "out"
    action  = "ACCEPT"
    dest    = "192.168.50.1"
    log     = "nolog"
    comment = "Allow access to LAN Gateway"
  }

  rule {
    enabled = true
    type    = "out"
    action  = "ACCEPT"
    source  = "192.168.50.0/24"
    dest    = "192.168.50.0/24"
    log     = "nolog"
    comment = "Allow LAN traffic for devices on LAN"
  }

  rule {
    enabled = true
    type    = "out"
    action  = "DROP"
    dest    = "192.168.0.0/16"
    log     = "nolog"
    comment = "Block traffic to LAN from Nodes"
  }
}
