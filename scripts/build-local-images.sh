#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(cd "${SCRIPT_DIR}/.." && pwd)

cd "${REPO_ROOT}/apps/rust-api"

echo "=== Building Rust API Image Locally ==="

# Check if podman is available
if ! command -v podman >/dev/null 2>&1; then
    echo "❌ podman not found. Please install podman first."
    exit 1
fi

echo "Building Rust API image..."
podman build -t rust-api:latest .

echo "Saving image to tar file..."
podman save rust-api:latest -o rust-api.tar

echo "Loading image into K3s..."
# K3s can load images directly from tar files
multipass transfer rust-api.tar k3s:/home/ubuntu/rust-api.tar
multipass exec k3s -- sudo k3s ctr images import /home/ubuntu/rust-api.tar

echo "Tagging image for K3s..."
multipass exec k3s -- sudo k3s ctr images tag docker.io/library/rust-api:latest localhost:5000/rust-api:latest

echo "✅ Rust API image loaded into K3s!"
echo "Now update the deployment to use the local image:"
echo "kubectl set image deployment/rust-api rust-api=localhost:5000/rust-api:latest -n app"
