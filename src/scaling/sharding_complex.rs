//! GridTokenX Sharding Implementation
//!
//! This module implements geographic and functional sharding for the GridTokenX blockchain
//! to handle Thailand's distributed energy trading market efficiently.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blockchain::{Blockchain, Transaction, TransactionType, Block};
use crate::consensus::ConsensusEngine;
use crate::storage::StorageManager;

/// Types of sharding strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShardType {
    Geographic(ThaiRegion),
    Functional(FunctionType),
    Hybrid(ThaiRegion, FunctionType),
}

/// Thai geographic regions for sharding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum ThaiRegion {
    Bangkok,
    Central,
    Northern,
    Northeastern,
    Eastern,
    Western,
    Southern,
}

/// Functional types for specialized shards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionType {
    EnergyTrading,
    Governance,
    GridManagement,
    Compliance,
    Analytics,
}

/// Main shard manager that coordinates all shards
pub struct ShardManager {
    shards: RwLock<HashMap<String, Arc<Shard>>>,
    routing_table: RwLock<ShardRoutingTable>,
    cross_shard_coordinator: Arc<CrossShardCoordinator>,
    config: ShardingConfig,
}

/// Individual shard containing blockchain state and services
pub struct Shard {
    pub id: String,
    pub shard_type: ShardType,
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub consensus_engine: Arc<RwLock<ConsensusEngine>>,
    pub storage: Arc<StorageManager>,
    pub is_active: RwLock<bool>,
    pub load_metrics: RwLock<ShardMetrics>,
    pub validator_set: RwLock<Vec<String>>,
}

/// Routing table for determining transaction destinations
#[derive(Debug, Default)]
pub struct ShardRoutingTable {
    region_mapping: HashMap<String, ThaiRegion>,
    function_mapping: HashMap<String, FunctionType>,
    grid_location_mapping: HashMap<String, ThaiRegion>,
    load_balancing_weights: HashMap<String, f64>,
}

/// Cross-shard transaction coordinator
pub struct CrossShardCoordinator {
    pending_cross_shard_txs: RwLock<HashMap<String, CrossShardTransaction>>,
    commit_log: RwLock<Vec<CrossShardCommit>>,
}

/// Configuration for sharding system
#[derive(Debug, Clone)]
pub struct ShardingConfig {
    pub enable_geographic_sharding: bool,
    pub enable_functional_sharding: bool,
    pub max_shards_per_region: usize,
    pub cross_shard_timeout_ms: u64,
    pub load_balancing_enabled: bool,
    pub auto_shard_splitting: bool,
    pub shard_split_threshold: f64, // TPS threshold for splitting
}

impl Default for ShardingConfig {
    fn default() -> Self {
        Self {
            enable_geographic_sharding: true,
            enable_functional_sharding: true,
            max_shards_per_region: 3,
            cross_shard_timeout_ms: 5000,
            load_balancing_enabled: true,
            auto_shard_splitting: true,
            shard_split_threshold: 1000.0, // 1000 TPS
        }
    }
}

/// Metrics for individual shards
#[derive(Debug, Default, Clone)]
pub struct ShardMetrics {
    pub transactions_per_second: f64,
    pub block_processing_time_ms: f64,
    pub storage_usage_mb: f64,
    pub memory_usage_mb: f64,
    pub active_connections: u32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Cross-shard transaction information
#[derive(Debug, Clone)]
pub struct CrossShardTransaction {
    pub tx_id: String,
    pub involved_shards: Vec<String>,
    pub status: CrossShardStatus,
    pub coordinator_shard: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub timeout_at: chrono::DateTime<chrono::Utc>,
}

/// Status of cross-shard transactions
#[derive(Debug, Clone, PartialEq)]
pub enum CrossShardStatus {
    Preparing,
    Prepared,
    Committing,
    Committed,
    Aborting,
    Aborted,
}

/// Cross-shard commit record
#[derive(Debug, Clone)]
pub struct CrossShardCommit {
    pub tx_id: String,
    pub shard_commits: HashMap<String, bool>,
    pub final_status: CrossShardStatus,
    pub committed_at: chrono::DateTime<chrono::Utc>,
}

impl ShardManager {
    /// Create a new shard manager
    pub async fn new(config: ShardingConfig) -> Result<Self> {
        let mut manager = Self {
            shards: RwLock::new(HashMap::new()),
            routing_table: RwLock::new(ShardRoutingTable::default()),
            cross_shard_coordinator: Arc::new(CrossShardCoordinator::new()),
            config,
        };

        // Initialize default shards
        manager.initialize_default_shards().await?;
        
        Ok(manager)
    }

