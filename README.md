# Cloud-Native Gauntlet 🚀

A comprehensive, production-ready cloud-native stack built entirely offline. This project demonstrates modern cloud-native technologies, practices, and patterns in a self-contained environment.

## 🎯 Project Overview

The Cloud-Native Gauntlet is a 12-day journey through building a complete cloud-native stack from scratch. It includes everything from basic infrastructure setup to advanced service mesh implementation, all running in an offline environment.

## 🏗️ Architecture

### Core Components
- **Kubernetes (K3s)**: Lightweight Kubernetes distribution
- **Rust API**: High-performance REST API with JWT authentication
- **Keycloak**: Identity and Access Management (IAM)
- **ArgoCD**: GitOps continuous deployment
- **Gitea**: Self-hosted Git repository
- **Tekton**: Kubernetes-native CI/CD pipelines
- **Traefik**: Ingress controller and reverse proxy
- **PostgreSQL**: Database (CloudNativePG operator)

### Network Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                    Cloud-Native Gauntlet                   │
├─────────────────────────────────────────────────────────────┤
│  Ingress Layer (Traefik)                                   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ api.local   │ │keycloak.local│ │argocd.local │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  Application Layer                                          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ Rust API    │ │  Keycloak   │ │   ArgoCD    │          │
│  │ (Port 3000) │ │ (Port 8080) │ │ (Port 8080) │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  Data Layer                                                 │
│  ┌─────────────┐ ┌─────────────┐                          │
│  │ PostgreSQL  │ │   Gitea     │                          │
│  │ (CloudNativePG)│ (Port 3000) │                          │
│  └─────────────┘ └─────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Ubuntu 20.04+ or macOS
- Multipass installed
- 8GB+ RAM available
- 20GB+ disk space

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd Cloud-Native-Gauntlet
   ```

2. **Run the setup script**
   ```bash
   chmod +x scripts/setup.sh
   ./scripts/setup.sh
   ```

3. **Access the applications**
   - Rust API: http://api.local
   - Keycloak: http://keycloak.local
   - ArgoCD: http://argocd.local
   - Gitea: http://gitea.local

## 📋 12-Day Journey

### Day 1-2: Infrastructure Setup
- [x] Multipass VM creation
- [x] K3s Kubernetes cluster
- [x] Ansible automation
- [x] Basic networking

### Day 3-4: Application Development
- [x] Rust API development
- [x] JWT authentication
- [x] Database integration
- [x] API testing

### Day 5-6: Containerization
- [x] Docker image creation
- [x] Local registry setup
- [x] Kubernetes deployment
- [x] Service exposure

### Day 7-8: Identity Management
- [x] Keycloak deployment
- [x] Realm configuration
- [x] JWT integration
- [x] Authentication flow

### Day 9-10: DevOps & GitOps
- [x] ArgoCD installation
- [x] Gitea setup
- [x] CI/CD pipeline
- [x] GitOps workflow

### Day 11-12: Advanced Features
- [x] Service mesh (simplified)
- [x] Documentation
- [x] Architecture diagrams
- [x] Final testing

## 🛠️ Technology Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Authentication**: JWT (via Keycloak)
- **Database**: PostgreSQL (CloudNativePG)
- **State Management**: In-memory (RwLock)

### Infrastructure
- **Container Runtime**: containerd
- **Container Registry**: Local registry
- **Service Mesh**: Linkerd (optional)
- **Monitoring**: Built-in Kubernetes monitoring

### DevOps
- **CI/CD**: Tekton Pipelines
- **GitOps**: ArgoCD
- **Git**: Gitea
- **IaC**: Ansible

## 🔧 Configuration

### Environment Variables
```bash
# Keycloak Configuration
KEYCLOAK_URL=http://keycloak.local
KEYCLOAK_REALM=gauntlet
KEYCLOAK_CLIENT_ID=rust-api

# Database Configuration
DATABASE_URL=postgresql://user:password@postgres:5432/gauntlet

