#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(cd "${SCRIPT_DIR}/.." && pwd)

cd "${REPO_ROOT}/apps/rust-api"

echo "Building Rust API Docker image with podman..."
podman build -t rust-api:latest .

echo "Loading image into K3s..."
# K3s can load images directly from the host
echo "Image built successfully: rust-api:latest"
echo "You can now deploy to K3s using the manifests in k8s/app/"
