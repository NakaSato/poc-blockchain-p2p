#!/bin/bash

# GridTokenX Blockchain - Azure Deployment Script
# This script automates the deployment process

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="gridtokenx"
AZURE_REGION="East US"
TERRAFORM_DIR="terraform"

# Functions
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if Azure CLI is installed
    if ! command -v az &> /dev/null; then
        print_error "Azure CLI is not installed. Please install it first."
        exit 1
    fi
    
    # Check if Terraform is installed
    if ! command -v terraform &> /dev/null; then
        print_error "Terraform is not installed. Please install it first."
        exit 1
    fi
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install it first."
        exit 1
    fi
    
    # Check Azure login
    if ! az account show &> /dev/null; then
        print_error "Azure not logged in. Run 'az login' first."
        exit 1
    fi
    
    print_success "All prerequisites met!"
}

terraform_init() {
    print_status "Initializing Terraform..."
    cd $TERRAFORM_DIR
    terraform init
    cd ..
    print_success "Terraform initialized!"
}

terraform_plan() {
    print_status "Planning Terraform deployment..."
    cd $TERRAFORM_DIR
    terraform plan -out=tfplan
    cd ..
    print_success "Terraform plan completed!"
}

terraform_apply() {
    print_status "Applying Terraform configuration..."
    cd $TERRAFORM_DIR
    terraform apply tfplan
    cd ..
    print_success "Infrastructure deployed!"
}

get_acr_details() {
    cd $TERRAFORM_DIR
    ACR_LOGIN_SERVER=$(terraform output -raw container_registry_login_server)
    ACR_NAME=$(terraform output -raw container_registry_name)
    GATEWAY_IP=$(terraform output -raw application_gateway_public_ip)
    cd ..
    print_success "ACR Login Server: $ACR_LOGIN_SERVER"
    print_success "Application URL: http://$GATEWAY_IP"
}

build_and_push_image() {
    print_status "Building and pushing Docker image..."
    
    # Login to ACR
    az acr login --name $ACR_NAME
    
    # Build Docker image
    print_status "Building Docker image..."
    docker build -t $PROJECT_NAME .
    
    # Tag image
    docker tag $PROJECT_NAME:latest $ACR_LOGIN_SERVER/gridtokenx-blockchain:latest
    
    # Push image
    print_status "Pushing image to ACR..."
    docker push $ACR_LOGIN_SERVER/gridtokenx-blockchain:latest
    
    print_success "Docker image built and pushed successfully!"
}

restart_containers() {
    print_status "Restarting container instances..."
    cd $TERRAFORM_DIR
    RESOURCE_GROUP=$(terraform output -raw resource_group_name)
    CONTAINER_GROUPS=($(terraform output -json container_group_names | jq -r '.[]'))
    cd ..
    
    for container_group in "${CONTAINER_GROUPS[@]}"; do
        print_status "Restarting container group: $container_group"
        az container restart --resource-group $RESOURCE_GROUP --name $container_group
    done
    
    print_success "Container instances restarted!"
}

show_deployment_info() {
    print_status "Deployment Information:"
    cd $TERRAFORM_DIR
    echo -e "${GREEN}=== Deployment Complete ===${NC}"
    echo -e "${BLUE}Application URL:${NC} http://$(terraform output -raw application_gateway_public_ip)"
    echo -e "${BLUE}ACR Login Server:${NC} $(terraform output -raw container_registry_login_server)"
    echo -e "${BLUE}Resource Group:${NC} $(terraform output -raw resource_group_name)"
    echo -e "${BLUE}Log Analytics Workspace:${NC} $(terraform output -raw log_analytics_workspace_id)"
    
    echo -e "\n${GREEN}Useful Commands:${NC}"
    echo -e "${BLUE}View container logs:${NC} az container logs --resource-group $(terraform output -raw resource_group_name) --name $(terraform output -json container_group_names | jq -r '.[0]') --container-name gridtokenx-blockchain --follow"
    echo -e "${BLUE}Check container status:${NC} az container show --resource-group $(terraform output -raw resource_group_name) --name $(terraform output -json container_group_names | jq -r '.[0]') --query 'instanceView.state'"
    echo -e "${BLUE}ACR repositories:${NC} az acr repository list --name $(terraform output -raw container_registry_name)"
    cd ..
}

cleanup() {
    print_warning "This will destroy all Azure resources. Are you sure? (y/N)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        print_status "Destroying infrastructure..."
        cd $TERRAFORM_DIR
        terraform destroy -auto-approve
        cd ..
        print_success "Infrastructure destroyed!"
    else
        print_status "Cleanup cancelled."
    fi
}

# Main menu
show_menu() {
    echo -e "${GREEN}GridTokenX Blockchain - Azure Deployment${NC}"
    echo "1. Deploy infrastructure"
    echo "2. Build and deploy application"
    echo "3. Update application only"
    echo "4. Show deployment info"
    echo "5. Cleanup (destroy all resources)"
    echo "6. Exit"
    echo -n "Choose an option: "
}

# Main script
main() {
    while true; do
        show_menu
        read -r choice
        case $choice in
            1)
                check_prerequisites
                
                # Check if terraform.tfvars exists
                if [ ! -f "$TERRAFORM_DIR/terraform.tfvars" ]; then
                    print_warning "terraform.tfvars not found. Copying from example..."
                    cp "$TERRAFORM_DIR/terraform.tfvars.example" "$TERRAFORM_DIR/terraform.tfvars"
                    print_warning "Please edit $TERRAFORM_DIR/terraform.tfvars with your configuration, then run this script again."
                    exit 1
                fi
                
                terraform_init
                terraform_plan
                terraform_apply
                get_acr_details
                show_deployment_info
                ;;
            2)
                check_prerequisites
                get_acr_details
                build_and_push_image
                restart_containers
                show_deployment_info
                ;;
            3)
                check_prerequisites
                get_acr_details
                build_and_push_image
                restart_containers
                print_success "Application updated!"
                ;;
            4)
                show_deployment_info
                ;;
            5)
                cleanup
                ;;
            6)
                print_status "Goodbye!"
                exit 0
                ;;
            *)
                print_error "Invalid option. Please try again."
                ;;
        esac
        echo
    done
}

# Run main function
main
