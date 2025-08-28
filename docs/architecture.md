# Cloud-Native Gauntlet Architecture

## Overview

This document describes the architecture of the Cloud-Native Gauntlet project, a fully offline, cloud-native stack running on K3s with Multipass VMs.

## System Architecture

### Infrastructure Layer

- **Host Machine**: Linux with Multipass for VM management
- **VM**: Ubuntu 24.04 LTS running K3s (single-node cluster)
- **Network**: Private networking with host access

### Kubernetes Layer

- **K3s**: Lightweight Kubernetes distribution
- **Storage**: Local storage with PersistentVolumeClaims
- **Ingress**: NGINX Ingress Controller for external access

### Application Layer

- **Rust API**: Axum-based web service with JWT authentication
- **Database**: CloudNativePG operator for PostgreSQL (when working)
- **Identity**: Keycloak for JWT token management
- **GitOps**: Gitea + ArgoCD for continuous deployment
- **Service Mesh**: Linkerd for mTLS and observability

### Registry Layer

- **Local Registry**: Container registry for offline image storage
- **Image Management**: Local build and push workflow

## Component Relationships

### Authentication Flow

1. User authenticates via Keycloak
2. Keycloak issues JWT token
3. Rust API validates JWT tokens
4. Access granted to protected endpoints

### Data Flow

1. Rust API receives requests
2. JWT validation via Keycloak
3. Database operations via CloudNativePG
4. Response returned to client

### GitOps Flow

1. Code changes pushed to Gitea
2. ArgoCD detects changes
3. Automatic deployment to K3s
4. Health checks and rollbacks

## Security Features

- **mTLS**: Linkerd provides encrypted communication
- **JWT Authentication**: Keycloak-managed tokens
- **RBAC**: Kubernetes role-based access control
- **Network Policies**: Isolated namespaces

## Monitoring & Observability

- **Linkerd Viz**: Service mesh dashboard
- **Kubernetes Metrics**: Built-in monitoring
- **Application Logs**: Structured logging via tracing
- **Health Checks**: Liveness and readiness probes

## Deployment Strategy

- **Infrastructure as Code**: Terraform + Ansible
- **GitOps**: ArgoCD for continuous deployment
- **Idempotent**: Safe to re-run all scripts
- **Offline-First**: All components work without internet
