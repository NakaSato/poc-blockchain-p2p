//! GridTokenX Storage Module
//!
//! This module provides persistent storage capabilities for the GridTokenX blockchain,
//! supporting RocksDB and in-memory storage backends.

use anyhow::{anyhow, Result};
use rocksdb::{Options, WriteBatch, DB};
use serde::{Deserialize, Serialize};
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
    /// RocksDB instance (if using RocksDB)
    rocksdb: Option<Arc<DB>>,
    /// In-memory storage (for testing and development)
    memory_storage: Arc<RwLock<MemoryStorage>>,
}

/// Storage backend implementations
#[derive(Debug, Clone)]
pub enum StorageBackend {
    RocksDB(String), // path
    Memory,
}

/// In-memory storage for testing and development
#[derive(Debug, Default)]
pub struct MemoryStorage {
    /// Blocks stored by height
    blocks_by_height: HashMap<u64, Block>,
    /// Blocks stored by hash
    blocks_by_hash: HashMap<String, Block>,
    /// Transactions stored by ID
    transactions: HashMap<String, Transaction>,
    /// Account information
    accounts: HashMap<String, Account>,
    /// Blockchain statistics
    stats: Option<BlockchainStats>,
    /// Key-value storage for arbitrary data
    kv_store: HashMap<String, Vec<u8>>,
}

/// Storage keys for different data types
pub struct StorageKeys;

impl StorageKeys {
    pub const BLOCK_HEIGHT_PREFIX: &'static str = "block:height:";
    pub const BLOCK_HASH_PREFIX: &'static str = "block:hash:";
    pub const TRANSACTION_PREFIX: &'static str = "tx:";
    pub const ACCOUNT_PREFIX: &'static str = "account:";
    pub const STATS_KEY: &'static str = "blockchain:stats";
    pub const LATEST_BLOCK_HEIGHT: &'static str = "blockchain:latest_height";
    pub const GENESIS_HASH: &'static str = "blockchain:genesis_hash";

    // Energy trading specific keys
    pub const ENERGY_ORDER_PREFIX: &'static str = "energy:order:";
    pub const ENERGY_TRADE_PREFIX: &'static str = "energy:trade:";
    pub const CARBON_CREDITS_PREFIX: &'static str = "carbon:";

    // Governance keys
    pub const PROPOSAL_PREFIX: &'static str = "gov:proposal:";
    pub const VOTE_PREFIX: &'static str = "gov:vote:";

    // UTXO keys
    pub const UTXO_PREFIX: &'static str = "utxo:";
    pub const SPENT_UTXO_PREFIX: &'static str = "spent_utxo:";
}

impl StorageManager {
    /// Create a new storage manager with RocksDB backend
    pub async fn new(data_path: &str) -> Result<Self> {
        let backend = StorageBackend::RocksDB(data_path.to_string());

        // Create data directory if it doesn't exist
        if !Path::new(data_path).exists() {
            std::fs::create_dir_all(data_path)
                .map_err(|e| anyhow!("Failed to create data directory: {}", e))?;
        }

        // Initialize RocksDB
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        opts.set_max_open_files(1000);
        opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        opts.set_target_file_size_base(64 * 1024 * 1024); // 64MB

        let db =
            DB::open(&opts, data_path).map_err(|e| anyhow!("Failed to open RocksDB: {}", e))?;

        Ok(Self {
            backend,
            rocksdb: Some(Arc::new(db)),
            memory_storage: Arc::new(RwLock::new(MemoryStorage::default())),
        })
    }

    /// Create a new storage manager with in-memory backend (for testing)
    pub async fn new_memory() -> Result<Self> {
        Ok(Self {
            backend: StorageBackend::Memory,
            rocksdb: None,
            memory_storage: Arc::new(RwLock::new(MemoryStorage::default())),
        })
    }

