#!/usr/bin/env bash
set -euo pipefail

INSTANCE_NAME="${1:-k3s}"
PUBKEY_PATH="${2:-$HOME/.ssh/id_rsa.pub}"

if ! command -v multipass >/dev/null 2>&1; then
  echo "multipass not found. Please install Multipass." >&2
  exit 1
fi

if [ ! -f "$PUBKEY_PATH" ]; then
  echo "Public key not found at $PUBKEY_PATH" >&2
  echo "Generate one with: ssh-keygen -t rsa -b 4096 -f $HOME/.ssh/id_rsa -N ''" >&2
  exit 1
fi

echo "Ensuring instance '$INSTANCE_NAME' exists..."
if ! multipass info "$INSTANCE_NAME" >/dev/null 2>&1; then
  echo "Creating instance '$INSTANCE_NAME' (Ubuntu LTS default)..."
  multipass launch -n "$INSTANCE_NAME"
fi

IP=$(multipass info "$INSTANCE_NAME" | awk '/IPv4/{print $2; exit}')
if [ -z "$IP" ]; then
  echo "Failed to get IP for instance $INSTANCE_NAME" >&2
  exit 1
fi

echo "Pushing SSH public key to ubuntu@$INSTANCE_NAME ($IP)..."
PUBKEY=$(cat "$PUBKEY_PATH")
multipass exec "$INSTANCE_NAME" -- bash -lc "\
  set -euo pipefail; \
  mkdir -p /home/ubuntu/.ssh; \
  touch /home/ubuntu/.ssh/authorized_keys; \
  grep -qxF '$PUBKEY' /home/ubuntu/.ssh/authorized_keys || echo '$PUBKEY' >> /home/ubuntu/.ssh/authorized_keys; \
  chown -R ubuntu:ubuntu /home/ubuntu/.ssh; \
  chmod 700 /home/ubuntu/.ssh; \
  chmod 600 /home/ubuntu/.ssh/authorized_keys"

INVENTORY_PATH="$(cd "$(dirname "$0")"/.. && pwd)/ansible/inventory.ini"
mkdir -p "$(dirname "$INVENTORY_PATH")"

cat > "$INVENTORY_PATH" <<EOF
[k3s_master]
$INSTANCE_NAME ansible_host=$IP ansible_user=ubuntu

[k3s_workers]

[k3s_cluster:children]
k3s_master
k3s_workers

[all:vars]
ansible_python_interpreter=/usr/bin/python3
EOF

echo "Inventory written to: $INVENTORY_PATH"
echo "You can test SSH with: ssh ubuntu@$IP"


