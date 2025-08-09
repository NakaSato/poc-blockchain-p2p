//! Comprehensive tests for storage, utilities, and system integration

use crate::storage::*;
use crate::utils::*;
use crate::blockchain::{Block, Transaction};
use crate::consensus_poa::Authority;
use std::collections::HashMap;
use chrono::{Utc, Duration};
use anyhow::Result;

#[cfg(test)]
mod storage_tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage_creation() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        // Test basic operations
        let key = "test_key";
        let value = b"test_value";
        
        storage.put(key, value).await?;
        let retrieved = storage.get(key).await?;
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), value);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_rocksdb_storage_creation() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let storage = StorageManager::new_rocksdb_storage(temp_dir.path().to_str().unwrap()).await?;
        
        let key = "rocks_test_key";
        let value = b"rocks_test_value";
        
        storage.put(key, value).await?;
        let retrieved = storage.get(key).await?;
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), value);
        
        // Test persistence by creating new storage instance
        drop(storage);
        let storage2 = StorageManager::new_rocksdb_storage(temp_dir.path().to_str().unwrap()).await?;
        let retrieved2 = storage2.get(key).await?;
        
        assert!(retrieved2.is_some());
        assert_eq!(retrieved2.unwrap(), value);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_batch_operations() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        let mut batch = storage.create_batch();
        
        // Add multiple operations to batch
        for i in 0..100 {
            let key = format!("batch_key_{}", i);
            let value = format!("batch_value_{}", i);
            batch.put(&key, value.as_bytes());
        }
        
        // Execute batch
        storage.execute_batch(batch).await?;
        
        // Verify all items were stored
        for i in 0..100 {
            let key = format!("batch_key_{}", i);
            let expected_value = format!("batch_value_{}", i);
            let retrieved = storage.get(&key).await?;
            
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap(), expected_value.as_bytes());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_range_iteration() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        // Store ordered data
        let data = vec![
            ("block_000001", "block_data_1"),
            ("block_000002", "block_data_2"),
            ("block_000003", "block_data_3"),
            ("block_000004", "block_data_4"),
            ("block_000005", "block_data_5"),
        ];
        
        for (key, value) in &data {
            storage.put(key, value.as_bytes()).await?;
        }
        
        // Test range iteration
        let mut iterator = storage.range("block_000002", "block_000004").await?;
        let mut count = 0;
        
        while let Some((key, value)) = iterator.next().await? {
            let key_str = String::from_utf8(key)?;
            let value_str = String::from_utf8(value)?;
            
            assert!(key_str.starts_with("block_00000"));
            assert!(value_str.starts_with("block_data_"));
            count += 1;
        }
        
        assert_eq!(count, 3); // Should include keys 2, 3, and 4
        
        Ok(())
    }

    #[tokio::test]
    async fn test_prefix_iteration() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        // Store data with different prefixes
        let test_data = vec![
            ("block:001", "block_data_1"),
            ("block:002", "block_data_2"),
            ("tx:001", "tx_data_1"),
            ("tx:002", "tx_data_2"),
            ("state:001", "state_data_1"),
        ];
        
        for (key, value) in &test_data {
            storage.put(key, value.as_bytes()).await?;
        }
        
        // Test prefix iteration for blocks
        let mut block_iterator = storage.prefix_scan("block:").await?;
        let mut block_count = 0;
        
        while let Some((key, value)) = block_iterator.next().await? {
            let key_str = String::from_utf8(key)?;
            let value_str = String::from_utf8(value)?;
            
            assert!(key_str.starts_with("block:"));
            assert!(value_str.starts_with("block_data_"));
            block_count += 1;
        }
        
        assert_eq!(block_count, 2);
        
        // Test prefix iteration for transactions
        let mut tx_iterator = storage.prefix_scan("tx:").await?;
        let mut tx_count = 0;
        
        while let Some(_) = tx_iterator.next().await? {
            tx_count += 1;
        }
        
        assert_eq!(tx_count, 2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_storage_snapshots() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        // Store initial data
        storage.put("key1", b"value1").await?;
        storage.put("key2", b"value2").await?;
        
        // Create snapshot
        let snapshot = storage.create_snapshot().await?;
        
        // Modify original storage
        storage.put("key1", b"modified_value1").await?;
        storage.put("key3", b"value3").await?;
        
        // Verify snapshot has original data
        let snap_value1 = snapshot.get("key1").await?;
        let snap_value3 = snapshot.get("key3").await?;
        
        assert_eq!(snap_value1.unwrap(), b"value1");
        assert!(snap_value3.is_none());
        
        // Verify current storage has modified data
        let current_value1 = storage.get("key1").await?;
        let current_value3 = storage.get("key3").await?;
        
        assert_eq!(current_value1.unwrap(), b"modified_value1");
        assert!(current_value3.is_some());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_storage_compression() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        // Create large data that should benefit from compression
        let large_data = "a".repeat(10000);
        let key = "large_data_key";
        
        storage.put_compressed(key, large_data.as_bytes()).await?;
        let retrieved = storage.get_decompressed(key).await?;
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), large_data.as_bytes());
        
        // Verify compression actually happened by checking raw storage
        let raw_data = storage.get(key).await?;
        assert!(raw_data.unwrap().len() < large_data.len());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_storage_encryption() -> Result<()> {
        let mut config = StorageConfig::default();
        config.enable_encryption = true;
        config.encryption_key = Some("test_encryption_key_32_characters".as_bytes().to_vec());
        
        let storage = StorageManager::new_with_config(config).await?;
        
        let sensitive_data = "sensitive_blockchain_data";
        let key = "encrypted_key";
        
        storage.put_encrypted(key, sensitive_data.as_bytes()).await?;
        let decrypted = storage.get_decrypted(key).await?;
        
        assert!(decrypted.is_some());
        assert_eq!(decrypted.unwrap(), sensitive_data.as_bytes());
        
        // Verify raw storage is encrypted
        let raw_data = storage.get(key).await?;
        assert_ne!(raw_data.unwrap(), sensitive_data.as_bytes());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_storage_indexing() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        
        // Create index for block heights
        storage.create_index("block_height_index").await?;
        
        // Store blocks with height indexing
        for i in 1..=10 {
            let block_key = format!("block_{:06}", i);
            let block_data = format!("block_data_{}", i);
            
            storage.put(&block_key, block_data.as_bytes()).await?;
            storage.add_to_index("block_height_index", &i.to_string(), &block_key).await?;
        }
        
        // Query by height
        let block_key_5 = storage.get_from_index("block_height_index", "5").await?;
        assert_eq!(block_key_5.unwrap(), "block_000005");
        
        let block_data_5 = storage.get("block_000005").await?;
        assert_eq!(block_data_5.unwrap(), b"block_data_5");
        
        Ok(())
    }
}

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_hash_functions() {
        let data = b"test_data_for_hashing";
        
        let sha256_hash = hash_sha256(data);
        assert_eq!(sha256_hash.len(), 32); // SHA256 produces 32 bytes
        
        let blake3_hash = hash_blake3(data);
        assert_eq!(blake3_hash.len(), 32); // BLAKE3 default output is 32 bytes
        
        // Verify deterministic hashing
        assert_eq!(hash_sha256(data), hash_sha256(data));
        assert_eq!(hash_blake3(data), hash_blake3(data));
        
        // Verify different inputs produce different hashes
        assert_ne!(hash_sha256(data), hash_sha256(b"different_data"));
        assert_ne!(hash_blake3(data), hash_blake3(b"different_data"));
    }

    #[test]
    fn test_merkle_tree() {
        let transactions = vec![
            "tx1".as_bytes(),
            "tx2".as_bytes(),
            "tx3".as_bytes(),
            "tx4".as_bytes(),
        ];
        
        let merkle_root = calculate_merkle_root(&transactions);
        assert_eq!(merkle_root.len(), 32);
        
        // Test with single transaction
        let single_tx = vec!["single_tx".as_bytes()];
        let single_root = calculate_merkle_root(&single_tx);
        assert_eq!(single_root, hash_sha256("single_tx".as_bytes()));
        
        // Test with empty transactions
        let empty_txs: Vec<&[u8]> = vec![];
        let empty_root = calculate_merkle_root(&empty_txs);
        assert_eq!(empty_root, [0u8; 32]); // Should return zero hash
    }

    #[test]
    fn test_base58_encoding() {
        let data = b"hello_world_base58";
        
        let encoded = encode_base58(data);
        let decoded = decode_base58(&encoded).unwrap();
        
        assert_eq!(decoded, data);
        
        // Test with address-like data
        let address_data = [0u8; 25]; // Typical Bitcoin address length
        let encoded_addr = encode_base58(&address_data);
        let decoded_addr = decode_base58(&encoded_addr).unwrap();
        
        assert_eq!(decoded_addr, address_data);
    }

    #[test]
    fn test_hex_encoding() {
        let data = b"hex_encoding_test";
        
        let hex_string = encode_hex(data);
        let decoded = decode_hex(&hex_string).unwrap();
        
        assert_eq!(decoded, data);
        
        // Test with hash data
        let hash = hash_sha256(b"test");
        let hex_hash = encode_hex(&hash);
        let decoded_hash = decode_hex(&hex_hash).unwrap();
        
        assert_eq!(decoded_hash, hash);
        assert_eq!(hex_hash.len(), 64); // 32 bytes * 2 hex chars
    }

    #[test]
    fn test_time_utilities() {
        let now = current_timestamp();
        assert!(now > 0);
        
        let now_millis = current_timestamp_millis();
        assert!(now_millis > now * 1000);
        
        let formatted = format_timestamp(now);
        assert!(formatted.contains("T")); // ISO 8601 format
        
        let parsed = parse_timestamp(&formatted).unwrap();
        assert_eq!(parsed, now);
    }

    #[test]
    fn test_validation_utilities() {
        // Test address validation
        assert!(is_valid_address("0x1234567890abcdef1234567890abcdef12345678"));
        assert!(!is_valid_address("invalid_address"));
        assert!(!is_valid_address("0x123")); // Too short
        
        // Test hash validation
        let valid_hash = "a".repeat(64); // 64 hex characters
        assert!(is_valid_hash(&valid_hash));
        assert!(!is_valid_hash("invalid_hash"));
        assert!(!is_valid_hash(&"a".repeat(63))); // Wrong length
        
        // Test transaction ID validation
        let valid_tx_id = format!("tx_{}", "b".repeat(60));
        assert!(is_valid_transaction_id(&valid_tx_id));
        assert!(!is_valid_transaction_id("invalid_tx_id"));
    }

    #[test]
    fn test_serialization_utilities() {
        use serde::{Deserialize, Serialize};
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            id: String,
            value: u64,
            data: Vec<u8>,
        }
        
        let test_obj = TestStruct {
            id: "test_123".to_string(),
            value: 42,
            data: vec![1, 2, 3, 4, 5],
        };
        
        // Test JSON serialization
        let json_bytes = serialize_to_json(&test_obj).unwrap();
        let deserialized_from_json: TestStruct = deserialize_from_json(&json_bytes).unwrap();
        assert_eq!(test_obj, deserialized_from_json);
        
        // Test binary serialization
        let binary_bytes = serialize_to_binary(&test_obj).unwrap();
        let deserialized_from_binary: TestStruct = deserialize_from_binary(&binary_bytes).unwrap();
        assert_eq!(test_obj, deserialized_from_binary);
        
        // Binary should be more compact than JSON
        assert!(binary_bytes.len() < json_bytes.len());
    }

    #[test]
    fn test_error_handling_utilities() {
        let result1: Result<i32> = Ok(42);
        let result2: Result<i32> = Err(anyhow::anyhow!("Test error"));
        
        assert!(result1.is_ok());
        assert!(result2.is_err());
        
        let chained_error = result2.context("Additional context");
        assert!(chained_error.is_err());
        
        let error_msg = format!("{}", chained_error.unwrap_err());
        assert!(error_msg.contains("Additional context"));
        assert!(error_msg.contains("Test error"));
    }

    #[test]
    fn test_crypto_utilities() {
        // Test key generation
        let (private_key, public_key) = generate_keypair();
        assert_eq!(private_key.len(), 32);
        assert_eq!(public_key.len(), 33); // Compressed public key
        
        // Test signing and verification
        let message = b"message_to_sign";
        let signature = sign_message(message, &private_key).unwrap();
        
        assert!(verify_signature(message, &signature, &public_key));
        assert!(!verify_signature(b"different_message", &signature, &public_key));
        
        // Test address derivation
        let address = derive_address(&public_key);
        assert!(is_valid_address(&address));
    }

    #[test]
    fn test_network_utilities() {
        let peer_id = generate_peer_id();
        assert_eq!(peer_id.len(), 40); // 20 bytes in hex
        
        let node_id = generate_node_id("192.168.1.100", 8080);
        assert!(node_id.contains("192.168.1.100"));
        assert!(node_id.contains("8080"));
        
        // Test network address validation
        assert!(is_valid_ip_address("192.168.1.1"));
        assert!(is_valid_ip_address("::1"));
        assert!(!is_valid_ip_address("invalid_ip"));
        
        assert!(is_valid_port(8080));
        assert!(is_valid_port(1));
        assert!(is_valid_port(65535));
        assert!(!is_valid_port(0));
        assert!(!is_valid_port(65536));
    }

    #[test]
    fn test_performance_utilities() {
        let timer = PerformanceTimer::start();
        
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);
        
        let elapsed_formatted = timer.elapsed_formatted();
        assert!(elapsed_formatted.contains("ms") || elapsed_formatted.contains("s"));
    }

    #[tokio::test]
    async fn test_async_utilities() {
        let result = retry_async(3, Duration::from_millis(100), || async {
            static mut ATTEMPT: u32 = 0;
            unsafe {
                ATTEMPT += 1;
                if ATTEMPT < 3 {
                    return Err(anyhow::anyhow!("Simulated failure"));
                }
                Ok("Success")
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut rate_limiter = RateLimiter::new(5, Duration::from_secs(1)); // 5 per second
        
        // Should allow first 5 requests
        for _ in 0..5 {
            assert!(rate_limiter.check_rate_limit("test_key").await);
        }
        
        // Should reject 6th request
        assert!(!rate_limiter.check_rate_limit("test_key").await);
        
        // Different key should be allowed
        assert!(rate_limiter.check_rate_limit("different_key").await);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::blockchain::Blockchain;
    use crate::consensus_poa::{ThaiAuthorityType, Authority};

    #[tokio::test]
    async fn test_full_system_integration() -> Result<()> {
        // Setup storage
        let storage = StorageManager::new_memory_storage().await?;
        
        // Setup blockchain
        let mut blockchain = Blockchain::new(storage.clone()).await?;
        
        // Create test authority
        let authority = Authority {
            address: "egat_validator_001".to_string(),
            public_key: vec![1, 2, 3, 4],
            authority_type: ThaiAuthorityType::EGAT,
            license_number: "EGAT-2024-001".to_string(),
            organization: "EGAT Testing".to_string(),
            stake_amount: 1_000_000,
            joined_at: Utc::now(),
            reputation_score: 95.0,
            total_blocks_validated: 0,
            last_block_time: None,
            region: "Central".to_string(),
            is_active: true,
            uptime_percentage: 100.0,
            governance_participation: 90.0,
            energy_capacity_mw: 1000.0,
            grid_connections: vec!["Test_Grid".to_string()],
        };
        
        // Create test transactions
        let mut transactions = Vec::new();
        for i in 0..10 {
            let tx = Transaction::new_energy_trade(
                format!("producer_{}", i),
                format!("consumer_{}", i),
                100.0 + i as f64,
                80.0 + i as f64,
                format!("{{\"trade_id\": \"trade_{}\"}}", i),
            )?;
            transactions.push(tx);
        }
        
        // Add transactions to blockchain
        for tx in transactions {
            blockchain.add_pending_transaction(tx).await?;
        }
        
        // Create and add blocks
        for block_num in 1..=5 {
            let pending = blockchain.get_pending_transactions().await;
            let validator_info = crate::blockchain::ValidatorInfo {
                address: authority.address.clone(),
                stake: authority.stake_amount,
                reputation: authority.reputation_score,
                authority_type: Some(format!("{:?}", authority.authority_type)),
            };
            
            let block = Block::new(
                blockchain.get_latest_block_hash().await?,
                pending.into_iter().take(2).collect(), // 2 transactions per block
                blockchain.get_height().await? + 1,
                validator_info,
            )?;
            
            blockchain.add_block(block).await?;
        }
        
        // Verify blockchain state
        assert_eq!(blockchain.get_height().await?, 5);
        assert!(blockchain.get_pending_transactions().await.is_empty());
        
        // Test storage operations
        let latest_block = blockchain.get_block(5).await?;
        assert!(latest_block.is_some());
        
        let block_data = latest_block.unwrap();
        assert_eq!(block_data.height, 5);
        assert_eq!(block_data.validator.address, authority.address);
        
        // Test storage persistence
        let storage_key = format!("block_{:06}", 5);
        let stored_block_data = storage.get(&storage_key).await?;
        assert!(stored_block_data.is_some());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_performance_under_load() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        let timer = PerformanceTimer::start();
        
        // Create large number of transactions
        let num_transactions = 1000;
        for i in 0..num_transactions {
            let tx = Transaction::new_energy_trade(
                format!("producer_{}", i % 100), // 100 unique producers
                format!("consumer_{}", i % 50),  // 50 unique consumers
                (100.0 + i as f64 % 500.0),
                (80.0 + i as f64 % 40.0),
                format!("{{\"batch_trade\": {}}}", i),
            )?;
            blockchain.add_pending_transaction(tx).await?;
        }
        
        let tx_creation_time = timer.elapsed();
        println!("Created {} transactions in {:?}", num_transactions, tx_creation_time);
        
        // Create blocks with transactions
        let authority = Authority {
            address: "load_test_validator".to_string(),
            public_key: vec![5, 6, 7, 8],
            authority_type: ThaiAuthorityType::EGAT,
            license_number: "LOAD-TEST-001".to_string(),
            organization: "Load Test Authority".to_string(),
            stake_amount: 5_000_000,
            joined_at: Utc::now(),
            reputation_score: 99.0,
            total_blocks_validated: 1000,
            last_block_time: Some(Utc::now()),
            region: "Test".to_string(),
            is_active: true,
            uptime_percentage: 99.9,
            governance_participation: 95.0,
            energy_capacity_mw: 5000.0,
            grid_connections: vec!["Test_Load_Grid".to_string()],
        };
        
        let block_timer = PerformanceTimer::start();
        let transactions_per_block = 50;
        let num_blocks = num_transactions / transactions_per_block;
        
        for _ in 0..num_blocks {
            let pending = blockchain.get_pending_transactions().await;
            let block_txs: Vec<_> = pending.into_iter().take(transactions_per_block).collect();
            
            let validator_info = crate::blockchain::ValidatorInfo {
                address: authority.address.clone(),
                stake: authority.stake_amount,
                reputation: authority.reputation_score,
                authority_type: Some(format!("{:?}", authority.authority_type)),
            };
            
            let block = Block::new(
                blockchain.get_latest_block_hash().await?,
                block_txs,
                blockchain.get_height().await? + 1,
                validator_info,
            )?;
            
            blockchain.add_block(block).await?;
        }
        
        let block_creation_time = block_timer.elapsed();
        println!("Created {} blocks in {:?}", num_blocks, block_creation_time);
        
        // Verify final state
        assert_eq!(blockchain.get_height().await?, num_blocks as u64);
        
        let total_time = timer.elapsed();
        println!("Total test time: {:?}", total_time);
        
        // Performance assertions
        assert!(tx_creation_time.as_millis() < 5000); // Should create 1000 txs in < 5s
        assert!(block_creation_time.as_millis() < 10000); // Should create blocks in < 10s
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_operations() -> Result<()> {
        use tokio::task::JoinSet;
        
        let storage = StorageManager::new_memory_storage().await?;
        let blockchain = std::sync::Arc::new(tokio::sync::Mutex::new(
            Blockchain::new(storage).await?
        ));
        
        let mut join_set = JoinSet::new();
        
        // Spawn multiple concurrent transaction creators
        for worker_id in 0..10 {
            let blockchain_clone = blockchain.clone();
            join_set.spawn(async move {
                let mut local_blockchain = blockchain_clone.lock().await;
                
                for i in 0..20 {
                    let tx = Transaction::new_energy_trade(
                        format!("producer_{}_{}", worker_id, i),
                        format!("consumer_{}_{}", worker_id, i),
                        100.0 + i as f64,
                        80.0 + worker_id as f64,
                        format!("{{\"concurrent_trade\": \"{}_{}\"}}", worker_id, i),
                    ).unwrap();
                    
                    local_blockchain.add_pending_transaction(tx).await.unwrap();
                }
                
                worker_id
            });
        }
        
        // Wait for all workers to complete
        let mut completed_workers = Vec::new();
        while let Some(result) = join_set.join_next().await {
            completed_workers.push(result.unwrap());
        }
        
        assert_eq!(completed_workers.len(), 10);
        
        // Verify all transactions were added
        let final_blockchain = blockchain.lock().await;
        let pending_count = final_blockchain.get_pending_transactions().await.len();
        assert_eq!(pending_count, 200); // 10 workers * 20 transactions each
        
        Ok(())
    }

    #[tokio::test]
    async fn test_system_recovery() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let db_path = temp_dir.path().to_str().unwrap();
        
        // Create blockchain and add some data
        {
            let storage = StorageManager::new_rocksdb_storage(db_path).await?;
            let mut blockchain = Blockchain::new(storage).await?;
            
            // Add some transactions and blocks
            for i in 0..5 {
                let tx = Transaction::new_energy_trade(
                    format!("recovery_producer_{}", i),
                    format!("recovery_consumer_{}", i),
                    100.0,
                    80.0,
                    format!("{{\"recovery_test\": {}}}", i),
                )?;
                blockchain.add_pending_transaction(tx).await?;
            }
            
            let authority = Authority {
                address: "recovery_validator".to_string(),
                public_key: vec![9, 10, 11, 12],
                authority_type: ThaiAuthorityType::EGAT,
                license_number: "RECOVERY-001".to_string(),
                organization: "Recovery Test".to_string(),
                stake_amount: 1_000_000,
                joined_at: Utc::now(),
                reputation_score: 95.0,
                total_blocks_validated: 0,
                last_block_time: None,
                region: "Test".to_string(),
                is_active: true,
                uptime_percentage: 100.0,
                governance_participation: 90.0,
                energy_capacity_mw: 1000.0,
                grid_connections: vec!["Recovery_Grid".to_string()],
            };
            
            let pending = blockchain.get_pending_transactions().await;
            let validator_info = crate::blockchain::ValidatorInfo {
                address: authority.address.clone(),
                stake: authority.stake_amount,
                reputation: authority.reputation_score,
                authority_type: Some(format!("{:?}", authority.authority_type)),
            };
            
            let block = Block::new(
                blockchain.get_latest_block_hash().await?,
                pending,
                1,
                validator_info,
            )?;
            
            blockchain.add_block(block).await?;
            
            assert_eq!(blockchain.get_height().await?, 1);
        } // blockchain dropped here, simulating shutdown
        
        // Recover blockchain from storage
        {
            let storage = StorageManager::new_rocksdb_storage(db_path).await?;
            let recovered_blockchain = Blockchain::new(storage).await?;
            
            // Verify data was recovered
            assert_eq!(recovered_blockchain.get_height().await?, 1);
            
            let recovered_block = recovered_blockchain.get_block(1).await?;
            assert!(recovered_block.is_some());
            
            let block_data = recovered_block.unwrap();
            assert_eq!(block_data.validator.address, "recovery_validator");
            assert_eq!(block_data.transactions.len(), 5);
        }
        
        Ok(())
    }
}