    /// Store a block
    pub async fn store_block(&self, block: &Block) -> Result<()> {
        let serialized =
            bincode::serialize(block).map_err(|e| anyhow!("Failed to serialize block: {}", e))?;

        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let mut batch = WriteBatch::default();

                    // Store by height
                    let height_key = format!(
                        "{}{}",
                        StorageKeys::BLOCK_HEIGHT_PREFIX,
                        block.header.height
                    );
                    batch.put(&height_key, &serialized);

                    // Store by hash
                    let hash_key =
                        format!("{}{}", StorageKeys::BLOCK_HASH_PREFIX, block.header.hash);
                    batch.put(&hash_key, &serialized);

                    // Update latest height
                    let height_bytes = block.header.height.to_be_bytes();
                    batch.put(StorageKeys::LATEST_BLOCK_HEIGHT, &height_bytes);

                    db.write(batch)
                        .map_err(|e| anyhow!("Failed to write block to RocksDB: {}", e))?;
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage
                    .blocks_by_height
                    .insert(block.header.height, block.clone());
                storage
                    .blocks_by_hash
                    .insert(block.header.hash.clone(), block.clone());
            }
        }

        Ok(())
    }

    /// Load a block by height
    pub async fn load_block_by_height(&self, height: u64) -> Result<Block> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let key = format!("{}{}", StorageKeys::BLOCK_HEIGHT_PREFIX, height);
                    let data = db
                        .get(&key)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Block not found at height {}", height))?;

                    let block: Block = bincode::deserialize(&data)
                        .map_err(|e| anyhow!("Failed to deserialize block: {}", e))?;

                    Ok(block)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage
                    .blocks_by_height
                    .get(&height)
                    .cloned()
                    .ok_or_else(|| anyhow!("Block not found at height {}", height))
            }
        }
    }

    /// Load a block by hash
    pub async fn load_block_by_hash(&self, hash: &str) -> Result<Block> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let key = format!("{}{}", StorageKeys::BLOCK_HASH_PREFIX, hash);
                    let data = db
                        .get(&key)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Block not found with hash {}", hash))?;

                    let block: Block = bincode::deserialize(&data)
                        .map_err(|e| anyhow!("Failed to deserialize block: {}", e))?;

                    Ok(block)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage
                    .blocks_by_hash
                    .get(hash)
                    .cloned()
                    .ok_or_else(|| anyhow!("Block not found with hash {}", hash))
            }
        }
    }

    /// Store a transaction
    pub async fn store_transaction(&self, transaction: &Transaction) -> Result<()> {
        let serialized = bincode::serialize(transaction)
            .map_err(|e| anyhow!("Failed to serialize transaction: {}", e))?;

        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let key = format!("{}{}", StorageKeys::TRANSACTION_PREFIX, transaction.id);
                    db.put(&key, &serialized)
                        .map_err(|e| anyhow!("Failed to write transaction to RocksDB: {}", e))?;
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage
                    .transactions
                    .insert(transaction.id.clone(), transaction.clone());
            }
        }

        Ok(())
    }

    /// Load a transaction by ID
    pub async fn load_transaction(&self, tx_id: &str) -> Result<Transaction> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let key = format!("{}{}", StorageKeys::TRANSACTION_PREFIX, tx_id);
                    let data = db
                        .get(&key)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Transaction not found: {}", tx_id))?;

                    let transaction: Transaction = bincode::deserialize(&data)
                        .map_err(|e| anyhow!("Failed to deserialize transaction: {}", e))?;

                    Ok(transaction)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage
                    .transactions
                    .get(tx_id)
                    .cloned()
                    .ok_or_else(|| anyhow!("Transaction not found: {}", tx_id))
            }
        }
    }

    /// Store account information
    pub async fn store_account(&self, account: &Account) -> Result<()> {
        let serialized = bincode::serialize(account)
            .map_err(|e| anyhow!("Failed to serialize account: {}", e))?;

        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let key = format!("{}{}", StorageKeys::ACCOUNT_PREFIX, account.address);
                    db.put(&key, &serialized)
                        .map_err(|e| anyhow!("Failed to write account to RocksDB: {}", e))?;
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage
                    .accounts
                    .insert(account.address.clone(), account.clone());
            }
        }

        Ok(())
    }

    /// Load account information
    pub async fn load_account(&self, address: &str) -> Result<Account> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let key = format!("{}{}", StorageKeys::ACCOUNT_PREFIX, address);
                    let data = db
                        .get(&key)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Account not found: {}", address))?;

                    let account: Account = bincode::deserialize(&data)
                        .map_err(|e| anyhow!("Failed to deserialize account: {}", e))?;

                    Ok(account)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage
                    .accounts
                    .get(address)
                    .cloned()
                    .ok_or_else(|| anyhow!("Account not found: {}", address))
            }
        }
    }

    /// Load all accounts
    pub async fn load_accounts(&self) -> Result<HashMap<String, Account>> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let mut accounts = HashMap::new();
                    let iter = db.prefix_iterator(StorageKeys::ACCOUNT_PREFIX);

                    for item in iter {
                        let (key, value) =
                            item.map_err(|e| anyhow!("RocksDB iteration error: {}", e))?;

                        let account: Account = bincode::deserialize(&value)
                            .map_err(|e| anyhow!("Failed to deserialize account: {}", e))?;

                        let address =
                            String::from_utf8_lossy(&key[StorageKeys::ACCOUNT_PREFIX.len()..])
                                .to_string();
                        accounts.insert(address, account);
                    }

                    Ok(accounts)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.accounts.clone())
            }
        }
    }

    /// Store blockchain statistics
    pub async fn store_blockchain_stats(&self, stats: &BlockchainStats) -> Result<()> {
        let serialized =
            bincode::serialize(stats).map_err(|e| anyhow!("Failed to serialize stats: {}", e))?;

        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    db.put(StorageKeys::STATS_KEY, &serialized)
                        .map_err(|e| anyhow!("Failed to write stats to RocksDB: {}", e))?;
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.stats = Some(stats.clone());
            }
        }

        Ok(())
    }

    /// Load blockchain statistics
    pub async fn load_blockchain_stats(&self) -> Result<BlockchainStats> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let data = db
                        .get(StorageKeys::STATS_KEY)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Blockchain stats not found"))?;

                    let stats: BlockchainStats = bincode::deserialize(&data)
                        .map_err(|e| anyhow!("Failed to deserialize stats: {}", e))?;

                    Ok(stats)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage
                    .stats
                    .clone()
                    .ok_or_else(|| anyhow!("Blockchain stats not found"))
            }
        }
    }

    /// Store arbitrary key-value data
    pub async fn store_kv(&self, key: &str, value: &[u8]) -> Result<()> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    db.put(key, value)
                        .map_err(|e| anyhow!("Failed to write KV to RocksDB: {}", e))?;
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.kv_store.insert(key.to_string(), value.to_vec());
            }
        }

        Ok(())
    }

    /// Load arbitrary key-value data
    pub async fn load_kv(&self, key: &str) -> Result<Vec<u8>> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let data = db
                        .get(key)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Key not found: {}", key))?;

                    Ok(data)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                storage
                    .kv_store
                    .get(key)
                    .cloned()
                    .ok_or_else(|| anyhow!("Key not found: {}", key))
            }
        }
    }

    /// Delete a key
    pub async fn delete(&self, key: &str) -> Result<()> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    db.delete(key)
                        .map_err(|e| anyhow!("Failed to delete from RocksDB: {}", e))?;
                }
            }
            StorageBackend::Memory => {
                let mut storage = self.memory_storage.write().await;
                storage.kv_store.remove(key);
            }
        }

        Ok(())
    }

    /// Get the latest block height
    pub async fn get_latest_height(&self) -> Result<u64> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let data = db
                        .get(StorageKeys::LATEST_BLOCK_HEIGHT)
                        .map_err(|e| anyhow!("RocksDB read error: {}", e))?
                        .ok_or_else(|| anyhow!("Latest height not found"))?;

                    let height = u64::from_be_bytes(
                        data.try_into()
                            .map_err(|_| anyhow!("Invalid height data"))?,
                    );

                    Ok(height)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                let max_height = storage.blocks_by_height.keys().max().copied().unwrap_or(0);
                Ok(max_height)
            }
        }
    }

    /// Create a backup of the database
    pub async fn create_backup(&self, backup_path: &str) -> Result<()> {
        match &self.backend {
            StorageBackend::RocksDB(data_path) => {
                // Create backup directory
                std::fs::create_dir_all(backup_path)
                    .map_err(|e| anyhow!("Failed to create backup directory: {}", e))?;

                // Copy database files
                let backup_db_path = format!("{}/rocksdb", backup_path);
                std::fs::create_dir_all(&backup_db_path)
                    .map_err(|e| anyhow!("Failed to create backup DB directory: {}", e))?;

                // Use system copy command for better performance
                let output = std::process::Command::new("cp")
                    .arg("-r")
                    .arg(data_path)
                    .arg(&backup_db_path)
                    .output()
                    .map_err(|e| anyhow!("Failed to execute backup command: {}", e))?;

                if !output.status.success() {
                    return Err(anyhow!(
                        "Backup command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }

                tracing::info!("Database backup created at: {}", backup_path);
                Ok(())
            }
            StorageBackend::Memory => {
                // For memory backend, serialize everything to JSON
                let storage = self.memory_storage.read().await;
                let backup_data = serde_json::to_string_pretty(&*storage)
                    .map_err(|e| anyhow!("Failed to serialize memory storage: {}", e))?;

                std::fs::create_dir_all(backup_path)
                    .map_err(|e| anyhow!("Failed to create backup directory: {}", e))?;

                let backup_file = format!("{}/memory_backup.json", backup_path);
                std::fs::write(&backup_file, backup_data)
                    .map_err(|e| anyhow!("Failed to write memory backup: {}", e))?;

                tracing::info!("Memory storage backup created at: {}", backup_file);
                Ok(())
            }
        }
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        match &self.backend {
            StorageBackend::RocksDB(data_path) => {
                let size = self.calculate_directory_size(data_path)?;
                Ok(StorageStats {
                    backend_type: "RocksDB".to_string(),
                    size_bytes: size,
                    total_blocks: self.count_blocks().await?,
                    total_transactions: self.count_transactions().await?,
                    total_accounts: self.count_accounts().await?,
                })
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                let size = std::mem::size_of_val(&*storage);

                Ok(StorageStats {
                    backend_type: "Memory".to_string(),
                    size_bytes: size as u64,
                    total_blocks: storage.blocks_by_height.len() as u64,
                    total_transactions: storage.transactions.len() as u64,
                    total_accounts: storage.accounts.len() as u64,
                })
            }
        }
    }

    /// Calculate directory size recursively
    fn calculate_directory_size(&self, path: &str) -> Result<u64> {
        let mut size = 0;
        let entries =
            std::fs::read_dir(path).map_err(|e| anyhow!("Failed to read directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
            let metadata = entry
                .metadata()
                .map_err(|e| anyhow!("Failed to get file metadata: {}", e))?;

            if metadata.is_file() {
                size += metadata.len();
            } else if metadata.is_dir() {
                size += self.calculate_directory_size(&entry.path().to_string_lossy())?;
            }
        }

        Ok(size)
    }

    /// Count total blocks
    async fn count_blocks(&self) -> Result<u64> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let mut count = 0;
                    let iter = db.prefix_iterator(StorageKeys::BLOCK_HEIGHT_PREFIX);
                    for _ in iter {
                        count += 1;
                    }
                    Ok(count)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.blocks_by_height.len() as u64)
            }
        }
    }

    /// Count total transactions
    async fn count_transactions(&self) -> Result<u64> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let mut count = 0;
                    let iter = db.prefix_iterator(StorageKeys::TRANSACTION_PREFIX);
                    for _ in iter {
                        count += 1;
                    }
                    Ok(count)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.transactions.len() as u64)
            }
        }
    }

    /// Count total accounts
    async fn count_accounts(&self) -> Result<u64> {
        match &self.backend {
            StorageBackend::RocksDB(_) => {
                if let Some(db) = &self.rocksdb {
                    let mut count = 0;
                    let iter = db.prefix_iterator(StorageKeys::ACCOUNT_PREFIX);
                    for _ in iter {
                        count += 1;
                    }
                    Ok(count)
                } else {
                    Err(anyhow!("RocksDB not initialized"))
                }
            }
            StorageBackend::Memory => {
                let storage = self.memory_storage.read().await;
                Ok(storage.accounts.len() as u64)
            }
        }
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub backend_type: String,
    pub size_bytes: u64,
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub total_accounts: u64,
}

