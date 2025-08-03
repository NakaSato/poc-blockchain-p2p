---
mode: edit
---

# GridTokenX API Development Prompt

You are developing the REST API interface for GridTokenX - Thailand's energy trading blockchain platform using Warp for high-performance web services.

## API Architecture Overview

The API system (`src/api.rs`) provides:
- **RESTful Services**: Standard HTTP/JSON endpoints for all platform features
- **Real-Time WebSocket**: Live energy trading and grid status updates
- **High Performance**: 10,000+ concurrent connections during peak trading
- **Security**: Authentication, authorization, and rate limiting
- **Thai Market Integration**: Specialized endpoints for energy authorities

## API Structure and Routing

### Core API Categories
```rust
use warp::{Filter, Reply, Rejection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct GridTokenXApi {
    blockchain: Arc<RwLock<Blockchain>>,
    energy_trading: Arc<EnergyTrading>,
    governance: Arc<Governance>,
    auth_service: Arc<AuthService>,
}

impl GridTokenXApi {
    pub fn routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let api = warp::path("api").and(warp::path("v1"));
        
        api.and(
            self.blockchain_routes()
                .or(self.energy_trading_routes())
                .or(self.governance_routes())
                .or(self.grid_management_routes())
                .or(self.authority_routes())
                .or(self.websocket_routes())
        )
    }
}
```

### Authentication and Authorization
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub key_id: String,
    pub participant_type: ParticipantType,
    pub permissions: Vec<Permission>,
    pub rate_limit: RateLimit,
    pub expires_at: Option<DateTime<Utc>>,
}

pub enum Permission {
    ReadBlockchain,
    ReadEnergyData,
    SubmitEnergyOrder,
    SubmitTransaction,
    ReadGridStatus,
    SubmitGridData,          // Authority only
    AccessGovernance,
    SubmitProposal,
    ManageEmergency,         // Authority only
    AdminAccess,             // Authority only
}

pub struct RateLimit {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
    pub data_transfer_limit: u64,    // Bytes per hour
}
```

## Blockchain API Endpoints

### Block and Transaction Queries
```rust
impl GridTokenXApi {
    fn blockchain_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let get_block = warp::path("blocks")
            .and(warp::path::param::<u64>())
            .and(warp::get())
            .and(with_blockchain(self.blockchain.clone()))
            .and_then(get_block_handler);
        
        let get_transaction = warp::path("transactions")
            .and(warp::path::param::<String>())
            .and(warp::get())
            .and(with_blockchain(self.blockchain.clone()))
            .and_then(get_transaction_handler);
        
        let submit_transaction = warp::path("transactions")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_auth())
            .and(with_blockchain(self.blockchain.clone()))
            .and_then(submit_transaction_handler);
        
        get_block.or(get_transaction).or(submit_transaction)
    }
}

#[derive(Serialize)]
pub struct BlockResponse {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<TransactionSummary>,
    pub total_energy: f64,
    pub grid_state: GridStateSummary,
    pub confirmations: u32,
}

async fn get_block_handler(
    height: u64,
    blockchain: Arc<RwLock<Blockchain>>,
) -> Result<impl Reply, Rejection> {
    let blockchain = blockchain.read().await;
    
    match blockchain.get_block_by_height(height).await {
        Ok(block) => {
            let response = BlockResponse {
                height: block.height,
                hash: block.hash,
                previous_hash: block.previous_hash,
                timestamp: block.timestamp,
                transactions: block.transactions.iter()
                    .map(|tx| TransactionSummary::from(tx))
                    .collect(),
                total_energy: block.calculate_total_energy(),
                grid_state: GridStateSummary::from(&block.grid_state),
                confirmations: blockchain.get_confirmations(height).await.unwrap_or(0),
            };
            Ok(warp::reply::json(&response))
        },
        Err(_) => Err(warp::reject::not_found()),
    }
}
```

### Chain Status and Statistics
```rust
#[derive(Serialize)]
pub struct ChainStatusResponse {
    pub height: u64,
    pub latest_block_hash: String,
    pub latest_block_time: DateTime<Utc>,
    pub total_transactions: u64,
    pub total_energy_traded: f64,
    pub active_validators: u32,
    pub network_hash_rate: f64,
    pub average_block_time: Duration,
    pub chain_id: String,
}

