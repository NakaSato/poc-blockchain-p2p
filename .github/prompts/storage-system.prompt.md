---
mode: edit
---

# GridTokenX Storage System Development Prompt

You are developing the storage layer for GridTokenX - Thailand's energy trading blockchain platform using RocksDB for high-performance, persistent data storage.

## Storage Architecture Overview

The storage system (`src/storage.rs`) provides:
- **High Performance**: 10,000+ reads/writes per second for energy trading
- **Data Integrity**: ACID compliance with cryptographic verification
- **Scalability**: Terabyte-scale storage for grid data and transaction history
- **Reliability**: 99.99% uptime with automated backup and recovery

## RocksDB Configuration

### Database Structure
```rust
use rocksdb::{DB, ColumnFamily, Options, WriteBatch, Direction, IteratorMode};

pub struct StorageManager {
    db: Arc<DB>,
    column_families: HashMap<String, Arc<ColumnFamily>>,
    write_options: WriteOptions,
    read_options: ReadOptions,
}

// Column Family Organization
pub enum ColumnFamily {
    Blocks,              // Block storage with height indexing
    Transactions,        // Transaction data with hash indexing
    EnergyOrders,        // Active energy trading orders
    GridState,           // Real-time grid status data
    Authorities,         // Authority node information
    Validators,          // Validator performance and stake data
    EnergyMeters,        // Smart meter readings and proofs
    Governance,          // DAO proposals and voting records
    Configuration,       // System configuration and parameters
    Metrics,             // Performance and analytics data
}
```

### Performance Optimization
```rust
pub struct StorageConfig {
    pub max_open_files: i32,                // 10000 for high concurrency
    pub write_buffer_size: usize,           // 256MB for write performance
    pub max_write_buffer_number: i32,       // 4 for parallel writes
    pub target_file_size_base: u64,         // 256MB per SST file
    pub level0_file_num_compaction_trigger: i32, // 4 files trigger compaction
    pub compression_type: CompressionType,  // LZ4 for speed vs size balance
    pub bloom_filter_bits_per_key: i32,     // 10 bits for optimal false positive rate
}
```

## Data Models and Schemas

### Block Storage
```rust
#[derive(Serialize, Deserialize)]
pub struct StoredBlock {
    pub block: Block,
    pub height: u64,
    pub confirmations: u32,
    pub total_energy: f64,               // Total energy in block transactions
    pub grid_state_hash: String,         // Grid state after block execution
    pub authority_signatures: Vec<AuthoritySignature>,
    pub storage_timestamp: DateTime<Utc>,
}

// Storage Keys
pub fn block_key(height: u64) -> Vec<u8> {
    format!("block:{:016x}", height).into_bytes()
}

pub fn block_hash_key(hash: &str) -> Vec<u8> {
    format!("block_hash:{}", hash).into_bytes()
}
```

### Transaction Indexing
```rust
#[derive(Serialize, Deserialize)]
pub struct TransactionIndex {
    pub transaction_hash: String,
    pub block_height: u64,
    pub block_index: u32,
    pub transaction_type: TransactionType,
    pub energy_amount: Option<f64>,
    pub grid_zone: Option<GridZone>,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
}

// Multi-dimensional indexing
pub fn tx_by_hash_key(hash: &str) -> Vec<u8> {
    format!("tx_hash:{}", hash).into_bytes()
}

pub fn tx_by_address_key(address: &str, timestamp: DateTime<Utc>) -> Vec<u8> {
    format!("tx_addr:{}:{}", address, timestamp.timestamp()).into_bytes()
}

pub fn tx_by_type_key(tx_type: TransactionType, timestamp: DateTime<Utc>) -> Vec<u8> {
    format!("tx_type:{:?}:{}", tx_type, timestamp.timestamp()).into_bytes()
}
```

### Energy Trading Data
```rust
#[derive(Serialize, Deserialize)]
pub struct StoredEnergyOrder {
    pub order: EnergyOrder,
    pub status: OrderStatus,
    pub creation_time: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
    pub partial_fills: Vec<PartialFill>,
    pub grid_constraints: GridConstraints,
}

#[derive(Serialize, Deserialize)]
pub struct EnergyMeterReading {
    pub meter_id: String,
    pub reading: f64,                    // kWh value
    pub timestamp: DateTime<Utc>,
    pub location: GridCoordinate,
    pub meter_signature: MeterSignature,
    pub authority_witness: Option<AuthoritySignature>,
    pub verification_status: VerificationStatus,
}
```

### Grid State Management
```rust
#[derive(Serialize, Deserialize)]
pub struct GridStateSnapshot {
    pub timestamp: DateTime<Utc>,
    pub grid_frequency: f64,
    pub voltage_levels: HashMap<String, f64>,
    pub power_flows: HashMap<String, PowerFlow>,
    pub congestion_status: HashMap<String, CongestionLevel>,
    pub renewable_percentage: f64,
    pub total_demand: f64,
    pub total_generation: f64,
    pub reserve_margins: ReserveMargins,
}
```

