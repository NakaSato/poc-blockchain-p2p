//! GridTokenX Blockchain Chain Module
//!
//! This module implements the main blockchain data structure and operations
//! for managing the chain of blocks in the GridTokenX energy trading platform.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{
    Account, AccountType, Block, BlockchainStats, ComplianceStatus, Transaction, TransactionType,
    ValidationResult,
};
use crate::storage::StorageManager;

/// Main blockchain structure managing the chain of blocks
#[derive(Debug)]
pub struct Blockchain {
    /// Storage manager for persistent data
    storage: Arc<StorageManager>,
    /// In-memory cache of recent blocks
    block_cache: RwLock<VecDeque<Block>>,
    /// Account balances and information
    accounts: RwLock<HashMap<String, Account>>,
    /// Pending transactions pool
    pending_transactions: RwLock<Vec<Transaction>>,
    /// Blockchain statistics
    stats: RwLock<BlockchainStats>,
    /// Configuration parameters
    config: BlockchainConfig,
    /// UTXO set for efficient transaction validation
    utxo_set: RwLock<HashMap<String, UTXO>>,
    /// Energy trading order book
    energy_orders: RwLock<EnergyOrderBook>,
    /// Active governance proposals
    governance_proposals: RwLock<HashMap<String, GovernanceProposal>>,
}

/// Blockchain configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Maximum number of blocks to keep in cache
    pub max_cache_blocks: usize,
    /// Maximum pending transactions
    pub max_pending_transactions: usize,
    /// Block time target in seconds
    pub target_block_time: u64,
    /// Difficulty adjustment period
    pub difficulty_adjustment_period: u64,
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Energy token ratio (1 kWh = X tokens)
    pub energy_token_ratio: f64,
    /// Minimum validator stake
    pub min_validator_stake: u64,
    /// Thai market specific settings
    pub thai_market_config: ThaiMarketConfig,
}

/// Thai energy market specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiMarketConfig {
    /// Peak hours pricing multiplier
    pub peak_hours_multiplier: f64,
    /// Maximum energy trade per transaction (kWh)
    pub max_energy_per_transaction: f64,
    /// Minimum carbon credit ratio for renewables
    pub min_carbon_credit_ratio: f64,
    /// Grid stability requirements
    pub grid_stability_threshold: f64,
    /// Regulatory compliance requirements
    pub require_erc_approval: bool,
}

/// UTXO (Unspent Transaction Output) for efficient validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    /// Transaction ID
    pub tx_id: String,
    /// Output index
    pub output_index: u32,
    /// Amount in tokens
    pub amount: u64,
    /// Owner address
    pub owner: String,
    /// Block height when created
    pub block_height: u64,
    /// Whether this UTXO is from an energy transaction
    pub is_energy_utxo: bool,
    /// Energy metadata (if applicable)
    pub energy_metadata: Option<EnergyUTXOMetadata>,
}

/// Energy-specific UTXO metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyUTXOMetadata {
    /// Energy amount in kWh
    pub energy_amount: f64,
    /// Energy source type
    pub energy_source: String,
    /// Carbon credits associated
    pub carbon_credits: f64,
    /// Delivery timestamp
    pub delivery_time: DateTime<Utc>,
    /// Grid location
    pub grid_location: String,
}

/// Energy trading order book
#[derive(Debug, Clone, Default)]
pub struct EnergyOrderBook {
    /// Buy orders (price, amount, trader)
    pub buy_orders: Vec<EnergyOrder>,
    /// Sell orders (price, amount, trader)
    pub sell_orders: Vec<EnergyOrder>,
    /// Matched trades history
    pub matched_trades: Vec<MatchedTrade>,
}

/// Energy order in the order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyOrder {
    /// Order ID
    pub id: String,
    /// Trader address
    pub trader: String,
    /// Energy amount in kWh
    pub energy_amount: f64,
    /// Price per kWh in tokens
    pub price_per_kwh: u64,
    /// Order type (Buy/Sell)
    pub order_type: String,
    /// Energy source (for sell orders)
    pub energy_source: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Grid location
    pub grid_location: String,
    /// Minimum trade amount
    pub min_trade_amount: f64,
}

