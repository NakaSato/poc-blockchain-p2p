# Initial Azure Variables for GridTokenX Blockchain

# Azure Configuration
variable "azure_location" {
  description = "Azure location for resources"
  type        = string
  default     = "East US"
  
  validation {
    condition = contains([
      "East US", "East US 2", "West US", "West US 2", "West US 3",
      "Central US", "North Central US", "South Central US", "West Central US",
      "Canada Central", "Canada East", "Brazil South", "UK South", "UK West",
      "West Europe", "North Europe", "France Central", "Germany West Central",
      "Switzerland North", "Norway East", "UAE North", "South Africa North",
      "Australia East", "Australia Southeast", "Southeast Asia", "East Asia",
      "Japan East", "Japan West", "Korea Central", "India Central"
    ], var.azure_location)
    error_message = "Please provide a valid Azure location."
  }
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "dev"
  
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be dev, staging, or prod."
  }
}

variable "project_name" {
  description = "Project name used for resource naming (lowercase, no spaces)"
  type        = string
  default     = "gridtokenx"
  
  validation {
    condition     = can(regex("^[a-z0-9]+$", var.project_name))
    error_message = "Project name must contain only lowercase letters and numbers."
  }
}

# Network Configuration
variable "vnet_address_space" {
  description = "Address space for Virtual Network"
  type        = string
  default     = "10.0.0.0/16"
  
  validation {
    condition     = can(cidrhost(var.vnet_address_space, 0))
    error_message = "VNet address space must be a valid CIDR block."
  }
}

variable "gateway_subnet_cidr" {
  description = "CIDR block for Application Gateway subnet"
  type        = string
  default     = "10.0.1.0/24"
}

variable "containers_subnet_cidr" {
  description = "CIDR block for Container Instances subnet"
  type        = string
  default     = "10.0.2.0/24"
}

variable "database_subnet_cidr" {
  description = "CIDR block for Database subnet"
  type        = string
  default     = "10.0.3.0/24"
}

# Application Configuration
variable "app_name" {
  description = "Application name"
  type        = string
  default     = "blockchain-node"
}

variable "app_port" {
  description = "Port exposed by the application"
  type        = number
  default     = 8080
  
  validation {
    condition     = var.app_port > 0 && var.app_port < 65536
    error_message = "App port must be between 1 and 65535."
  }
}

variable "p2p_port" {
  description = "Port for P2P blockchain communication"
  type        = number
  default     = 30303
  
  validation {
    condition     = var.p2p_port > 0 && var.p2p_port < 65536
    error_message = "P2P port must be between 1 and 65535."
  }
}

# Container Configuration
variable "container_cpu" {
  description = "CPU units for container (0.5, 1, 2, etc.)"
  type        = number
  default     = 1
  
  validation {
    condition     = contains([0.5, 1, 2, 4], var.container_cpu)
    error_message = "Container CPU must be 0.5, 1, 2, or 4."
  }
}

variable "container_memory" {
  description = "Memory for container in GB (1, 2, 4, 8, etc.)"
  type        = number
  default     = 2
  
  validation {
    condition     = var.container_memory >= 1 && var.container_memory <= 16
    error_message = "Container memory must be between 1 and 16 GB."
  }
}

variable "replica_count" {
  description = "Number of container replicas"
  type        = number
  default     = 1
  
  validation {
    condition     = var.replica_count >= 1 && var.replica_count <= 10
    error_message = "Replica count must be between 1 and 10."
  }
}

# Database Configuration
variable "db_sku_name" {
  description = "Database SKU name"
  type        = string
  default     = "B_Standard_B1ms"
  
  validation {
    condition = can(regex("^[BGP]_Standard_", var.db_sku_name))
    error_message = "Database SKU must be a valid PostgreSQL Flexible Server SKU."
  }
}

variable "db_storage_mb" {
  description = "Database storage in MB"
  type        = number
  default     = 32768
  
  validation {
    condition     = var.db_storage_mb >= 32768 && var.db_storage_mb <= 16777216
    error_message = "Database storage must be between 32GB and 16TB."
  }
}

variable "db_username" {
  description = "Database administrator username"
  type        = string
  default     = "pgadmin"
  
  validation {
    condition     = can(regex("^[a-zA-Z][a-zA-Z0-9_]*$", var.db_username)) && length(var.db_username) <= 63
    error_message = "Database username must start with a letter and contain only letters, numbers, and underscores."
  }
}

# Container Registry Configuration
variable "acr_sku" {
  description = "Azure Container Registry SKU"
  type        = string
  default     = "Basic"
  
  validation {
    condition     = contains(["Basic", "Standard", "Premium"], var.acr_sku)
    error_message = "ACR SKU must be Basic, Standard, or Premium."
  }
}

# Monitoring Configuration
variable "log_retention_days" {
  description = "Log Analytics retention in days"
  type        = number
  default     = 30
  
  validation {
    condition     = var.log_retention_days >= 30 && var.log_retention_days <= 730
    error_message = "Log retention must be between 30 and 730 days."
  }
}

# Application Settings
variable "node_type" {
  description = "Type of blockchain node"
  type        = string
  default     = "validator"
  
  validation {
    condition     = contains(["validator", "trader", "observer"], var.node_type)
    error_message = "Node type must be validator, trader, or observer."
  }
}

variable "enable_mining" {
  description = "Enable mining on this node"
  type        = bool
  default     = true
}

variable "log_level" {
  description = "Application log level"
  type        = string
  default     = "INFO"
  
  validation {
    condition     = contains(["DEBUG", "INFO", "WARN", "ERROR"], var.log_level)
    error_message = "Log level must be DEBUG, INFO, WARN, or ERROR."
  }
}

# Security Configuration
variable "allowed_ip_ranges" {
  description = "IP ranges allowed to access the application"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

# Tags
variable "additional_tags" {
  description = "Additional tags to apply to resources"
  type        = map(string)
  default     = {}
}
