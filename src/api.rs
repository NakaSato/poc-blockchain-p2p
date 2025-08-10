//! GridTokenX API Server Module
//!
//! This module implements a REST API server for the GridTokenX blockchain,
//! providing endpoints for blockchain operations, energy trading, and governance.

use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, State},
    http::{StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::blockchain::{Blockchain, Transaction, TransactionType};
use crate::config::ApiConfig;
use crate::energy::{EnergyTrading, GridManager};
use crate::governance::GovernanceSystem;

/// API Server state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub energy_trading: Arc<RwLock<EnergyTrading>>,
    pub grid_manager: Arc<RwLock<GridManager>>,
    pub governance: Arc<RwLock<GovernanceSystem>>,
}

/// API Server structure
pub struct ApiServer {
    state: AppState,
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
    /// Create a new API server instance
    pub fn new(
        config: ApiConfig,
        blockchain: Arc<RwLock<Blockchain>>,
        energy_trading: Arc<RwLock<EnergyTrading>>,
        grid_manager: Arc<RwLock<GridManager>>,
        governance: Arc<RwLock<GovernanceSystem>>,
    ) -> Self {
        let state = AppState {
            config,
            blockchain,
            energy_trading,
            grid_manager,
            governance,
        };

        Self { state }
    }

    /// Start the API server
    pub async fn start(&self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.state.config.host, self.state.config.port)
            .parse()
            .map_err(|e| anyhow!("Invalid address: {}", e))?;

        tracing::info!("🚀 Starting GridTokenX API server on http://{}", addr);

        let app = self.create_app();

        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| anyhow!("Failed to bind to address {}: {}", addr, e))?;

        tracing::info!("✅ GridTokenX API server listening on {}", addr);
        
        axum::serve(listener, app).await
            .map_err(|e| anyhow!("Server error: {}", e))?;

        Ok(())
    }

    /// Create the Axum application with all routes
    fn create_app(&self) -> Router {
        let api_routes = Router::new()
            .route("/status", get(handle_status))
            .route("/blocks/:height", get(handle_get_block_by_height))
            .route("/blocks/hash/:hash", get(handle_get_block_by_hash))
            .route("/transactions", post(handle_submit_transaction))
            .route("/transactions/:tx_id", get(handle_get_transaction))
            .route("/energy/orders", post(handle_submit_energy_order))
            .route("/energy/orders", get(handle_get_energy_orders))
            .route("/energy/pricing", get(handle_get_energy_pricing))
            .route("/governance/proposals", post(handle_submit_proposal))
            .route("/governance/proposals", get(handle_get_proposals))
            .route("/governance/proposals/:id/vote", post(handle_vote_proposal))
            .route("/accounts/:address", get(handle_get_account_info))
            .route("/devices", post(handle_register_device))
            .route("/devices/:device_id", get(handle_get_device_info))
            .route("/devices/:device_id/meter", post(handle_submit_meter_reading))
            .route("/network/peers", get(handle_get_network_peers))
            .with_state(self.state.clone());

        Router::new()
            .nest("/api/v1", api_routes)
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
    }
}

// Helper function to create successful API responses
fn success_response<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        timestamp: Utc::now(),
    })
}

// Helper function to create error API responses
fn error_response(error: String) -> Json<ApiResponse<()>> {
    Json(ApiResponse {
        success: false,
        data: None,
        error: Some(error),
        timestamp: Utc::now(),
    })
}

/// Status endpoint handler
async fn handle_status(State(state): State<AppState>) -> Json<ApiResponse<BlockchainStatus>> {
    let blockchain = state.blockchain.read().await;
    
    let status = match blockchain.get_latest_block().await {
        Ok(latest_block) => BlockchainStatus {
            height: latest_block.header.height,
            total_transactions: 0, // TODO: Implement transaction count
            latest_block_hash: latest_block.header.hash,
            network_id: 1, // TODO: Make configurable
            active_peers: 0, // TODO: Implement peer count
            energy_orders: 0, // TODO: Implement order count
            governance_proposals: 0, // TODO: Implement proposal count
        },
        Err(_) => BlockchainStatus {
            height: 0,
            total_transactions: 0,
            latest_block_hash: "none".to_string(),
            network_id: 1,
            active_peers: 0,
            energy_orders: 0,
            governance_proposals: 0,
        },
    };

    success_response(status)
}

