//! GridTokenX API Server Module
//! 
//! Comprehensive REST API for GridTokenX energy trading blockchain.
//! Base URL: http://localhost:8080/api/v1/
//! 
//! Features:
//! - Energy trading orders and market data
//! - Blockchain operations and queries  
//! - Grid status monitoring
//! - Account management
//! - Governance participation
//! - Real-time market pricing

use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::blockchain::Blockchain;
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
    pub state: AppState,
}

/// API Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Energy order request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub order_type: String,        // "buy" or "sell"
    pub energy_amount: f64,        // kWh
    pub price_per_kwh: u64,       // Token price per kWh
    pub energy_source: String,     // "solar", "wind", "hydro", etc.
    pub grid_location: String,     // Grid location identifier
    pub expiration_hours: u64,     // Order expiration time
}

/// Transaction submission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTransactionRequest {
    pub transaction_type: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
    pub signature: String,
    pub metadata: Option<String>,
}

/// Account balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub address: String,
    pub balance: u64,
    pub staked_amount: u64,
    pub energy_tokens: f64,
}

/// Energy trading statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyStats {
    pub total_energy_traded: f64,
    pub active_orders: u64,
    pub completed_trades: u64,
    pub average_price: f64,
    pub market_depth: f64,
}

/// Grid status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStatus {
    pub frequency: f64,
    pub voltage: f64,
    pub load_factor: f64,
    pub stability_index: f64,
    pub connected_nodes: u64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

/// Helper function for success responses
fn success_response<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse::success(data))
}

/// Helper function for error responses
fn error_response<T>(error: String) -> Json<ApiResponse<T>> {
    Json(ApiResponse::error(error))
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
            .map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;

        tracing::info!("🚀 Starting GridTokenX API server on http://{}", addr);

        let app = self.create_app();

        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| anyhow::anyhow!("Failed to bind to address {}: {}", addr, e))?;

        tracing::info!("✅ GridTokenX API server listening on {}", addr);
        
        axum::serve(listener, app).await
            .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

        Ok(())
    }

    /// Create the Axum application with all routes
    fn create_app(&self) -> Router {
        let api_routes = Router::new()
            // System endpoints
            .route("/health", get(handle_health))
            .route("/status", get(handle_status))
            .route("/stats", get(handle_get_stats))
            
            // Blockchain endpoints
            .route("/blocks/{height}", get(handle_get_block_by_height))
            .route("/blocks/hash/{hash}", get(handle_get_block_by_hash))
            .route("/transactions", post(handle_submit_transaction))
            .route("/transactions/{id}", get(handle_get_transaction))
            
            // Energy trading endpoints
            .route("/energy/orders", post(handle_create_energy_order))
            .route("/energy/orders", get(handle_get_energy_orders))
            .route("/energy/stats", get(handle_get_energy_stats))
            .route("/energy/trades", get(handle_get_energy_trades))
            
            // Grid management endpoints
            .route("/grid/status", get(handle_get_grid_status))
            .route("/grid/frequency", get(handle_get_grid_frequency))
            .route("/grid/load", get(handle_get_grid_load))
            
            // Account management endpoints
            .route("/accounts/{address}", get(handle_get_account))
            .route("/accounts/{address}/balance", get(handle_get_account_balance))
            .route("/accounts/{address}/history", get(handle_get_account_history))
            
            // Governance endpoints
            .route("/governance/proposals", get(handle_get_proposals))
            .route("/governance/proposals", post(handle_create_proposal))
            .route("/governance/proposals/{id}/vote", post(handle_vote_proposal))
            .route("/governance/staking/{address}", get(handle_get_staking_info))
            
            // Market data endpoints
            .route("/market/price/{energy_source}", get(handle_get_market_price))
            .route("/market/depth", get(handle_get_market_depth))
            .route("/market/volume", get(handle_get_market_volume));

        Router::new()
            .nest("/api/v1", api_routes)
            .with_state(self.state.clone())
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
    }
}

/// Health check endpoint
async fn handle_health() -> Json<ApiResponse<String>> {
    success_response("GridTokenX API is healthy".to_string())
}

/// Status endpoint
async fn handle_status(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_height().await {
        Ok(height) => success_response(format!("Blockchain height: {}", height)),
        Err(e) => error_response(format!("Failed to get blockchain status: {}", e)),
    }
}

/// Get block by height endpoint
async fn handle_get_block_by_height(
    Path(height): Path<u64>,
    State(state): State<AppState>,
) -> Json<ApiResponse<String>> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_block_by_height(height).await {
        Ok(block) => success_response(format!("Block at height {}: {}", height, block.header.hash)),
        Err(e) => error_response(format!("Failed to get block: {}", e)),
    }
}

/// Get statistics endpoint
async fn handle_get_stats(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_stats().await {
        stats => {
            let response = format!(
                "Height: {}, Total Transactions: {}, Total Energy Traded: {} kWh",
                stats.height, stats.total_transactions, stats.total_energy_traded
            );
            success_response(response)
        }
    }
}

// ===== BLOCKCHAIN ENDPOINTS =====

