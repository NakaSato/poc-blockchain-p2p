//! GridTokenX Consensus Engine Module
//!
//! This module implements the consensus mechanism for the GridTokenX blockchain,
//! supporting hybrid consensus with Proof of Stake and Proof of Work components.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blockchain::{Block, Blockchain, Transaction, ValidatorInfo};
use crate::config::ConsensusConfig;

/// Consensus engine for block validation and creation
#[derive(Debug)]
pub struct ConsensusEngine {
    blockchain: Arc<RwLock<Blockchain>>,
    config: ConsensusConfig,
    validator_set: RwLock<ValidatorSet>,
    consensus_state: RwLock<ConsensusState>,
    mining_enabled: bool,
}

/// Set of validators participating in consensus
#[derive(Debug, Default)]
pub struct ValidatorSet {
    validators: HashMap<String, Validator>,
    active_validators: Vec<String>,
    current_proposer: Option<String>,
}

/// Individual validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub reputation: f64,
    pub last_block_time: Option<DateTime<Utc>>,
    pub consecutive_misses: u32,
    pub total_blocks_proposed: u64,
    pub is_active: bool,
}

/// Current consensus state
#[derive(Debug, Default)]
pub struct ConsensusState {
    current_round: u64,
    current_height: u64,
    last_block_time: Option<DateTime<Utc>>,
    difficulty: u64,
    block_rewards_distributed: u64,
}

/// Consensus algorithm types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    ProofOfStake,
    ProofOfWork,
    Hybrid,
}

/// Block proposal for consensus
#[derive(Debug, Clone)]
pub struct BlockProposal {
    pub block: Block,
    pub proposer: String,
    pub proposed_at: DateTime<Utc>,
    pub votes: HashMap<String, Vote>,
}

/// Validator vote on a block proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub validator: String,
    pub block_hash: String,
    pub vote_type: VoteType,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

/// Types of votes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Prevote,
    Precommit,
    Commit,
}

/// Consensus metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMetrics {
    pub current_validators: u64,
    pub total_stake: u64,
    pub average_block_time: f64,
    pub last_finalized_height: u64,
    pub missed_blocks: u64,
    pub consensus_rounds: u64,
}

impl ConsensusEngine {
    /// Create new consensus engine
    pub async fn new(
        blockchain: Arc<RwLock<Blockchain>>,
        config: ConsensusConfig,
        mining_enabled: bool,
    ) -> Result<Self> {
        let validator_set = ValidatorSet::default();
        let consensus_state = ConsensusState::default();

        Ok(Self {
            blockchain,
            config,
            validator_set: RwLock::new(validator_set),
            consensus_state: RwLock::new(consensus_state),
            mining_enabled,
        })
    }

