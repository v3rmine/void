locals {
  protocol_number = {
    icmp   = 1
    icmpv6 = 58
    tcp    = 6
    udp    = 17
  }

  instance_name = lower(replace(var.instance_name, "/\\s/", ""))
}
