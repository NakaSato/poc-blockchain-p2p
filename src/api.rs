//! GridTokenX API Server Module
//!
//! This module implements a REST API server for the GridTokenX blockchain,
//! providing endpoints for blockchain operations, energy trading, and governance.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{Filter, Reply};

use crate::blockchain::{Blockchain, Transaction, TransactionType};
use crate::config::ApiConfig;
use crate::energy::{EnergyTrading, GridManager};
use crate::governance::GovernanceSystem;
// use crate::p2p::P2PNetwork;  // Temporarily disabled

/// API Server structure
pub struct ApiServer {
    config: ApiConfig,
    blockchain: Arc<RwLock<Blockchain>>,
    energy_trading: Arc<RwLock<EnergyTrading>>,
    grid_manager: Arc<RwLock<GridManager>>,
    governance: Arc<RwLock<GovernanceSystem>>,
    // p2p_network: Arc<RwLock<P2PNetwork>>,  // Temporarily disabled
}

/// API Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Transaction submission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTransactionRequest {
    pub transaction: Transaction,
}

/// Energy order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyOrderRequest {
    pub order_type: String, // "buy" or "sell"
    pub energy_amount: f64,
    pub price_per_kwh: u64,
    pub energy_source: Option<String>,
    pub grid_location: String,
    pub expiration_hours: Option<u64>,
}

/// IoT device energy meter reading request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterReadingRequest {
    pub device_id: String,
    pub timestamp: String,
    pub energy_consumed: f64,
    pub energy_produced: Option<f64>,
    pub instantaneous_power: f64,
    pub voltage: f64,
    pub current: Option<f64>,
    pub frequency: Option<f64>,
    pub power_factor: Option<f64>,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub location: String,
    pub energy_source: String,
    pub grid_operator: Option<String>,
    pub carbon_credits: Option<f64>,
    pub sequence_number: u32,
    pub device_type: String,
}

/// IoT device registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegistrationRequest {
    pub device_id: String,
    pub device_type: String,
    pub firmware_version: String,
    pub location: String,
    pub zone: String,
    pub grid_operator: String,
    pub capabilities: Vec<String>,
}

/// Energy pricing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyPricingResponse {
    pub base_price_per_kwh: f64,
    pub peak_multiplier: f64,
    pub off_peak_multiplier: f64,
    pub renewable_bonus: f64,
    pub carbon_credit_value: f64,
    pub tariff_structure: String,
    pub valid_until_timestamp: u64,
}

/// Device information response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_type: String,
    pub registration_timestamp: String,
    pub last_reading_timestamp: Option<String>,
    pub total_energy_consumed: f64,
    pub total_energy_produced: f64,
    pub status: String,
    pub location: String,
    pub grid_operator: String,
}

/// Blockchain status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStatus {
    pub height: u64,
    pub total_transactions: u64,
    pub latest_block_hash: String,
    pub network_id: u64,
    pub active_peers: usize,
    pub energy_orders: u64,
    pub governance_proposals: u64,
}

/// Account information response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub address: String,
    pub balance: u64,
    pub energy_production: f64,
    pub energy_consumption: f64,
    pub carbon_credits: f64,
    pub reputation: f64,
    pub account_type: String,
}

impl ApiServer {
    /// Create a new API server
    pub async fn new(
        config: ApiConfig,
        blockchain: Arc<RwLock<Blockchain>>,
        energy_trading: Arc<RwLock<EnergyTrading>>,
        grid_manager: Arc<RwLock<GridManager>>,
        governance: Arc<RwLock<GovernanceSystem>>,
        // p2p_network: Arc<RwLock<P2PNetwork>>,  // Temporarily disabled
    ) -> Result<Self> {
        Ok(Self {
            config,
            blockchain,
            energy_trading,
            grid_manager,
            governance,
            // p2p_network,  // Temporarily disabled
        })
    }

