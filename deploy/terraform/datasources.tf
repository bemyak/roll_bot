data "oci_identity_availability_domains" "ADs" {
  compartment_id = var.tenancy_ocid
}

data "oci_limits_services" "test_services" {
  compartment_id = var.tenancy_ocid

  filter {
    name   = "name"
    values = ["compute"]
  }
}

data "oci_limits_limit_values" "test_limit_values" {
  count          = length(data.oci_identity_availability_domains.ADs.availability_domains)
  compartment_id = var.tenancy_ocid
  service_name   = data.oci_limits_services.test_services.services.0.name

  availability_domain = data.oci_identity_availability_domains.ADs.availability_domains[count.index].name
  name                = "vm-standard-e2-1-micro-count"
  scope_type          = "AD"
}

data "oci_core_private_ips" "monitor-ip" {
  ip_address = oci_core_instance.monitor.private_ip
  subnet_id  = oci_core_subnet.rollbotSubnet.id
}
