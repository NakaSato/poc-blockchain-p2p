# DDD Implementation Guide - Phase 1: Foundation

This guide provides step-by-step instructions for implementing the first phase of the DDD migration.

## Phase 1 Setup Instructions

### Step 1: Create New Directory Structure

```bash
# From project root
mkdir -p src/shared/{domain,infrastructure,application}
mkdir -p src/shared/domain/{events,value_objects,errors}
mkdir -p src/shared/infrastructure/{persistence,messaging,cryptography}
mkdir -p src/shared/application/{command_bus,event_bus,query_bus}

mkdir -p src/domains
mkdir -p src/interfaces/{api,cli,events}
mkdir -p src/interfaces/api/{rest,graphql,websocket}
```

### Step 2: Implement Shared Domain Infrastructure

Create the foundation files that will support all domains:

```rust
// src/shared/domain/events.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

pub trait DomainEvent: Send + Sync + Debug {
    fn event_id(&self) -> Uuid;
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> String;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn event_version(&self) -> u32 { 1 }
    fn event_data(&self) -> serde_json::Value;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEventEnvelope {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: String,
    pub occurred_at: DateTime<Utc>,
    pub event_version: u32,
    pub event_data: serde_json::Value,
}

impl DomainEventEnvelope {
    pub fn from_event<T: DomainEvent>(event: &T) -> Self {
        Self {
            event_id: event.event_id(),
            event_type: event.event_type().to_string(),
            aggregate_id: event.aggregate_id(),
            occurred_at: event.occurred_at(),
            event_version: event.event_version(),
            event_data: event.event_data(),
        }
    }
}

pub trait EventHandler<T: DomainEvent>: Send + Sync {
    async fn handle(&self, event: &T) -> anyhow::Result<()>;
}
```

```rust
// src/shared/application/command_bus.rs
use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Command: Send + Sync + Debug {}

#[async_trait]
pub trait CommandHandler<TCommand, TResult>: Send + Sync
where
    TCommand: Command,
    TResult: Send + Sync,
{
    async fn handle(&self, command: TCommand) -> Result<TResult>;
}

pub struct CommandBus {
    // Implementation will be added in Phase 2
}

impl CommandBus {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn execute<TCommand, TResult>(
        &self,
        _command: TCommand,
    ) -> Result<TResult>
    where
        TCommand: Command,
        TResult: Send + Sync,
    {
        todo!("Implement in Phase 2")
    }
}
```

### Step 3: Create Energy Trading Domain Foundation

```bash
# Create energy trading domain structure
mkdir -p src/domains/energy_trading/{domain,application,infrastructure}
mkdir -p src/domains/energy_trading/domain/{entities,value_objects,aggregates,services,repositories,events}
mkdir -p src/domains/energy_trading/application/{commands,queries,handlers}
mkdir -p src/domains/energy_trading/infrastructure/{persistence,external_services,api}
```

### Step 4: Implement First Domain Entity

```rust
// src/domains/energy_trading/domain/value_objects/mod.rs
pub mod price;
pub mod energy_amount;
pub mod grid_location;
pub mod time_slot;
pub mod order_id;
pub mod trader_id;

pub use price::Price;
pub use energy_amount::EnergyAmount;
pub use grid_location::GridLocation;
pub use time_slot::TimeSlot;
pub use order_id::OrderId;
pub use trader_id::TraderId;
```

