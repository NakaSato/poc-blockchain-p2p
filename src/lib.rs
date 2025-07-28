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
//!
//! ## Usage
//!
//! ```rust,no_run
//! use gridtokenx_blockchain::{Blockchain, NodeConfig, StorageManager};
//! use std::sync::Arc;
//! use tokio::sync::RwLock;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Initialize storage
//!     let storage = Arc::new(StorageManager::new("./data").await?);
//!
//!     // Create blockchain
//!     let blockchain = Arc::new(RwLock::new(Blockchain::new(storage).await?));
//!
//!     // Load configuration
//!     let config = NodeConfig::default();
//!
//!     println!("GridTokenX blockchain initialized!");
//!     Ok(())
//! }
//! ```

pub mod blockchain;
pub mod config;
pub mod storage;
pub mod utils;

// Re-export commonly used types
pub use blockchain::{Block, Blockchain, Transaction, TransactionType};
pub use config::NodeConfig;
pub use storage::StorageManager;
pub use utils::{crypto, EnergyConversion, ThaiEnergyMarket, Utils};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(!NAME.is_empty());
    }

    #[tokio::test]
    async fn test_basic_blockchain_creation() {
        let storage = StorageManager::new_memory().await.unwrap();
        let blockchain = Blockchain::new(std::sync::Arc::new(storage)).await.unwrap();

        let height = blockchain.get_height().await.unwrap();
        assert_eq!(height, 0);
    }
}