/// Make MemoryStorage serializable for backups
impl Serialize for MemoryStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MemoryStorage", 6)?;
        state.serialize_field("blocks_by_height", &self.blocks_by_height)?;
        state.serialize_field("blocks_by_hash", &self.blocks_by_hash)?;
        state.serialize_field("transactions", &self.transactions)?;
        state.serialize_field("accounts", &self.accounts)?;
        state.serialize_field("stats", &self.stats)?;
        // Skip kv_store as it may contain binary data
        state.end()
    }
}

impl<'de> Deserialize<'de> for MemoryStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            BlocksByHeight,
            BlocksByHash,
            Transactions,
            Accounts,
            Stats,
        }

        struct MemoryStorageVisitor;

        impl<'de> serde::de::Visitor<'de> for MemoryStorageVisitor {
            type Value = MemoryStorage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct MemoryStorage")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MemoryStorage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocks_by_height = None;
                let mut blocks_by_hash = None;
                let mut transactions = None;
                let mut accounts = None;
                let mut stats = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::BlocksByHeight => {
                            if blocks_by_height.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocks_by_height"));
                            }
                            blocks_by_height = Some(map.next_value()?);
                        }
                        Field::BlocksByHash => {
                            if blocks_by_hash.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocks_by_hash"));
                            }
                            blocks_by_hash = Some(map.next_value()?);
                        }
                        Field::Transactions => {
                            if transactions.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions = Some(map.next_value()?);
                        }
                        Field::Accounts => {
                            if accounts.is_some() {
                                return Err(serde::de::Error::duplicate_field("accounts"));
                            }
                            accounts = Some(map.next_value()?);
                        }
                        Field::Stats => {
                            if stats.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats = Some(map.next_value()?);
                        }
                    }
                }

                let blocks_by_height = blocks_by_height
                    .ok_or_else(|| serde::de::Error::missing_field("blocks_by_height"))?;
                let blocks_by_hash = blocks_by_hash
                    .ok_or_else(|| serde::de::Error::missing_field("blocks_by_hash"))?;
                let transactions =
                    transactions.ok_or_else(|| serde::de::Error::missing_field("transactions"))?;
                let accounts =
                    accounts.ok_or_else(|| serde::de::Error::missing_field("accounts"))?;
                let stats = stats.ok_or_else(|| serde::de::Error::missing_field("stats"))?;

                Ok(MemoryStorage {
                    blocks_by_height,
                    blocks_by_hash,
                    transactions,
                    accounts,
                    stats,
                    kv_store: HashMap::new(), // Initialize empty
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "blocks_by_height",
            "blocks_by_hash",
            "transactions",
            "accounts",
            "stats",
        ];
        deserializer.deserialize_struct("MemoryStorage", FIELDS, MemoryStorageVisitor)
    }
}