/// Matched energy trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedTrade {
    /// Trade ID
    pub id: String,
    /// Buy order ID
    pub buy_order_id: String,
    /// Sell order ID
    pub sell_order_id: String,
    /// Traded energy amount
    pub energy_amount: f64,
    /// Trade price per kWh
    pub price_per_kwh: u64,
    /// Total value
    pub total_value: u64,
    /// Match timestamp
    pub matched_at: DateTime<Utc>,
    /// Settlement status
    pub settlement_status: SettlementStatus,
}

/// Settlement status for energy trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementStatus {
    Pending,
    Confirmed,
    Delivered,
    Failed,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    /// Proposal ID
    pub id: String,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposer address
    pub proposer: String,
    /// Proposal type
    pub proposal_type: String,
    /// Voting start time
    pub voting_start: DateTime<Utc>,
    /// Voting end time
    pub voting_end: DateTime<Utc>,
    /// Current votes
    pub votes: HashMap<String, (String, u64)>, // voter -> (choice, voting_power)
    /// Execution data
    pub execution_data: Vec<u8>,
    /// Current status
    pub status: ProposalStatus,
}

/// Governance proposal status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            max_cache_blocks: 1000,
            max_pending_transactions: 10000,
            target_block_time: 10, // 10 seconds for energy trading responsiveness
            difficulty_adjustment_period: 144,
            max_block_size: 1_048_576,    // 1MB
            energy_token_ratio: 1.0,      // 1 kWh = 1 Token
            min_validator_stake: 100_000, // 100k tokens minimum stake
            thai_market_config: ThaiMarketConfig::default(),
        }
    }
}

impl Default for ThaiMarketConfig {
    fn default() -> Self {
        Self {
            peak_hours_multiplier: 1.5,
            max_energy_per_transaction: 10_000.0, // 10 MWh
            min_carbon_credit_ratio: 0.1,
            grid_stability_threshold: 0.95,
            require_erc_approval: true,
        }
    }
}

impl Blockchain {
    /// Create a new blockchain instance
    pub async fn new(storage: Arc<StorageManager>) -> Result<Self> {
        let config = BlockchainConfig::default();

        // Load existing blockchain state or initialize
        let stats = storage.load_blockchain_stats().await.unwrap_or_default();
        let accounts = storage.load_accounts().await.unwrap_or_default();

        Ok(Self {
            storage,
            block_cache: RwLock::new(VecDeque::with_capacity(config.max_cache_blocks)),
            accounts: RwLock::new(accounts),
            pending_transactions: RwLock::new(Vec::new()),
            stats: RwLock::new(stats),
            config,
            utxo_set: RwLock::new(HashMap::new()),
            energy_orders: RwLock::new(EnergyOrderBook::default()),
            governance_proposals: RwLock::new(HashMap::new()),
        })
    }

    /// Add genesis block to the blockchain
    pub async fn add_genesis_block(&mut self, genesis_block: Block) -> Result<()> {
        // Validate genesis block
        if genesis_block.header.height != 0 {
            return Err(anyhow!("Genesis block must have height 0"));
        }

        if !genesis_block.header.previous_hash.is_empty() {
            return Err(anyhow!("Genesis block cannot have previous hash"));
        }

        // Process genesis transactions
        self.process_genesis_transactions(&genesis_block.transactions)
            .await?;

        // Store genesis block
        self.storage.store_block(&genesis_block).await?;

        // Update cache
        let mut cache = self.block_cache.write().await;
        cache.push_back(genesis_block.clone());

        // Update stats
        let mut stats = self.stats.write().await;
        stats.height = 1;
        stats.total_transactions = genesis_block.transactions.len() as u64;
        stats.last_block_time = genesis_block.header.timestamp;

        tracing::info!("Genesis block added successfully");
        Ok(())
    }