    /// Initialize default shard configuration for Thailand
    async fn initialize_default_shards(&self) -> Result<()> {
        let mut shards = self.shards.write().await;

        // Create geographic shards for major Thai regions
        for region in [
            ThaiRegion::Bangkok,
            ThaiRegion::Central,
            ThaiRegion::Northern,
            ThaiRegion::Northeastern,
            ThaiRegion::Eastern,
            ThaiRegion::Western,
            ThaiRegion::Southern,
        ] {
            let shard_id = format!("geo_{:?}", region).to_lowercase();
            let shard = Arc::new(self.create_shard(
                shard_id.clone(),
                ShardType::Geographic(region.clone()),
            ).await?);
            
            shards.insert(shard_id, shard);
        }

        // Create functional shards
        for function in [
            FunctionType::EnergyTrading,
            FunctionType::Governance,
            FunctionType::GridManagement,
            FunctionType::Compliance,
            FunctionType::Analytics,
        ] {
            let shard_id = format!("func_{:?}", function).to_lowercase();
            let shard = Arc::new(self.create_shard(
                shard_id.clone(),
                ShardType::Functional(function),
            ).await?);
            
            shards.insert(shard_id, shard);
        }

        // Initialize routing table
        self.initialize_routing_table().await?;

        info!("Initialized {} shards", shards.len());
        Ok(())
    }

    /// Create a new shard
    async fn create_shard(&self, id: String, shard_type: ShardType) -> Result<Shard> {
        // Create storage for this shard
        let storage_path = format!("./data/shards/{}", id);
        let storage = Arc::new(StorageManager::new(&storage_path).await?);
        
        // Create blockchain for this shard
        let blockchain = Arc::new(RwLock::new(Blockchain::new(storage.clone()).await?));
        
        // Create consensus engine for this shard
        let consensus_config = self.get_consensus_config_for_shard(&shard_type);
        let consensus_engine = Arc::new(RwLock::new(
            ConsensusEngine::new(blockchain.clone(), consensus_config).await?
        ));

        Ok(Shard {
            id,
            shard_type,
            blockchain,
            consensus_engine,
            storage,
            is_active: RwLock::new(true),
            load_metrics: RwLock::new(ShardMetrics::default()),
            validator_set: RwLock::new(Vec::new()),
        })
    }

    /// Route a transaction to the appropriate shard
    pub async fn route_transaction(&self, tx: &Transaction) -> Result<String> {
        let routing_table = self.routing_table.read().await;

        match &tx.transaction_type {
            TransactionType::EnergyTrade(energy_tx) => {
                // Route based on grid location
                if let Some(region) = routing_table.grid_location_mapping.get(&energy_tx.grid_location) {
                    if self.config.enable_geographic_sharding {
                        return Ok(format!("geo_{:?}", region).to_lowercase());
                    }
                }
                
                // Fallback to energy trading functional shard
                Ok("func_energytrading".to_string())
            }
            TransactionType::Governance(_) => {
                Ok("func_governance".to_string())
            }
            TransactionType::GridManagement(_) => {
                Ok("func_gridmanagement".to_string())
            }
            TransactionType::Compliance(_) => {
                Ok("func_compliance".to_string())
            }
            _ => {
                // Load balance across available shards
                self.get_least_loaded_shard().await
            }
        }
    }

    /// Get the least loaded shard for load balancing
    async fn get_least_loaded_shard(&self) -> Result<String> {
        let shards = self.shards.read().await;
        let mut min_load = f64::MAX;
        let mut selected_shard = None;

        for (shard_id, shard) in shards.iter() {
            if *shard.is_active.read().await {
                let metrics = shard.load_metrics.read().await;
                let current_load = metrics.transactions_per_second + 
                                 (metrics.memory_usage_mb / 1000.0) + 
                                 (metrics.block_processing_time_ms / 100.0);
                
                if current_load < min_load {
                    min_load = current_load;
                    selected_shard = Some(shard_id.clone());
                }
            }
        }

        selected_shard.ok_or_else(|| anyhow!("No active shards available"))
    }