async fn get_chain_status_handler(
    blockchain: Arc<RwLock<Blockchain>>,
) -> Result<impl Reply, Rejection> {
    let blockchain = blockchain.read().await;
    
    let status = ChainStatusResponse {
        height: blockchain.get_height().await.map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        latest_block_hash: blockchain.get_latest_block_hash().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        latest_block_time: blockchain.get_latest_block_time().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        total_transactions: blockchain.get_total_transaction_count().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        total_energy_traded: blockchain.get_total_energy_traded().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        active_validators: blockchain.get_active_validator_count().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        network_hash_rate: blockchain.get_network_hash_rate().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        average_block_time: blockchain.get_average_block_time().await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?,
        chain_id: blockchain.get_chain_id(),
    };
    
    Ok(warp::reply::json(&status))
}
```

## Energy Trading API Endpoints

### Order Management
```rust
impl GridTokenXApi {
    fn energy_trading_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let submit_order = warp::path("orders")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_auth())
            .and(with_energy_trading(self.energy_trading.clone()))
            .and_then(submit_energy_order_handler);
        
        let get_orders = warp::path("orders")
            .and(warp::get())
            .and(warp::query::<OrderQuery>())
            .and(with_auth())
            .and(with_energy_trading(self.energy_trading.clone()))
            .and_then(get_energy_orders_handler);
        
        let cancel_order = warp::path("orders")
            .and(warp::path::param::<String>())
            .and(warp::delete())
            .and(with_auth())
            .and(with_energy_trading(self.energy_trading.clone()))
            .and_then(cancel_energy_order_handler);
        
        submit_order.or(get_orders).or(cancel_order)
    }
}

#[derive(Deserialize)]
pub struct SubmitOrderRequest {
    pub order_type: OrderType,
    pub energy_amount: f64,         // kWh
    pub price: Option<u64>,         // GridTokens per kWh (None for market orders)
    pub grid_zone: GridZone,
    pub time_slot: TimeSlot,
    pub renewable_only: bool,
    pub expiry: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub estimated_settlement: Option<DateTime<Utc>>,
    pub partial_fills: Vec<PartialFillInfo>,
}

async fn submit_energy_order_handler(
    order_request: SubmitOrderRequest,
    auth: AuthContext,
    energy_trading: Arc<EnergyTrading>,
) -> Result<impl Reply, Rejection> {
    // Verify participant can trade in specified zone
    energy_trading.verify_trading_eligibility(&auth.participant_id, &order_request.grid_zone)
        .await
        .map_err(|_| warp::reject::custom(ApiError::Unauthorized))?;
    
    let order = EnergyOrder {
        id: generate_order_id(),
        participant_id: auth.participant_id,
        order_type: order_request.order_type,
        energy_amount: order_request.energy_amount,
        price: order_request.price,
        grid_zone: order_request.grid_zone,
        time_slot: order_request.time_slot,
        renewable_only: order_request.renewable_only,
        expiry: order_request.expiry,
        created_at: Utc::now(),
        status: OrderStatus::Pending,
    };
    
    match energy_trading.submit_order(order).await {
        Ok(order_response) => Ok(warp::reply::json(&order_response)),
        Err(e) => Err(warp::reject::custom(ApiError::TradingError(e.to_string()))),
    }
}
```

### Market Data and Analytics
```rust
#[derive(Serialize)]
pub struct MarketDataResponse {
    pub grid_zone: GridZone,
    pub current_price: u64,
    pub price_24h_change: f64,
    pub volume_24h: f64,
    pub total_buy_orders: u32,
    pub total_sell_orders: u32,
    pub renewable_percentage: f64,
    pub grid_congestion_level: f64,
    pub last_update: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct MarketDataQuery {
    pub zones: Option<Vec<GridZone>>,
    pub time_range: Option<TimeRange>,
    pub include_forecast: Option<bool>,
}

async fn get_market_data_handler(
    query: MarketDataQuery,
    energy_trading: Arc<EnergyTrading>,
) -> Result<impl Reply, Rejection> {
    let zones = query.zones.unwrap_or_else(|| vec![GridZone::All]);
    let mut market_data = Vec::new();
    
    for zone in zones {
        let data = energy_trading.get_market_data(&zone).await
            .map_err(|_| warp::reject::custom(ApiError::InternalError))?;
        
        market_data.push(MarketDataResponse {
            grid_zone: zone,
            current_price: data.current_price,
            price_24h_change: data.calculate_24h_change(),
            volume_24h: data.volume_24h,
            total_buy_orders: data.active_buy_orders,
            total_sell_orders: data.active_sell_orders,
            renewable_percentage: data.renewable_percentage,
            grid_congestion_level: data.congestion_level,
            last_update: data.last_update,
        });
    }
    
    Ok(warp::reply::json(&market_data))
}
```

## Grid Management API Endpoints

### Real-Time Grid Status
```rust
impl GridTokenXApi {
    fn grid_management_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let get_grid_status = warp::path("grid")
            .and(warp::path("status"))
            .and(warp::get())
            .and(warp::query::<GridStatusQuery>())
            .and(with_auth())
            .and_then(get_grid_status_handler);
        
