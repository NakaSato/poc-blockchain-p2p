# Initial Azure Infrastructure Outputs

# Resource Group
output "resource_group_name" {
  description = "Name of the Resource Group"
  value       = azurerm_resource_group.main.name
}

output "location" {
  description = "Azure location"
  value       = azurerm_resource_group.main.location
}

# Networking
output "vnet_name" {
  description = "Virtual Network name"
  value       = azurerm_virtual_network.main.name
}

output "vnet_id" {
  description = "Virtual Network ID"
  value       = azurerm_virtual_network.main.id
}

# Container Registry
output "acr_login_server" {
  description = "ACR login server URL"
  value       = azurerm_container_registry.main.login_server
}

output "acr_name" {
  description = "ACR name"
  value       = azurerm_container_registry.main.name
}

output "acr_admin_username" {
  description = "ACR admin username"
  value       = azurerm_container_registry.main.admin_username
  sensitive   = true
}

# Application Gateway
output "gateway_public_ip" {
  description = "Application Gateway public IP"
  value       = azurerm_public_ip.gateway.ip_address
}

output "application_url" {
  description = "Application URL"
  value       = "http://${azurerm_public_ip.gateway.ip_address}"
}

# Database
output "database_fqdn" {
  description = "Database FQDN"
  value       = azurerm_postgresql_flexible_server.main.fqdn
  sensitive   = true
}

output "database_name" {
  description = "Database name"
  value       = azurerm_postgresql_flexible_server_database.main.name
}

# Key Vault
output "key_vault_uri" {
  description = "Key Vault URI"
  value       = azurerm_key_vault.main.vault_uri
}

# Monitoring
output "log_analytics_workspace_id" {
  description = "Log Analytics Workspace ID"
  value       = azurerm_log_analytics_workspace.main.workspace_id
}

output "application_insights_connection_string" {
  description = "Application Insights connection string"
  value       = azurerm_application_insights.main.connection_string
  sensitive   = true
}

# Storage
output "storage_account_name" {
  description = "Storage account name"
  value       = azurerm_storage_account.main.name
}

# Deployment Commands
output "deployment_commands" {
  description = "Useful deployment commands"
  value = {
    acr_login = "az acr login --name ${azurerm_container_registry.main.name}"
    
    build_and_push = "docker build -t ${var.project_name} . && docker tag ${var.project_name}:latest ${azurerm_container_registry.main.login_server}/${var.project_name}:latest && docker push ${azurerm_container_registry.main.login_server}/${var.project_name}:latest"
    
    check_rg = "az group show --name ${azurerm_resource_group.main.name}"
    
    list_resources = "az resource list --resource-group ${azurerm_resource_group.main.name} --output table"
  }
}

# Summary
output "deployment_summary" {
  description = "Deployment summary"
  value = {
    project_name      = var.project_name
    environment       = var.environment
    location          = azurerm_resource_group.main.location
    resource_group    = azurerm_resource_group.main.name
    container_registry = azurerm_container_registry.main.login_server
    application_url   = "http://${azurerm_public_ip.gateway.ip_address}"
    database_server   = azurerm_postgresql_flexible_server.main.name
    key_vault        = azurerm_key_vault.main.name
  }
}
