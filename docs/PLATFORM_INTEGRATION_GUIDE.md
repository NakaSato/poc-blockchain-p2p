# GridTokenX Platform Integration Guide

## Overview

This guide demonstrates how to integrate the enhanced energy trading platform features with the existing GridTokenX blockchain infrastructure. The integration builds upon the current modular architecture and adds smart contract-like functionality through enhanced service modules.

## Quick Start Integration

### 1. Enhanced Energy Trading with Dynamic Pricing

```rust
use gridtokenx_blockchain::{
    energy::{EnergyTrading, GridManager},
    blockchain::Blockchain,
    api::ApiServer,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize enhanced trading system
    let trading_system = EnhancedTradingSystem::new().await?;
    
    // Start enhanced services
    trading_system.start_enhanced_services().await?;
    
    // Place orders with automatic price discovery
    let order_result = trading_system.place_order_with_dynamic_pricing(
        "trader_001",
        "Buy",
        100.0,
        "thailand_central"
    ).await?;
    
    println!("Order placed: {:?}", order_result);
    
    Ok(())
}

struct EnhancedTradingSystem {
    energy_trading: Arc<RwLock<EnergyTrading>>,
    grid_manager: Arc<RwLock<GridManager>>,
    blockchain: Arc<RwLock<Blockchain>>,
}

impl EnhancedTradingSystem {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize existing order books
        let order_books = Arc::new(RwLock::new(HashMap::new()));
        
        // Create enhanced services
        let pricing_config = PricingConfig {
            p_balance: 4.0,  // 4 THB per kWh base price
            p_con: 2.0,      // ±2x price range
            k: 1.0,          // Standard sensitivity
            update_interval: 300, // 5 minutes
            min_ratio: 0.01,
        };
        
        let auction_config = AuctionConfig {
            interval_minutes: 30,
            market_open_time: "06:00".to_string(),
            market_close_time: "22:00".to_string(),
            min_order_volume: 1.0,
            max_price_deviation: 0.50,
        };
        
        let trading_service = Arc::new(EnergyTradingDomainService::new()?);
        let pricing_service = Arc::new(DynamicPricingService::new(order_books.clone(), pricing_config));
        let auction_service = Arc::new(AuctionSchedulerService::new(
            order_books,
            pricing_service.clone(),
            auction_config,
        ));
        
        Ok(Self {
            trading_service,
            pricing_service,
            auction_service,
        })
    }
    
    async fn start_enhanced_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Start automatic price updates
        self.pricing_service.start_price_updates().await?;
        
        // Start auction scheduling
        self.auction_service.start_auction_scheduler().await?;
        
        println!("Enhanced trading services started");
        Ok(())
    }
    
    async fn place_order_with_dynamic_pricing(
        &self,
        trader_id: &str,
        order_type: &str,
        energy_amount: f64,
        market_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get current indicative price
        let price_signal = self.pricing_service
            .get_current_price_signal(market_name)
            .await;
        
        let suggested_price = if let Some(signal) = price_signal {
            signal.indicative_price
        } else {
            // Calculate new price if no signal exists
            self.pricing_service
                .calculate_indicative_price(market_name)
                .await?
        };
        
        // Create and execute order command
        let command = PlaceEnergyOrderCommand {
            trader_id: trader_id.to_string(),
            order_type: order_type.to_string(),
            energy_amount,
            price_per_kwh: suggested_price,
            trading_window_start: chrono::Utc::now(),
            trading_window_end: chrono::Utc::now() + chrono::Duration::hours(1),
            market_name: market_name.to_string(),
        };
        
        // Use existing command handler
        let handler = PlaceEnergyOrderHandler::new(self.trading_service.clone());
        let result = handler.handle(command).await?;
        
        Ok(result.order_id)
    }
}
```

### 2. Governance Token Integration

```rust
use poc_blockchain_p2p::domains::governance::{
    StakingService, TokenAmount, VotingPower,
    IncentiveMechanismService,
};

async fn setup_governance_system() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize governance services
    let staking_service = StakingService::new().await?;
    let incentive_service = IncentiveMechanismService::new().await?;
    
    // Stake tokens for governance participation
    let stake_amount = TokenAmount::new(1000.0, 18)?; // 1000 NRG tokens
    let stake_position = staking_service
        .stake_tokens("staker_001", stake_amount, 90) // 90-day lock
        .await?;
    
    println!("Staked tokens: {:?}", stake_position);
    
    // Calculate voting power
    let voting_power = VotingPower::new(
        TokenAmount::new(1000.0, 18)?,
        90 // lock duration
    )?;
    
    println!("Voting power: {} ({}x multiplier)", 
             voting_power.effective_power(), 
             voting_power.multiplier());
    
    // Reward energy conservation
    incentive_service
        .reward_conservation("trader_001", 50.0) // 50 kWh saved
        .await?;
    
    Ok(())
}
```

