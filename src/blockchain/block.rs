//! GridTokenX Block Module
//!
//! This module implements the block structure and validation logic for the
//! GridTokenX peer-to-peer energy trading blockchain.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use super::{Transaction, ValidationResult};

/// Block structure for GridTokenX blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header containing metadata
    pub header: BlockHeader,
    /// List of transactions in this block
    pub transactions: Vec<Transaction>,
    /// Block size in bytes
    pub size: usize,
    /// Energy trading statistics for this block
    pub energy_stats: BlockEnergyStats,
    /// Governance actions in this block
    pub governance_actions: Vec<GovernanceAction>,
}

/// Block header containing essential block metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block version
    pub version: u32,
    /// Previous block hash
    pub previous_hash: String,
    /// Merkle root of all transactions
    pub merkle_root: String,
    /// Block timestamp
    pub timestamp: DateTime<Utc>,
    /// Block difficulty target
    pub difficulty: u64,
    /// Block nonce for proof-of-work
    pub nonce: u64,
    /// Block height in the chain
    pub height: u64,
    /// Hash of this block
    pub hash: String,
    /// Validator/miner information
    pub validator: ValidatorInfo,
    /// Gas used in this block
    pub gas_used: u64,
    /// Gas limit for this block
    pub gas_limit: u64,
    /// Extra data field (up to 32 bytes)
    pub extra_data: Vec<u8>,
}

/// Validator information for block creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// Validator's address
    pub address: String,
    /// Validator's stake amount
    pub stake: u64,
    /// Validator's reputation score
    pub reputation: f64,
    /// Energy authority type (if applicable)
    pub authority_type: Option<String>,
}

/// Energy trading statistics for a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEnergyStats {
    /// Total energy traded in this block (kWh)
    pub total_energy_traded: f64,
    /// Number of energy transactions
    pub energy_transaction_count: u64,
    /// Average energy price in this block
    pub average_energy_price: f64,
    /// Peak energy demand during block period
    pub peak_demand: f64,
    /// Renewable energy percentage
    pub renewable_percentage: f64,
    /// Carbon credits generated
    pub carbon_credits_generated: f64,
    /// Grid stability metrics
    pub grid_stability: GridStabilityMetrics,
    /// Energy sources breakdown
    pub energy_sources: HashMap<String, f64>,
}

/// Grid stability metrics for energy trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStabilityMetrics {
    /// Frequency deviation from 50Hz
    pub frequency_deviation: f64,
    /// Voltage stability score (0-100)
    pub voltage_stability: u8,
    /// Load balancing efficiency (0-100)
    pub load_balance_efficiency: u8,
    /// Congestion level (0-100, lower is better)
    pub congestion_level: u8,
}

/// Governance actions executed in a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAction {
    /// Action type
    pub action_type: String,
    /// Proposal ID (if applicable)
    pub proposal_id: Option<String>,
    /// Parameters affected
    pub parameters: HashMap<String, String>,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
}

impl Block {
    /// Create a new block with given transactions
    pub fn new(
        previous_hash: String,
        transactions: Vec<Transaction>,
        height: u64,
        validator: ValidatorInfo,
    ) -> Result<Self> {
        let timestamp = Utc::now();
        let merkle_root = Self::calculate_merkle_root(&transactions)?;

        // Calculate energy statistics
        let energy_stats = Self::calculate_energy_stats(&transactions)?;

        // Extract governance actions
        let governance_actions = Self::extract_governance_actions(&transactions)?;

        // Calculate gas used
        let gas_used = transactions.iter().map(|tx| tx.gas_limit).sum();

        let header = BlockHeader {
            version: 1,
            previous_hash,
            merkle_root,
            timestamp,
            difficulty: 1000, // Default difficulty
            nonce: 0,
            height,
            hash: String::new(), // Will be calculated after block creation
            validator,
            gas_used,
            gas_limit: 10_000_000, // 10M gas limit per block
            extra_data: Vec::new(),
        };

        let size = Self::calculate_block_size(&header, &transactions)?;

        let mut block = Self {
            header,
            transactions,
            size,
            energy_stats,
            governance_actions,
        };

        // Calculate and set block hash
        block.header.hash = block.calculate_hash()?;

        Ok(block)
    }