        let submit_grid_data = warp::path("grid")
            .and(warp::path("data"))
            .and(warp::post())
            .and(warp::body::json())
            .and(with_authority_auth())
            .and_then(submit_grid_data_handler);
        
        get_grid_status.or(submit_grid_data)
    }
}

#[derive(Serialize)]
pub struct GridStatusResponse {
    pub timestamp: DateTime<Utc>,
    pub overall_status: GridStatus,
    pub frequency: f64,              // Hz
    pub system_load: f64,            // MW
    pub total_generation: f64,       // MW
    pub renewable_generation: f64,   // MW
    pub reserve_margin: f64,         // %
    pub congestion_points: Vec<CongestionPoint>,
    pub emergency_level: EmergencyLevel,
    pub regional_status: HashMap<Region, RegionalGridStatus>,
}

#[derive(Deserialize)]
pub struct GridStatusQuery {
    pub regions: Option<Vec<Region>>,
    pub include_forecast: Option<bool>,
    pub detail_level: Option<DetailLevel>,
}

async fn get_grid_status_handler(
    query: GridStatusQuery,
    auth: AuthContext,
) -> Result<impl Reply, Rejection> {
    // Check if user has permission to access grid data
    if !auth.has_permission(Permission::ReadGridStatus) {
        return Err(warp::reject::custom(ApiError::Unauthorized));
    }
    
    let grid_manager = get_grid_manager();
    let status = grid_manager.get_current_status(&query).await
        .map_err(|_| warp::reject::custom(ApiError::InternalError))?;
    
    // Filter sensitive data based on user permissions
    let filtered_status = filter_grid_data_by_permissions(status, &auth.permissions);
    
    Ok(warp::reply::json(&filtered_status))
}
```

### Emergency Management
```rust
#[derive(Deserialize)]
pub struct EmergencyDeclarationRequest {
    pub emergency_type: EmergencyType,
    pub affected_regions: Vec<Region>,
    pub severity_level: SeverityLevel,
    pub estimated_duration: Option<Duration>,
    pub required_actions: Vec<EmergencyAction>,
    pub authority_signature: AuthoritySignature,
}

async fn declare_emergency_handler(
    emergency_request: EmergencyDeclarationRequest,
    auth: AuthContext,
) -> Result<impl Reply, Rejection> {
    // Verify authority has emergency declaration powers
    if !auth.has_permission(Permission::ManageEmergency) {
        return Err(warp::reject::custom(ApiError::Unauthorized));
    }
    
    // Verify authority signature
    verify_authority_signature(&emergency_request.authority_signature, &auth)
        .map_err(|_| warp::reject::custom(ApiError::InvalidSignature))?;
    
    let emergency_manager = get_emergency_manager();
    let emergency_id = emergency_manager.declare_emergency(emergency_request).await
        .map_err(|e| warp::reject::custom(ApiError::EmergencyError(e.to_string())))?;
    
    // Broadcast emergency notification
    broadcast_emergency_notification(&emergency_id).await;
    
    Ok(warp::reply::json(&EmergencyResponse {
        emergency_id,
        status: EmergencyStatus::Active,
        declared_at: Utc::now(),
    }))
}
```

## Authority-Specific API Endpoints

### EGAT/MEA/PEA Integration
```rust
impl GridTokenXApi {
    fn authority_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let authority_auth = with_authority_auth();
        
        let grid_data_submission = warp::path("authority")
            .and(warp::path("grid-data"))
            .and(warp::post())
            .and(warp::body::json())
            .and(authority_auth.clone())
            .and_then(submit_authority_grid_data);
        
        let market_intervention = warp::path("authority")
            .and(warp::path("market-intervention"))
            .and(warp::post())
            .and(warp::body::json())
            .and(authority_auth.clone())
            .and_then(execute_market_intervention);
        
        let regulatory_compliance = warp::path("authority")
            .and(warp::path("compliance"))
            .and(warp::get())
            .and(warp::query::<ComplianceQuery>())
            .and(authority_auth)
            .and_then(get_compliance_report);
        
        grid_data_submission.or(market_intervention).or(regulatory_compliance)
    }
}

