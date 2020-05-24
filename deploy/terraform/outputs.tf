output "monitor_public_ip" {
  value = oci_core_instance.monitor.public_ip
}

output "rollbot_public_ip" {
  value = oci_core_instance.rollbot.public_ip
}
