//! GridTokenX Storage Module
//!
//! This module provides persistent storage capabilities for the GridTokenX blockchain,
//! using Sled (pure Rust embedded database) for fast compilation and excellent performance.

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blockchain::{Account, Block, BlockchainStats, Transaction};

/// Storage manager that handles all persistent data operations
#[derive(Debug)]
pub struct StorageManager {
    /// Storage backend type
    backend: StorageBackend,
    /// Sled database instance (if using Sled)
    sled_db: Option<Arc<sled::Db>>,
    /// In-memory storage (for testing and development)
    memory_storage: Arc<RwLock<MemoryStorage>>,
}

/// Storage backend implementations
#[derive(Debug, Clone)]
pub enum StorageBackend {
    Sled(String), // path
    Memory,       // in-memory only
}

/// In-memory storage backend for testing and development
#[derive(Debug, Default)]
pub struct MemoryStorage {
    blocks: HashMap<String, Block>,
    transactions: HashMap<String, Transaction>,
    accounts: HashMap<String, Account>,
    stats: Option<BlockchainStats>,
    height: u64,
}

impl StorageManager {
    /// Create a new storage manager with the specified data path
    pub async fn new(data_path: &str) -> Result<Self> {
        if data_path == ":memory:" {
            return Ok(Self::new_memory());
        }

        let backend = StorageBackend::Sled(data_path.to_string());

        // Ensure the directory exists
        if let Some(parent) = Path::new(data_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Open Sled database
        let db = sled::open(data_path)
            .map_err(|e| anyhow!("Failed to open Sled database: {}", e))?;

        Ok(StorageManager {
            backend,
            sled_db: Some(Arc::new(db)),
            memory_storage: Arc::new(RwLock::new(MemoryStorage::default())),
        })
    }

    /// Create a new in-memory storage manager (for testing)
    pub fn new_memory() -> Self {
        StorageManager {
            backend: StorageBackend::Memory,
            sled_db: None,
            memory_storage: Arc::new(RwLock::new(MemoryStorage::default())),
        }
    }

    /// Store a block in the database
    pub async fn store_block(&self, block: &Block) -> Result<()> {
        let key = format!("block:{}", block.header.hash);
        
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let serialized = bincode::serialize(block)
                        .map_err(|e| anyhow!("Failed to serialize block: {}", e))?;
                    
                    // Store block
                    db.insert(&key, serialized)
                        .map_err(|e| anyhow!("Failed to store block: {}", e))?;
                    
                    // Update height index
                    let height_key = format!("height:{}", block.header.height);
                    db.insert(&height_key, block.header.hash.as_bytes())
                        .map_err(|e| anyhow!("Failed to store height index: {}", e))?;
                    
                    // Update latest block
                    db.insert("latest_block", block.header.hash.as_bytes())
                        .map_err(|e| anyhow!("Failed to update latest block: {}", e))?;
                    
                    db.flush().map_err(|e| anyhow!("Failed to flush: {}", e))?;
                    Ok(())
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.blocks.insert(block.header.hash.clone(), block.clone());
                storage.height = storage.height.max(block.header.height);
                Ok(())
            }
        }
    }

    /// Get a block by hash
    pub async fn get_block(&self, hash: &str) -> Result<Option<Block>> {
        let key = format!("block:{}", hash);
        
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    if let Some(data) = db.get(&key)
                        .map_err(|e| anyhow!("Failed to get block: {}", e))? {
                        let block: Block = bincode::deserialize(&data)
                            .map_err(|e| anyhow!("Failed to deserialize block: {}", e))?;
                        Ok(Some(block))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.blocks.get(hash).cloned())
            }
        }
    }

    /// Get a block by height
    pub async fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let height_key = format!("height:{}", height);
                    if let Some(hash_data) = db.get(&height_key)
                        .map_err(|e| anyhow!("Failed to get height index: {}", e))? {
                        let hash = String::from_utf8(hash_data.to_vec())
                            .map_err(|e| anyhow!("Failed to parse hash: {}", e))?;
                        self.get_block(&hash).await
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.blocks.values()
                    .find(|block| block.header.height == height)
                    .cloned())
            }
        }
    }

    /// Store a transaction
    pub async fn store_transaction(&self, transaction: &Transaction) -> Result<()> {
        let key = format!("tx:{}", transaction.id);
        
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let serialized = bincode::serialize(transaction)
                        .map_err(|e| anyhow!("Failed to serialize transaction: {}", e))?;
                    
                    db.insert(&key, serialized)
                        .map_err(|e| anyhow!("Failed to store transaction: {}", e))?;
                    
                    db.flush().map_err(|e| anyhow!("Failed to flush: {}", e))?;
                    Ok(())
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.transactions.insert(transaction.id.clone(), transaction.clone());
                Ok(())
            }
        }
    }

    /// Get a transaction by ID
    pub async fn get_transaction(&self, tx_id: &str) -> Result<Option<Transaction>> {
        let key = format!("tx:{}", tx_id);
        
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    if let Some(data) = db.get(&key)
                        .map_err(|e| anyhow!("Failed to get transaction: {}", e))? {
                        let transaction: Transaction = bincode::deserialize(&data)
                            .map_err(|e| anyhow!("Failed to deserialize transaction: {}", e))?;
                        Ok(Some(transaction))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.transactions.get(tx_id).cloned())
            }
        }
    }

    /// Store account data
    pub async fn store_account(&self, account: &Account) -> Result<()> {
        let key = format!("account:{}", account.address);
        
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let serialized = bincode::serialize(account)
                        .map_err(|e| anyhow!("Failed to serialize account: {}", e))?;
                    
                    db.insert(&key, serialized)
                        .map_err(|e| anyhow!("Failed to store account: {}", e))?;
                    
                    db.flush().map_err(|e| anyhow!("Failed to flush: {}", e))?;
                    Ok(())
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.accounts.insert(account.address.clone(), account.clone());
                Ok(())
            }
        }
    }

    /// Get account data
    pub async fn get_account(&self, address: &str) -> Result<Option<Account>> {
        let key = format!("account:{}", address);
        
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    if let Some(data) = db.get(&key)
                        .map_err(|e| anyhow!("Failed to get account: {}", e))? {
                        let account: Account = bincode::deserialize(&data)
                            .map_err(|e| anyhow!("Failed to deserialize account: {}", e))?;
                        Ok(Some(account))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.accounts.get(address).cloned())
            }
        }
    }

    /// Get all accounts
    pub async fn get_all_accounts(&self) -> Result<Vec<Account>> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let mut accounts = Vec::new();
                    let prefix = "account:";
                    
                    for item in db.scan_prefix(prefix) {
                        let (_, value) = item.map_err(|e| anyhow!("Failed to scan accounts: {}", e))?;
                        let account: Account = bincode::deserialize(&value)
                            .map_err(|e| anyhow!("Failed to deserialize account: {}", e))?;
                        accounts.push(account);
                    }
                    
                    Ok(accounts)
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.accounts.values().cloned().collect())
            }
        }
    }

    /// Store blockchain statistics
    pub async fn store_stats(&self, stats: &BlockchainStats) -> Result<()> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let serialized = bincode::serialize(stats)
                        .map_err(|e| anyhow!("Failed to serialize stats: {}", e))?;
                    
                    db.insert("stats", serialized)
                        .map_err(|e| anyhow!("Failed to store stats: {}", e))?;
                    
                    db.flush().map_err(|e| anyhow!("Failed to flush: {}", e))?;
                    Ok(())
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.stats = Some(stats.clone());
                Ok(())
            }
        }
    }

    /// Get blockchain statistics
    pub async fn get_stats(&self) -> Result<Option<BlockchainStats>> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    if let Some(data) = db.get("stats")
                        .map_err(|e| anyhow!("Failed to get stats: {}", e))? {
                        let stats: BlockchainStats = bincode::deserialize(&data)
                            .map_err(|e| anyhow!("Failed to deserialize stats: {}", e))?;
                        Ok(Some(stats))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.stats.clone())
            }
        }
    }

    /// Get the current blockchain height
    pub async fn get_height(&self) -> Result<u64> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let mut max_height = 0u64;
                    let prefix = "height:";
                    
                    for item in db.scan_prefix(prefix) {
                        let (key, _) = item.map_err(|e| anyhow!("Failed to scan heights: {}", e))?;
                        let key_str = String::from_utf8(key.to_vec())
                            .map_err(|e| anyhow!("Failed to parse key: {}", e))?;
                        
                        if let Some(height_str) = key_str.strip_prefix("height:") {
                            if let Ok(height) = height_str.parse::<u64>() {
                                max_height = max_height.max(height);
                            }
                        }
                    }
                    
                    Ok(max_height)
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.height)
            }
        }
    }

    /// Get the latest block
    pub async fn get_latest_block(&self) -> Result<Option<Block>> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    if let Some(hash_data) = db.get("latest_block")
                        .map_err(|e| anyhow!("Failed to get latest block hash: {}", e))? {
                        let hash = String::from_utf8(hash_data.to_vec())
                            .map_err(|e| anyhow!("Failed to parse hash: {}", e))?;
                        self.get_block(&hash).await
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                let latest = storage.blocks.values()
                    .max_by_key(|block| block.header.height);
                Ok(latest.cloned())
            }
        }
    }

    /// Clear all data (for testing)
    pub async fn clear(&self) -> Result<()> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    db.clear().map_err(|e| anyhow!("Failed to clear database: {}", e))?;
                    db.flush().map_err(|e| anyhow!("Failed to flush: {}", e))?;
                    Ok(())
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.blocks.clear();
                storage.transactions.clear();
                storage.accounts.clear();
                storage.stats = None;
                storage.height = 0;
                Ok(())
            }
        }
    }

    /// Get pending transactions (simplified for the new storage)
    pub async fn get_pending_transactions(&self, limit: usize) -> Vec<Transaction> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                // For now, return empty Vec - would need a separate pending tx storage
                Vec::new()
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage.transactions.values().take(limit).cloned().collect()
            }
        }
    }

    /// Remove pending transactions (simplified)
    pub async fn remove_pending_transactions(&self, _tx_ids: &[String]) {
        // For now, this is a no-op since we simplified the storage
        // In a full implementation, you'd have a separate pending transactions store
    }

    /// Get total transactions count
    pub async fn get_total_transactions(&self) -> Result<u64> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(db) = &self.sled_db {
                    let count = db.scan_prefix("tx:").count();
                    Ok(count as u64)
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.transactions.len() as u64)
            }
        }
    }

    /// Get block by hash
    pub async fn get_block_by_hash(&self, hash: &str) -> Result<Option<Block>> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(ref db) = self.sled_db {
                    let key = format!("block:{}", hash);
                    if let Some(data) = db.get(key)? {
                        let block: Block = bincode::deserialize(&data)?;
                        Ok(Some(block))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.blocks.get(hash).cloned())
            }
        }
    }

    /// Get blockchain statistics
    pub async fn get_blockchain_stats(&self) -> Result<Option<BlockchainStats>> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(ref db) = self.sled_db {
                    let key = "blockchain_stats";
                    if let Some(data) = db.get(key)? {
                        let stats: BlockchainStats = bincode::deserialize(&data)?;
                        Ok(Some(stats))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.stats.clone())
            }
        }
    }

    /// Store blockchain statistics
    pub async fn store_blockchain_stats(&self, stats: &BlockchainStats) -> Result<()> {
        match &self.backend {
            StorageBackend::Sled(_) => {
                if let Some(ref db) = self.sled_db {
                    let key = "blockchain_stats";
                    let value = bincode::serialize(stats)?;
                    db.insert(key, value)?;
                    Ok(())
                } else {
                    Err(anyhow!("Sled database not initialized"))
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.stats = Some(stats.clone());
                Ok(())
            }
        }
    }
    pub async fn load_block_by_height(&self, height: u64) -> Result<Block> {
        self.get_block_by_height(height).await?
            .ok_or_else(|| anyhow!("Block not found at height {}", height))
    }

    /// Load block by hash (wrapper for compatibility)
    pub async fn load_block_by_hash(&self, hash: &str) -> Result<Block> {
        self.get_block_by_hash(hash).await?
            .ok_or_else(|| anyhow!("Block not found with hash {}", hash))
    }

    /// Load blockchain statistics (wrapper for compatibility)
    pub async fn load_blockchain_stats(&self) -> Result<BlockchainStats> {
        self.get_blockchain_stats().await?
            .ok_or_else(|| anyhow!("Blockchain stats not found"))
    }

    /// Load all accounts (returns hashmap for compatibility)
    pub async fn load_accounts(&self) -> Result<HashMap<String, Account>> {
        let accounts_list = self.get_all_accounts().await?;
        let mut accounts_map = HashMap::new();
        
        for account in accounts_list {
            accounts_map.insert(account.address.clone(), account);
        }
        
        Ok(accounts_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::{Block, BlockHeader, BlockEnergyStats, ValidatorInfo, GridStabilityMetrics};

    #[tokio::test]
    async fn test_memory_storage() -> Result<()> {
        let storage = StorageManager::new_memory();

        // Test storing and retrieving a block
        let validator = ValidatorInfo {
            address: "test_validator".to_string(),
            stake: 1000,
            reputation: 100.0,
            authority_type: Some("TEST".to_string()),
        };

        let header = BlockHeader {
            version: 1,
            previous_hash: "genesis".to_string(),
            merkle_root: "".to_string(),
            timestamp: chrono::Utc::now(),
            difficulty: 1000,
            nonce: 0,
            height: 1,
            hash: "test_hash".to_string(),
            validator: validator.clone(),
            gas_used: 0,
            gas_limit: 1000000,
            extra_data: vec![],
        };

        let block = Block {
            header,
            transactions: vec![],
            size: 1024,
            energy_stats: BlockEnergyStats {
                total_energy_traded: 1000.0,
                energy_transaction_count: 10,
                average_energy_price: 3.5,
                peak_demand: 500.0,
                renewable_percentage: 75.0,
                carbon_credits_generated: 100.0,
                grid_stability: GridStabilityMetrics {
                    frequency_deviation: 0.1,
                    voltage_stability: 95,
                    load_balance_efficiency: 90,
                    congestion_level: 5,
                },
                energy_sources: std::collections::HashMap::new(),
            },
            governance_actions: vec![],
        };

        storage.store_block(&block).await?;
        let retrieved = storage.get_block("test_hash").await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().header.hash, "test_hash");

        Ok(())
    }
}
