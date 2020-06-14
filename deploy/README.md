# Deployment

This project uses "Forever Free" hosting provided by Oracle Cloud, so no operation costs are there, but the deployment process is somewhat complicated.

## Requirements
In order to run/debug pipeline process you'll need the following environment variables set:

| Variable name | Example value | Description |
|---------------|---------------|-------------|
| `ROLL_BOT_TOKEN` | `123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefgh` | Telegram token used to talk to telegram, can be obtained from the [BotFather](https://t.me/BotFather) |
| `ROLLBOT_ADMIN_PASS` | `SuperSecretPass` | Admin password for Grafana. There might be some problems with special characters, especially with `$` |
| `ROLLBOT_DNS_NAME` | `cool-bot.tk` | Domain name for Caddy to use. Your Grafana instance will be available at this url. Can be registered for free at freenom.com |
| `TF_VAR_fingerprint` | `87:a4:51:bf:68:11:81:75:06:e0:b9:a5:d3:3a:a4:54` | Your OCI public key fingerprint. [How to get](https://docs.cloud.oracle.com/en-us/iaas/Content/API/Concepts/apisigningkey.htm) |
| `TF_VAR_oci_private_key` | `-----BEGIN RSA PRIVATE KEY-----\nMIIE...\n-----END RSA PRIVATE KEY-----` | Your OCI private key fingerprint. [How to get](https://docs.cloud.oracle.com/en-us/iaas/Content/API/Concepts/apisigningkey.htm) |
| `TF_VAR_region` | `eu-frankfurt-1` | Your OCI region. [How to get](https://docs.cloud.oracle.com/en-us/iaas/Content/General/Concepts/regions.htm) |
| `TF_VAR_ssh_public_key` | `ssh-rsa AAAAB3Nza...UIE= sgureev@behir` | Your OCI public key. [How to get](https://docs.cloud.oracle.com/en-us/iaas/Content/API/Concepts/apisigningkey.htm) |
| `TF_VAR_tenancy_ocid` | `ocid1.tenancy.oc1..aaaaaaa...` | Your OCI tenancy ocid. [How to get](https://docs.cloud.oracle.com/en-us/iaas/Content/API/Concepts/apisigningkey.htm) |
| `TF_VAR_user_ocid` | `ocid1.user.oc1..aaaaaaa...` | Your OCI user ocid. [How to get](https://docs.cloud.oracle.com/en-us/iaas/Content/API/Concepts/apisigningkey.htm) |
| `TF_STATE` | `tfstate.json` | Just use this value, no need to change it. It is to point `terraform-inventory` to correct state file |
| `TF_STATE_URL` | `https://objectstorage.eu-frankfurt-1.oraclecloud.com/p/<big_hash>/n/<small_hash>/b/tfstate/o/terraform.tfstate` | URL of your terraform state. Usually it is uploaded to Oracle Object Storage: [guide](https://docs.cloud.oracle.com/en-us/iaas/Content/API/SDKDocs/terraformUsingObjectStore.htm) |
| `TLS_EMAIL` | `cool-dev@email.com` | Email that will be used by Caddy for obtaining SSL certs |


That's a lot of stuff! To understand why all this is needed, let's talk about architecture.

## Architecture
The running "production" looks like this:
```
+---------------+         +-----------------------------------------------+
|               |         |                                               |
|               |         |                                               |
|               |         |     +-----------------------------------+     |
|               |         |     |                                   |     |
| +----------+  |         |     |       +------------------+        |     |
| |          |  |         |     |       |                  |        |     |
| |   Admin  --------------------------->       Caddy      -----+   |     |
| |          |  |         |     |       |                  |    |   |     |
| +----------+  |         |     |       +---------|--------+    |   |     |
|               |         |     |                 |             |   |     |
|               |         |     |               / |     /prometheus |     |
|               |         |     |                 |             |   |     |
|               |         |     |       +---------|--------+    |   |     |
|               |         |     |       |                  |    |   |     |
|               |         |     |       |     Grafana      |    |   |     |
|               |         |     |       |                  |    |   |     |
|               |         |     |       +------------------+    |   |     |
|               |         |     |                               |   |     |
|               |         |     |                               |   |     |
|               |         |     |                               |   |     |
|               |         |     |       +------------------+    |   |     |
|               |         |     |       |                  |    |   |     |
|               |         |     |       |    Prometheus    <----+   |     |
|               |         |     |       |                  |        |     |
|               |         |     |       +---------+--------+        |     |
|               |         |     |                 |                 |     |
|               |         |     +-monitor---------|-----------------+     |
|               |         |                       |                       |
|               |         |                       |/metrics               |
|               |         |                       |                       |
|               |         |     +-----------------|-----------------+     |
|               |         |     |                 |                 |     |
| +-----------+ |         |     |      +----------v---------+       |     |
| |           | |         |     |      |                    |       |     |
| |  Telegram <-------------------------      roll_bot      |       |     |
| |           | |    Fetch Updates     |                    |       |     |
| +-----------+ |         |     |      +--------------------+       |     |
|               |         |     |                                   |     |
|               |         |     +-rollbot---------------------------+     |
|               |         |                                               |
|               |         |                                               |
|               |         |                                               |
+Internet-------+         +-Oracle Cloud----------------------------------+
```

The main part is, of corse, `roll_bot` daemon which works on `rollbot` instance.
It constantly makes requests to Telegram server using _long poling_ method.

Everything else is needed just for monitoring purposes:
* Prometheus scrapes roll_bot metrics api and stores it in time-series database
* Grafana is a nice frontend for Prometheus that allows you to create dashboards and panels
* Caddy is needed to take care of SSL certificates and to protect prometheus UI with te basic auth

## Deployment process
### Terraform
First comes the terraform. It's role is to create Virtual Machines for further provisioning.\
Basic documentation can be found here: https://www.terraform.io/docs/providers/oci/index.html

Nothing special is going on in our case, just configuring network, instances and so on.

```bash
cd deploy/terraform
terraform init -backend-config="address=$TF_STATE_URL"
terraform apply
```

The output will contain self-describing variables:
* `monitor_public_ip`
* `rollbot_public_ip`
* `monitor_private_ip`
* `rollbot_private_ip`

`monitor_public_ip` is fixed and won't be changed. We cannot fix `rollbot_public_ip` because of "Free Tier" limitations, but that is not very important.

The domain name should be registered for this ip (`monitor_public_ip`). I suggest using `freenom.com` provider (free 2nd-level domains, yay!).

> **Note** If one of the output variable is empty, just run `terraform apply` once again

### terraform-inventory
We don't want to manually write the ansible inventory file, so we will automatically generate one using [terraform-inventory](https://github.com/adammck/terraform-inventory/)

You can install it with:
```bash
go get github.com/adammck/terraform-inventory
```

After applying terraform changes at the previous step, we'll need to generate a correct state json file:
```bash
cd deploy/terraform
terraform show -json > ../ansible/$TF_STATE
```

You can check if everything works correctly by executing:
```bash
# from deploy/terraform
cd ../ansible
terraform-inventory -inventory
```
Remember to set `TF_STATE` variable as described above!

### Ansible

Now we can use terraform-inventory as ansible [inventory plugin](https://docs.ansible.com/ansible/latest/plugins/inventory.html):
```bash
cd deploy/ansible
ansible-playbook -i ./terraform-inventory -u ubuntu site.yml
```

This will:
1. Install roll_bot on `rollbot` instance, run it with systemd
1. Install [prometheus](https://github.com/cloudalchemy/ansible-prometheus), [grafana](https://github.com/cloudalchemy/ansible-grafana) on `monitor` instance using cloudalchemy roles
1. Install and configure [Caddy](https://github.com/caddy-ansible/caddy-ansible)

Now we are ready to roll!
