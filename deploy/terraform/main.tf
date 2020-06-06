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

// https://docs.cloud.oracle.com/en-us/iaas/images/image/7581ea04-f340-418c-a495-2968437bea8e/
// Canonical-Ubuntu-20.04-Minimal-2020.05.19-0
locals {
  images = {
    ap-chuncheon-1   = "ocid1.image.oc1.ap-chuncheon-1.aaaaaaaa2prmyrhymbevpi3cakwlv7plryzn5igclm3yhe6ddwpip2ifsp6a"
    ap-hyderabad-1   = "ocid1.image.oc1.ap-hyderabad-1.aaaaaaaavlmx2vzskvaok6exy3326emjlvwt2mq4pzedmitay5usaslfewva"
    ap-melbourne-1   = "ocid1.image.oc1.ap-melbourne-1.aaaaaaaaeoczcg7xj3bk4dwpryhgnq565hazbvhhqwfeflaxoqscdlksucwa"
    ap-mumbai-1      = "ocid1.image.oc1.ap-mumbai-1.aaaaaaaato3huopeqzvwp235f3nxnoenjlohev3tor7mv7n2vxn5rskcrrdq"
    ap-osaka-1       = "ocid1.image.oc1.ap-osaka-1.aaaaaaaaygpmwvfp5lvk4dbxuijh7perwtgktbbcsalhoba7dyuc2mbwohwa"
    ap-seoul-1       = "ocid1.image.oc1.ap-seoul-1.aaaaaaaafrmbd52cxmoyobffrqontmfr6fxshtamouwis2qduazyutmismqq"
    ap-sydney-1      = "ocid1.image.oc1.ap-sydney-1.aaaaaaaafxql26mepevnjr4em2utv4lxm46ai3ppymfw6bnfrr7bcozjocia"
    ap-tokyo-1       = "ocid1.image.oc1.ap-tokyo-1.aaaaaaaaqyx3zeas37rdsyaomtz7txwosakqmkrha6ucm646wjlpxazg3wza"
    ca-montreal-1    = "ocid1.image.oc1.ca-montreal-1.aaaaaaaab2khlxmzrwljhyg7rhgqqgatln2cmk63cpq4judvheda3xupqplq"
    ca-toronto-1     = "ocid1.image.oc1.ca-toronto-1.aaaaaaaabs2t6xawpabyjw3lx62hyogvu7say2u2ve65yzt3fw7idpingocq"
    eu-amsterdam-1   = "ocid1.image.oc1.eu-amsterdam-1.aaaaaaaawlskfep4prrqzp5ooj2j4lqprfefjqzgsulcdulrxhzqmaqqddbq"
    eu-frankfurt-1   = "ocid1.image.oc1.eu-frankfurt-1.aaaaaaaawnnvmhojtfjvmsxuklektxhpmfhbofn4zunbv7waqmmf7z3oteba"
    eu-zurich-1      = "ocid1.image.oc1.eu-zurich-1.aaaaaaaa5a35uvzfrbqxykqsf3vgdnnrbduj7k7t5hgas26hp2seq77cbsrq"
    me-jeddah-1      = "ocid1.image.oc1.me-jeddah-1.aaaaaaaap5qszhdu5zh6fjd64734zevnxaap6seqlbe7fnxom3ppgpl5sdmq"
    sa-saopaulo-1    = "ocid1.image.oc1.sa-saopaulo-1.aaaaaaaack6sclj2pjw2c5nsr2fw3orh3yofegfgo2b5dikamllfykrqqzta"
    uk-gov-london-1  = "ocid1.image.oc4.uk-gov-london-1.aaaaaaaadoahmg7nlyuf3ragxinmrrdyo4gyavxhy3jutpwnc4p45av3lycq"
    uk-london-1      = "ocid1.image.oc1.uk-london-1.aaaaaaaadwwv7j2gjjhyhtren6gtwzn24rxf42cl75x42ycvwu4rehtjbw3q"
    us-ashburn-1     = "ocid1.image.oc1.iad.aaaaaaaad5yt6qrkhk3hur3ld26zjeyljhd4ghvevztli466sedo42tthjua"
    us-gov-ashburn-1 = "ocid1.image.oc3.us-gov-ashburn-1.aaaaaaaaxdft75dlj7jvkekmqkoz3xxinmk55ygkmkupqkqlpkbwlsh3dtzq"
    us-gov-chicago-1 = "ocid1.image.oc3.us-gov-chicago-1.aaaaaaaaaymekedikn6tzbelejgmfcc6dv2w6xv3kfi65p4xc73m4fkfae4a"
    us-gov-phoenix-1 = "ocid1.image.oc3.us-gov-phoenix-1.aaaaaaaahfg6snrlxbwj5g4r2ou54syr7xvwvs3dclbx4qjjls6szntodtaa"
    us-langley-1     = "ocid1.image.oc2.us-langley-1.aaaaaaaa4omqkjj3ia4jqjzlcyzrqarwo3fum5crkzo43i673ufgxqcivlmq"
    us-luke-1        = "ocid1.image.oc2.us-luke-1.aaaaaaaax76u2ivdza273clzzec3aqgiljjw5fu6pp2jtd6bwdziaseayn6a"
    us-phoenix-1     = "ocid1.image.oc1.phx.aaaaaaaabrsxasb7lpykfx5hzfss6n644ffx25m5ymy3najhoyebemfofsba"
  }

  instance_shape = "VM.Standard.E2.1.Micro"

  availability_domain = [for limit in data.oci_limits_limit_values.test_limit_values : limit.limit_values[0].availability_domain if limit.limit_values[0].value > 0]
}
