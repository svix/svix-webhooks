# Svix Helm Chart

This Helm chart deploys the complete Svix webhook service stack on Kubernetes, including:

- **Svix Backend**: The main webhook service
- **PostgreSQL**: Database for storing webhook data
- **PgBouncer**: Connection pooler for PostgreSQL
- **Redis**: Cache and session storage

## Prerequisites

Before installing the Svix Helm chart, ensure you have the following components:

- **Kubernetes Cluster**: A running Kubernetes cluster (version 1.19 or later)
- **kubectl**: Kubernetes command-line tool installed and configured to communicate with your cluster
- **Helm**: Helm package manager installed (version 3.x)
- **Persistent Storage**: Access to a storage class for provisioning PersistentVolumeClaims
- **Ingress Controller**: (Optional but recommended) An ingress controller like NGINX for external access

## Installation

### 1. Clone the Repository

Since this is not an official Helm chart, you'll need to clone the Svix repository:

```bash
git clone https://github.com/svix/svix-webhooks.git
cd svix-webhooks/self-hosting/helm
```

### 2. Install the Chart

```bash
# Install with default values
helm install my-svix . --namespace default --create-namespace

# Install with custom values
helm install my-svix . -f custom-values.yaml --namespace default --create-namespace

# Install in a specific namespace
kubectl create namespace svix
helm install my-svix . --namespace svix
```

### 3. Access the Service 

```bash
# Port forward to access the service
kubectl port-forward svc/my-svix-backend 8071:8071 --namespace default

# Access the Service at http://localhost:8071
```


## Production Deployment

For production deployments, consider the following:

### 1. Set Strong Secrets

```yaml
backend:
  secrets:
    jwtSecret: "your-production-jwt-secret-here"
    mainSecret: "your-production-main-secret-here"
```

### 2. Configure Ingress with TLS

```yaml
ingress:
  enabled: true
  className: "nginx"
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
  hosts:
    - host: svix.yourdomain.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: svix-tls
      hosts:
        - svix.yourdomain.com
```

### 3. Enable Autoscaling

```yaml
backend:
  autoscaling:
    enabled: true
    minReplicas: 2
    maxReplicas: 10
    targetCPUUtilizationPercentage: 70
```

### 4. Configure Resource Limits

```yaml
backend:
  resources:
    requests:
      memory: "512Mi"
      cpu: "500m"
    limits:
      memory: "1Gi"
      cpu: "1000m"
```

### 5. Use External Databases (Optional)

If you want to use external PostgreSQL and Redis instances:

```yaml
postgres:
  enabled: false

redis:
  enabled: false

backend:
  secrets:
    dbDsn: "postgresql://user:password@your-external-postgresql-host:5432/database"
    redisDsn: "redis://your-external-redis-host:6379"
```

### 6. Configure Persistence

```yaml
postgres:
  persistence:
    enabled: true
    storageClass: "fast-ssd"
    size: "50Gi"

redis:
  persistence:
    enabled: true
    storageClass: "fast-ssd"
    size: "20Gi"
```

## Monitoring and Health Checks

### Health Check Endpoints

The Svix backend provides health check endpoints:

```bash
# Check Svix health endpoint
kubectl exec -it deployment/my-svix-backend -- curl http://localhost:8071/api/v1/health

# Expected response: {"status":"ok"}
```

```

## Security Considerations

1. **JWT Secret**: Always use a cryptographically secure JWT secret (minimum 64 characters)
2. **Database Passwords**: Use strong, unique passwords for database access
3. **Network Policies**: Implement network policies to restrict pod-to-pod communication:

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: svix-network-policy
spec:
  podSelector:
    matchLabels:
      app.kubernetes.io/name: svix
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
```

4. **RBAC**: Review and adjust service account permissions as needed
5. **Secrets Management**: Use external secret management systems like:
   - AWS Secrets Manager
   - HashiCorp Vault
   - Azure Key Vault
   - Google Secret Manager

6. **Image Security**: Use specific image tags instead of `latest` and scan images for vulnerabilities


## Contributing

We welcome contributions to improve this Helm chart! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request


# Validate against Kubernetes
helm template my-svix . | kubectl apply --dry-run=client -f -
```

## License

This Helm chart is released under the MIT License. See the [LICENSE](LICENSE) file for details.

The Svix software itself is licensed separately. Please refer to the [Svix repository](https://github.com/svix/svix-webhooks) for licensing information.

## Support

For issues and questions:

- **Documentation**: Check the [Svix documentation](https://docs.svix.com/)
- **GitHub Issues**: Report bugs in the [GitHub repository](https://github.com/svix/svix-webhooks/issues)
- **Community**: Join the [Svix Discord community](https://discord.gg/svix)
- **Support**: Contact Svix support at support@svix.com
- **Enterprise**: For enterprise support and custom deployments, contact contact@svix.com