#[derive(Deserialize)]
pub struct AuthorityGridDataSubmission {
    pub authority_type: AuthorityType,
    pub data_type: GridDataType,
    pub measurements: Vec<GridMeasurement>,
    pub timestamp: DateTime<Utc>,
    pub quality_indicators: DataQualityIndicators,
    pub authority_signature: AuthoritySignature,
}

async fn submit_authority_grid_data(
    submission: AuthorityGridDataSubmission,
    auth: AuthorityAuthContext,
) -> Result<impl Reply, Rejection> {
    // Verify authority type matches authenticated authority
    if submission.authority_type != auth.authority_type {
        return Err(warp::reject::custom(ApiError::Unauthorized));
    }
    
    // Validate data quality and completeness
    validate_grid_data_quality(&submission.measurements, &submission.quality_indicators)
        .map_err(|_| warp::reject::custom(ApiError::InvalidData))?;
    
    // Store authoritative grid data
    let grid_data_manager = get_grid_data_manager();
    grid_data_manager.store_authority_data(submission).await
        .map_err(|e| warp::reject::custom(ApiError::StorageError(e.to_string())))?;
    
    // Update real-time grid state
    update_grid_state_from_authority_data(&submission).await;
    
    Ok(warp::reply::json(&GridDataSubmissionResponse {
        submission_id: generate_submission_id(),
        status: SubmissionStatus::Accepted,
        processed_at: Utc::now(),
    }))
}
```

## WebSocket API for Real-Time Updates

### Real-Time Trading Feed
```rust
use warp::ws::{WebSocket, Message};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

impl GridTokenXApi {
    fn websocket_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let trading_feed = warp::path("ws")
            .and(warp::path("trading"))
            .and(warp::ws())
            .and(with_auth())
            .map(|ws: warp::ws::Ws, auth: AuthContext| {
                ws.on_upgrade(move |socket| handle_trading_websocket(socket, auth))
            });
        
        let grid_status_feed = warp::path("ws")
            .and(warp::path("grid"))
            .and(warp::ws())
            .and(with_auth())
            .map(|ws: warp::ws::Ws, auth: AuthContext| {
                ws.on_upgrade(move |socket| handle_grid_websocket(socket, auth))
            });
        
        trading_feed.or(grid_status_feed)
    }
}

async fn handle_trading_websocket(websocket: WebSocket, auth: AuthContext) {
    let (ws_sender, mut ws_receiver) = websocket.split();
    let (tx, rx) = tokio::sync::broadcast::channel(100);
    
    // Subscribe to trading updates
    let trading_updates = get_trading_update_stream(&auth).await;
    
    // Forward trading updates to WebSocket
    let send_task = tokio::spawn(async move {
        let mut stream = BroadcastStream::new(rx);
        while let Some(update) = stream.next().await {
            if let Ok(update) = update {
                let message = serde_json::to_string(&update).unwrap();
                if ws_sender.send(Message::text(message)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming WebSocket messages
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            if let Ok(msg) = msg {
                if msg.is_text() {
                    if let Ok(request) = serde_json::from_str::<WebSocketRequest>(msg.to_str().unwrap()) {
                        handle_websocket_request(request, &tx).await;
                    }
                }
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = receive_task => {},
    }
}
```

## Error Handling and Response Formats

### Custom Error Types
```rust
#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    InvalidData,
    InvalidSignature,
    TradingError(String),
    GridError(String),
    EmergencyError(String),
    StorageError(String),
    InternalError,
    RateLimitExceeded,
    MaintenanceMode,
}

impl warp::reject::Reject for ApiError {}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: u16,
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let (code, message) = if err.is_not_found() {
        (404, "Not Found".to_string())
    } else if let Some(api_error) = err.find::<ApiError>() {
        match api_error {
            ApiError::Unauthorized => (401, "Unauthorized".to_string()),
            ApiError::InvalidData => (400, "Invalid data provided".to_string()),
            ApiError::RateLimitExceeded => (429, "Rate limit exceeded".to_string()),
            _ => (500, "Internal server error".to_string()),
        }
    } else {
        (500, "Internal server error".to_string())
    };
    
    let error_response = ErrorResponse {
        error: "API_ERROR".to_string(),
        message,
        code,
        timestamp: Utc::now(),
        request_id: generate_request_id(),
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&error_response),
        warp::http::StatusCode::from_u16(code).unwrap(),
    ))
}
```

When implementing API features, prioritize security, performance, and comprehensive documentation while ensuring seamless integration with Thailand's energy market systems and regulatory requirements.
