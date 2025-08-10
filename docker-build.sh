#!/bin/bash
# GridTokenX Docker Build Script
# Version: 0.1.1 - Updated August 2025

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="gridtokenx-blockchain"
VERSION="0.1.1"
REGISTRY=""  # Set this if pushing to a registry

echo -e "${BLUE}🔨 GridTokenX Blockchain Docker Build${NC}"
echo -e "${BLUE}Version: ${VERSION}${NC}"
echo

# Build the Docker image
echo -e "${YELLOW}📦 Building Docker image...${NC}"
docker build \
    --tag "${IMAGE_NAME}:${VERSION}" \
    --tag "${IMAGE_NAME}:latest" \
    --build-arg VERSION="${VERSION}" \
    --file Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Docker image built successfully${NC}"
    echo -e "${GREEN}   Image: ${IMAGE_NAME}:${VERSION}${NC}"
    echo -e "${GREEN}   Image: ${IMAGE_NAME}:latest${NC}"
else
    echo -e "${RED}❌ Docker build failed${NC}"
    exit 1
fi

# Show image size
echo
echo -e "${BLUE}📊 Image Information:${NC}"
docker images "${IMAGE_NAME}:latest" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"

# Optional: Push to registry
if [ ! -z "$REGISTRY" ]; then
    echo
    echo -e "${YELLOW}🚀 Pushing to registry...${NC}"
    docker tag "${IMAGE_NAME}:${VERSION}" "${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    docker tag "${IMAGE_NAME}:latest" "${REGISTRY}/${IMAGE_NAME}:latest"
    
    docker push "${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    docker push "${REGISTRY}/${IMAGE_NAME}:latest"
    
    echo -e "${GREEN}✅ Images pushed to registry${NC}"
fi

echo
echo -e "${GREEN}🎉 Build complete! You can now run:${NC}"
echo -e "${GREEN}   docker-compose up${NC}"
echo -e "${GREEN}   or${NC}"
echo -e "${GREEN}   docker run -p 8080:8080 -p 9000:9000 ${IMAGE_NAME}:latest${NC}"