    /// Start the consensus engine
    pub async fn start(&self) -> Result<()> {
        tracing::info!(
            "Starting consensus engine (mining: {})",
            self.mining_enabled
        );

        // Initialize validator set
        self.initialize_validators().await?;

        // Start consensus loop
        loop {
            match self.config.algorithm {
                crate::config::ConsensusAlgorithm::PoS => {
                    self.run_pos_consensus().await?;
                }
                crate::config::ConsensusAlgorithm::PoW => {
                    self.run_pow_consensus().await?;
                }
                crate::config::ConsensusAlgorithm::Hybrid => {
                    self.run_hybrid_consensus().await?;
                }
                crate::config::ConsensusAlgorithm::PoA => {
                    self.run_poa_consensus().await?;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    /// Initialize validator set
    async fn initialize_validators(&self) -> Result<()> {
        let mut validator_set = self.validator_set.write().await;

        // Add genesis validators (in real implementation, load from blockchain state)
        let genesis_validator = Validator {
            address: "genesis_validator".to_string(),
            stake: 1_000_000,
            reputation: 100.0,
            last_block_time: None,
            consecutive_misses: 0,
            total_blocks_proposed: 0,
            is_active: true,
        };

        validator_set
            .validators
            .insert(genesis_validator.address.clone(), genesis_validator);
        validator_set
            .active_validators
            .push("genesis_validator".to_string());

        tracing::info!(
            "Initialized validator set with {} validators",
            validator_set.validators.len()
        );
        Ok(())
    }

    /// Run Proof of Stake consensus
    async fn run_pos_consensus(&self) -> Result<()> {
        if !self.mining_enabled {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            return Ok(());
        }

        // Select validator based on stake
        let proposer = self.select_pos_validator().await?;

        // Create block proposal
        if let Some(proposer_address) = proposer {
            self.create_and_propose_block(&proposer_address).await?;
        }

        Ok(())
    }

    /// Run Proof of Work consensus
    async fn run_pow_consensus(&self) -> Result<()> {
        if !self.mining_enabled {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            return Ok(());
        }

        // Simplified PoW mining
        let blockchain = self.blockchain.read().await;
        let pending_transactions = blockchain.get_pending_transactions(1000).await;

        if !pending_transactions.is_empty() {
            self.mine_pow_block(pending_transactions).await?;
        }

        Ok(())
    }

    /// Run hybrid consensus (PoS + PoW for energy validation)
    async fn run_hybrid_consensus(&self) -> Result<()> {
        // Use PoS for regular transactions and PoW for energy validation
        let blockchain = self.blockchain.read().await;
        let pending_transactions = blockchain.get_pending_transactions(1000).await;

        // Separate energy transactions from regular transactions
        let (energy_txs, regular_txs): (Vec<_>, Vec<_>) = pending_transactions
            .into_iter()
            .partition(|tx| tx.is_energy_transaction());

        drop(blockchain);

        // Process regular transactions with PoS
        if !regular_txs.is_empty() {
            self.process_pos_transactions(regular_txs).await?;
        }

        // Process energy transactions with PoW validation
        if !energy_txs.is_empty() {
            self.process_pow_energy_transactions(energy_txs).await?;
        }

        Ok(())
    }

    /// Run Proof of Authority consensus
    async fn run_poa_consensus(&self) -> Result<()> {
        // Simplified PoA - authorities take turns proposing blocks
        let proposer = self.select_poa_validator().await?;

        if let Some(proposer_address) = proposer {
            self.create_and_propose_block(&proposer_address).await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(self.config.block_time)).await;
        Ok(())
    }

    /// Select validator for PoS based on stake
    async fn select_pos_validator(&self) -> Result<Option<String>> {
        let validator_set = self.validator_set.read().await;

        if validator_set.active_validators.is_empty() {
            return Ok(None);
        }

        // Simple random selection weighted by stake
        let total_stake: u64 = validator_set
            .validators
            .values()
            .filter(|v| v.is_active)
            .map(|v| v.stake)
            .sum();

        if total_stake == 0 {
            return Ok(None);
        }

        let random_stake = (rand::random::<f64>() * total_stake as f64) as u64;
        let mut cumulative_stake = 0;

        for validator in validator_set.validators.values() {
            if validator.is_active {
                cumulative_stake += validator.stake;
                if cumulative_stake >= random_stake {
                    return Ok(Some(validator.address.clone()));
                }
            }
        }

        Ok(validator_set.active_validators.first().cloned())
    }

    /// Select authority validator for PoA
    async fn select_poa_validator(&self) -> Result<Option<String>> {
        let validator_set = self.validator_set.read().await;
        let consensus_state = self.consensus_state.read().await;

        if validator_set.active_validators.is_empty() {
            return Ok(None);
        }

        // Round-robin selection
        let index =
            (consensus_state.current_round as usize) % validator_set.active_validators.len();
        Ok(Some(validator_set.active_validators[index].clone()))
    }

    /// Create and propose a new block
    async fn create_and_propose_block(&self, proposer: &str) -> Result<()> {
        let blockchain = self.blockchain.read().await;
        let pending_transactions = blockchain.get_pending_transactions(self.config.max_block_size);

        if pending_transactions.is_empty() {
            return Ok(());
        }

        let latest_block = blockchain.get_latest_block().await?;
        let height = latest_block.header.height + 1;

        let validator_info = ValidatorInfo {
            address: proposer.to_string(),
            stake: self.get_validator_stake(proposer).await,
            reputation: self.get_validator_reputation(proposer).await,
            authority_type: Some("VALIDATOR".to_string()),
        };

        drop(blockchain);

        let new_block = Block::new(
            latest_block.header.hash,
            pending_transactions.clone(),
            height,
            validator_info,
        )?;

        // Add block to blockchain
        let blockchain = self.blockchain.read().await;
        blockchain.add_block(new_block.clone()).await?;

        // Remove processed transactions from pending pool
        let tx_ids: Vec<String> = pending_transactions
            .iter()
            .map(|tx| tx.id.clone())
            .collect();
        blockchain.remove_pending_transactions(&tx_ids).await;

        // Update validator metrics
        self.update_validator_metrics(proposer).await?;

        // Update consensus state
        let mut consensus_state = self.consensus_state.write().await;
        consensus_state.current_height = height;
        consensus_state.current_round += 1;
        consensus_state.last_block_time = Some(new_block.header.timestamp);

        tracing::info!("Block {} proposed by validator {}", height, proposer);
        Ok(())
    }

    /// Mine a PoW block
    async fn mine_pow_block(&self, transactions: Vec<Transaction>) -> Result<()> {
        let blockchain = self.blockchain.read().await;
        let latest_block = blockchain.get_latest_block().await?;
        let height = latest_block.header.height + 1;

        let validator_info = ValidatorInfo {
            address: "pow_miner".to_string(),
            stake: 0,
            reputation: 50.0,
            authority_type: Some("MINER".to_string()),
        };

        drop(blockchain);

        let mut new_block = Block::new(
            latest_block.header.hash,
            transactions.clone(),
            height,
            validator_info,
        )?;

        // Simple PoW mining (find nonce that produces hash with leading zeros)
        let target_difficulty = self.get_current_difficulty().await;
        new_block.header.difficulty = target_difficulty;

        while !self.validate_pow_hash(&new_block.header.hash, target_difficulty) {
            new_block.header.nonce += 1;
            new_block.header.hash = new_block.calculate_hash()?;
        }

        // Add block to blockchain
        let blockchain = self.blockchain.read().await;
        blockchain.add_block(new_block.clone()).await?;

        // Remove processed transactions
        let tx_ids: Vec<String> = transactions.iter().map(|tx| tx.id.clone()).collect();
        blockchain.remove_pending_transactions(&tx_ids).await;

        tracing::info!(
            "PoW block {} mined with nonce {}",
            height,
            new_block.header.nonce
        );
        Ok(())
    }

    /// Process regular transactions with PoS
    async fn process_pos_transactions(&self, transactions: Vec<Transaction>) -> Result<()> {
        // Simplified - just create block with PoS validator
        if let Some(proposer) = self.select_pos_validator().await? {
            // Create block with these transactions
            tracing::info!(
                "Processing {} regular transactions with PoS",
                transactions.len()
            );
        }
        Ok(())
    }

    /// Process energy transactions with PoW validation
    async fn process_pow_energy_transactions(&self, transactions: Vec<Transaction>) -> Result<()> {
        // Energy transactions require PoW validation for grid stability
        tracing::info!(
            "Processing {} energy transactions with PoW",
            transactions.len()
        );
        self.mine_pow_block(transactions).await
    }

    /// Get validator stake
    async fn get_validator_stake(&self, address: &str) -> u64 {
        let validator_set = self.validator_set.read().await;
        validator_set
            .validators
            .get(address)
            .map(|v| v.stake)
            .unwrap_or(0)
    }

    /// Get validator reputation
    async fn get_validator_reputation(&self, address: &str) -> f64 {
        let validator_set = self.validator_set.read().await;
        validator_set
            .validators
            .get(address)
            .map(|v| v.reputation)
            .unwrap_or(50.0)
    }

    /// Update validator metrics after block proposal
    async fn update_validator_metrics(&self, address: &str) -> Result<()> {
        let mut validator_set = self.validator_set.write().await;

        if let Some(validator) = validator_set.validators.get_mut(address) {
            validator.total_blocks_proposed += 1;
            validator.last_block_time = Some(Utc::now());
            validator.consecutive_misses = 0;

            // Update reputation based on performance
            validator.reputation = (validator.reputation * 0.99 + 1.0).min(100.0);
        }

        Ok(())
    }

    /// Get current PoW difficulty
    async fn get_current_difficulty(&self) -> u64 {
        let consensus_state = self.consensus_state.read().await;
        consensus_state.difficulty.max(1000) // Minimum difficulty
    }

    /// Validate PoW hash meets difficulty requirement
    fn validate_pow_hash(&self, hash: &str, difficulty: u64) -> bool {
        if hash.len() < 8 {
            return false;
        }

        // Simple difficulty check - hash must start with certain number of zeros
        let leading_zeros = difficulty / 1000; // Scale difficulty
        let zero_prefix = "0".repeat(leading_zeros as usize);
        hash.starts_with(&zero_prefix)
    }

    /// Add new validator to the set
    pub async fn add_validator(&self, validator: Validator) -> Result<()> {
        let mut validator_set = self.validator_set.write().await;

        validator_set
            .validators
            .insert(validator.address.clone(), validator.clone());

        if validator.is_active {
            validator_set.active_validators.push(validator.address);
        }

        tracing::info!("Added new validator: {}", validator.address);
        Ok(())
    }

    /// Remove validator from the set
    pub async fn remove_validator(&self, address: &str) -> Result<()> {
        let mut validator_set = self.validator_set.write().await;

        validator_set.validators.remove(address);
        validator_set
            .active_validators
            .retain(|addr| addr != address);

        tracing::info!("Removed validator: {}", address);
        Ok(())
    }

    /// Get consensus metrics
    pub async fn get_metrics(&self) -> ConsensusMetrics {
        let validator_set = self.validator_set.read().await;
        let consensus_state = self.consensus_state.read().await;

        let total_stake = validator_set
            .validators
            .values()
            .filter(|v| v.is_active)
            .map(|v| v.stake)
            .sum();

        ConsensusMetrics {
            current_validators: validator_set.active_validators.len() as u64,
            total_stake,
            average_block_time: self.config.block_time as f64,
            last_finalized_height: consensus_state.current_height,
            missed_blocks: 0, // Would track actual missed blocks
            consensus_rounds: consensus_state.current_round,
        }
    }
}

