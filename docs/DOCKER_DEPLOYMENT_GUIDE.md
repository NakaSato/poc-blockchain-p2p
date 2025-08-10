# GridTokenX Docker Deployment Guide

**Version:** 0.1.1 - Updated August 2025  
**Last Updated:** August 10, 2025

## Overview

This guide covers the complete Docker deployment setup for GridTokenX blockchain nodes, including development, testing, and production environments.

## Quick Start

### 1. Build the Docker Image

```bash
# Build the image
./docker-build.sh

# Or manually
docker build -t gridtokenx-blockchain:latest .
```

### 2. Choose Your Environment

```bash
# Development (single node)
./docker-manage.sh dev

# Production (full authority setup)
./docker-manage.sh prod

# Test environment
./docker-manage.sh test
```

## Docker Files Overview

### Core Files

- **`Dockerfile`** - Main container definition
- **`.dockerignore`** - Excludes unnecessary files from build context
- **`docker-build.sh`** - Automated build script
- **`docker-manage.sh`** - Management and operations script

### Compose Files

- **`docker-compose.yml`** - Full multi-authority testnet setup
- **`docker-compose.dev.yml`** - Single-node development environment
- **`docker-compose.prod.yml`** - Production-ready deployment with scaling

## Port Configuration

### Updated Port Mapping

| Service | Internal Port | External Port | Purpose |
|---------|---------------|---------------|---------|
| REST API | 8080 | 8080 | HTTP API endpoints |
| P2P Network | 9000 | 9000 | Blockchain P2P communication |
| Metrics | 9090 | 9090 | Prometheus metrics |
| Grafana | 3000 | 3000 | Monitoring dashboard |
| Nginx LB | 80/443 | 80/443 | Load balancer |

### Authority Node Ports

| Authority | API Port | P2P Port | Metrics Port |
|-----------|----------|----------|--------------|
| EGAT | 8080 | 9000 | 9090 |
| MEA | 8081* | 9001* | 9091* |
| PEA | 8082* | 9002* | 9092* |
| ERC | 8083 | 9003 | 9093 |

*Internal only, accessed via load balancer

## Environment Configurations

### Development Environment

```yaml
# docker-compose.dev.yml
services:
  dev-node:
    image: gridtokenx-blockchain:latest
    ports:
      - "8080:8080"   # API
      - "9000:9000"   # P2P
      - "9090:9090"   # Metrics
    environment:
      - RUST_LOG=debug
      - GRIDTOKEN_DEV_MODE=true
```

**Features:**
- Single authority node (EGAT)
- Debug logging enabled
- Hot-reload support
- Development database browser

### Production Environment

```yaml
# docker-compose.prod.yml
services:
  egat-node:        # Bootstrap node
  mea-node:         # Scalable (2 replicas)
  pea-node:         # Scalable (2 replicas)
  erc-node:         # Regulatory node
  nginx-lb:         # Load balancer
  prometheus-prod:  # Production metrics
  grafana-prod:     # Monitoring dashboard
```

**Features:**
- Multi-authority setup (EGAT, MEA, PEA, ERC)
- Auto-scaling for MEA and PEA nodes
- Load balancing with Nginx
- Production monitoring
- Resource limits and health checks

## Management Commands

### Using docker-manage.sh

```bash
# Build and deployment
./docker-manage.sh build              # Build Docker image
./docker-manage.sh dev                # Start development
./docker-manage.sh prod               # Start production
./docker-manage.sh stop               # Stop all services

# Monitoring and debugging
./docker-manage.sh status             # Show service status
./docker-manage.sh logs               # Show all logs
./docker-manage.sh logs egat-node     # Show specific service logs
./docker-manage.sh shell dev-node     # Open shell in container

# Maintenance
./docker-manage.sh clean              # Clean containers/volumes
./docker-manage.sh update             # Update and restart
./docker-manage.sh backup             # Backup blockchain data
```

## Configuration Files

### Authority Node Configs

Each authority type requires its own configuration:

```
config/
├── egat.toml     # EGAT authority configuration
├── mea.toml      # MEA authority configuration
├── pea.toml      # PEA authority configuration
└── erc.toml      # ERC regulatory configuration
```

### Environment Variables

```bash
# Node configuration
GRIDTOKEN_AUTHORITY_TYPE=EGAT|MEA|PEA|ERC
GRIDTOKEN_NETWORK_ID=gridtokenx-testnet|gridtokenx-mainnet
GRIDTOKEN_PRODUCTION=true|false
GRIDTOKEN_DEV_MODE=true|false

# Network configuration
GRIDTOKEN_BOOTSTRAP_PEERS=egat-node:9000
RUST_LOG=debug|info|warn|error
```