    /// Execute a cross-shard transaction
    pub async fn execute_cross_shard_transaction(&self, tx: &Transaction) -> Result<()> {
        let involved_shards = self.get_involved_shards(tx).await?;
        
        if involved_shards.len() <= 1 {
            // Single shard transaction
            let shard_id = self.route_transaction(tx).await?;
            let shard = self.get_shard(&shard_id).await?;
            return self.execute_single_shard_transaction(shard, tx).await;
        }

        // Multi-shard transaction using 2PC protocol
        self.cross_shard_coordinator.coordinate_transaction(tx, involved_shards).await
    }

    /// Get all shards involved in a transaction
    async fn get_involved_shards(&self, tx: &Transaction) -> Result<Vec<String>> {
        let mut involved_shards = std::collections::HashSet::new();

        match &tx.transaction_type {
            TransactionType::EnergyTrade(energy_tx) => {
                // Sender and receiver might be in different shards
                involved_shards.insert(self.route_address_to_shard(&energy_tx.seller).await?);
                involved_shards.insert(self.route_address_to_shard(&energy_tx.buyer).await?);
                
                // Grid management shard might be involved
                if self.requires_grid_coordination(energy_tx) {
                    involved_shards.insert("func_gridmanagement".to_string());
                }
            }
            _ => {
                involved_shards.insert(self.route_transaction(tx).await?);
            }
        }

        Ok(involved_shards.into_iter().collect())
    }

    /// Route an address to its corresponding shard
    async fn route_address_to_shard(&self, address: &str) -> Result<String> {
        // Simple hash-based routing for now
        // In production, this would use more sophisticated routing
        let hash = sha2::Digest::digest(address.as_bytes());
        let shard_index = u64::from_be_bytes([
            hash[0], hash[1], hash[2], hash[3],
            hash[4], hash[5], hash[6], hash[7],
        ]) % 7; // 7 geographic regions

        let regions = [
            ThaiRegion::Bangkok,
            ThaiRegion::Central,
            ThaiRegion::Northern,
            ThaiRegion::Northeastern,
            ThaiRegion::Eastern,
            ThaiRegion::Western,
            ThaiRegion::Southern,
        ];

        Ok(format!("geo_{:?}", regions[shard_index as usize]).to_lowercase())
    }

    /// Check if an energy transaction requires grid coordination
    fn requires_grid_coordination(&self, energy_tx: &crate::blockchain::EnergyTransaction) -> bool {
        // Require grid coordination for large transactions or cross-region trades
        energy_tx.energy_amount > 1000.0 || // Large trades
        energy_tx.requires_grid_balancing.unwrap_or(false)
    }

    /// Get a shard by ID
    pub async fn get_shard(&self, shard_id: &str) -> Result<Arc<Shard>> {
        let shards = self.shards.read().await;
        shards.get(shard_id)
            .cloned()
            .ok_or_else(|| anyhow!("Shard not found: {}", shard_id))
    }

    /// Execute transaction in a single shard
    async fn execute_single_shard_transaction(&self, shard: Arc<Shard>, tx: &Transaction) -> Result<()> {
        let mut blockchain = shard.blockchain.write().await;
        blockchain.add_pending_transaction(tx.clone()).await?;
        
        // Update shard metrics
        let mut metrics = shard.load_metrics.write().await;
        metrics.transactions_per_second += 1.0;
        metrics.last_updated = chrono::Utc::now();
        
        Ok(())
    }

    /// Get the number of active shards
    pub async fn get_active_shard_count(&self) -> usize {
        let shards = self.shards.read().await;
        let mut count = 0;
        
        for shard in shards.values() {
            if *shard.is_active.read().await {
                count += 1;
            }
        }
        
        count
    }

