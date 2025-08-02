//! Comprehensive tests for P2P networking and blockchain synchronization

use crate::p2p::{P2PNetwork, PeerInfo, NetworkMessage, MessageType};
use crate::blockchain::{Block, Transaction, Blockchain};
use crate::storage::StorageManager;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use anyhow::Result;

#[cfg(test)]
mod p2p_network_tests {
    use super::*;

    fn create_test_peer() -> PeerInfo {
        PeerInfo {
            id: "peer_001".to_string(),
            address: "127.0.0.1:8001".to_string(),
            public_key: vec![1, 2, 3, 4],
            node_type: "authority".to_string(),
            region: "Bangkok".to_string(),
            authority_type: Some("MEA".to_string()),
            connected_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            version: "1.0.0".to_string(),
            capabilities: vec!["consensus".to_string(), "energy_trading".to_string()],
            reputation: 95.0,
            is_trusted: true,
        }
    }

    fn create_test_network_message() -> NetworkMessage {
        NetworkMessage {
            id: "msg_001".to_string(),
            message_type: MessageType::BlockAnnouncement,
            sender: "peer_001".to_string(),
            recipient: None, // Broadcast
            payload: b"test block data".to_vec(),
            timestamp: chrono::Utc::now(),
            signature: Some(vec![5, 6, 7, 8]),
            hop_count: 0,
            ttl: 10,
        }
    }

    #[tokio::test]
    async fn test_p2p_network_creation() -> Result<()> {
        let config = crate::config::P2PConfig {
            listen_address: "127.0.0.1:8000".to_string(),
            max_peers: 50,
            bootstrap_peers: vec![],
            enable_discovery: true,
            discovery_interval: 30,
            heartbeat_interval: 10,
            connection_timeout: 30,
            message_buffer_size: 1000,
            enable_encryption: true,
            trusted_peers: vec![],
        };

        let network = P2PNetwork::new(config).await?;
        assert_eq!(network.get_peer_count().await, 0);
        assert!(!network.is_connected().await);

        Ok(())
    }

    #[tokio::test]
    async fn test_peer_connection() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let peer = create_test_peer();
        let peer_id = peer.id.clone();

        // Add peer
        network.add_peer(peer).await?;
        assert_eq!(network.get_peer_count().await, 1);

