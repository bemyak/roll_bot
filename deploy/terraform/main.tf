terraform {
  required_version = ">= 0.12.0"
}

resource "oci_core_instance" "rollbot" {
  availability_domain = local.availability_domain[0]
  compartment_id      = var.tenancy_ocid
  display_name        = "rollbot"
  shape               = local.instance_shape

  create_vnic_details {
    subnet_id        = oci_core_subnet.rollbotSubnet.id
    display_name     = "primaryvnic"
    assign_public_ip = true
    hostname_label   = "rollbot"
  }

  source_details {
    source_type = "image"
    source_id   = local.images[var.region]
  }

  metadata = {
    ssh_authorized_keys = var.ssh_public_key
  }

}

resource "oci_core_instance" "monitor" {
  availability_domain = local.availability_domain[0]
  compartment_id      = var.tenancy_ocid
  display_name        = "monitor"
  shape               = local.instance_shape

  create_vnic_details {
    subnet_id        = oci_core_subnet.rollbotSubnet.id
    display_name     = "primaryvnic"
    assign_public_ip = false
    hostname_label   = "monitor"
  }

  source_details {
    source_type = "image"
    source_id   = local.images[var.region]
  }

  metadata = {
    ssh_authorized_keys = var.ssh_public_key
  }

}

// https://docs.cloud.oracle.com/en-us/iaas/images/image/548dfe4b-282c-41a0-b151-69a3a4fed8a1/
// Canonical-Ubuntu-20.04-Minimal-2020.04.23-0
locals {
  images = {
    ap-hyderabad-1   = "ocid1.image.oc1.ap-hyderabad-1.aaaaaaaa5fauvvguntdoat7tishuzvpmtyhczppv7dxo4mmrl6rrdr5mhela"
    ap-melbourne-1   = "ocid1.image.oc1.ap-melbourne-1.aaaaaaaa4b65f4jd7fmbsnrpvv5yaq7hsmmyv3ekti54updp3w4vmi6szlka"
    ap-mumbai-1      = "ocid1.image.oc1.ap-mumbai-1.aaaaaaaav4gtyj2rw7vg2laqm5k4ylpafryxa4q5gg46wkxrx7xduz3kwkha"
    ap-osaka-1       = "ocid1.image.oc1.ap-osaka-1.aaaaaaaabt27yritdv7w4mbo3g5vhv5o3saowepzktvfrhvhl33fak56iwpq"
    ap-seoul-1       = "ocid1.image.oc1.ap-seoul-1.aaaaaaaafjufd5ugdoe76alz7st6gkiljhcnlhotea2nky4oojjygl62y5hq"
    ap-sydney-1      = "ocid1.image.oc1.ap-sydney-1.aaaaaaaayizobnw6m3lo34j6j6dtffaluosmgnerq5ravxuhtehdow67ip3q"
    ap-tokyo-1       = "ocid1.image.oc1.ap-tokyo-1.aaaaaaaat4kfe427svas2tocgp3sz6py6hr3od7tgsohuw43lm3fl3gcioiq"
    ca-montreal-1    = "ocid1.image.oc1.ca-montreal-1.aaaaaaaaveh5p5e7vzddwt7ekrh7epxbuqbnbf6kkubtnk5hyla5mvkra5ya"
    ca-toronto-1     = "ocid1.image.oc1.ca-toronto-1.aaaaaaaaaxo4jpb2tojcaai4ex5qf3wdtvmqgcs7ux5dhcmmke3yxnmmozia"
    eu-amsterdam-1   = "ocid1.image.oc1.eu-amsterdam-1.aaaaaaaawsvb7mxgqkroo6uaqxw3y53jbox3vyksa5znqralfo6o6qur5j6a"
    eu-frankfurt-1   = "ocid1.image.oc1.eu-frankfurt-1.aaaaaaaaukhdqqokh3evzyqvashnxld2gyl2wx6k5cratnb7hcxij4u7eh3q"
    eu-zurich-1      = "ocid1.image.oc1.eu-zurich-1.aaaaaaaawa5o7jwrrrd76i4m4rrox6hmea64d735otloujdimrw7uoj3kr6a"
    me-jeddah-1      = "ocid1.image.oc1.me-jeddah-1.aaaaaaaaqi45j2kng34kya6o54kfca5vyib72suzk5u4jmyvssy7gljcrcia"
    sa-saopaulo-1    = "ocid1.image.oc1.sa-saopaulo-1.aaaaaaaat6zascrpkjopqo52cfpyh7pjdt6t6zopqbh5xkhajlyebuediuaa"
    uk-gov-london-1  = "ocid1.image.oc4.uk-gov-london-1.aaaaaaaambs5vyhf7eck6crcsakzhh4d6jd7hsdvrtbfcza3rve4szs7qifq"
    uk-london-1      = "ocid1.image.oc1.uk-london-1.aaaaaaaalzhr65u6h3gbq3p2nc2ooke3xss7vkc2riwzsuc66kf75jifthzq"
    us-ashburn-1     = "ocid1.image.oc1.iad.aaaaaaaaxjheaxicr5omchcu4mxpb3cyivvxb2ptey4x3xtkd5ce5gqbwm2a"
    us-gov-ashburn-1 = "ocid1.image.oc3.us-gov-ashburn-1.aaaaaaaaxw3uon4fga7zgaznwgwhkai5ppxlixvs67d7vgqzk57fqtvyggha"
    us-gov-chicago-1 = "ocid1.image.oc3.us-gov-chicago-1.aaaaaaaayi4wxk7xyleqt26fyih3feakvdfxfxintamtji4ws2aghv764ria"
    us-gov-phoenix-1 = "ocid1.image.oc3.us-gov-phoenix-1.aaaaaaaajrv7qhgeuiaknllm6qdek3bewzp4k5ibhuk5bslrzuhm2nrodjgq"
    us-langley-1     = "ocid1.image.oc2.us-langley-1.aaaaaaaarvu6zumkvhpjnth6zurajs6f6xbglfkqwc2g7ncjeoonom5gfk7a"
    us-luke-1        = "ocid1.image.oc2.us-luke-1.aaaaaaaam4as5zbc4rzkgwc62jsgdgjmscmkn7vueihhjqhbwulrxjziswra"
    us-phoenix-1     = "ocid1.image.oc1.phx.aaaaaaaat3pk73rqpon3ul4prbn7ktul3nedjqadkwf4sjlwqiifyawyjf3a"
  }

  instance_shape = "VM.Standard.E2.1.Micro"

  availability_domain = [for limit in data.oci_limits_limit_values.test_limit_values : limit.limit_values[0].availability_domain if limit.limit_values[0].value > 0]
}