    /// Initialize the routing table with Thai geography
    async fn initialize_routing_table(&self) -> Result<()> {
        let mut routing_table = self.routing_table.write().await;

        // Map Thai provinces to regions
        let province_mappings = vec![
            ("Bangkok", ThaiRegion::Bangkok),
            ("Nonthaburi", ThaiRegion::Central),
            ("Pathum Thani", ThaiRegion::Central),
            ("Samut Prakan", ThaiRegion::Central),
            ("Chiang Mai", ThaiRegion::Northern),
            ("Chiang Rai", ThaiRegion::Northern),
            ("Khon Kaen", ThaiRegion::Northeastern),
            ("Udon Thani", ThaiRegion::Northeastern),
            ("Chonburi", ThaiRegion::Eastern),
            ("Rayong", ThaiRegion::Eastern),
            ("Kanchanaburi", ThaiRegion::Western),
            ("Ratchaburi", ThaiRegion::Western),
            ("Songkhla", ThaiRegion::Southern),
            ("Phuket", ThaiRegion::Southern),
        ];

        for (province, region) in province_mappings {
            routing_table.region_mapping.insert(province.to_string(), region);
        }

        // Map grid locations to regions
        let grid_mappings = vec![
            ("Central_Grid", ThaiRegion::Central),
            ("Northern_Grid", ThaiRegion::Northern),
            ("Northeastern_Grid", ThaiRegion::Northeastern),
            ("Southern_Grid", ThaiRegion::Southern),
            ("Bangkok_Metro_Grid", ThaiRegion::Bangkok),
        ];

        for (grid_location, region) in grid_mappings {
            routing_table.grid_location_mapping.insert(grid_location.to_string(), region);
        }

        Ok(())
    }

    /// Get consensus configuration for a specific shard type
    fn get_consensus_config_for_shard(&self, shard_type: &ShardType) -> crate::config::ConsensusConfig {
        match shard_type {
            ShardType::Functional(FunctionType::Governance) => {
                // Governance shards need higher security
                crate::config::ConsensusConfig {
                    algorithm: crate::consensus::ConsensusAlgorithm::ProofOfStake,
                    block_time_ms: 5000, // 5 seconds
                    min_validators: 5,
                    max_validators: 21,
                    stake_threshold: 100000,
                    ..Default::default()
                }
            }
            ShardType::Functional(FunctionType::EnergyTrading) => {
                // Trading shards need speed
                crate::config::ConsensusConfig {
                    algorithm: crate::consensus::ConsensusAlgorithm::Hybrid,
                    block_time_ms: 1000, // 1 second
                    min_validators: 3,
                    max_validators: 15,
                    stake_threshold: 50000,
                    ..Default::default()
                }
            }
            _ => {
                // Default configuration
                crate::config::ConsensusConfig::default()
            }
        }
    }
}

impl CrossShardCoordinator {
    pub fn new() -> Self {
        Self {
            pending_cross_shard_txs: RwLock::new(HashMap::new()),
            commit_log: RwLock::new(Vec::new()),
        }
    }

    /// Coordinate a cross-shard transaction using 2PC protocol
    pub async fn coordinate_transaction(
        &self,
        tx: &Transaction,
        involved_shards: Vec<String>,
    ) -> Result<()> {
        let tx_id = tx.id.clone();
        let timeout = chrono::Utc::now() + chrono::Duration::milliseconds(5000);

        // Create cross-shard transaction record
        let cross_shard_tx = CrossShardTransaction {
            tx_id: tx_id.clone(),
            involved_shards: involved_shards.clone(),
            status: CrossShardStatus::Preparing,
            coordinator_shard: involved_shards[0].clone(), // First shard is coordinator
            created_at: chrono::Utc::now(),
            timeout_at: timeout,
        };

        // Add to pending transactions
        {
            let mut pending = self.pending_cross_shard_txs.write().await;
            pending.insert(tx_id.clone(), cross_shard_tx);
        }

        // Phase 1: Prepare
        let prepare_results = self.prepare_phase(&tx_id, tx, &involved_shards).await?;
        
        // Check if all shards are prepared
        let all_prepared = prepare_results.iter().all(|&result| result);

        if all_prepared {
            // Phase 2: Commit
            self.commit_phase(&tx_id, &involved_shards).await?;
        } else {
            // Abort transaction
            self.abort_phase(&tx_id, &involved_shards).await?;
        }

        // Clean up
        {
            let mut pending = self.pending_cross_shard_txs.write().await;
            pending.remove(&tx_id);
        }

        Ok(())
    }

    /// Phase 1 of 2PC: Prepare
    async fn prepare_phase(
        &self,
        tx_id: &str,
        tx: &Transaction,
        involved_shards: &[String],
    ) -> Result<Vec<bool>> {
        let mut results = Vec::new();

        for shard_id in involved_shards {
            // In a real implementation, this would send prepare messages to shards
            // For now, we'll simulate preparation
            let prepared = self.simulate_shard_prepare(shard_id, tx).await?;
            results.push(prepared);
        }

        // Update status
        {
            let mut pending = self.pending_cross_shard_txs.write().await;
            if let Some(cross_tx) = pending.get_mut(tx_id) {
                cross_tx.status = if results.iter().all(|&r| r) {
                    CrossShardStatus::Prepared
                } else {
                    CrossShardStatus::Aborting
                };
            }
        }

        Ok(results)
    }