## Networking

### Docker Networks

- **gridtoken-network** (172.20.0.0/16) - Main testnet
- **gridtoken-dev** (172.21.0.0/16) - Development
- **gridtoken-prod** (172.22.0.0/16) - Production

### Service Discovery

Authority nodes discover each other through:
1. Bootstrap peers (EGAT node)
2. libp2p mDNS discovery
3. Kademlia DHT

## Monitoring

### Prometheus Metrics

Available at: `http://localhost:9090`

Key metrics:
- `gridtoken_blocks_total` - Total blocks processed
- `gridtoken_transactions_total` - Total transactions
- `gridtoken_energy_trades_total` - Energy trading activity
- `gridtoken_p2p_peers` - Connected peers

### Grafana Dashboards

Available at: `http://localhost:3000`
- Default credentials: `admin / gridtoken123`
- Pre-configured dashboards for blockchain metrics

## Storage

### Volume Management

```bash
# List volumes
docker volume ls | grep gridtoken

# Backup volume
docker run --rm -v gridtoken_egat_data:/data -v $PWD/backup:/backup ubuntu tar czf /backup/egat_data.tar.gz -C /data .

# Restore volume
docker run --rm -v gridtoken_egat_data:/data -v $PWD/backup:/backup ubuntu tar xzf /backup/egat_data.tar.gz -C /data
```

### Data Directories

```
/app/data/           # Blockchain data (Sled database)
/app/logs/           # Application logs
/app/config.toml     # Node configuration
```

## Security

### Production Security

1. **Network Security**
   - Use TLS/SSL for external connections
   - Firewall configuration for P2P ports
   - VPN access for management interfaces

2. **Container Security**
   - Non-root user execution
   - Read-only filesystem where possible
   - Resource limits and quotas

3. **Key Management**
   - Secure validator key storage
   - Key rotation procedures
   - Hardware security modules (HSM)

## Troubleshooting

### Common Issues

#### Port Conflicts
```bash
# Check port usage
sudo lsof -i :8080
sudo lsof -i :9000

# Stop conflicting services
./docker-manage.sh stop
```

#### Container Won't Start
```bash
# Check logs
./docker-manage.sh logs [service-name]

# Check container status
docker ps -a

# Rebuild if needed
./docker-manage.sh clean
./docker-manage.sh build
```

#### Network Issues
```bash
# Check Docker networks
docker network ls

# Inspect network
docker network inspect gridtoken-network

# Restart networking
docker-compose down
docker-compose up -d
```

### Health Checks

All services include health checks:
```bash
# Check health status
docker-compose ps

# Manual health check
curl -f http://localhost:8080/api/v1/health
```

## Performance Tuning

### Resource Limits

Production containers have resource limits:
```yaml
deploy:
  resources:
    limits:
      cpus: '2.0'
      memory: 4G
    reservations:
      cpus: '1.0'
      memory: 2G
```

### Database Optimization

Sled database configuration in `config.toml`:
```toml
[storage]
backend = "sled"
cache_capacity = 1000000  # 1M entries
flush_every_ms = 1000     # 1 second
compression = true
```

## Deployment Checklist

### Pre-deployment
- [ ] Build and test Docker image
- [ ] Verify configuration files
- [ ] Check port availability
- [ ] Ensure sufficient disk space
- [ ] Review security settings

### Deployment
- [ ] Deploy using appropriate compose file
- [ ] Verify all services start successfully
- [ ] Check service health endpoints
- [ ] Verify P2P connectivity
- [ ] Test API endpoints

### Post-deployment
- [ ] Monitor logs for errors
- [ ] Verify blockchain synchronization
- [ ] Test energy trading functionality
- [ ] Set up alerting and monitoring
- [ ] Document deployment specifics

## Support

For deployment issues:
1. Check logs: `./docker-manage.sh logs`
2. Verify configuration files
3. Check network connectivity
4. Review this documentation
5. Contact GridTokenX support team

---

**Next Steps:**
- Review [API Integration Guide](API_EXTERNAL_INTEGRATION.md)
- Check [Performance Configuration](PERFORMANCE_CONFIGURATION.md)
- See [Production Deployment Guide](PLATFORM_INTEGRATION_GUIDE.md)
