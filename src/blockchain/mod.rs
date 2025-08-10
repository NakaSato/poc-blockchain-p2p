//! GridTokenX Blockchain Core Module
//!
//! This module implements the core blockchain functionality for the GridTokenX
//! peer-to-peer energy trading platform in Thailand's electricity market.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub mod block;
pub mod chain;
pub mod transaction;

pub use block::{Block, ValidatorInfo};
pub use chain::Blockchain;
pub use transaction::{EnergyTransaction, GovernanceTransaction, Transaction, TransactionType};

/// Blockchain configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Target block time in seconds
    pub target_block_time: u64,
    /// Difficulty adjustment period in blocks
    pub difficulty_adjustment_period: u64,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Minimum transaction fee in tokens
    pub min_transaction_fee: u64,
    /// Energy token ratio (1 kWh = 1 Token)
    pub energy_token_ratio: f64,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            max_block_size: 1_048_576,         // 1MB
            target_block_time: 10,             // 10 seconds for energy trading responsiveness
            difficulty_adjustment_period: 144, // ~24 minutes
            max_transactions_per_block: 1000,
            min_transaction_fee: 1,  // 1 token minimum fee
            energy_token_ratio: 1.0, // 1:1 ratio as specified
        }
    }
}

/// Blockchain statistics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStats {
    /// Current blockchain height
    pub height: u64,
    /// Total number of transactions
    pub total_transactions: u64,
    /// Total energy traded (in kWh)
    pub total_energy_traded: f64,
    /// Total tokens in circulation
    pub total_tokens_circulation: u64,
    /// Number of active energy producers
    pub active_producers: u64,
    /// Number of active energy consumers
    pub active_consumers: u64,
    /// Average block time
    pub average_block_time: f64,
    /// Network hash rate (for PoW components)
    pub network_hashrate: u64,
    /// Last block timestamp
    pub last_block_time: DateTime<Utc>,
}

impl Default for BlockchainStats {
    fn default() -> Self {
        Self {
            height: 0,
            total_transactions: 0,
            total_energy_traded: 0.0,
            total_tokens_circulation: 0,
            active_producers: 0,
            active_consumers: 0,
            average_block_time: 0.0,
            network_hashrate: 0,
            last_block_time: Utc::now(),
        }
    }
}

/// Account balance and energy trading information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account address (public key hash)
    pub address: String,
    /// Token balance
    pub token_balance: u64,
    /// Energy production capacity (kWh)
    pub energy_production_capacity: f64,
    /// Energy consumption demand (kWh)
    pub energy_consumption_demand: f64,
    /// Account type (Producer, Consumer, Trader, Authority)
    pub account_type: AccountType,
    /// Carbon credits balance
    pub carbon_credits: f64,
    /// Reputation score for trading
    pub reputation_score: f64,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Regulatory compliance status
    pub compliance_status: ComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    /// Energy producer (solar, wind, etc.)
    Producer,
    /// Energy consumer (households, businesses)
    Consumer,
    /// Energy trader (buy/sell for profit)
    Trader,
    /// Grid operator or energy authority
    Authority,
    /// Hybrid producer-consumer (prosumer)
    Prosumer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    /// Fully compliant with Thai energy regulations
    Compliant,
    /// Pending regulatory approval
    Pending,
    /// Non-compliant, restricted trading
    NonCompliant,
    /// Suspended by authorities
    Suspended,
}

/// Blockchain validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            ValidationResult::Valid => None,
            ValidationResult::Invalid(msg) => Some(msg),
        }
    }
}

/// Utility functions for blockchain operations
pub mod utils {
    use super::*;

    /// Calculate hash of given data
    pub fn calculate_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Calculate Merkle root of transactions
    pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<String> {
        if transactions.is_empty() {
            return Ok(String::new());
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.hash())
            .collect::<Result<Vec<_>>>()?;

        while hashes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in hashes.chunks(2) {
                if chunk.len() == 2 {
                    let combined = format!("{}{}", chunk[0], chunk[1]);
                    next_level.push(calculate_hash(combined.as_bytes()));
                } else {
                    // Odd number of hashes, duplicate the last one
                    let combined = format!("{}{}", chunk[0], chunk[0]);
                    next_level.push(calculate_hash(combined.as_bytes()));
                }
            }

            hashes = next_level;
        }

        Ok(hashes.into_iter().next().unwrap_or_default())
    }

    /// Validate address format
    pub fn validate_address(address: &str) -> bool {
        // Thai energy address format: 40 character hex string
        address.len() == 40 && address.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Generate unique transaction ID
    pub fn generate_transaction_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Convert energy amount to tokens (1 kWh = 1 Token)
    pub fn energy_to_tokens(kwh: f64) -> u64 {
        (kwh * 1_000_000.0) as u64 // Convert to micro-tokens for precision
    }

    /// Convert tokens to energy amount
    pub fn tokens_to_energy(tokens: u64) -> f64 {
        tokens as f64 / 1_000_000.0 // Convert from micro-tokens
    }

    /// Calculate carbon credits for renewable energy
    pub fn calculate_carbon_credits(energy_kwh: f64, source_type: &str) -> f64 {
        match source_type.to_lowercase().as_str() {
            "solar" => energy_kwh * 0.5,      // 0.5 credits per kWh
            "wind" => energy_kwh * 0.6,       // 0.6 credits per kWh
            "hydro" => energy_kwh * 0.4,      // 0.4 credits per kWh
            "biomass" => energy_kwh * 0.3,    // 0.3 credits per kWh
            "geothermal" => energy_kwh * 0.7, // 0.7 credits per kWh
            _ => 0.0,                         // No credits for non-renewable
        }
    }

    /// Validate energy trading compliance with Thai regulations
    pub fn validate_thai_energy_compliance(
        transaction: &Transaction,
        producer_type: &AccountType,
        _consumer_type: &AccountType,
    ) -> ValidationResult {
        match transaction.transaction_type {
            TransactionType::EnergyTrade(ref energy_tx) => {
                // Check energy amount limits
                if energy_tx.energy_amount > 1000.0 {
                    // Limit large trades to registered entities only
                    if !matches!(
                        producer_type,
                        AccountType::Authority | AccountType::Producer
                    ) {
                        return ValidationResult::Invalid(
                            "Large energy trades require registered producer status".to_string(),
                        );
                    }
                }

                // Check time-of-use restrictions
                use chrono::Timelike;
                let current_hour = Utc::now().hour();
                if energy_tx.energy_amount > 100.0 && (current_hour >= 18 && current_hour <= 22) {
                    // Peak hours restriction
                    return ValidationResult::Invalid(
                        "Large trades restricted during peak hours (18:00-22:00)".to_string(),
                    );
                }

                ValidationResult::Valid
            }
            _ => ValidationResult::Valid,
        }
    }
}