    /// Add a new block to the blockchain
    pub async fn add_block(&self, block: Block) -> Result<()> {
        // Get the latest block for validation
        let latest_block = self.get_latest_block().await?;

        // Validate the new block
        let validation_result = block.validate(Some(&latest_block));
        if !validation_result.is_valid() {
            return Err(anyhow!("Block validation failed: {:?}", validation_result));
        }

        // Process block transactions
        self.process_block_transactions(&block).await?;

        // Store block
        self.storage.store_block(&block).await?;

        // Update cache
        let mut cache = self.block_cache.write().await;
        cache.push_back(block.clone());

        // Maintain cache size
        if cache.len() > self.config.max_cache_blocks {
            cache.pop_front();
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.height = block.header.height + 1;
        stats.total_transactions += block.transactions.len() as u64;
        stats.total_energy_traded += block.energy_stats.total_energy_traded;
        stats.last_block_time = block.header.timestamp;

        tracing::info!("Block {} added successfully", block.header.height);
        Ok(())
    }

    /// Get the latest block in the chain
    pub async fn get_latest_block(&self) -> Result<Block> {
        let cache = self.block_cache.read().await;

        if let Some(latest_block) = cache.back() {
            Ok(latest_block.clone())
        } else {
            // Load from storage
            let stats = self.stats.read().await;
            if stats.height > 0 {
                self.storage.load_block_by_height(stats.height - 1).await
            } else {
                Err(anyhow!("No blocks in blockchain"))
            }
        }
    }

    /// Get block by height
    pub async fn get_block_by_height(&self, height: u64) -> Result<Block> {
        // Check cache first
        let cache = self.block_cache.read().await;
        for block in cache.iter() {
            if block.header.height == height {
                return Ok(block.clone());
            }
        }

        // Load from storage
        self.storage.load_block_by_height(height).await
    }

    /// Get block by hash
    pub async fn get_block_by_hash(&self, hash: &str) -> Result<Block> {
        // Check cache first
        let cache = self.block_cache.read().await;
        for block in cache.iter() {
            if block.header.hash == hash {
                return Ok(block.clone());
            }
        }

        // Load from storage
        self.storage.load_block_by_hash(hash).await
    }

    /// Get current blockchain height
    pub async fn get_height(&self) -> Result<u64> {
        let stats = self.stats.read().await;
        Ok(stats.height)
    }

    /// Get total number of transactions
    pub async fn get_total_transactions(&self) -> Result<u64> {
        let stats = self.stats.read().await;
        Ok(stats.total_transactions)
    }

    /// Add transaction to pending pool
    pub async fn add_pending_transaction(&self, transaction: Transaction) -> Result<()> {
        // Validate transaction
        transaction.validate()?;

        // Check if transaction already exists
        {
            let pending = self.pending_transactions.read().await;
            if pending.iter().any(|tx| tx.id == transaction.id) {
                return Err(anyhow!("Transaction already in pending pool"));
            }
        }

        // Validate account balance for token transactions
        if let super::TransactionType::TokenTransfer { amount, .. } = &transaction.transaction_type
        {
            let accounts = self.accounts.read().await;
            if let Some(account) = accounts.get(&transaction.from) {
                if account.token_balance < *amount + transaction.fee {
                    return Err(anyhow!("Insufficient balance"));
                }
            } else {
                return Err(anyhow!("Sender account not found"));
            }
        }

        // Add to pending pool
        let mut pending = self.pending_transactions.write().await;
        if pending.len() >= self.config.max_pending_transactions {
            return Err(anyhow!("Pending transaction pool is full"));
        }

        pending.push(transaction);
        Ok(())
    }

    /// Get pending transactions for block creation
    pub async fn get_pending_transactions(&self, limit: usize) -> Vec<Transaction> {
        let pending = self.pending_transactions.read().await;
        pending.iter().take(limit).cloned().collect()
    }

    /// Remove transactions from pending pool (after inclusion in block)
    pub async fn remove_pending_transactions(&self, tx_ids: &[String]) {
        let mut pending = self.pending_transactions.write().await;
        pending.retain(|tx| !tx_ids.contains(&tx.id));
    }

    /// Get account information
    pub async fn get_account(&self, address: &str) -> Option<Account> {
        let accounts = self.accounts.read().await;
        accounts.get(address).cloned()
    }

    /// Get account balance
    pub async fn get_balance(&self, address: &str) -> u64 {
        let accounts = self.accounts.read().await;
        accounts
            .get(address)
            .map(|acc| acc.token_balance)
            .unwrap_or(0)
    }

    /// Process genesis block transactions
    async fn process_genesis_transactions(&self, transactions: &[Transaction]) -> Result<()> {
        let mut accounts = self.accounts.write().await;

        for tx in transactions {
            match &tx.transaction_type {
                super::TransactionType::GenesisMint {
                    amount,
                    description: _,
                } => {
                    if let Some(to) = &tx.to {
                        let account = accounts.entry(to.clone()).or_insert_with(|| Account {
                            address: to.clone(),
                            token_balance: 0,
                            energy_production_capacity: 0.0,
                            energy_consumption_demand: 0.0,
                            account_type: AccountType::Consumer,
                            carbon_credits: 0.0,
                            reputation_score: 50.0,
                            registered_at: Utc::now(),
                            last_activity: Utc::now(),
                            compliance_status: ComplianceStatus::Pending,
                        });
                        account.token_balance += amount;
                    }
                }
                super::TransactionType::AuthorityRegistration { authority_name, .. } => {
                    let account =
                        accounts
                            .entry(authority_name.clone())
                            .or_insert_with(|| Account {
                                address: authority_name.clone(),
                                token_balance: 0,
                                energy_production_capacity: 0.0,
                                energy_consumption_demand: 0.0,
                                account_type: AccountType::Authority,
                                carbon_credits: 0.0,
                                reputation_score: 100.0,
                                registered_at: Utc::now(),
                                last_activity: Utc::now(),
                                compliance_status: ComplianceStatus::Compliant,
                            });
                    account.account_type = AccountType::Authority;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Process block transactions and update state
    async fn process_block_transactions(&self, block: &Block) -> Result<()> {
        let mut accounts = self.accounts.write().await;
        let mut utxo_set = self.utxo_set.write().await;
        let mut energy_orders = self.energy_orders.write().await;

        for tx in &block.transactions {
            match &tx.transaction_type {
                super::TransactionType::TokenTransfer { amount, .. } => {
                    // Update sender balance
                    if let Some(sender) = accounts.get_mut(&tx.from) {
                        sender.token_balance -= amount + tx.fee;
                        sender.last_activity = tx.timestamp;
                    }

                    // Update receiver balance
                    if let Some(to) = &tx.to {
                        let receiver = accounts.entry(to.clone()).or_insert_with(|| Account {
                            address: to.clone(),
                            token_balance: 0,
                            energy_production_capacity: 0.0,
                            energy_consumption_demand: 0.0,
                            account_type: AccountType::Consumer,
                            carbon_credits: 0.0,
                            reputation_score: 50.0,
                            registered_at: Utc::now(),
                            last_activity: Utc::now(),
                            compliance_status: ComplianceStatus::Pending,
                        });
                        receiver.token_balance += amount;
                        receiver.last_activity = tx.timestamp;
                    }
                }
                TransactionType::EnergyTrade(energy_tx) => {
                    self.process_energy_transaction(
                        tx,
                        energy_tx,
                        &mut accounts,
                        &mut energy_orders,
                    )
                    .await?;
                }
                TransactionType::Governance(gov_tx) => {
                    self.process_governance_transaction(tx, gov_tx).await?;
                }
                _ => {}
            }

            // Create UTXO for transaction output
            if let Some(to) = &tx.to {
                let utxo = UTXO {
                    tx_id: tx.id.clone(),
                    output_index: 0,
                    amount: match &tx.transaction_type {
                        TransactionType::TokenTransfer { amount, .. } => *amount,
                        TransactionType::EnergyTrade(energy_tx) => energy_tx.total_value,
                        _ => 0,
                    },
                    owner: to.clone(),
                    block_height: block.header.height,
                    is_energy_utxo: tx.is_energy_transaction(),
                    energy_metadata: if tx.is_energy_transaction() {
                        Some(EnergyUTXOMetadata {
                            energy_amount: 0.0, // Would be filled from energy_tx
                            energy_source: "unknown".to_string(),
                            carbon_credits: tx.get_carbon_impact(),
                            delivery_time: tx.timestamp,
                            grid_location: "unknown".to_string(),
                        })
                    } else {
                        None
                    },
                };
                utxo_set.insert(format!("{}:0", tx.id), utxo);
            }
        }

        Ok(())
    }

    /// Process energy trading transaction
    async fn process_energy_transaction(
        &self,
        tx: &Transaction,
        energy_tx: &super::transaction::EnergyTransaction,
        accounts: &mut HashMap<String, Account>,
        energy_orders: &mut EnergyOrderBook,
    ) -> Result<()> {
        // Update energy trading balances
        if let Some(sender) = accounts.get_mut(&tx.from) {
            sender.token_balance -= energy_tx.total_value + tx.fee;
            sender.carbon_credits += energy_tx.carbon_credits;
            sender.last_activity = tx.timestamp;
        }

        if let Some(to) = &tx.to {
            let receiver = accounts.entry(to.clone()).or_insert_with(|| Account {
                address: to.clone(),
                token_balance: 0,
                energy_production_capacity: 0.0,
                energy_consumption_demand: 0.0,
                account_type: AccountType::Consumer,
                carbon_credits: 0.0,
                reputation_score: 50.0,
                registered_at: Utc::now(),
                last_activity: Utc::now(),
                compliance_status: ComplianceStatus::Pending,
            });
            receiver.token_balance += energy_tx.total_value;
            receiver.last_activity = tx.timestamp;
        }

        // Process matched trades
        if let super::transaction::EnergyOrderType::Match {
            buy_order_id,
            sell_order_id,
        } = &energy_tx.order_type
        {
            let matched_trade = MatchedTrade {
                id: tx.id.clone(),
                buy_order_id: buy_order_id.clone(),
                sell_order_id: sell_order_id.clone(),
                energy_amount: energy_tx.energy_amount,
                price_per_kwh: energy_tx.price_per_kwh,
                total_value: energy_tx.total_value,
                matched_at: tx.timestamp,
                settlement_status: SettlementStatus::Pending,
            };
            energy_orders.matched_trades.push(matched_trade);

            // Remove matched orders
            energy_orders
                .buy_orders
                .retain(|order| order.id != *buy_order_id);
            energy_orders
                .sell_orders
                .retain(|order| order.id != *sell_order_id);
        }

        Ok(())
    }

    /// Process governance transaction
    async fn process_governance_transaction(
        &self,
        tx: &Transaction,
        gov_tx: &super::transaction::GovernanceTransaction,
    ) -> Result<()> {
        let mut proposals = self.governance_proposals.write().await;

        match gov_tx {
            super::transaction::GovernanceTransaction::ProposalSubmission {
                title,
                description,
                proposal_type,
                voting_period_days,
                execution_delay_days: _,
            } => {
                let proposal = GovernanceProposal {
                    id: tx.id.clone(),
                    title: title.clone(),
                    description: description.clone(),
                    proposer: tx.from.clone(),
                    proposal_type: format!("{:?}", proposal_type),
                    voting_start: tx.timestamp,
                    voting_end: tx.timestamp + chrono::Duration::days(*voting_period_days as i64),
                    votes: HashMap::new(),
                    execution_data: Vec::new(),
                    status: ProposalStatus::Active,
                };
                proposals.insert(tx.id.clone(), proposal);
            }
            super::transaction::GovernanceTransaction::Vote {
                proposal_id,
                vote,
                voting_power,
                reason: _,
            } => {
                if let Some(proposal) = proposals.get_mut(proposal_id) {
                    proposal
                        .votes
                        .insert(tx.from.clone(), (format!("{:?}", vote), *voting_power));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Get blockchain statistics
    pub async fn get_stats(&self) -> BlockchainStats {
        self.stats.read().await.clone()
    }

    /// Validate the entire blockchain
    pub async fn validate_chain(&self) -> Result<ValidationResult> {
        let height = self.get_height().await?;

        if height == 0 {
            return Ok(ValidationResult::Valid);
        }

        let mut previous_block: Option<Block> = None;

        for h in 0..height {
            let block = self.get_block_by_height(h).await?;

            let validation_result = block.validate(previous_block.as_ref());
            if !validation_result.is_valid() {
                return Ok(ValidationResult::Invalid(format!(
                    "Block {} validation failed: {:?}",
                    h, validation_result
                )));
            }

            previous_block = Some(block);
        }

        Ok(ValidationResult::Valid)
    }

    /// Get energy trading statistics
    pub async fn get_energy_stats(&self) -> Result<EnergyTradingStats> {
        let stats = self.stats.read().await;
        let energy_orders = self.energy_orders.read().await;

        Ok(EnergyTradingStats {
            total_energy_traded: stats.total_energy_traded,
            active_buy_orders: energy_orders.buy_orders.len() as u64,
            active_sell_orders: energy_orders.sell_orders.len() as u64,
            completed_trades: energy_orders.matched_trades.len() as u64,
            average_price: if stats.total_energy_traded > 0.0 {
                // This would be calculated from actual trade data
                4000.0 // Placeholder
            } else {
                0.0
            },
        })
    }
}

/// Energy trading statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradingStats {
    pub total_energy_traded: f64,
    pub active_buy_orders: u64,
    pub active_sell_orders: u64,
    pub completed_trades: u64,
    pub average_price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::StorageManager;

    #[tokio::test]
    async fn test_blockchain_creation() {
        let storage = Arc::new(StorageManager::new_memory());
        let blockchain = Blockchain::new(storage).await.unwrap();

        let height = blockchain.get_height().await.unwrap();
        assert_eq!(height, 0);
    }

    #[tokio::test]
    async fn test_genesis_block_addition() {
        let storage = Arc::new(StorageManager::new_memory());
        let mut blockchain = Blockchain::new(storage).await.unwrap();

        let transactions = vec![Transaction::new_genesis_mint(
            "genesis_recipient".to_string(),
            1_000_000,
            "Genesis mint".to_string(),
        )
        .unwrap()];

        let genesis_block = Block::new_genesis(transactions, "Test Genesis".to_string()).unwrap();

        blockchain.add_genesis_block(genesis_block).await.unwrap();

        let height = blockchain.get_height().await.unwrap();
        assert_eq!(height, 1);

        let balance = blockchain.get_balance("genesis_recipient").await;
        assert_eq!(balance, 1_000_000);
    }

    #[tokio::test]
    async fn test_pending_transactions() {
        let storage = Arc::new(StorageManager::new_memory());
        let blockchain = Blockchain::new(storage).await.unwrap();

        let tx = Transaction::new(
            TransactionType::TokenTransfer {
                amount: 1000,
                message: Some("Test".to_string()),
            },
            "sender".to_string(),
            Some("receiver".to_string()),
            10,
            1,
        )
        .unwrap();

        // This would fail due to insufficient balance, but tests the validation
        let result = blockchain.add_pending_transaction(tx).await;
        assert!(result.is_err());
    }
}