# API Configuration
API_PORT=3000
API_HOST=0.0.0.0
```

### Kubernetes Namespaces
- `app`: Rust API application
- `keycloak`: Identity management
- `argocd`: GitOps deployment
- `gitea`: Git repository
- `tekton-pipelines`: CI/CD pipelines

## 🔐 Security Features

### Authentication & Authorization
- **Multi-factor Authentication**: Keycloak integration
- **JWT Tokens**: Secure API access
- **Role-based Access Control**: Granular permissions
- **OIDC Integration**: Standard authentication flow

### Network Security
- **TLS Encryption**: All traffic encrypted
- **Ingress Security**: Traefik security policies
- **Network Policies**: Kubernetes network segmentation
- **Service Mesh**: mTLS communication (optional)

### Container Security
- **Image Scanning**: Container vulnerability scanning
- **Runtime Security**: Kubernetes security contexts
- **Secret Management**: Kubernetes secrets and ConfigMaps

## 📊 Monitoring & Observability

### Built-in Monitoring
- **Kubernetes Metrics**: Node and pod metrics
- **Application Logs**: Structured logging via tracing
- **Health Checks**: Kubernetes liveness/readiness probes

### Custom Metrics
- **API Metrics**: Request count, latency, errors
- **Authentication Metrics**: Login attempts, token validation
- **Database Metrics**: Connection pool, query performance

## 🚨 Troubleshooting

### Common Issues

#### API Routing Issues
```bash
# Check API logs
kubectl logs -n app deployment/rust-api

# Verify service endpoints
kubectl get endpoints -n app

# Test API directly
curl -v http://api.local/health
```

#### Authentication Problems
```bash
# Check Keycloak status
kubectl get pods -n keycloak

# Verify JWT token
curl -H "Authorization: Bearer <token>" http://api.local/api/tasks

# Check Keycloak logs
kubectl logs -n keycloak deployment/keycloak
```

#### Database Connection Issues
```bash
# Check PostgreSQL status
kubectl get pods -n postgres

# Verify database connectivity
kubectl exec -it <postgres-pod> -- psql -U user -d gauntlet

# Check connection logs
kubectl logs -n postgres deployment/postgres
```

### Debugging Commands
```bash
# Get all resources
kubectl get all --all-namespaces

# Check ingress status
kubectl get ingress --all-namespaces

# View service endpoints
kubectl get endpoints --all-namespaces

# Check pod logs
kubectl logs -f <pod-name> -n <namespace>
```

## 📈 Performance

### Expected Performance
- **API Response Time**: < 100ms (95th percentile)
- **Throughput**: 1000+ requests/second
- **Memory Usage**: < 100MB per API instance
- **CPU Usage**: < 50% per API instance

### Optimization Strategies
- **Connection Pooling**: Database connection optimization
- **Caching**: In-memory caching strategies
- **Load Balancing**: Traefik load balancing
- **Resource Limits**: Kubernetes resource management

## 🔄 CI/CD Pipeline

### Automated Workflow
1. **Code Push** → Gitea repository
2. **Webhook** → Tekton pipeline trigger
3. **Build** → Docker image creation
4. **Test** → Automated testing
5. **Deploy** → ArgoCD GitOps deployment
6. **Monitor** → Application monitoring

### Pipeline Stages
- **Build**: Compile Rust application
- **Test**: Run unit and integration tests
- **Security**: Container vulnerability scanning
- **Deploy**: Kubernetes deployment
- **Verify**: Health checks and smoke tests

## 📚 Learning Resources

### Documentation
- [Architecture Overview](ARCHITECTURE.md)
- [API Documentation](apps/rust-api/README.md)
- [Deployment Guide](k8s/README.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)

### External Resources
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Keycloak Documentation](https://www.keycloak.org/documentation)
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)

## 🤝 Contributing

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

### Code Standards
- **Rust**: Follow Rust formatting standards
- **Kubernetes**: Use standard YAML formatting
- **Documentation**: Update README and comments
- **Testing**: Include unit and integration tests

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Kubernetes Community**: For the amazing orchestration platform
- **Rust Community**: For the high-performance language
- **Cloud Native Foundation**: For the ecosystem tools
- **Open Source Contributors**: For all the amazing tools

## 📞 Support

### Getting Help
- **Issues**: Create a GitHub issue
- **Discussions**: Use GitHub discussions
- **Documentation**: Check the docs folder
- **Community**: Join our Discord server

### Contact
- **Email**: support@cloud-native-gauntlet.dev
- **Website**: https://cloud-native-gauntlet.dev
- **Twitter**: @CloudNativeGauntlet

---

**Built with ❤️ by the Cloud-Native Gauntlet Team**

*Empowering developers to master cloud-native technologies, one day at a time.*