    /// Create genesis block with initial transactions
    pub fn new_genesis(transactions: Vec<Transaction>, extra_data: String) -> Result<Self> {
        let validator = ValidatorInfo {
            address: "genesis".to_string(),
            stake: 0,
            reputation: 100.0,
            authority_type: Some("SYSTEM".to_string()),
        };

        let mut genesis_block = Self::new(
            String::new(), // No previous hash for genesis
            transactions,
            0, // Genesis height is 0
            validator,
        )?;

        genesis_block.header.extra_data = extra_data.as_bytes().to_vec();
        genesis_block.header.hash = genesis_block.calculate_hash()?;

        Ok(genesis_block)
    }

    /// Calculate block hash
    pub fn calculate_hash(&self) -> Result<String> {
        // Create a copy of header with hash field set to empty to ensure consistent hashing
        let mut header_for_hash = self.header.clone();
        header_for_hash.hash = String::new();
        
        let header_data = bincode::serialize(&header_for_hash)
            .map_err(|e| anyhow!("Failed to serialize block header: {}", e))?;

        let mut hasher = Sha256::new();
        hasher.update(&header_data);
        Ok(hex::encode(hasher.finalize()))
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
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    next_level.push(hex::encode(hasher.finalize()));
                } else {
                    // Odd number of hashes, duplicate the last one
                    let combined = format!("{}{}", chunk[0], chunk[0]);
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    next_level.push(hex::encode(hasher.finalize()));
                }
            }

            hashes = next_level;
        }

        Ok(hashes.into_iter().next().unwrap_or_default())
    }

    /// Calculate energy statistics for the block
    fn calculate_energy_stats(transactions: &[Transaction]) -> Result<BlockEnergyStats> {
        let mut total_energy = 0.0;
        let mut energy_tx_count = 0;
        let mut total_value = 0u64;
        let mut carbon_credits = 0.0;
        let mut energy_sources: HashMap<String, f64> = HashMap::new();
        let mut renewable_energy = 0.0;

        for tx in transactions {
            if let super::TransactionType::EnergyTrade(energy_tx) = &tx.transaction_type {
                total_energy += energy_tx.energy_amount;
                energy_tx_count += 1;
                total_value += energy_tx.total_value;
                carbon_credits += energy_tx.carbon_credits;

                // Track energy sources
                let source_name = format!("{:?}", energy_tx.energy_source);
                *energy_sources.entry(source_name.clone()).or_insert(0.0) +=
                    energy_tx.energy_amount;

                // Calculate renewable energy
                match energy_tx.energy_source {
                    super::transaction::EnergySource::Solar
                    | super::transaction::EnergySource::Wind
                    | super::transaction::EnergySource::Hydro
                    | super::transaction::EnergySource::Biomass
                    | super::transaction::EnergySource::Geothermal => {
                        renewable_energy += energy_tx.energy_amount;
                    }
                    _ => {}
                }
            }
        }

        let average_price = if total_energy > 0.0 {
            total_value as f64 / total_energy
        } else {
            0.0
        };

        let renewable_percentage = if total_energy > 0.0 {
            (renewable_energy / total_energy) * 100.0
        } else {
            0.0
        };

        Ok(BlockEnergyStats {
            total_energy_traded: total_energy,
            energy_transaction_count: energy_tx_count,
            average_energy_price: average_price,
            peak_demand: total_energy, // Simplified - would be calculated from real grid data
            renewable_percentage,
            carbon_credits_generated: carbon_credits,
            grid_stability: GridStabilityMetrics::default(),
            energy_sources,
        })
    }

    /// Extract governance actions from transactions
    fn extract_governance_actions(transactions: &[Transaction]) -> Result<Vec<GovernanceAction>> {
        let mut actions = Vec::new();

        for tx in transactions {
            if let super::TransactionType::Governance(gov_tx) = &tx.transaction_type {
                match gov_tx {
                    super::transaction::GovernanceTransaction::ProposalExecution {
                        proposal_id,
                        ..
                    } => {
                        actions.push(GovernanceAction {
                            action_type: "ProposalExecution".to_string(),
                            proposal_id: Some(proposal_id.clone()),
                            parameters: HashMap::new(),
                            executed_at: tx.timestamp,
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(actions)
    }

    /// Calculate block size in bytes
    fn calculate_block_size(header: &BlockHeader, transactions: &[Transaction]) -> Result<usize> {
        let header_size = bincode::serialize(header)
            .map_err(|e| anyhow!("Failed to serialize header: {}", e))?
            .len();

        let mut tx_size = 0;
        for tx in transactions {
            tx_size += tx.size()?;
        }

        Ok(header_size + tx_size)
    }

    /// Validate block structure and contents
    pub fn validate(&self, previous_block: Option<&Block>) -> ValidationResult {
        // Validate block header
        if let ValidationResult::Invalid(msg) = self.validate_header(previous_block) {
            return ValidationResult::Invalid(msg);
        }

        // Validate transactions
        for tx in &self.transactions {
            if let Err(e) = tx.validate() {
                return ValidationResult::Invalid(format!("Invalid transaction: {}", e));
            }
        }

        // Validate Merkle root
        match Self::calculate_merkle_root(&self.transactions) {
            Ok(calculated_root) => {
                if calculated_root != self.header.merkle_root {
                    return ValidationResult::Invalid("Invalid Merkle root".to_string());
                }
            }
            Err(e) => {
                return ValidationResult::Invalid(format!(
                    "Failed to calculate Merkle root: {}",
                    e
                ));
            }
        }

        // Validate block hash
        match self.calculate_hash() {
            Ok(calculated_hash) => {
                if calculated_hash != self.header.hash {
                    return ValidationResult::Invalid("Invalid block hash".to_string());
                }
            }
            Err(e) => {
                return ValidationResult::Invalid(format!("Failed to calculate block hash: {}", e));
            }
        }

        // Validate energy trading constraints
        if let ValidationResult::Invalid(msg) = self.validate_energy_constraints() {
            return ValidationResult::Invalid(msg);
        }

        ValidationResult::Valid
    }

    /// Validate block header
    fn validate_header(&self, previous_block: Option<&Block>) -> ValidationResult {
        // Check timestamp
        if self.header.timestamp > Utc::now() + chrono::Duration::minutes(10) {
            return ValidationResult::Invalid("Block timestamp too far in future".to_string());
        }

        // Check height and previous hash
        if let Some(prev_block) = previous_block {
            if self.header.height != prev_block.header.height + 1 {
                return ValidationResult::Invalid("Invalid block height".to_string());
            }
            if self.header.previous_hash != prev_block.header.hash {
                return ValidationResult::Invalid("Invalid previous hash".to_string());
            }
            if self.header.timestamp <= prev_block.header.timestamp {
                return ValidationResult::Invalid(
                    "Block timestamp must be after previous block".to_string(),
                );
            }
        } else if self.header.height != 0 {
            return ValidationResult::Invalid("Genesis block must have height 0".to_string());
        }

        // Check gas limits
        if self.header.gas_used > self.header.gas_limit {
            return ValidationResult::Invalid("Gas used exceeds gas limit".to_string());
        }

        ValidationResult::Valid
    }

    /// Validate energy trading constraints specific to Thai market
    fn validate_energy_constraints(&self) -> ValidationResult {
        let max_energy_per_block = 100_000.0; // 100 MWh per block
        let max_price_deviation = 50.0; // 50% price deviation allowed

        if self.energy_stats.total_energy_traded > max_energy_per_block {
            return ValidationResult::Invalid(format!(
                "Total energy traded ({} kWh) exceeds maximum per block ({} kWh)",
                self.energy_stats.total_energy_traded, max_energy_per_block
            ));
        }

        // Check for reasonable energy prices (Thai market constraints)
        if self.energy_stats.average_energy_price > 0.0 {
            let thai_base_price = 4000.0; // 4 tokens per kWh (example base rate)
            let deviation = (self.energy_stats.average_energy_price - thai_base_price).abs()
                / thai_base_price
                * 100.0;

            if deviation > max_price_deviation {
                return ValidationResult::Invalid(format!(
                    "Average energy price deviates too much from base rate: {:.2}%",
                    deviation
                ));
            }
        }

        ValidationResult::Valid
    }

    /// Get block summary information
    pub fn get_summary(&self) -> BlockSummary {
        BlockSummary {
            height: self.header.height,
            hash: self.header.hash.clone(),
            timestamp: self.header.timestamp,
            transaction_count: self.transactions.len() as u64,
            energy_traded: self.energy_stats.total_energy_traded,
            carbon_credits: self.energy_stats.carbon_credits_generated,
            renewable_percentage: self.energy_stats.renewable_percentage,
            validator_address: self.header.validator.address.clone(),
            size_bytes: self.size,
        }
    }

    /// Check if block contains any governance actions
    pub fn has_governance_actions(&self) -> bool {
        !self.governance_actions.is_empty()
    }

    /// Get total fees collected in this block
    pub fn get_total_fees(&self) -> u64 {
        self.transactions.iter().map(|tx| tx.fee).sum()
    }

    /// Get energy trading volume in tokens
    pub fn get_energy_trading_volume(&self) -> u64 {
        self.transactions
            .iter()
            .filter_map(|tx| {
                if let super::TransactionType::EnergyTrade(energy_tx) = &tx.transaction_type {
                    Some(energy_tx.total_value)
                } else {
                    None
                }
            })
            .sum()
    }
}

/// Block summary for quick reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSummary {
    pub height: u64,
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_count: u64,
    pub energy_traded: f64,
    pub carbon_credits: f64,
    pub renewable_percentage: f64,
    pub validator_address: String,
    pub size_bytes: usize,
}

impl Default for GridStabilityMetrics {
    fn default() -> Self {
        Self {
            frequency_deviation: 0.1,    // ±0.1 Hz is acceptable
            voltage_stability: 95,       // 95% stability
            load_balance_efficiency: 90, // 90% efficiency
            congestion_level: 10,        // 10% congestion
        }
    }
}

impl Default for ValidatorInfo {
    fn default() -> Self {
        Self {
            address: "unknown".to_string(),
            stake: 0,
            reputation: 50.0,
            authority_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::transaction::Transaction;

    #[test]
    fn test_genesis_block_creation() {
        let transactions = vec![Transaction::new_genesis_mint(
            "genesis_recipient".to_string(),
            1_000_000,
            "Genesis mint".to_string(),
        )
        .unwrap()];

        let genesis_block =
            Block::new_genesis(transactions, "GridTokenX Genesis Block".to_string()).unwrap();

        assert_eq!(genesis_block.header.height, 0);
        assert_eq!(genesis_block.header.previous_hash, "");
        assert!(!genesis_block.header.hash.is_empty());
        assert_eq!(genesis_block.transactions.len(), 1);
    }

    #[test]
    fn test_block_validation() {
        let transactions =
            vec![
                Transaction::new_genesis_mint("test".to_string(), 1000, "Test".to_string())
                    .unwrap(),
            ];

        let block = Block::new_genesis(transactions, "Test block".to_string()).unwrap();
        let validation_result = block.validate(None);

        assert!(validation_result.is_valid());
    }

    #[test]
    fn test_merkle_root_calculation() {
        let transactions = vec![
            Transaction::new_genesis_mint("test1".to_string(), 1000, "Test 1".to_string()).unwrap(),
            Transaction::new_genesis_mint("test2".to_string(), 2000, "Test 2".to_string()).unwrap(),
        ];

        let merkle_root = Block::calculate_merkle_root(&transactions).unwrap();
        assert!(!merkle_root.is_empty());
        assert_eq!(merkle_root.len(), 64); // SHA256 hex string
    }

    #[test]
    fn test_energy_stats_calculation() {
        // This test would require creating energy transactions
        // Will be implemented when the transaction module is complete
    }

    #[test]
    fn test_block_hash_consistency() {
        let transactions =
            vec![
                Transaction::new_genesis_mint("test".to_string(), 1000, "Test".to_string())
                    .unwrap(),
            ];

        let block = Block::new_genesis(transactions, "Test".to_string()).unwrap();
        let hash1 = block.calculate_hash().unwrap();
        let hash2 = block.calculate_hash().unwrap();

        assert_eq!(hash1, hash2);
        assert_eq!(hash1, block.header.hash);
    }
}