```rust
// src/domains/energy_trading/domain/value_objects/price.rs
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// Price value object representing energy price in GridTokenX tokens per kWh
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Price {
    /// Price in smallest token unit (micro-tokens)
    value_in_micro_tokens: u64,
}

impl Price {
    /// Minimum price (0.000001 GTX/kWh)
    const MIN_PRICE: u64 = 1;
    /// Maximum price (1,000,000 GTX/kWh)  
    const MAX_PRICE: u64 = 1_000_000_000_000;
    /// Conversion factor (1 GTX = 1,000,000 micro-tokens)
    const MICRO_TOKEN_MULTIPLIER: f64 = 1_000_000.0;

    /// Create new price from micro-tokens
    pub fn new(value_in_micro_tokens: u64) -> Result<Self> {
        if value_in_micro_tokens < Self::MIN_PRICE {
            return Err(anyhow!("Price cannot be less than minimum: {}", Self::MIN_PRICE));
        }
        
        if value_in_micro_tokens > Self::MAX_PRICE {
            return Err(anyhow!("Price cannot exceed maximum: {}", Self::MAX_PRICE));
        }
        
        Ok(Self { value_in_micro_tokens })
    }
    
    /// Create price from GTX per kWh
    pub fn from_gtx_per_kwh(gtx_price: f64) -> Result<Self> {
        if gtx_price <= 0.0 {
            return Err(anyhow!("GTX price must be positive"));
        }
        
        if gtx_price > (Self::MAX_PRICE as f64 / Self::MICRO_TOKEN_MULTIPLIER) {
            return Err(anyhow!("GTX price too high"));
        }
        
        let micro_tokens = (gtx_price * Self::MICRO_TOKEN_MULTIPLIER).round() as u64;
        Self::new(micro_tokens)
    }
    
    /// Get price in micro-tokens
    pub fn value(&self) -> u64 {
        self.value_in_micro_tokens
    }
    
    /// Convert to GTX per kWh
    pub fn to_gtx_per_kwh(&self) -> f64 {
        self.value_in_micro_tokens as f64 / Self::MICRO_TOKEN_MULTIPLIER
    }
    
    /// Multiply price by a factor
    pub fn multiply(&self, factor: f64) -> Result<Self> {
        if factor <= 0.0 {
            return Err(anyhow!("Factor must be positive"));
        }
        
        let new_value = (self.value_in_micro_tokens as f64 * factor).round() as u64;
        Self::new(new_value)
    }
    
    /// Add two prices
    pub fn add(&self, other: &Price) -> Result<Self> {
        let new_value = self.value_in_micro_tokens
            .checked_add(other.value_in_micro_tokens)
            .ok_or_else(|| anyhow!("Price addition overflow"))?;
        Self::new(new_value)
    }
    
    /// Check if price is within Thai energy market regulations
    pub fn is_within_thai_regulations(&self) -> bool {
        // Thai energy market regulations (example ranges)
        let min_regulated = 2_000_000; // 2 GTX/kWh minimum
        let max_regulated = 8_000_000; // 8 GTX/kWh maximum
        
        self.value_in_micro_tokens >= min_regulated && 
        self.value_in_micro_tokens <= max_regulated
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value_in_micro_tokens.cmp(&other.value_in_micro_tokens)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6} GTX/kWh", self.to_gtx_per_kwh())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_creation() {
        let price = Price::from_gtx_per_kwh(3.5).unwrap();
        assert_eq!(price.to_gtx_per_kwh(), 3.5);
    }

    #[test]
    fn test_price_validation() {
        assert!(Price::from_gtx_per_kwh(0.0).is_err());
        assert!(Price::from_gtx_per_kwh(-1.0).is_err());
        assert!(Price::from_gtx_per_kwh(2000000.0).is_err());
    }

    #[test]
    fn test_price_operations() {
        let price1 = Price::from_gtx_per_kwh(2.0).unwrap();
        let price2 = Price::from_gtx_per_kwh(3.0).unwrap();
        
        let sum = price1.add(&price2).unwrap();
        assert_eq!(sum.to_gtx_per_kwh(), 5.0);
        
        let doubled = price1.multiply(2.0).unwrap();
        assert_eq!(doubled.to_gtx_per_kwh(), 4.0);
    }

    #[test]
    fn test_thai_regulations() {
        let valid_price = Price::from_gtx_per_kwh(3.0).unwrap();
        assert!(valid_price.is_within_thai_regulations());
        
        let too_low = Price::from_gtx_per_kwh(1.0).unwrap();
        assert!(!too_low.is_within_thai_regulations());
        
        let too_high = Price::from_gtx_per_kwh(10.0).unwrap();
        assert!(!too_high.is_within_thai_regulations());
    }
}
```

### Step 5: Update Main Library