### 3. Renewable Energy Certificate (REC) Marketplace

```rust
use poc_blockchain_p2p::domains::governance::{
    RECMarketplaceService, EnergySourceType,
};

async fn setup_rec_marketplace() -> Result<(), Box<dyn std::error::Error>> {
    let rec_service = RECMarketplaceService::new().await?;
    
    // Mint REC for renewable energy generation
    let rec_token = rec_service
        .mint_rec(
            100.0, // 100 kWh generated
            EnergySourceType::Solar,
            "generator_001",
        )
        .await?;
    
    println!("Minted REC: {:?}", rec_token);
    
    // List REC for sale
    rec_service
        .list_rec_for_sale(rec_token.id().clone(), 0.50) // 0.50 THB per kWh
        .await?;
    
    // Purchase and retire REC
    let purchased_rec = rec_service
        .purchase_rec(rec_token.id().clone(), "buyer_001")
        .await?;
    
    rec_service
        .retire_rec(purchased_rec.id().clone(), "Environmental offset")
        .await?;
    
    Ok(())
}
```

## Architecture Integration Points

### 1. Event-Driven Integration

The enhanced platform integrates seamlessly with the existing event system:

```rust
use poc_blockchain_p2p::shared::application::IntegrationEvent;

// Price signal events
#[derive(Debug, Clone)]
pub struct PriceSignalUpdatedEvent {
    pub market_name: String,
    pub old_price: f64,
    pub new_price: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl IntegrationEvent for PriceSignalUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "PriceSignalUpdated"
    }
    
    fn aggregate_id(&self) -> String {
        self.market_name.clone()
    }
}

// Auction completion events
#[derive(Debug, Clone)]
pub struct AuctionCompletedEvent {
    pub market_name: String,
    pub auction_id: String,
    pub clearing_price: f64,
    pub total_volume: f64,
    pub trades_count: usize,
}

impl IntegrationEvent for AuctionCompletedEvent {
    fn event_type(&self) -> &'static str {
        "AuctionCompleted"
    }
    
    fn aggregate_id(&self) -> String {
        self.auction_id.clone()
    }
}
```

### 2. Command/Query Separation Enhancement

Building on the existing CQRS infrastructure:

```rust
// Enhanced auction commands
#[derive(Debug, Clone)]
pub struct ScheduleAuctionCommand {
    pub market_name: String,
    pub auction_time: chrono::DateTime<chrono::Utc>,
    pub config_overrides: Option<AuctionConfig>,
}

impl Command for ScheduleAuctionCommand {
    type Result = ScheduledAuction;
    
    fn command_type(&self) -> &'static str {
        "ScheduleAuction"
    }
}

// Price discovery queries
#[derive(Debug, Clone)]
pub struct GetMarketPriceQuery {
    pub market_name: String,
    pub include_history: bool,
    pub limit: Option<usize>,
}

impl Query for GetMarketPriceQuery {
    type Result = PriceSignalResponse;
    
    fn query_type(&self) -> &'static str {
        "GetMarketPrice"
    }
}
```

### 3. Repository Pattern Extension

Extending existing repository patterns for new entities:

```rust
use poc_blockchain_p2p::shared::domain::repository::Repository;
use async_trait::async_trait;

#[async_trait]
pub trait StakePositionRepository: Repository<StakePosition, StakeId> {
    async fn find_by_staker(&self, staker_id: &str) -> Result<Vec<StakePosition>, DomainError>;
    async fn find_active_stakes(&self) -> Result<Vec<StakePosition>, DomainError>;
    async fn find_by_voting_power_threshold(&self, threshold: u64) -> Result<Vec<StakePosition>, DomainError>;
}

#[async_trait]
pub trait RECTokenRepository: Repository<RECToken, RECId> {
    async fn find_by_energy_source(&self, source: &EnergySourceType) -> Result<Vec<RECToken>, DomainError>;
    async fn find_available_for_sale(&self) -> Result<Vec<RECToken>, DomainError>;
    async fn find_by_generator(&self, generator_id: &str) -> Result<Vec<RECToken>, DomainError>;
}
```

## Migration Strategy

### Phase 1: Core Enhancement (Current → Enhanced)

1. **Dynamic Pricing Integration** (Week 1-2)
   ```bash
   # Add dynamic pricing to existing energy trading
   cargo test --package poc_blockchain_p2p --lib domains::energy_trading::domain::services::dynamic_pricing_service
   ```

