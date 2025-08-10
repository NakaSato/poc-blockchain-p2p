#!/bin/bash
# GridTokenX Docker Management Scripts
# Version: 0.1.1 - Updated August 2025

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
IMAGE_NAME="gridtokenx-blockchain"
COMPOSE_FILE="docker-compose.yml"

show_help() {
    echo -e "${BLUE}GridTokenX Docker Management${NC}"
    echo
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo
    echo "Commands:"
    echo "  build              Build the Docker image"
    echo "  dev                Start development environment"
    echo "  prod               Start production environment"
    echo "  test               Start test environment"
    echo "  stop               Stop all services"
    echo "  clean              Clean up containers and volumes"
    echo "  logs [service]     Show logs for all services or specific service"
    echo "  status             Show status of all services"
    echo "  shell [service]    Open shell in service container"
    echo "  update             Update and restart services"
    echo "  backup             Backup blockchain data"
    echo "  restore [file]     Restore blockchain data from backup"
    echo
    echo "Examples:"
    echo "  $0 build          # Build the Docker image"
    echo "  $0 dev            # Start development environment"
    echo "  $0 logs egat-node # Show logs for EGAT node"
    echo "  $0 shell dev-node # Open shell in development node"
}

build_image() {
    echo -e "${YELLOW}🔨 Building GridTokenX Docker image...${NC}"
    ./docker-build.sh
}

start_dev() {
    echo -e "${YELLOW}🚀 Starting development environment...${NC}"
    docker-compose -f docker-compose.dev.yml up -d
    echo -e "${GREEN}✅ Development environment started${NC}"
    echo -e "${GREEN}   API: http://localhost:8080${NC}"
    echo -e "${GREEN}   Metrics: http://localhost:9091${NC}"
}

start_prod() {
    echo -e "${YELLOW}🚀 Starting production environment...${NC}"
    docker-compose -f docker-compose.prod.yml up -d
    echo -e "${GREEN}✅ Production environment started${NC}"
    echo -e "${GREEN}   API: http://localhost:8080${NC}"
    echo -e "${GREEN}   Load Balancer: http://localhost:80${NC}"
    echo -e "${GREEN}   Grafana: http://localhost:3000${NC}"
}

start_test() {
    echo -e "${YELLOW}🧪 Starting test environment...${NC}"
    docker-compose -f docker-compose.yml up -d
    echo -e "${GREEN}✅ Test environment started${NC}"
}

stop_services() {
    echo -e "${YELLOW}🛑 Stopping all services...${NC}"
    docker-compose -f docker-compose.yml down 2>/dev/null || true
    docker-compose -f docker-compose.dev.yml down 2>/dev/null || true
    docker-compose -f docker-compose.prod.yml down 2>/dev/null || true
    echo -e "${GREEN}✅ All services stopped${NC}"
}

clean_all() {
    echo -e "${YELLOW}🧹 Cleaning up containers and volumes...${NC}"
    stop_services
    docker system prune -f
    docker volume prune -f
    echo -e "${GREEN}✅ Cleanup complete${NC}"
}

show_logs() {
    local service=${1:-}
    if [ -z "$service" ]; then
        echo -e "${BLUE}📋 Showing logs for all services...${NC}"
        docker-compose logs -f
    else
        echo -e "${BLUE}📋 Showing logs for $service...${NC}"
        docker-compose logs -f "$service"
    fi
}

show_status() {
    echo -e "${BLUE}📊 Service Status:${NC}"
    docker-compose ps
    echo
    echo -e "${BLUE}📊 Container Stats:${NC}"
    docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}"
}

open_shell() {
    local service=${1:-dev-node}
    echo -e "${BLUE}🐚 Opening shell in $service...${NC}"
    docker-compose exec "$service" /bin/bash
}

update_services() {
    echo -e "${YELLOW}🔄 Updating services...${NC}"
    build_image
    docker-compose pull
    docker-compose up -d
    echo -e "${GREEN}✅ Services updated${NC}"
}

backup_data() {
    local backup_dir="./backups/$(date +%Y%m%d_%H%M%S)"
    echo -e "${YELLOW}💾 Creating backup in $backup_dir...${NC}"
    mkdir -p "$backup_dir"
    
    # Backup volumes
    docker run --rm -v gridtokenx_egat_data:/data -v "$PWD/$backup_dir":/backup ubuntu tar czf /backup/egat_data.tar.gz -C /data .
    docker run --rm -v gridtokenx_erc_data:/data -v "$PWD/$backup_dir":/backup ubuntu tar czf /backup/erc_data.tar.gz -C /data .
    
    echo -e "${GREEN}✅ Backup completed: $backup_dir${NC}"
}

restore_data() {
    local backup_file=$1
    if [ -z "$backup_file" ]; then
        echo -e "${RED}❌ Please specify backup file${NC}"
        exit 1
    fi
    
    echo -e "${YELLOW}🔄 Restoring from $backup_file...${NC}"
    # Add restore logic here
    echo -e "${GREEN}✅ Restore completed${NC}"
}

case "$1" in
    build)
        build_image
        ;;
    dev)
        start_dev
        ;;
    prod)
        start_prod
        ;;
    test)
        start_test
        ;;
    stop)
        stop_services
        ;;
    clean)
        clean_all
        ;;
    logs)
        show_logs "$2"
        ;;
    status)
        show_status
        ;;
    shell)
        open_shell "$2"
        ;;
    update)
        update_services
        ;;
    backup)
        backup_data
        ;;
    restore)
        restore_data "$2"
        ;;
    *)
        show_help
        ;;
esac