/// Get block by height handler
async fn handle_get_block_by_height(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<ApiResponse<crate::blockchain::Block>>, StatusCode> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_block_by_height(height).await {
        Ok(block) => Ok(success_response(block)),
        Err(e) => {
            tracing::error!("Failed to get block by height {}: {}", height, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Get block by hash handler
async fn handle_get_block_by_hash(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<crate::blockchain::Block>>, StatusCode> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_block_by_hash(&hash).await {
        Ok(block) => Ok(success_response(block)),
        Err(e) => {
            tracing::error!("Failed to get block by hash {}: {}", hash, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Submit transaction handler
async fn handle_submit_transaction(
    State(state): State<AppState>,
    Json(payload): Json<SubmitTransactionRequest>,
) -> Json<ApiResponse<String>> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.add_pending_transaction(payload.transaction.clone()).await {
        Ok(_) => success_response(payload.transaction.id),
        Err(e) => {
            tracing::error!("Failed to submit transaction: {}", e);
            error_response(format!("Failed to submit transaction: {}", e))
        }
    }
}

/// Get transaction handler
async fn handle_get_transaction(
    State(state): State<AppState>,
    Path(tx_id): Path<String>,
) -> Result<Json<ApiResponse<Transaction>>, StatusCode> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_transaction(&tx_id).await {
        Ok(transaction) => Ok(success_response(transaction)),
        Err(e) => {
            tracing::error!("Failed to get transaction {}: {}", tx_id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Submit energy order handler
async fn handle_submit_energy_order(
    State(state): State<AppState>,
    Json(payload): Json<EnergyOrderRequest>,
) -> Json<ApiResponse<String>> {
    let energy_trading = state.energy_trading.read().await;
    
    // Create energy trading transaction
    match energy_trading.create_order(
        &payload.order_type,
        payload.energy_amount,
        payload.price_per_kwh,
        &payload.grid_location,
    ).await {
        Ok(order_id) => success_response(order_id),
        Err(e) => {
            tracing::error!("Failed to create energy order: {}", e);
            error_response(format!("Failed to create energy order: {}", e))
        }
    }
}

/// Get energy orders handler
async fn handle_get_energy_orders(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<String>>> {
    let energy_trading = state.energy_trading.read().await;
    
    match energy_trading.get_active_orders().await {
        Ok(orders) => success_response(orders),
        Err(e) => {
            tracing::error!("Failed to get energy orders: {}", e);
            error_response(format!("Failed to get energy orders: {}", e))
        }
    }
}

/// Get energy pricing handler
async fn handle_get_energy_pricing(
    State(_state): State<AppState>,
) -> Json<ApiResponse<EnergyPricingResponse>> {
    let pricing = EnergyPricingResponse {
        base_price_per_kwh: 2.5,
        peak_multiplier: 1.5,
        off_peak_multiplier: 0.8,
        renewable_bonus: 0.3,
        carbon_credit_value: 0.15,
        tariff_structure: "time_of_use".to_string(),
        valid_until_timestamp: (Utc::now().timestamp() + 3600) as u64,
    };
    
    success_response(pricing)
}

/// Submit governance proposal handler
async fn handle_submit_proposal(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    let governance = state.governance.read().await;
    
    // TODO: Implement proposal submission
    let proposal_id = format!("proposal_{}", Utc::now().timestamp());
    success_response(proposal_id)
}

/// Get governance proposals handler
async fn handle_get_proposals(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<String>>> {
    let governance = state.governance.read().await;
    
    // TODO: Implement proposal retrieval
    success_response(vec![])
}

/// Vote on proposal handler
async fn handle_vote_proposal(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    let governance = state.governance.read().await;
    
    // TODO: Implement voting
    success_response(format!("Vote recorded for proposal {}", id))
}

/// Get account info handler
async fn handle_get_account_info(
    State(_state): State<AppState>,
    Path(address): Path<String>,
) -> Json<ApiResponse<AccountInfo>> {
    // TODO: Implement actual account lookup
    let account_info = AccountInfo {
        address: address.clone(),
        balance: 1000,
        energy_production: 150.5,
        energy_consumption: 120.3,
        carbon_credits: 25.7,
        reputation: 85.0,
        account_type: "prosumer".to_string(),
    };
    
    success_response(account_info)
}

/// Register device handler
async fn handle_register_device(
    State(_state): State<AppState>,
    Json(payload): Json<DeviceRegistrationRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Implement device registration
    success_response(format!("Device {} registered successfully", payload.device_id))
}

/// Get device info handler
async fn handle_get_device_info(
    State(_state): State<AppState>,
    Path(device_id): Path<String>,
) -> Json<ApiResponse<DeviceInfo>> {
    // TODO: Implement actual device lookup
    let device_info = DeviceInfo {
        device_id: device_id.clone(),
        device_type: "smart_meter".to_string(),
        registration_timestamp: Utc::now().to_rfc3339(),
        last_reading_timestamp: Some(Utc::now().to_rfc3339()),
        total_energy_consumed: 1234.5,
        total_energy_produced: 567.8,
        status: "active".to_string(),
        location: "Bangkok, Thailand".to_string(),
        grid_operator: "MEA".to_string(),
    };
    
    success_response(device_info)
}

/// Submit meter reading handler
async fn handle_submit_meter_reading(
    State(_state): State<AppState>,
    Path(device_id): Path<String>,
    Json(payload): Json<MeterReadingRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Implement meter reading processing
    success_response(format!("Meter reading for device {} recorded", device_id))
}

/// Get network peers handler
async fn handle_get_network_peers(
    State(_state): State<AppState>,
) -> Json<ApiResponse<Vec<String>>> {
    // TODO: Implement peer list retrieval
    success_response(vec!["peer1".to_string(), "peer2".to_string()])
}