```rust
// src/lib.rs (updated)
//! GridTokenX Blockchain Library - DDD Architecture
//!
//! A revolutionary blockchain-based platform that enables peer-to-peer energy trading
//! in Thailand's electricity market. Built with Domain-Driven Design principles.

// Legacy modules (maintain during migration)
pub mod api;
pub mod blockchain;
pub mod config;
pub mod consensus;
pub mod energy;
pub mod governance;
pub mod p2p;
pub mod scaling;
pub mod storage;
pub mod utils;

// New DDD architecture
pub mod shared;
pub mod domains;
pub mod interfaces;

// Re-export key types for backwards compatibility
pub use blockchain::{Block, Blockchain, Transaction, ValidatorInfo};
pub use config::NodeConfig;
pub use storage::StorageManager;

// New DDD exports
pub use shared::domain::events::{DomainEvent, DomainEventEnvelope};
pub use shared::application::command_bus::{Command, CommandHandler, CommandBus};

// Domain exports
pub use domains::energy_trading::domain::value_objects::{Price, EnergyAmount, GridLocation};

// Feature flags for gradual migration
#[cfg(feature = "ddd-energy-trading")]
pub use domains::energy_trading as new_energy_trading;

#[cfg(not(feature = "ddd-energy-trading"))]
pub use energy as legacy_energy_trading;
```

### Step 6: Add Feature Flags to Cargo.toml

```toml
# Cargo.toml (add to existing features section)
[features]
default = []
ddd-migration = []
ddd-energy-trading = ["ddd-migration"]
ddd-governance = ["ddd-migration"]
ddd-grid-management = ["ddd-migration"]
legacy-compatibility = []
compare-implementations = ["ddd-migration", "legacy-compatibility"]
```

### Step 7: Create Migration Tests

```rust
// tests/ddd_migration_tests.rs
use gridtokenx_blockchain::domains::energy_trading::domain::value_objects::Price;

#[cfg(test)]
mod migration_tests {
    use super::*;

    #[test]
    fn test_price_value_object() {
        // Test new DDD Price value object
        let price = Price::from_gtx_per_kwh(3.5).unwrap();
        assert_eq!(price.to_gtx_per_kwh(), 3.5);
        assert!(price.is_within_thai_regulations());
    }

    #[test]
    fn test_price_business_rules() {
        // Test business rule enforcement
        assert!(Price::from_gtx_per_kwh(0.0).is_err());
        assert!(Price::from_gtx_per_kwh(-1.0).is_err());
        
        let valid_price = Price::from_gtx_per_kwh(4.0).unwrap();
        let doubled = valid_price.multiply(2.0).unwrap();
        assert_eq!(doubled.to_gtx_per_kwh(), 8.0);
    }

    #[tokio::test]
    async fn test_domain_events_infrastructure() {
        use gridtokenx_blockchain::shared::domain::events::DomainEventEnvelope;
        use chrono::Utc;
        use uuid::Uuid;
        
        // Test event infrastructure
        // This will be expanded as we add actual domain events
    }
}
```

## Running Phase 1

### Build Commands

```bash
# Build with new DDD features
cargo build --features ddd-migration

# Build with specific domain features
cargo build --features ddd-energy-trading

# Build in legacy mode (current working state)
cargo build --features legacy-compatibility

# Test new DDD components
cargo test ddd_migration_tests --features ddd-migration
```

### Validation Checklist

- [ ] New directory structure created
- [ ] Shared domain infrastructure compiles
- [ ] Price value object works correctly
- [ ] Feature flags control compilation
- [ ] Tests pass for new components
- [ ] Existing functionality still works
- [ ] CI/CD pipeline handles new structure

## Next Steps

After completing Phase 1:

1. **Implement EnergyAmount value object** (similar to Price)
2. **Create OrderId and TraderId value objects**
3. **Implement GridLocation with Thai validation**
4. **Build EnergyOrder entity with business logic**
5. **Create OrderBook aggregate**
6. **Implement order placement commands**

## Common Issues and Solutions

### Issue: Compilation Errors
**Solution**: Make sure all new modules are properly declared in mod.rs files

### Issue: Feature Flag Conflicts  
**Solution**: Use cfg attributes consistently and test all feature combinations

### Issue: Import Path Confusion
**Solution**: Update imports gradually and maintain backwards compatibility

### Issue: Test Failures
**Solution**: Run tests with specific feature flags and verify test isolation

---

This foundation provides the infrastructure for the full DDD migration while maintaining system functionality.
