# -*- mode: ruby -*-
# vi: set ft=ruby :

VAGRANTFILE_API_VERSION = "2"

NUM_WORKERS = 1
MASTER_MEM_MB = 4096
WORKER_MEM_MB = 4096
MASTER_CPUS = 2
WORKER_CPUS = 2
BOX_NAME = "ubuntu/jammy64"
NETWORK_PREFIX = "192.168.56"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|
  config.vm.box = BOX_NAME
  config.vm.synced_folder ".", "/vagrant", disabled: true

  # Common provisioning: ensure Python for Ansible
  common_provision = <<-SHELL
    set -euo pipefail
    sudo apt-get update -y
    sudo apt-get install -y python3 python3-apt python3-distutils apt-transport-https ca-certificates curl gnupg lsb-release
  SHELL

  # Master node
  config.vm.define "cn-master" do |master|
    master.vm.hostname = "cn-master"
    master.vm.network :private_network, ip: "#{NETWORK_PREFIX}.10"
    master.vm.provider :virtualbox do |vb|
      vb.memory = MASTER_MEM_MB
      vb.cpus = MASTER_CPUS
    end
    master.vm.provision "shell", inline: common_provision
  end

  # Worker nodes
  (1..NUM_WORKERS).each do |i|
    config.vm.define "cn-worker#{i}" do |node|
      node.vm.hostname = "cn-worker#{i}"
      node.vm.network :private_network, ip: "#{NETWORK_PREFIX}.1#{i}"
      node.vm.provider :virtualbox do |vb|
        vb.memory = WORKER_MEM_MB
        vb.cpus = WORKER_CPUS
      end
      node.vm.provision "shell", inline: common_provision
    end
  end
end