## Storage Operations

### Atomic Batch Operations
```rust
impl StorageManager {
    pub async fn store_block_with_transactions(
        &self,
        block: &Block,
        transactions: &[Transaction],
        grid_state: &GridState,
    ) -> Result<()> {
        let mut batch = WriteBatch::default();
        
        // Store block
        let block_key = self.block_key(block.height);
        let stored_block = StoredBlock::from_block(block, grid_state);
        batch.put_cf(&self.blocks_cf(), block_key, bincode::serialize(&stored_block)?);
        
        // Store transactions with indexing
        for (index, tx) in transactions.iter().enumerate() {
            self.add_transaction_to_batch(&mut batch, tx, block.height, index as u32)?;
        }
        
        // Update grid state
        let grid_state_key = format!("grid_state:{}", block.timestamp.timestamp());
        batch.put_cf(&self.grid_state_cf(), grid_state_key, bincode::serialize(grid_state)?);
        
        // Atomic write
        self.db.write(batch)?;
        
        // Update indexes asynchronously
        self.update_indexes_async(block, transactions).await?;
        
        Ok(())
    }
}
```

### High-Performance Queries
```rust
impl StorageManager {
    pub async fn get_energy_orders_by_zone(
        &self,
        zone: GridZone,
        price_range: Option<(u64, u64)>,
        limit: usize,
    ) -> Result<Vec<EnergyOrder>> {
        let start_key = format!("energy_order:{}:", zone);
        let end_key = format!("energy_order:{}~", zone);
        
        let iter = self.db.iterator_cf(
            &self.energy_orders_cf(),
            IteratorMode::From(start_key.as_bytes(), Direction::Forward),
        );
        
        let mut orders = Vec::new();
        for item in iter.take(limit) {
            let (key, value) = item?;
            if !key.starts_with(start_key.as_bytes()) {
                break;
            }
            
            let order: StoredEnergyOrder = bincode::deserialize(&value)?;
            
            // Apply price filtering
            if let Some((min_price, max_price)) = price_range {
                if order.order.price < min_price || order.order.price > max_price {
                    continue;
                }
            }
            
            orders.push(order.order);
        }
        
        Ok(orders)
    }
}
```

### Time-Series Data Management
```rust
impl StorageManager {
    pub async fn store_grid_metrics(
        &self,
        metrics: &GridMetrics,
        retention_days: u32,
    ) -> Result<()> {
        let key = format!("metrics:{}:{}", 
            metrics.timestamp.timestamp(), 
            metrics.metric_type);
        
        self.db.put_cf(
            &self.metrics_cf(),
            key,
            bincode::serialize(metrics)?,
        )?;
        
        // Schedule cleanup of old data
        self.schedule_cleanup(retention_days).await?;
        
        Ok(())
    }
    
    pub async fn get_grid_metrics_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        metric_type: MetricType,
    ) -> Result<Vec<GridMetrics>> {
        let start_key = format!("metrics:{}:{}", start_time.timestamp(), metric_type);
        let end_key = format!("metrics:{}:{}", end_time.timestamp(), metric_type);
        
        let iter = self.db.iterator_cf(
            &self.metrics_cf(),
            IteratorMode::From(start_key.as_bytes(), Direction::Forward),
        );
        
        let mut metrics = Vec::new();
        for item in iter {
            let (key, value) = item?;
            if key.as_ref() > end_key.as_bytes() {
                break;
            }
            
            let metric: GridMetrics = bincode::deserialize(&value)?;
            metrics.push(metric);
        }
        
        Ok(metrics)
    }
}
```

## Backup and Recovery

### Automated Backup Strategy
```rust
pub struct BackupManager {
    storage: Arc<StorageManager>,
    backup_schedule: BackupSchedule,
    remote_storage: Option<RemoteStorageConfig>,
}

#[derive(Debug)]
pub struct BackupSchedule {
    pub full_backup_interval: Duration,      // Weekly full backups
    pub incremental_interval: Duration,      // Daily incremental backups
    pub snapshot_interval: Duration,         // Hourly snapshots
    pub retention_period: Duration,          // 1 year retention
}

impl BackupManager {
    pub async fn create_snapshot(&self) -> Result<SnapshotInfo> {
        let snapshot = self.storage.db.snapshot();
        let backup_path = self.generate_backup_path();
        
        // Create incremental backup
        let backup_engine = BackupEngine::open(&backup_path)?;
        backup_engine.create_new_backup_flush(&self.storage.db, true)?;
        
        // Upload to remote storage if configured
        if let Some(remote) = &self.remote_storage {
            self.upload_backup(&backup_path, remote).await?;
        }
        
        Ok(SnapshotInfo {
            timestamp: Utc::now(),
            path: backup_path,
            size: self.calculate_backup_size(&backup_path)?,
        })
    }
}
```

