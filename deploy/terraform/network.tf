resource "oci_core_vcn" "rollbotVCN" {
  cidr_block     = "10.1.0.0/16"
  compartment_id = var.tenancy_ocid
  display_name   = "rollbot-vcn"
  dns_label      = "rollbotvcn"
}

resource "oci_core_subnet" "rollbotSubnet" {
  cidr_block        = "10.1.21.0/24"
  display_name      = "rollbot-subnet"
  dns_label         = "rollbotsubnet"
  security_list_ids = [oci_core_security_list.rollbotSecurityList.id]
  compartment_id    = var.tenancy_ocid
  vcn_id            = oci_core_vcn.rollbotVCN.id
  route_table_id    = oci_core_route_table.rollbotRT.id
  dhcp_options_id   = oci_core_vcn.rollbotVCN.default_dhcp_options_id
}

resource "oci_core_internet_gateway" "rollbotIG" {
  compartment_id = var.tenancy_ocid
  display_name   = "rollbot-IG"
  vcn_id         = oci_core_vcn.rollbotVCN.id
}

resource "oci_core_route_table" "rollbotRT" {
  compartment_id = var.tenancy_ocid
  vcn_id         = oci_core_vcn.rollbotVCN.id
  display_name   = "rollbot-routing-table"

  route_rules {
    destination       = "0.0.0.0/0"
    destination_type  = "CIDR_BLOCK"
    network_entity_id = oci_core_internet_gateway.rollbotIG.id
  }
}

resource "oci_core_public_ip" "monitor_ip" {
  compartment_id = var.tenancy_ocid
  lifetime       = "RESERVED"
  private_ip_id  = data.oci_core_private_ips.monitor-ip.private_ips[0].id
  display_name   = "monitor-ip"
}
