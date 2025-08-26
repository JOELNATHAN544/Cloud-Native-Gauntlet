terraform {
  required_version = ">= 1.3.0"
  required_providers {
    local = {
      source  = "hashicorp/local"
      version = ">= 2.4.0"
    }
    template = {
      source  = "hashicorp/template"
      version = ">= 2.2.0"
    }
    null = {
      source  = "hashicorp/null"
      version = ">= 3.2.0"
    }
  }
}

locals {
  network_prefix = var.network_prefix
  master = {
    name = "cn-master"
    ip   = "${local.network_prefix}.10"
  }
  workers = [for i in range(1, var.num_workers + 1) : {
    name = "cn-worker${i}"
    ip   = "${local.network_prefix}.1${i}"
  }]
  all_nodes = concat([local.master], local.workers)
}

data "template_file" "inventory" {
  template = file("${path.module}/inventory.tmpl")
  vars = {
    master_name = local.master.name
    master_ip   = local.master.ip
    workers     = jsonencode(local.workers)
  }
}

resource "local_file" "ansible_inventory" {
  filename = "${path.module}/../ansible/inventory.ini"
  content  = data.template_file.inventory.rendered
}

output "inventory_path" {
  value = local_file.ansible_inventory.filename
}