### Point-in-Time Recovery
```rust
impl StorageManager {
    pub async fn restore_from_backup(
        &self,
        backup_path: &str,
        target_time: DateTime<Utc>,
    ) -> Result<()> {
        // Find closest backup before target time
        let backup_info = self.find_backup_for_time(target_time)?;
        
        // Restore from backup
        let backup_engine = BackupEngine::open(backup_path)?;
        backup_engine.restore_from_latest_backup(
            &self.data_path,
            &self.data_path,
            &RestoreOptions::default(),
        )?;
        
        // Replay transactions from backup time to target time
        self.replay_transactions(backup_info.timestamp, target_time).await?;
        
        // Verify data integrity
        self.verify_integrity().await?;
        
        Ok(())
    }
}
```

## Data Integrity and Verification

### Cryptographic Verification
```rust
impl StorageManager {
    pub async fn verify_block_integrity(&self, height: u64) -> Result<bool> {
        let stored_block = self.get_block_by_height(height).await?;
        
        // Verify block hash
        let calculated_hash = stored_block.block.calculate_hash();
        if calculated_hash != stored_block.block.hash {
            return Ok(false);
        }
        
        // Verify transaction merkle root
        let tx_hashes: Vec<String> = stored_block.block.transactions
            .iter()
            .map(|tx| tx.hash.clone())
            .collect();
        let merkle_root = self.calculate_merkle_root(&tx_hashes);
        if merkle_root != stored_block.block.merkle_root {
            return Ok(false);
        }
        
        // Verify authority signatures
        for signature in &stored_block.authority_signatures {
            if !self.verify_authority_signature(signature, &stored_block.block.hash).await? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

### Consistency Checks
```rust
impl StorageManager {
    pub async fn run_consistency_check(&self) -> Result<ConsistencyReport> {
        let mut report = ConsistencyReport::new();
        
        // Check energy conservation across all blocks
        let total_energy_supply = self.calculate_total_energy_supply().await?;
        let total_energy_demand = self.calculate_total_energy_demand().await?;
        let energy_balance = (total_energy_supply - total_energy_demand).abs();
        
        if energy_balance > ENERGY_TOLERANCE {
            report.add_error("Energy conservation violation detected");
        }
        
        // Check transaction reference integrity
        self.verify_transaction_references(&mut report).await?;
        
        // Check grid state consistency
        self.verify_grid_state_consistency(&mut report).await?;
        
        // Check authority signature validity
        self.verify_authority_signatures(&mut report).await?;
        
        Ok(report)
    }
}
```

## Performance Monitoring

### Storage Metrics
```rust
#[derive(Debug, Serialize)]
pub struct StorageMetrics {
    pub read_operations_per_second: f64,
    pub write_operations_per_second: f64,
    pub average_read_latency: Duration,
    pub average_write_latency: Duration,
    pub disk_space_used: u64,
    pub memory_usage: u64,
    pub cache_hit_rate: f64,
    pub compaction_stats: CompactionStats,
}

impl StorageManager {
    pub async fn collect_metrics(&self) -> StorageMetrics {
        let db_stats = self.db.property_value("rocksdb.stats").unwrap_or_default();
        
        StorageMetrics {
            read_operations_per_second: self.calculate_read_ops(),
            write_operations_per_second: self.calculate_write_ops(),
            average_read_latency: self.calculate_avg_read_latency(),
            average_write_latency: self.calculate_avg_write_latency(),
            disk_space_used: self.calculate_disk_usage(),
            memory_usage: self.calculate_memory_usage(),
            cache_hit_rate: self.calculate_cache_hit_rate(),
            compaction_stats: self.get_compaction_stats(),
        }
    }
}
```

### Automated Optimization
```rust
impl StorageManager {
    pub async fn optimize_storage(&self) -> Result<()> {
        // Trigger manual compaction during low activity periods
        if self.is_low_activity_period().await {
            self.db.compact_range::<&[u8], &[u8]>(None, None);
        }
        
        // Adjust cache sizes based on usage patterns
        let metrics = self.collect_metrics().await;
        if metrics.cache_hit_rate < 0.9 {
            self.increase_cache_size().await?;
        }
        
        // Clean up old data based on retention policies
        self.cleanup_expired_data().await?;
        
        Ok(())
    }
}
```

When implementing storage features, prioritize data integrity, performance, and reliability while ensuring efficient access patterns for energy trading operations and comprehensive audit trails for regulatory compliance.