2. **Auction Scheduling** (Week 3-4)
   ```bash
   # Integrate auction scheduler with existing order book
   cargo test --package poc_blockchain_p2p --lib domains::energy_trading::domain::services::auction_scheduler_service
   ```

### Phase 2: Governance System (Enhanced → Full Platform)

1. **Token System** (Week 5-6)
   ```bash
   # Add governance domain
   cargo test --package poc_blockchain_p2p --lib domains::governance
   ```

2. **REC Marketplace** (Week 7-8)
   ```bash
   # Integrate REC functionality
   cargo test --package poc_blockchain_p2p --lib domains::governance::domain::services::rec_marketplace_service
   ```

### Phase 3: Production Optimization

1. **Performance Tuning** (Week 9-10)
   ```bash
   # Benchmark enhanced order matching
   cargo bench --package poc_blockchain_p2p
   ```

2. **API Integration** (Week 11-12)
   ```bash
   # Add REST/GraphQL endpoints
   cargo run --bin api_server
   ```

## Testing Strategy

### Unit Tests
```bash
# Test all enhanced services
cargo test --package poc_blockchain_p2p --lib -- --test-threads=1

# Test specific domain
cargo test --package poc_blockchain_p2p --lib domains::energy_trading

# Test governance features
cargo test --package poc_blockchain_p2p --lib domains::governance
```

### Integration Tests
```bash
# Test end-to-end auction flow
cargo test --package poc_blockchain_p2p --test integration_auction_flow

# Test price discovery integration
cargo test --package poc_blockchain_p2p --test integration_price_discovery
```

### Performance Tests
```bash
# Benchmark order matching with dynamic pricing
cargo bench order_matching_performance

# Benchmark auction clearing
cargo bench auction_clearing_performance
```

## Configuration

### Enhanced Configuration File
```toml
# config/enhanced.toml
[dynamic_pricing]
p_balance = 4.0          # Base price (THB/kWh)
p_con = 2.0              # Price range multiplier
k = 1.0                  # Sensitivity parameter
update_interval = 300    # Update every 5 minutes

[auction]
interval_minutes = 30    # 30-minute auctions
market_open = "06:00"    # Market opens at 6 AM
market_close = "22:00"   # Market closes at 10 PM
min_volume = 1.0         # Minimum 1 kWh orders

[governance]
nrg_total_supply = 1_000_000_000  # 1 billion NRG tokens
stake_min_amount = 100.0          # Minimum 100 NRG to stake
vote_threshold = 1000             # 1000 voting power for proposals

[stable_credits]
supported_currencies = ["THB", "USD", "EUR"]
default_currency = "THB"
conversion_fee = 0.001   # 0.1% conversion fee
```

## Monitoring and Observability

### Enhanced Metrics
```rust
use tracing::{info, warn, error};

// Price discovery metrics
info!(
    market = %market_name,
    price = %indicative_price,
    ratio = %supply_demand_ratio,
    "Price signal updated"
);

// Auction metrics
info!(
    market = %market_name,
    auction_id = %auction_id,
    clearing_price = %clearing_price,
    volume = %total_volume,
    trades = %trades_count,
    "Auction completed"
);

// Governance metrics
info!(
    staker = %staker_id,
    amount = %stake_amount,
    duration = %lock_duration,
    voting_power = %voting_power,
    "Tokens staked"
);
```

### Health Checks
```rust
pub async fn health_check() -> Result<HealthStatus, Box<dyn std::error::Error>> {
    let mut status = HealthStatus::new();
    
    // Check pricing service
    status.add_check("pricing_service", check_pricing_service().await?);
    
    // Check auction scheduler
    status.add_check("auction_scheduler", check_auction_scheduler().await?);
    
    // Check governance services
    status.add_check("governance_services", check_governance_services().await?);
    
    Ok(status)
}
```

## API Endpoints

### Enhanced REST API
```rust
// Energy trading with dynamic pricing
POST /api/v1/orders
GET  /api/v1/markets/{market}/price
GET  /api/v1/markets/{market}/auctions

// Governance
POST /api/v1/governance/stake
GET  /api/v1/governance/voting-power/{staker}
POST /api/v1/governance/vote

// REC marketplace
POST /api/v1/recs/mint
GET  /api/v1/recs/marketplace
POST /api/v1/recs/{id}/retire
```

This integration guide provides a comprehensive roadmap for implementing the enhanced energy trading platform features while maintaining compatibility with the existing GridTokenX infrastructure. The phased approach ensures minimal disruption to current operations while delivering advanced smart contract-like functionality through the modular architecture.
