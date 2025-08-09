//! GridTokenX Blockchain Library with Domain-Driven Design (DDD)
//!
//! A revolutionary blockchain-based platform that enables peer-to-peer energy trading
//! in Thailand's electricity market. Built on a hybrid architecture combining traditional
//! and decentralized systems, it facilitates efficient energy distribution while promoting
//! renewable energy adoption and grid stability.
//!
//! ## Architecture
//!
//! This library follows Domain-Driven Design (DDD) principles with:
//! - **Shared Kernel**: Common infrastructure and domain components
//! - **Bounded Contexts**: Domain-specific modules for energy trading, governance, etc.
//! - **Anti-Corruption Layers**: Clean interfaces between contexts
//! - **Event-Driven Architecture**: Domain and integration events for loose coupling
//!
//! ## Features
//!
//! - **Peer-to-Peer Energy Trading**: Direct energy transactions between producers and consumers
//! - **1:1 Token-Energy Ratio**: Stable token economics with 1 kWh = 1 Token
//! - **Grid Integration**: Real-time grid management and congestion control
//! - **Renewable Energy Focus**: Carbon tracking and sustainability metrics
//! - **Governance System**: Community-driven decision making
//! - **Regulatory Compliance**: Full compliance with Thai energy regulations
//! - **Scalability**: Designed for high throughput and low latency

// Shared infrastructure and domain components (DDD Shared Kernel)
pub mod shared;

// Domain-specific bounded contexts
pub mod domains;

// Core infrastructure modules
pub mod scaling;

// Legacy modules (being migrated to DDD structure)
pub mod blockchain;
pub mod config;
pub mod storage;
pub mod utils;

pub use shared::{
    domain::{DomainEvent, DomainError, ValueObject},
    application::{Command, CommandBus, Query, QueryBus, IntegrationEvent, IntegrationEventBus},
    infrastructure::{StorageProvider, NetworkProvider, Logger},
};

// Re-export legacy commonly used types (during migration)
pub use blockchain::{Block, Blockchain, Transaction, TransactionType, ValidatorInfo};
pub use config::{NodeConfig, ApiConfig, GridConfig, P2PConfig, ConsensusConfig};
pub use storage::StorageManager;
pub use scaling::{ScalingCoordinator, ScalingConfig};
pub use utils::{crypto, EnergyConversion, ThaiEnergyMarket, Utils};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
