# GridTokenX Blockchain - Azure Deployment Guide

This guide will help you deploy the GridTokenX blockchain application to Azure using Terraform.

## Prerequisites

1. **Azure CLI** installed and configured
2. **Terraform** installed (version >= 1.0)
3. **Docker** installed for building images
4. Azure subscription with appropriate permissions

## Setup Instructions

### 1. Configure Azure CLI

```bash
az login
```

Follow the prompts to authenticate with your Azure account.

### 2. Initialize Terraform

```bash
cd terraform
terraform init
```

### 3. Configure Variables

Copy the example variables file and customize it:

```bash
cp terraform.tfvars.example terraform.tfvars
```

Edit `terraform.tfvars` with your specific configuration:

```hcl
azure_region = "East US"
environment = "dev"
project_name = "gridtokenx"
db_password = "your-secure-database-password"
```

### 4. Plan Deployment

Review what Terraform will create:

```bash
terraform plan
```

### 5. Deploy Infrastructure

```bash
terraform apply
```

Type `yes` when prompted to confirm the deployment.

## Building and Pushing Docker Image

After the infrastructure is deployed, you need to build and push your Docker image:

### 1. Login to Azure Container Registry

```bash
az acr login --name <ACR_NAME>
```

### 2. Build Docker Image

```bash
docker build -t gridtokenx-blockchain .
```

### 3. Tag and Push Image

```bash
docker tag gridtokenx-blockchain:latest <ACR_LOGIN_SERVER>/gridtokenx-blockchain:latest
docker push <ACR_LOGIN_SERVER>/gridtokenx-blockchain:latest
```

Replace `<ACR_LOGIN_SERVER>` with the actual URL from Terraform output.

## Application Configuration

The application will be configured with the following environment variables:

- `NODE_TYPE`: Type of blockchain node (validator, trader, observer)
- `ENABLE_MINING`: Whether to enable mining
- `LOG_LEVEL`: Application log level
- `DATABASE_URL`: PostgreSQL connection string

## Accessing the Application

After deployment, you can access:

- **API Endpoint**: `http://<GATEWAY_PUBLIC_IP>` (from Terraform output)
- **P2P Network**: Individual container FQDNs on port 30303
- **Azure Monitor**: Azure Portal → Monitor → Logs

## Infrastructure Components

The deployment includes:

- **Azure Container Registry (ACR)**: For storing Docker images
- **Azure Container Instances (ACI)**: For running blockchain nodes
- **Azure Database for PostgreSQL**: Managed database service
- **Application Gateway**: Load balancer with health checks
- **Virtual Network**: Private networking with subnets
- **Log Analytics Workspace**: Centralized logging
- **Application Insights**: Application performance monitoring
- **Key Vault**: Secure storage for secrets
- **Storage Account**: Persistent storage for blockchain data

## Monitoring

The deployment includes:

- Azure Monitor Dashboard with container and database metrics
- Metric alerts for critical resources
- Application Insights for application telemetry
- Log Analytics for centralized logging

## Scaling

Container instances can be manually scaled by:

1. Updating `replica_count` in `terraform.tfvars`
2. Running `terraform apply`

## Security Features

- Private subnets for database
- Virtual network isolation
- Key Vault for secure secret storage
- Encrypted database storage
- Network security groups (implicit)
- Private DNS zones for database connectivity

## Maintenance

### Updating the Application

1. Build and push new Docker image
2. Restart container instances:
   ```bash
   az container restart --resource-group <RESOURCE_GROUP> --name <CONTAINER_GROUP_NAME>
   ```

### Database Backups

- Automated daily backups (7-day retention)
- Point-in-time recovery available
- Geo-redundant backup option for production

### Monitoring Logs

```bash
az container logs --resource-group <RESOURCE_GROUP> --name <CONTAINER_GROUP_NAME> --container-name gridtokenx-blockchain --follow
```

## Cleanup

To destroy all resources:

```bash
terraform destroy
```

**Warning**: This will delete all resources including the database. Make sure to backup any important data first.

## Troubleshooting

### Common Issues

1. **Container Instances Not Starting**
   - Check container logs in Azure Portal
   - Verify Docker image exists in ACR
   - Check environment variables configuration

2. **Database Connection Issues**
   - Verify database server is running
   - Check network connectivity from container subnet
   - Verify database credentials in Key Vault

3. **Application Gateway Issues**
   - Check backend pool health
   - Verify health probe configuration
   - Review Application Gateway logs

### Useful Commands

```bash
# Check container status
az container show --resource-group <RESOURCE_GROUP> --name <CONTAINER_GROUP_NAME>

# View container logs
az container logs --resource-group <RESOURCE_GROUP> --name <CONTAINER_GROUP_NAME> --container-name gridtokenx-blockchain

# Check ACR repositories
az acr repository list --name <ACR_NAME>

# Check database status
az postgres flexible-server show --resource-group <RESOURCE_GROUP> --name <DB_SERVER_NAME>

# View Application Gateway backend health
az network application-gateway show-backend-health --resource-group <RESOURCE_GROUP> --name <GATEWAY_NAME>
```

## Production Considerations

For production deployment:

1. Use a custom domain with SSL certificate
2. Restrict `allowed_ip_ranges` to your office/VPN IP ranges
3. Enable database high availability and geo-redundancy
4. Increase container resources and replica count
5. Set up proper monitoring and alerting
6. Configure automated backups and disaster recovery
7. Use Terraform remote state storage (Azure Storage)
8. Use Premium ACR for better performance and security

## Cost Optimization

- Use Azure Cost Management for monitoring
- Schedule container instances to scale down during off-hours
- Right-size database instances based on usage
- Use Azure Advisor recommendations
- Consider Azure Reserved Instances for long-term deployments

## Networking

The deployment creates:

- **Virtual Network**: 10.0.0.0/16
- **Public Subnet**: 10.0.1.0/24 (Application Gateway)
- **Private Subnet**: 10.0.2.0/24 (Container Instances)
- **Database Subnet**: 10.0.3.0/24 (PostgreSQL with delegation)

## Security Best Practices

1. **Use Azure Key Vault** for all secrets
2. **Enable Azure Defender** for container security
3. **Configure network security groups** for additional protection
4. **Use managed identities** where possible
5. **Enable audit logging** for all resources
6. **Implement least privilege access** with Azure RBAC

## Support

For issues related to:
- **Infrastructure**: Check Terraform documentation and Azure documentation
- **Application**: Review container logs and blockchain documentation
- **Monitoring**: Use Azure Monitor dashboards and alerts
