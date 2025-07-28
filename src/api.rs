//! GridTokenX API Server Module
//!
//! This module implements a REST API server for the GridTokenX blockchain,
//! providing endpoints for blockchain operations, energy trading, and governance.

use anyhow::{anyhow, Result};
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
use crate::p2p::P2PNetwork;

/// API Server structure
pub struct ApiServer {
    config: ApiConfig,
    blockchain: Arc<RwLock<Blockchain>>,
    energy_trading: Arc<RwLock<EnergyTrading>>,
    grid_manager: Arc<RwLock<GridManager>>,
    governance: Arc<RwLock<GovernanceSystem>>,
    p2p_network: Arc<RwLock<P2PNetwork>>,
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
        p2p_network: Arc<RwLock<P2PNetwork>>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            blockchain,
            energy_trading,
            grid_manager,
            governance,
            p2p_network,
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

        Ok(submit_order
            .or(get_orders)
            .or(get_stats)
            .or(get_grid_status))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_serialization() {
        let response = ApiResponse {
            success: true,
            data: Some("test data"),
            error: None,
            timestamp: chrono::Utc::now(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("success"));
        assert!(serialized.contains("test data"));
    }

    #[test]
    fn test_energy_order_request_serialization() {
        let request = EnergyOrderRequest {
            order_type: "buy".to_string(),
            energy_amount: 100.0,
            price_per_kwh: 5000,
            energy_source: None,
            grid_location: "BKK-01-SUB001".to_string(),
            expiration_hours: Some(24),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("buy"));
        assert!(serialized.contains("100"));
    }
}