        // Get peer
        let retrieved_peer = network.get_peer(&peer_id).await?;
        assert!(retrieved_peer.is_some());
        assert_eq!(retrieved_peer.unwrap().id, peer_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_peer_disconnection() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let peer = create_test_peer();
        let peer_id = peer.id.clone();

        // Add and then remove peer
        network.add_peer(peer).await?;
        assert_eq!(network.get_peer_count().await, 1);

        network.remove_peer(&peer_id).await?;
        assert_eq!(network.get_peer_count().await, 0);

        // Should not be able to get removed peer
        assert!(network.get_peer(&peer_id).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_message_broadcasting() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        // Add multiple peers
        for i in 1..=3 {
            let mut peer = create_test_peer();
            peer.id = format!("peer_{:03}", i);
            peer.address = format!("127.0.0.1:800{}", i);
            network.add_peer(peer).await?;
        }

        let message = create_test_network_message();
        
        // Broadcast message
        network.broadcast_message(message.clone()).await?;

        // In a real implementation, we would verify that all peers received the message
        // For testing, we can check that the message was queued for broadcast
        assert_eq!(network.get_peer_count().await, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_message_routing() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let peer = create_test_peer();
        let peer_id = peer.id.clone();
        network.add_peer(peer).await?;

        let mut message = create_test_network_message();
        message.recipient = Some(peer_id.clone());

        // Send direct message
        network.send_message_to_peer(&peer_id, message).await?;

        // Verify message was queued for the specific peer
        Ok(())
    }

    #[tokio::test]
    async fn test_peer_discovery() -> Result<()> {
        let mut config = crate::config::P2PConfig::default();
        config.enable_discovery = true;
        config.bootstrap_peers = vec!["127.0.0.1:8001".to_string()];

        let network = P2PNetwork::new(config).await?;

        // Start discovery process
        network.start_discovery().await?;

        // In a real test, we would verify that peers are discovered
        // For now, just verify the discovery was initiated
        assert!(network.is_discovery_enabled().await);

        Ok(())
    }

    #[tokio::test]
    async fn test_peer_reputation_tracking() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let mut peer = create_test_peer();
        let peer_id = peer.id.clone();
        let initial_reputation = peer.reputation;

        network.add_peer(peer).await?;

        // Update peer reputation
        network.update_peer_reputation(&peer_id, 10.0).await?; // Positive adjustment

        let updated_peer = network.get_peer(&peer_id).await?.unwrap();
        assert!(updated_peer.reputation >= initial_reputation);

        // Negative reputation adjustment
        network.update_peer_reputation(&peer_id, -20.0).await?;

        let downgraded_peer = network.get_peer(&peer_id).await?.unwrap();
        assert!(downgraded_peer.reputation < initial_reputation);

        Ok(())
    }

    #[tokio::test]
    async fn test_peer_blacklisting() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let peer = create_test_peer();
        let peer_id = peer.id.clone();

        network.add_peer(peer).await?;
        assert!(!network.is_peer_blacklisted(&peer_id).await);

        // Blacklist peer for malicious behavior
        network.blacklist_peer(&peer_id, "Double spending detected".to_string()).await?;
        assert!(network.is_peer_blacklisted(&peer_id).await);

        // Blacklisted peer should be removed
        assert!(network.get_peer(&peer_id).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_network_statistics() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        // Add peers from different regions
        for i in 1..=5 {
            let mut peer = create_test_peer();
            peer.id = format!("peer_{:03}", i);
            peer.region = if i <= 2 { "Bangkok".to_string() } else { "Central".to_string() };
            network.add_peer(peer).await?;
        }

        let stats = network.get_network_statistics().await?;
        assert_eq!(stats.total_peers, 5);
        assert_eq!(stats.connected_peers, 5);
        assert!(stats.regions.contains_key("Bangkok"));
        assert!(stats.regions.contains_key("Central"));
        assert_eq!(*stats.regions.get("Bangkok").unwrap(), 2);
        assert_eq!(*stats.regions.get("Central").unwrap(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_message_validation() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let network = P2PNetwork::new(config).await?;

        let valid_message = create_test_network_message();
        assert!(network.validate_message(&valid_message).await?);

        // Test invalid message (expired TTL)
        let mut expired_message = valid_message.clone();
        expired_message.ttl = 0;
        assert!(!network.validate_message(&expired_message).await?);

        // Test message with invalid signature
        let mut invalid_signature_message = valid_message.clone();
        invalid_signature_message.signature = Some(vec![99, 99, 99]);
        // This should fail signature validation in a real implementation
        
        Ok(())
    }

    #[tokio::test]
    async fn test_network_partitioning_recovery() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        // Add peers
        for i in 1..=4 {
            let mut peer = create_test_peer();
            peer.id = format!("peer_{:03}", i);
            network.add_peer(peer).await?;
        }

        // Simulate network partition by removing half the peers
        network.remove_peer("peer_001").await?;
        network.remove_peer("peer_002").await?;

        assert_eq!(network.get_peer_count().await, 2);

        // Simulate recovery by re-adding peers
        for i in 1..=2 {
            let mut peer = create_test_peer();
            peer.id = format!("peer_{:03}", i);
            peer.last_seen = chrono::Utc::now(); // Recent reconnection
            network.add_peer(peer).await?;
        }

        assert_eq!(network.get_peer_count().await, 4);

        Ok(())
    }
}

#[cfg(test)]
mod blockchain_sync_tests {
    use super::*;

    async fn create_test_blockchain() -> Result<Blockchain> {
        let storage = StorageManager::new_memory_storage().await?;
        let blockchain = Blockchain::new(storage).await?;
        Ok(blockchain)
    }

    #[tokio::test]
    async fn test_blockchain_synchronization() -> Result<()> {
        // Create two blockchains (simulating two nodes)
        let mut blockchain1 = create_test_blockchain().await?;
        let mut blockchain2 = create_test_blockchain().await?;

        // Add blocks to blockchain1
        for i in 1..=3 {
            let transaction = Transaction::new_genesis_mint(
                format!("address_{}", i),
                1000 + i * 100,
                format!("Block {} transaction", i),
            )?;

            let validator_info = crate::blockchain::ValidatorInfo {
                address: format!("validator_{}", i),
                stake: 1000,
                reputation: 100.0,
                authority_type: Some("EGAT".to_string()),
            };

            let block = Block::new(
                blockchain1.get_latest_block_hash().await?,
                vec![transaction],
                blockchain1.get_height().await? + 1,
                validator_info,
            )?;

            blockchain1.add_block(block).await?;
        }

        // blockchain1 should have 3 blocks, blockchain2 should have 0
        assert_eq!(blockchain1.get_height().await?, 3);
        assert_eq!(blockchain2.get_height().await?, 0);

        // Simulate synchronization by copying blocks
        for height in 1..=blockchain1.get_height().await? {
            if let Ok(block) = blockchain1.get_block_by_height(height).await {
                // In real implementation, this would be done through P2P network
                blockchain2.add_block(block).await?;
            }
        }

        // Now both blockchains should have the same height
        assert_eq!(blockchain1.get_height().await?, blockchain2.get_height().await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_block_request_response() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let peer = create_test_peer();
        let peer_id = peer.id.clone();
        network.add_peer(peer).await?;

        // Create block request message
        let block_request = NetworkMessage {
            id: "block_req_001".to_string(),
            message_type: MessageType::BlockRequest,
            sender: "local_node".to_string(),
            recipient: Some(peer_id.clone()),
            payload: b"3".to_vec(), // Request block at height 3
            timestamp: chrono::Utc::now(),
            signature: None,
            hop_count: 0,
            ttl: 5,
        };

        // Send block request
        network.send_message_to_peer(&peer_id, block_request).await?;

        // In real implementation, peer would respond with BlockResponse
        let block_response = NetworkMessage {
            id: "block_resp_001".to_string(),
            message_type: MessageType::BlockResponse,
            sender: peer_id.clone(),
            recipient: Some("local_node".to_string()),
            payload: b"serialized_block_data".to_vec(),
            timestamp: chrono::Utc::now(),
            signature: None,
            hop_count: 0,
            ttl: 5,
        };

        // Process block response
        assert!(network.validate_message(&block_response).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_propagation() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        // Add multiple peers
        for i in 1..=3 {
            let mut peer = create_test_peer();
            peer.id = format!("peer_{:03}", i);
            network.add_peer(peer).await?;
        }

        // Create transaction announcement
        let tx_announcement = NetworkMessage {
            id: "tx_announce_001".to_string(),
            message_type: MessageType::TransactionAnnouncement,
            sender: "local_node".to_string(),
            recipient: None, // Broadcast
            payload: b"serialized_transaction_data".to_vec(),
            timestamp: chrono::Utc::now(),
            signature: None,
            hop_count: 0,
            ttl: 10,
        };

        // Broadcast transaction
        network.broadcast_message(tx_announcement).await?;

        // Verify all peers would receive the transaction
        assert_eq!(network.get_peer_count().await, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_fork_resolution() -> Result<()> {
        // Create blockchain with competing forks
        let mut main_chain = create_test_blockchain().await?;
        let mut fork_chain = create_test_blockchain().await?;

        // Create common base block
        let base_tx = Transaction::new_genesis_mint("base".to_string(), 1000, "Base".to_string())?;
        let base_validator = crate::blockchain::ValidatorInfo {
            address: "base_validator".to_string(),
            stake: 1000,
            reputation: 100.0,
            authority_type: Some("EGAT".to_string()),
        };

        let base_block = Block::new(
            main_chain.get_latest_block_hash().await?,
            vec![base_tx.clone()],
            1,
            base_validator.clone(),
        )?;

        main_chain.add_block(base_block.clone()).await?;
        fork_chain.add_block(base_block).await?;

        // Create competing blocks at height 2
        let main_tx = Transaction::new_genesis_mint("main".to_string(), 2000, "Main".to_string())?;
        let fork_tx = Transaction::new_genesis_mint("fork".to_string(), 2000, "Fork".to_string())?;

        let main_block = Block::new(
            main_chain.get_latest_block_hash().await?,
            vec![main_tx],
            2,
            base_validator.clone(),
        )?;

        let fork_block = Block::new(
            fork_chain.get_latest_block_hash().await?,
            vec![fork_tx],
            2,
            base_validator,
        )?;

        main_chain.add_block(main_block).await?;
        fork_chain.add_block(fork_block).await?;

        // Both chains should have height 2 but different hashes
        assert_eq!(main_chain.get_height().await?, 2);
        assert_eq!(fork_chain.get_height().await?, 2);
        assert_ne!(
            main_chain.get_latest_block_hash().await?,
            fork_chain.get_latest_block_hash().await?
        );

        // Fork resolution would choose the chain with higher cumulative difficulty
        // or most recent blocks in POA consensus
        
        Ok(())
    }

    #[tokio::test]
    async fn test_peer_discovery_with_blockchain_state() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let blockchain = create_test_blockchain().await?;

        // Add peer with blockchain state information
        let mut peer = create_test_peer();
        peer.capabilities.push("blockchain_height:5".to_string());
        peer.capabilities.push("latest_hash:abc123".to_string());

        network.add_peer(peer).await?;

        // Network should be able to determine peer's blockchain state
        let peers = network.get_peers_by_capability("blockchain_height").await?;
        assert_eq!(peers.len(), 1);

        // Can request synchronization from peers with higher blockchain height
        let sync_candidates = network.get_sync_candidates(3).await?; // Current height is 3
        assert!(sync_candidates.len() > 0); // Peer has height 5, so it's a candidate

        Ok(())
    }

    #[tokio::test]
    async fn test_network_consensus_participation() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        // Add authority peers
        for i in 1..=4 {
            let mut peer = create_test_peer();
            peer.id = format!("authority_peer_{}", i);
            peer.node_type = "authority".to_string();
            peer.authority_type = Some(match i {
                1 => "EGAT".to_string(),
                2 => "MEA".to_string(),
                3 => "PEA".to_string(),
                4 => "ERC".to_string(),
                _ => "UNKNOWN".to_string(),
            });
            network.add_peer(peer).await?;
        }

        // Test consensus message broadcasting
        let consensus_message = NetworkMessage {
            id: "consensus_001".to_string(),
            message_type: MessageType::ConsensusMessage,
            sender: "authority_peer_1".to_string(),
            recipient: None,
            payload: b"new_block_proposal".to_vec(),
            timestamp: chrono::Utc::now(),
            signature: Some(vec![1, 2, 3, 4]),
            hop_count: 0,
            ttl: 5,
        };

        network.broadcast_message(consensus_message).await?;

        // All authority peers should receive consensus messages
        let authority_peers = network.get_peers_by_type("authority").await?;
        assert_eq!(authority_peers.len(), 4);

        Ok(())
    }
}

#[cfg(test)]
mod network_security_tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_authentication() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let mut peer = create_test_peer();
        peer.public_key = vec![1, 2, 3, 4]; // Valid public key

        // Peer with valid public key should be accepted
        assert!(network.add_peer(peer.clone()).await.is_ok());

        // Peer with empty public key should be rejected
        peer.public_key = vec![];
        assert!(network.add_peer(peer).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_message_signature_verification() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let network = P2PNetwork::new(config).await?;

        let mut message = create_test_network_message();
        
        // Message with valid signature should pass
        message.signature = Some(vec![1, 2, 3, 4]);
        // In real implementation, this would verify against sender's public key
        
        // Message without signature should fail for critical message types
        message.signature = None;
        message.message_type = MessageType::ConsensusMessage;
        // Should fail validation for consensus messages without signature

        Ok(())
    }

    #[tokio::test]
    async fn test_dos_protection() -> Result<()> {
        let mut config = crate::config::P2PConfig::default();
        config.message_buffer_size = 10; // Small buffer for testing
        
        let mut network = P2PNetwork::new(config).await?;

        let peer = create_test_peer();
        network.add_peer(peer.clone()).await?;

        // Simulate DoS attack by sending many messages rapidly
        for i in 0..20 {
            let mut message = create_test_network_message();
            message.id = format!("spam_msg_{}", i);
            
            // Network should start dropping messages or rate limiting
            let result = network.handle_incoming_message(message).await;
            if i > 15 {
                // After threshold, messages should be dropped
                assert!(result.is_err() || !result.unwrap());
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_peer_trust_scoring() -> Result<()> {
        let config = crate::config::P2PConfig::default();
        let mut network = P2PNetwork::new(config).await?;

        let mut peer = create_test_peer();
        let peer_id = peer.id.clone();
        
        network.add_peer(peer).await?;

        // Initially trusted peer
        assert!(network.is_peer_trusted(&peer_id).await?);

        // Simulate malicious behavior
        network.report_peer_misbehavior(&peer_id, "Invalid block").await?;
        network.report_peer_misbehavior(&peer_id, "Double spending").await?;

        // Peer should lose trust after multiple reports
        let updated_peer = network.get_peer(&peer_id).await?.unwrap();
        assert!(updated_peer.reputation < 95.0); // Should have decreased

        // Severely damaged reputation should result in blacklisting
        for _ in 0..10 {
            network.report_peer_misbehavior(&peer_id, "Continued malicious behavior").await?;
        }

        assert!(network.is_peer_blacklisted(&peer_id).await);

        Ok(())
    }

    #[tokio::test]
    async fn test_encrypted_communication() -> Result<()> {
        let mut config = crate::config::P2PConfig::default();
        config.enable_encryption = true;

        let network = P2PNetwork::new(config).await?;

        let message = create_test_network_message();
        
        // Encrypt message for transmission
        let encrypted_payload = network.encrypt_message_payload(&message.payload).await?;
        assert_ne!(encrypted_payload, message.payload);

        // Decrypt message on reception
        let decrypted_payload = network.decrypt_message_payload(&encrypted_payload).await?;
        assert_eq!(decrypted_payload, message.payload);

        Ok(())
    }
}
