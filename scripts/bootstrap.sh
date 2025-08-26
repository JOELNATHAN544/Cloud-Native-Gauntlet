#!/usr/bin/env bash
set -euo pipefail

# Simple bootstrap to bring Vagrant VMs up and print their IPs.

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(cd "${SCRIPT_DIR}/.." && pwd)

cd "${REPO_ROOT}"
vagrant up

echo "Nodes and IPs:"
vagrant ssh-config | awk '/Host cn-/{host=$2} /HostName/{ip=$2; print host, ip}'