    /// Start the API server
    pub async fn start(self) -> Result<()> {
        let server = Arc::new(self);

        // Create API routes
        let routes = self.create_routes(server.clone()).await?;

        // Configure CORS if enabled
        let routes = if server.config.enable_cors {
            routes
                .with(
                    warp::cors()
                        .allow_any_origin()
                        .allow_headers(vec!["content-type", "authorization"])
                        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]),
                )
                .boxed()
        } else {
            routes.boxed()
        };

        // Start server
        let addr = ([127, 0, 0, 1], server.config.port);
        tracing::info!(
            "Starting API server on http://{}:{}",
            server.config.host,
            server.config.port
        );

        warp::serve(routes).run(addr).await;

        Ok(())
    }

    /// Create API routes
    async fn create_routes(
        &self,
        server: Arc<ApiServer>,
    ) -> Result<impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone> {
        let api = warp::path("api").and(warp::path("v1"));

        // Status endpoint
        let status = api
            .and(warp::path("status"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_status);

        // Blockchain endpoints
        let blockchain_routes = self.blockchain_routes(api.clone(), server.clone()).await?;

        // Energy trading endpoints
        let energy_routes = self.energy_routes(api.clone(), server.clone()).await?;

        // Governance endpoints
        let governance_routes = self.governance_routes(api.clone(), server.clone()).await?;

        // Account endpoints
        let account_routes = self.account_routes(api.clone(), server.clone()).await?;

        // P2P network endpoints
        let network_routes = self.network_routes(api.clone(), server.clone()).await?;

        Ok(status
            .or(blockchain_routes)
            .or(energy_routes)
            .or(governance_routes)
            .or(account_routes)
            .or(network_routes)
            .with(warp::log("gridtokenx_api")))
    }

    /// Create blockchain-related routes
    async fn blockchain_routes(
        &self,
        api: impl Filter<Extract = (), Error = Infallible> + Clone,
        server: Arc<ApiServer>,
    ) -> Result<impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone> {
        // Get block by height
        let get_block_by_height = api
            .clone()
            .and(warp::path("blocks"))
            .and(warp::path::param::<u64>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_block_by_height);

        // Get block by hash
        let get_block_by_hash = api
            .clone()
            .and(warp::path("blocks"))
            .and(warp::path("hash"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_block_by_hash);

        // Submit transaction
        let submit_transaction = api
            .clone()
            .and(warp::path("transactions"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(with_server(server.clone()))
            .and_then(handle_submit_transaction);

        // Get transaction
        let get_transaction = api
            .clone()
            .and(warp::path("transactions"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_transaction);

        Ok(get_block_by_height
            .or(get_block_by_hash)
            .or(submit_transaction)
            .or(get_transaction))
    }

    /// Create energy trading routes
    async fn energy_routes(
        &self,
        api: impl Filter<Extract = (), Error = Infallible> + Clone,
        server: Arc<ApiServer>,
    ) -> Result<impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone> {
        // Submit energy order
        let submit_order = api
            .clone()
            .and(warp::path("energy"))
            .and(warp::path("orders"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(with_server(server.clone()))
            .and_then(handle_submit_energy_order);

        // Get energy orders
        let get_orders = api
            .clone()
            .and(warp::path("energy"))
            .and(warp::path("orders"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_energy_orders);

        // Get energy statistics
        let get_stats = api
            .clone()
            .and(warp::path("energy"))
            .and(warp::path("stats"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_energy_stats);

        // Get grid status
        let get_grid_status = api
            .clone()
            .and(warp::path("grid"))
            .and(warp::path("status"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_grid_status);

        // IoT device meter reading endpoint
        let submit_meter_reading = api
            .clone()
            .and(warp::path("energy"))
            .and(warp::path("meter-reading"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(with_server(server.clone()))
            .and_then(handle_submit_meter_reading);

        // Get energy pricing
        let get_pricing = api
            .clone()
            .and(warp::path("energy"))
            .and(warp::path("pricing"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_energy_pricing);

        // Device registration endpoint
        let register_device = api
            .clone()
            .and(warp::path("devices"))
            .and(warp::path("register"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(with_server(server.clone()))
            .and_then(handle_register_device);

        // Get device info
        let get_device = api
            .clone()
            .and(warp::path("devices"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_device);

        Ok(submit_order
            .or(get_orders)
            .or(get_stats)
            .or(get_grid_status)
            .or(submit_meter_reading)
            .or(get_pricing)
            .or(register_device)
            .or(get_device))
    }

    /// Create governance routes
    async fn governance_routes(
        &self,
        api: impl Filter<Extract = (), Error = Infallible> + Clone,
        server: Arc<ApiServer>,
    ) -> Result<impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone> {
        // Get proposals
        let get_proposals = api
            .clone()
            .and(warp::path("governance"))
            .and(warp::path("proposals"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_proposals);

        // Submit vote
        let submit_vote = api
            .clone()
            .and(warp::path("governance"))
            .and(warp::path("vote"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(with_server(server.clone()))
            .and_then(handle_submit_vote);

        Ok(get_proposals.or(submit_vote))
    }

    /// Create account routes
    async fn account_routes(
        &self,
        api: impl Filter<Extract = (), Error = Infallible> + Clone,
        server: Arc<ApiServer>,
    ) -> Result<impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone> {
        // Get account info
        let get_account = api
            .clone()
            .and(warp::path("accounts"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_account);

        // Get account balance
        let get_balance = api
            .clone()
            .and(warp::path("accounts"))
            .and(warp::path::param::<String>())
            .and(warp::path("balance"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_balance);

        Ok(get_account.or(get_balance))
    }

    /// Create network routes
    async fn network_routes(
        &self,
        api: impl Filter<Extract = (), Error = Infallible> + Clone,
        server: Arc<ApiServer>,
    ) -> Result<impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone> {
        // Get peers
        let get_peers = api
            .clone()
            .and(warp::path("network"))
            .and(warp::path("peers"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_server(server.clone()))
            .and_then(handle_get_peers);

        Ok(get_peers)
    }
}

/// Helper function to pass server instance to handlers
fn with_server(
    server: Arc<ApiServer>,
) -> impl Filter<Extract = (Arc<ApiServer>,), Error = Infallible> + Clone {
    warp::any().map(move || server.clone())
}

/// Create success response
fn success_response<T: Serialize>(data: T) -> impl Reply {
    let response = ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        timestamp: chrono::Utc::now(),
    };
    warp::reply::json(&response)
}

/// Create error response
fn error_response(error: String) -> impl Reply {
    let response = ApiResponse::<()> {
        success: false,
        data: None,
        error: Some(error),
        timestamp: chrono::Utc::now(),
    };
    warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::BAD_REQUEST,
    )
}

// Handler functions

async fn handle_status(server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    let blockchain = server.blockchain.read().await;

    let status = match get_blockchain_status(&blockchain, &server).await {
        Ok(status) => status,
        Err(e) => {
            tracing::error!("Failed to get blockchain status: {}", e);
            return Ok(error_response(format!("Failed to get status: {}", e)).into_response());
        }
    };

    Ok(success_response(status).into_response())
}

async fn get_blockchain_status(
    blockchain: &Blockchain,
    server: &ApiServer,
) -> Result<BlockchainStatus> {
    let height = blockchain.get_height().await?;
    let total_transactions = blockchain.get_total_transactions().await?;
    let latest_block = blockchain.get_latest_block().await?;

    // Get network info (simplified)
    let active_peers = 0; // Would get from P2P network
    let energy_orders = 0; // Would get from energy trading system
    let governance_proposals = 0; // Would get from governance system

    Ok(BlockchainStatus {
        height,
        total_transactions,
        latest_block_hash: latest_block.header.hash,
        network_id: 1001, // From config
        active_peers,
        energy_orders,
        governance_proposals,
    })
}

async fn handle_get_block_by_height(
    height: u64,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    let blockchain = server.blockchain.read().await;

    match blockchain.get_block_by_height(height).await {
        Ok(block) => Ok(success_response(block).into_response()),
        Err(e) => Ok(error_response(format!("Block not found: {}", e)).into_response()),
    }
}

async fn handle_get_block_by_hash(
    hash: String,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    let blockchain = server.blockchain.read().await;

    match blockchain.get_block_by_hash(&hash).await {
        Ok(block) => Ok(success_response(block).into_response()),
        Err(e) => Ok(error_response(format!("Block not found: {}", e)).into_response()),
    }
}

async fn handle_submit_transaction(
    request: SubmitTransactionRequest,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    let blockchain = server.blockchain.read().await;

    match blockchain
        .add_pending_transaction(request.transaction.clone())
        .await
    {
        Ok(()) => {
            let response = HashMap::from([
                ("transaction_id", request.transaction.id),
                ("status", "pending".to_string()),
            ]);
            Ok(success_response(response).into_response())
        }
        Err(e) => {
            Ok(error_response(format!("Failed to submit transaction: {}", e)).into_response())
        }
    }
}

async fn handle_get_transaction(
    tx_id: String,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    // This would typically load from storage
    Ok(error_response("Transaction lookup not implemented".to_string()).into_response())
}

async fn handle_submit_energy_order(
    request: EnergyOrderRequest,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    Ok(error_response("Energy order submission not implemented".to_string()).into_response())
}

async fn handle_get_energy_orders(server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    Ok(error_response("Energy orders not implemented".to_string()).into_response())
}

async fn handle_get_energy_stats(server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    Ok(error_response("Energy stats not implemented".to_string()).into_response())
}

async fn handle_get_grid_status(server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    Ok(error_response("Grid status not implemented".to_string()).into_response())
}

async fn handle_get_proposals(server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    Ok(error_response("Governance proposals not implemented".to_string()).into_response())
}

async fn handle_submit_vote(
    request: serde_json::Value,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    Ok(error_response("Voting not implemented".to_string()).into_response())
}

async fn handle_get_account(
    address: String,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    let blockchain = server.blockchain.read().await;

    match blockchain.get_account(&address).await {
        Some(account) => {
            let account_info = AccountInfo {
                address: account.address,
                balance: account.token_balance,
                energy_production: account.energy_production_capacity,
                energy_consumption: account.energy_consumption_demand,
                carbon_credits: account.carbon_credits,
                reputation: account.reputation_score,
                account_type: format!("{:?}", account.account_type),
            };
            Ok(success_response(account_info).into_response())
        }
        None => Ok(error_response("Account not found".to_string()).into_response()),
    }
}

async fn handle_get_balance(
    address: String,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    let blockchain = server.blockchain.read().await;
    let balance = blockchain.get_balance(&address).await;

    let response = HashMap::from([("address", address), ("balance", balance.to_string())]);

    Ok(success_response(response).into_response())
}

async fn handle_get_peers(server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    Ok(error_response("Peer information not implemented".to_string()).into_response())
}

// IoT Device Handlers

async fn handle_submit_meter_reading(
    request: MeterReadingRequest,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    tracing::info!("Received meter reading from device: {}", request.device_id);
    
    // Validate device signature (in real implementation)
    // let signature_valid = validate_device_signature(&request);
    
    // Create energy measurement transaction
    let transaction = Transaction::new_energy_measurement(
        request.device_id.clone(),
        request.energy_consumed,
        request.energy_produced.unwrap_or(0.0),
        request.instantaneous_power,
        request.energy_source.clone(),
        request.location.clone(),
    );
    
    // Add transaction to blockchain
    let mut blockchain = server.blockchain.write().await;
    match blockchain.add_pending_transaction(transaction.clone()).await {
        Ok(()) => {
            // Update energy trading system
            if let Ok(mut energy_trading) = server.energy_trading.try_write() {
                energy_trading.record_energy_reading(
                    &request.device_id,
                    request.energy_consumed,
                    request.energy_produced.unwrap_or(0.0),
                    &request.energy_source,
                ).await.unwrap_or_else(|e| {
                    tracing::warn!("Failed to record energy reading: {}", e);
                });
            }
            
            // Calculate current energy price based on demand/supply
            let energy_price = calculate_dynamic_energy_price(&request).await;
            
            let response = serde_json::json!({
                "success": true,
                "transaction_hash": transaction.id,
                "device_id": request.device_id,
                "energy_consumed": request.energy_consumed,
                "energy_produced": request.energy_produced.unwrap_or(0.0),
                "timestamp": request.timestamp,
                "energy_price_current": energy_price,
                "carbon_credits": calculate_carbon_credits(&request),
                "status": "recorded"
            });
            
            Ok(success_response(response).into_response())
        }
        Err(e) => {
            tracing::error!("Failed to submit meter reading: {}", e);
            Ok(error_response(format!("Failed to record meter reading: {}", e)).into_response())
        }
    }
}

async fn handle_get_energy_pricing(_server: Arc<ApiServer>) -> Result<impl Reply, Infallible> {
    // In production, this would be dynamic based on supply/demand
    let pricing = EnergyPricingResponse {
        base_price_per_kwh: 3500.0,  // tokens per kWh
        peak_multiplier: 1.5,
        off_peak_multiplier: 0.8,
        renewable_bonus: 500.0,
        carbon_credit_value: 100.0,
        tariff_structure: "time_of_use".to_string(),
        valid_until_timestamp: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour
    };
    
    Ok(success_response(pricing).into_response())
}

async fn handle_register_device(
    request: DeviceRegistrationRequest,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    tracing::info!("Registering new IoT device: {}", request.device_id);
    
    // Validate device information
    if request.device_id.is_empty() || request.device_type.is_empty() {
        return Ok(error_response("Invalid device information".to_string()).into_response());
    }
    
    // Create device registration transaction
    let transaction = Transaction::new_device_registration(
        request.device_id.clone(),
        request.device_type.clone(),
        request.location.clone(),
        request.grid_operator.clone(),
        request.capabilities.clone(),
    );
    
    // Add to blockchain
    let mut blockchain = server.blockchain.write().await;
    match blockchain.add_pending_transaction(transaction.clone()).await {
        Ok(()) => {
            let response = serde_json::json!({
                "success": true,
                "device_id": request.device_id,
                "transaction_hash": transaction.id,
                "status": "registered",
                "registration_timestamp": chrono::Utc::now().to_rfc3339(),
                "account_created": true
            });
            
            Ok(success_response(response).into_response())
        }
        Err(e) => {
            tracing::error!("Failed to register device: {}", e);
            Ok(error_response(format!("Device registration failed: {}", e)).into_response())
        }
    }
}

async fn handle_get_device(
    device_id: String,
    server: Arc<ApiServer>,
) -> Result<impl Reply, Infallible> {
    tracing::info!("Getting device info for: {}", device_id);
    
    // In production, this would query device registry from blockchain storage
    let device_info = DeviceInfo {
        device_id: device_id.clone(),
        device_type: "smart_meter".to_string(),
        registration_timestamp: "2025-08-03T10:00:00Z".to_string(),
        last_reading_timestamp: Some(chrono::Utc::now().to_rfc3339()),
        total_energy_consumed: 1250.5,
        total_energy_produced: 85.2,
        status: "active".to_string(),
        location: "13.7563,100.5018".to_string(),
        grid_operator: "MEA".to_string(),
    };
    
    Ok(success_response(device_info).into_response())
}

// Helper functions for IoT device processing

async fn calculate_dynamic_energy_price(request: &MeterReadingRequest) -> f64 {
    // Base price in tokens per kWh
    let base_price = 3500.0;
    
    // Time-of-use multiplier
    let hour = chrono::Utc::now().hour();
    let time_multiplier = match hour {
        6..=9 | 18..=22 => 1.5,   // Peak hours
        23..=5 => 0.7,            // Off-peak hours
        _ => 1.0,                 // Normal hours
    };
    
    // Renewable energy bonus
    let renewable_bonus = match request.energy_source.as_str() {
        "solar" => 500.0,
        "wind" => 400.0,
        "hydro" => 300.0,
        _ => 0.0,
    };
    
    // Grid operator adjustment (Thai market specific)
    let grid_adjustment = match request.grid_operator.as_deref() {
        Some("MEA") => 1.05,  // Bangkok metropolitan premium
        Some("PEA") => 0.95,  // Provincial discount
        Some("EGAT") => 1.0,  // Standard rate
        _ => 1.0,
    };
    
    (base_price * time_multiplier + renewable_bonus) * grid_adjustment
}

fn calculate_carbon_credits(request: &MeterReadingRequest) -> f64 {
    let energy_kwh = request.energy_consumed;
    
    match request.energy_source.as_str() {
        "solar" => energy_kwh * 0.8,
        "wind" => energy_kwh * 0.7,
        "hydro" => energy_kwh * 0.6,
        "biomass" => energy_kwh * 0.4,
        "geothermal" => energy_kwh * 0.7,
        _ => energy_kwh * 0.2, // Grid default
    }
}