    /// Phase 2 of 2PC: Commit
    async fn commit_phase(&self, tx_id: &str, involved_shards: &[String]) -> Result<()> {
        let mut shard_commits = HashMap::new();

        for shard_id in involved_shards {
            let committed = self.simulate_shard_commit(shard_id, tx_id).await?;
            shard_commits.insert(shard_id.clone(), committed);
        }

        // Record commit
        let commit_record = CrossShardCommit {
            tx_id: tx_id.to_string(),
            shard_commits,
            final_status: CrossShardStatus::Committed,
            committed_at: chrono::Utc::now(),
        };

        {
            let mut commit_log = self.commit_log.write().await;
            commit_log.push(commit_record);
        }

        Ok(())
    }

    /// Abort phase for failed transactions
    async fn abort_phase(&self, tx_id: &str, involved_shards: &[String]) -> Result<()> {
        for shard_id in involved_shards {
            self.simulate_shard_abort(shard_id, tx_id).await?;
        }

        // Update status
        {
            let mut pending = self.pending_cross_shard_txs.write().await;
            if let Some(cross_tx) = pending.get_mut(tx_id) {
                cross_tx.status = CrossShardStatus::Aborted;
            }
        }

        Ok(())
    }

    /// Simulate shard preparation (would be replaced with actual shard communication)
    async fn simulate_shard_prepare(&self, _shard_id: &str, _tx: &Transaction) -> Result<bool> {
        // Simulate some preparation work and success
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        Ok(true) // Assume preparation succeeds
    }

    /// Simulate shard commit (would be replaced with actual shard communication)
    async fn simulate_shard_commit(&self, _shard_id: &str, _tx_id: &str) -> Result<bool> {
        // Simulate commit work
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        Ok(true) // Assume commit succeeds
    }

    /// Simulate shard abort (would be replaced with actual shard communication)
    async fn simulate_shard_abort(&self, _shard_id: &str, _tx_id: &str) -> Result<()> {
        // Simulate abort work
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shard_manager_creation() -> Result<()> {
        let config = ShardingConfig::default();
        let shard_manager = ShardManager::new(config).await?;
        
        // Test that shards are created
        assert!(shard_manager.get_active_shard_count().await > 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_routing() -> Result<()> {
        let config = ShardingConfig::default();
        let shard_manager = ShardManager::new(config).await?;
        
        // Create a test energy transaction
        let energy_tx = crate::blockchain::EnergyTransaction {
            seller: "seller_001".to_string(),
            buyer: "buyer_001".to_string(),
            energy_amount: 100.0,
            price_per_kwh: 4.50,
            grid_location: "Central_Grid".to_string(),
            energy_type: Some("Solar".to_string()),
            requires_grid_balancing: Some(false),
            transaction_timestamp: chrono::Utc::now(),
            settlement_timestamp: None,
            renewable_certificate: None,
        };

        let tx = Transaction::new_energy_trade(energy_tx)?;
        let shard_id = shard_manager.route_transaction(&tx).await?;
        
        // Should route to geographic shard for Central region
        assert!(shard_id.contains("central") || shard_id.contains("energytrading"));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_cross_shard_coordination() -> Result<()> {
        let coordinator = CrossShardCoordinator::new();
        
        // Create a test transaction that spans multiple shards
        let energy_tx = crate::blockchain::EnergyTransaction {
            seller: "seller_bangkok".to_string(),
            buyer: "buyer_northern".to_string(),
            energy_amount: 1500.0, // Large transaction requiring grid coordination
            price_per_kwh: 4.50,
            grid_location: "Inter_Regional".to_string(),
            energy_type: Some("Hydro".to_string()),
            requires_grid_balancing: Some(true),
            transaction_timestamp: chrono::Utc::now(),
            settlement_timestamp: None,
            renewable_certificate: None,
        };

        let tx = Transaction::new_energy_trade(energy_tx)?;
        let involved_shards = vec![
            "geo_bangkok".to_string(),
            "geo_northern".to_string(),
            "func_gridmanagement".to_string(),
        ];

        // Test coordination
        coordinator.coordinate_transaction(&tx, involved_shards).await?;
        
        Ok(())
    }
}
