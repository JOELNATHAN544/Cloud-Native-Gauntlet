# ⚔️ Cloud-Native Gauntlet: Your Two-Week Ordeal ⚔️

## 🎯 Project Status: Day 6-7 (Database & Deployment)

**Current Progress**: We have successfully deployed most components to K3s, but need to resolve image availability issues to complete the stack.

### ✅ Completed Components

- **K3s Cluster**: Running on Multipass VM (Ubuntu 24.04)
- **Rust API**: Complete Axum application with JWT auth, models, and routes
- **Kubernetes Manifests**: App, Keycloak, Gitea, Registry, and CNPG deployments
- **Local Registry**: Container registry for offline image storage
- **Documentation**: Architecture docs and Mermaid diagrams

### 🔄 Current Status

- **App Namespace**: Rust API deployed (pending image)
- **Keycloak Namespace**: Identity service deployed (pending image)
- **Gitea Namespace**: Git server deployed (pending image)
- **Registry Namespace**: Local registry running successfully
- **CNPG System**: Operator deployed but needs troubleshooting

### 🚧 Next Steps

1. **Resolve Image Issues**: Build and load Rust API image locally
2. **Complete CNPG**: Fix CloudNativePG operator or use alternative
3. **Deploy ArgoCD**: Complete GitOps pipeline
4. **Install Linkerd**: Add service mesh capabilities
5. **End-to-End Testing**: Validate complete workflow

## 🏗️ Architecture Overview

### System Components

```
Host Machine (Multipass) → K3s Cluster → Application Stack
                                    ├── Rust API (Axum + JWT)
                                    ├── Keycloak (Identity)
                                    ├── Gitea (Git Server)
                                    ├── Local Registry
                                    ├── CloudNativePG
                                    ├── ArgoCD (GitOps)
                                    └── Linkerd (Service Mesh)
```

### Key Features

- **Offline-First**: All components work without internet
- **JWT Authentication**: Keycloak-managed tokens
- **GitOps Pipeline**: Gitea + ArgoCD for continuous deployment
- **Service Mesh**: Linkerd for mTLS and observability
- **Idempotent**: Safe to re-run all scripts

## 🚀 Quick Start

### Prerequisites

- Linux host with Multipass
- 16GB RAM, 4 CPUs minimum
- 50GB free disk space

### 1. Bootstrap Environment

```bash
# Start Multipass VM
multipass launch -n k3s

# Bootstrap SSH and Ansible
bash scripts/multipass_bootstrap.sh k3s $HOME/.ssh/id_ed25519.pub
```

### 2. Deploy Infrastructure

```bash
# Apply base OS configuration
ANSIBLE_CONFIG=ansible/ansible.cfg ansible-playbook -i ansible/inventory.ini ansible/playbooks/base.yml

# Deploy application stack
multipass exec k3s -- sudo k3s kubectl apply -f /home/ubuntu/app-namespace.yaml
# ... (other manifests)
```

### 3. Check Status

```bash
# View all components
scripts/status-check.sh

# Check specific namespace
multipass exec k3s -- sudo k3s kubectl -n app get all
```

## 📁 Project Structure

```
Cloud-Native-Gauntlet/
├── ansible/           # Infrastructure automation
├── apps/             # Application source code
│   └── rust-api/     # Rust API with Axum
├── k8s/              # Kubernetes manifests
│   ├── app/          # Rust API deployment
│   ├── keycloak/     # Identity service
│   ├── gitea/        # Git server
│   ├── registry/     # Local container registry
│   └── argocd/       # GitOps controller
├── scripts/           # Automation scripts
├── docs/             # Documentation and diagrams
│   └── diagrams/     # Mermaid architecture diagrams
└── terraform/        # Infrastructure as Code
```

## 🔧 Troubleshooting

### Common Issues

1. **ImagePullBackOff**: Images not available in local registry
2. **CNPG CrashLoop**: Operator compatibility issues with K3s 1.32
3. **Network Timeouts**: External registry access blocked (expected for offline mode)

### Solutions

- Use local registry for all images
- Build images locally with podman/docker
- Check pod logs: `kubectl logs <pod-name> -n <namespace>`
- Verify storage: `kubectl get pvc -A`

## 📊 Progress Tracking

### Day 1-2: ✅ Cluster Setup

- [x] Multipass VM creation
- [x] K3s installation
- [x] Base OS configuration

### Day 3-4: ✅ Application Development

- [x] Rust API with Axum
- [x] JWT authentication
- [x] Task management endpoints

### Day 5: 🔄 Containerization

- [x] Dockerfile creation
- [x] Local registry setup
- [ ] Image building and loading

### Day 6-7: 🔄 Database & Deployment

- [x] Kubernetes manifests
- [x] Component deployment
- [ ] Image availability resolution

### Day 8: 🔄 Keycloak

- [x] Deployment manifests
- [ ] Service configuration
- [ ] JWT integration

### Day 9-10: 🔄 GitOps

- [x] Gitea deployment
- [ ] ArgoCD setup
- [ ] Pipeline configuration

### Day 11: 🔄 Service Mesh

- [ ] Linkerd installation
- [ ] mTLS configuration
- [ ] Observability setup

### Day 12: 🔄 Documentation

- [x] Architecture documentation
- [x] Mermaid diagrams
- [ ] Final testing and validation

## 🎭 Comic Relief

> "In YAML, no one can hear you scream" 😱📄
>
> "kubectl describe is your friend" 🙏
>
> "When in doubt, check the logs" 🔍

## 📚 Resources

- [K3s Documentation](https://docs.k3s.io/)
- [Keycloak Documentation](https://www.keycloak.org/documentation)
- [Linkerd Documentation](https://linkerd.io/docs/)
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)

## 🏆 Victory Conditions

- [ ] Entire system runs offline
- [ ] Infra + configs are idempotent
- [ ] GitOps pipeline works
- [ ] Keycloak protects app
- [ ] Linkerd provides mTLS
- [ ] Complete documentation included

---

**Remember**: This is not a cozy group project. Each of you must suffer alone, staring at logs like hieroglyphics. But that hatred fuels victory! 🔥

_Now go forth and conquer the Cloud-Native Gauntlet!._ ⚔️
