## Cloud-Native Gauntlet

A fully offline, idempotent, local, cloud-native stack to torture your laptop and earn bragging rights. This repo is structured to implement the full assignment in progressive, testable steps.

### High-Level Goals
- End-to-end local cluster with K3s on Vagrant VMs
- Infra provisioned via Terraform, configured via Ansible
- Rust API (Axum) with JWT auth, Postgres (CloudNativePG), SQLx
- Keycloak for identity and JWT validation
- GitOps via Gitea + ArgoCD (offline)
- Service mesh with Linkerd (mTLS, viz)
- Offline registry and image mirroring
- Idempotent scripts and reproducible flows
- Documentation and Mermaid diagrams

### Repository Layout (initial)
```
ansible/
  playbooks/
  roles/
apps/
  rust-api/
docs/
scripts/
terraform/
```

As we progress, we will fill in each section with working, testable components.

### Work Plan (Twelve Trials mapped)
1. Summon the Cluster Beasts
   - Vagrant: 1–2 Ubuntu VMs with private networking
   - Terraform: render inventory and variables for Ansible
   - Ansible: base packages, users, SSH hardening, K3s prerequisites
   - Offline DNS/hosts scaffolding; optional local registry
2. Forge the Application
   - Axum + SQLx + JWT (Keycloak-compatible) + migrations
   - Entities: `User`, `Task`
3. Containerize
   - Multi-stage Dockerfile, local build, load to registry
4. Database & Deployment
   - CloudNativePG operator + Postgres cluster CR
   - K8s manifests: Deployment, Service, Ingress, ConfigMaps, Secrets
5. Keycloak
   - Deployment, realm/client bootstrap, JWT validation in app
6. GitOps
   - Gitea; ArgoCD sync from infra repo
7. Mesh
   - Linkerd + viz, namespace injection, mTLS verification
8. Documentation
   - Mermaid diagrams: architecture, pipeline, auth flow

### Idempotence Principles
- All scripts and playbooks must be safe to re-run
- Terraform state kept locally; Ansible uses checks and handlers
- Kubernetes uses declarative manifests with clear ownership

### Prerequisites
- Host: Linux with Vagrant + VirtualBox/Libvirt (or Multipass w/ minor tweaks)
- Enough disk and RAM (recommend 16GB RAM, 4 CPUs minimum)
- Bash, Make, Docker/Podman locally for builds and image mirroring

### Getting Started (will evolve)
```bash
# 0) Ensure Vagrant and a provider are installed
vagrant --version

# 1) Bring up base VMs (coming next step)
vagrant up

# 2) Bootstrap infra with Terraform → Ansible
# make bootstrap (TBD)

# 3) Install K3s and validate (TBD)
# make k3s
```

### Diagrams
Diagrams will be added under `docs/diagrams` and kept in Mermaid format. Rendered previews will be committed as images for offline viewing.

### License
MIT


