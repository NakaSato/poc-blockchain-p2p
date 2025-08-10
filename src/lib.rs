//! GridTokenX Blockchain Library
//!
//! A revolutionary blockchain-based platform that enables peer-to-peer energy trading
//! in Thailand's electricity market. Built on a hybrid architecture combining traditional
//! and decentralized systems, it facilitates efficient energy distribution while promoting
//! renewable energy adoption and grid stability.
//!
//! ## Features
//!
//! - **Peer-to-Peer Energy Trading**: Direct energy transactions between producers and consumers
//! - **1:1 Token-Energy Ratio**: Stable token economics with 1 kWh = 1 Token
//! - **Grid Integration**: Real-time grid management and congestion control
//! - **Renewable Energy Focus**: Carbon tracking and sustainability metrics
//! - **Governance System**: Community-driven decision making
//! - **Regulatory Compliance**: Full compliance with Thai energy regulations

// Core blockchain modules
pub mod blockchain;
pub mod config;
pub mod storage;
pub mod utils;

// Re-export commonly used types
pub use blockchain::{Block, Blockchain, Transaction, TransactionType, ValidatorInfo};
pub use config::{NodeConfig, ApiConfig, GridConfig, P2PConfig, ConsensusConfig};
pub use storage::StorageManager;
pub use utils::{crypto, EnergyConversion, ThaiEnergyMarket, Utils};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