/// Get block by hash endpoint
async fn handle_get_block_by_hash(
    Path(hash): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<String>> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_block_by_hash(&hash).await {
        Ok(block) => success_response(format!("Block with hash {}: {}", hash, block.header.hash)),
        Err(e) => error_response(format!("Failed to get block by hash: {}", e)),
    }
}

/// Submit transaction endpoint
async fn handle_submit_transaction(
    State(state): State<AppState>,
    Json(request): Json<SubmitTransactionRequest>,
) -> Json<ApiResponse<String>> {
    // This would create and submit a transaction to the blockchain
    // For now, returning a placeholder response
    success_response(format!(
        "Transaction submitted: {} -> {} (amount: {})", 
        request.from_address, request.to_address, request.amount
    ))
}

/// Get transaction by ID endpoint
async fn handle_get_transaction(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<String>> {
    // This would look up a transaction by ID
    success_response(format!("Transaction details for ID: {}", id))
}

// ===== ENERGY TRADING ENDPOINTS =====

/// Create energy order endpoint
async fn handle_create_energy_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Json<ApiResponse<String>> {
    let energy_trading = state.energy_trading.read().await;
    
    // This would create an energy order in the trading system
    success_response(format!(
        "Energy order created: {} {} kWh at {} tokens/kWh from {}",
        request.order_type, request.energy_amount, request.price_per_kwh, request.energy_source
    ))
}

/// Get energy orders endpoint
async fn handle_get_energy_orders(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    let energy_trading = state.energy_trading.read().await;
    
    // This would return all active energy orders
    success_response("Active energy orders retrieved".to_string())
}

/// Get energy statistics endpoint
async fn handle_get_energy_stats(State(state): State<AppState>) -> Json<ApiResponse<EnergyStats>> {
    let blockchain = state.blockchain.read().await;
    
    match blockchain.get_energy_stats().await {
        Ok(stats) => success_response(EnergyStats {
            total_energy_traded: stats.total_energy_traded,
            active_orders: stats.active_buy_orders + stats.active_sell_orders,
            completed_trades: stats.completed_trades,
            average_price: stats.average_price,
            market_depth: 1000.0, // Placeholder
        }),
        Err(e) => error_response(format!("Failed to get energy stats: {}", e)),
    }
}

/// Get energy trades endpoint
async fn handle_get_energy_trades(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    success_response("Energy trade history retrieved".to_string())
}

// ===== GRID MANAGEMENT ENDPOINTS =====

/// Get grid status endpoint
async fn handle_get_grid_status(State(state): State<AppState>) -> Json<ApiResponse<GridStatus>> {
    let grid_manager = state.grid_manager.read().await;
    
    success_response(GridStatus {
        frequency: 50.0,      // Hz
        voltage: 230.0,       // V
        load_factor: 0.75,    // 75%
        stability_index: 0.95, // 95%
        connected_nodes: 42,
    })
}

/// Get grid frequency endpoint
async fn handle_get_grid_frequency(State(state): State<AppState>) -> Json<ApiResponse<f64>> {
    success_response(50.0) // 50 Hz standard frequency
}

/// Get grid load endpoint
async fn handle_get_grid_load(State(state): State<AppState>) -> Json<ApiResponse<f64>> {
    success_response(0.75) // 75% load factor
}

// ===== ACCOUNT MANAGEMENT ENDPOINTS =====

/// Get account information endpoint
async fn handle_get_account(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<String>> {
    success_response(format!("Account information for address: {}", address))
}

/// Get account balance endpoint
async fn handle_get_account_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<AccountBalance>> {
    success_response(AccountBalance {
        address: address.clone(),
        balance: 100000,        // 100,000 tokens
        staked_amount: 50000,   // 50,000 staked tokens
        energy_tokens: 250.5,   // 250.5 kWh equivalent
    })
}

/// Get account transaction history endpoint
async fn handle_get_account_history(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<String>> {
    success_response(format!("Transaction history for address: {}", address))
}

// ===== GOVERNANCE ENDPOINTS =====

/// Get governance proposals endpoint
async fn handle_get_proposals(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    let governance = state.governance.read().await;
    success_response("Active governance proposals retrieved".to_string())
}

/// Create governance proposal endpoint
async fn handle_create_proposal(
    State(state): State<AppState>,
    Json(proposal): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    success_response("Governance proposal created".to_string())
}

/// Vote on proposal endpoint
async fn handle_vote_proposal(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(vote): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    success_response(format!("Vote submitted for proposal: {}", id))
}

/// Get staking information endpoint
async fn handle_get_staking_info(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<String>> {
    success_response(format!("Staking information for address: {}", address))
}

// ===== MARKET DATA ENDPOINTS =====

/// Get market price for energy source endpoint
async fn handle_get_market_price(
    Path(energy_source): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<f64>> {
    let price = match energy_source.as_str() {
        "solar" => 4500.0,
        "wind" => 4200.0,
        "hydro" => 3800.0,
        "nuclear" => 3500.0,
        _ => 4000.0,
    };
    success_response(price)
}

/// Get market depth endpoint
async fn handle_get_market_depth(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    success_response("Market depth data retrieved".to_string())
}

/// Get market volume endpoint
async fn handle_get_market_volume(State(state): State<AppState>) -> Json<ApiResponse<String>> {
    success_response("Market volume data retrieved".to_string())
}
