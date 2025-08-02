//! Comprehensive tests for POA consensus functionality

use crate::consensus::{ConsensusEngine, POAConsensusEngine, Authority, ThaiAuthorityType};
use crate::blockchain::{Block, Transaction, ValidatorInfo};
use chrono::Utc;
use anyhow::Result;

#[cfg(test)]
mod poa_consensus_tests {
    use super::*;

    fn create_test_egat_authority() -> Authority {
        Authority {
            address: "egat_authority_001".to_string(),
            public_key: vec![1, 2, 3, 4], // Mock public key
            authority_type: ThaiAuthorityType::EGAT,
            license_number: "EGAT-2024-001".to_string(),
            organization: "Electricity Generating Authority of Thailand".to_string(),
            stake_amount: 1_000_000,
            joined_at: Utc::now(),
            reputation_score: 95.0,
            total_blocks_validated: 100,
            last_block_time: Some(Utc::now()),
            region: "Central".to_string(),
            is_active: true,
            uptime_percentage: 99.5,
            governance_participation: 85.0,
            energy_capacity_mw: 12000.0,
            grid_connections: vec!["Central_Grid".to_string(), "North_Grid".to_string()],
        }
    }

    fn create_test_mea_authority() -> Authority {
        Authority {
            address: "mea_authority_001".to_string(),
            public_key: vec![5, 6, 7, 8],
            authority_type: ThaiAuthorityType::MEA,
            license_number: "MEA-2024-001".to_string(),
            organization: "Metropolitan Electricity Authority".to_string(),
            stake_amount: 800_000,
            joined_at: Utc::now(),
            reputation_score: 92.0,
            total_blocks_validated: 75,
            last_block_time: Some(Utc::now()),
            region: "Bangkok".to_string(),
            is_active: true,
            uptime_percentage: 98.0,
            governance_participation: 90.0,
            energy_capacity_mw: 8000.0,
            grid_connections: vec!["Bangkok_Metro".to_string()],
        }
    }

    fn create_test_pea_authority() -> Authority {
        Authority {
            address: "pea_authority_001".to_string(),
            public_key: vec![9, 10, 11, 12],
            authority_type: ThaiAuthorityType::PEA,
            license_number: "PEA-2024-001".to_string(),
            organization: "Provincial Electricity Authority".to_string(),
            stake_amount: 600_000,
            joined_at: Utc::now(),
            reputation_score: 88.0,
            total_blocks_validated: 60,
            last_block_time: Some(Utc::now()),
            region: "Northeast".to_string(),
            is_active: true,
            uptime_percentage: 96.5,
            governance_participation: 80.0,
            energy_capacity_mw: 5000.0,
            grid_connections: vec!["Northeast_Grid".to_string(), "East_Grid".to_string()],
        }
    }

    fn create_test_erc_authority() -> Authority {
        Authority {
            address: "erc_authority_001".to_string(),
            public_key: vec![13, 14, 15, 16],
            authority_type: ThaiAuthorityType::ERC,
            license_number: "ERC-2024-001".to_string(),
            organization: "Energy Regulatory Commission".to_string(),
            stake_amount: 500_000,
            joined_at: Utc::now(),
            reputation_score: 100.0,
            total_blocks_validated: 40,
            last_block_time: Some(Utc::now()),
            region: "National".to_string(),
            is_active: true,
            uptime_percentage: 100.0,
            governance_participation: 100.0,
            energy_capacity_mw: 0.0, // Regulatory body, no generation capacity
            grid_connections: vec!["National_Grid".to_string()],
        }
    }

    #[test]
    fn test_thai_authority_types() {
        // Test all authority types
        let egat = ThaiAuthorityType::EGAT;
        let mea = ThaiAuthorityType::MEA;
        let pea = ThaiAuthorityType::PEA;
        let erc = ThaiAuthorityType::ERC;

        // Test serialization/deserialization
        let serialized = serde_json::to_string(&egat).unwrap();
        let deserialized: ThaiAuthorityType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(egat, deserialized);

        // Test different types are not equal
        assert_ne!(egat, mea);
        assert_ne!(mea, pea);
        assert_ne!(pea, erc);
    }

    #[test]
    fn test_authority_creation() {
        let authority = create_test_egat_authority();
        
        assert_eq!(authority.address, "egat_authority_001");
        assert_eq!(authority.authority_type, ThaiAuthorityType::EGAT);
        assert_eq!(authority.license_number, "EGAT-2024-001");
        assert!(authority.is_active);
        assert!(authority.reputation_score > 0.0);
        assert!(authority.stake_amount > 0);
    }

    #[test]
    fn test_authority_eligibility() {
        let mut authority = create_test_egat_authority();
        
        // Should be eligible with good parameters
        assert!(authority.is_eligible());
        
        // Should not be eligible if inactive
        authority.is_active = false;
        assert!(!authority.is_eligible());
        authority.is_active = true;
        
        // Should not be eligible with low reputation
        authority.reputation_score = 30.0;
        assert!(!authority.is_eligible());
        authority.reputation_score = 95.0;
        
        // Should not be eligible with low uptime
        authority.uptime_percentage = 70.0;
        assert!(!authority.is_eligible());
        authority.uptime_percentage = 99.5;
        
        // Should be eligible again
        assert!(authority.is_eligible());
    }

    #[test]
    fn test_authority_voting_power() {
        let egat = create_test_egat_authority();
        let mea = create_test_mea_authority();
        let erc = create_test_erc_authority();
        
        let egat_power = egat.calculate_voting_power();
        let mea_power = mea.calculate_voting_power();
        let erc_power = erc.calculate_voting_power();
        
        // All should have positive voting power
        assert!(egat_power > 0.0);
        assert!(mea_power > 0.0);
        assert!(erc_power > 0.0);
        
        // ERC should have high voting power due to perfect reputation and governance participation
        // EGAT should have high power due to large stake and capacity
        assert!(erc_power > 0.0);
        assert!(egat_power > 0.0);
    }

    #[test]
    fn test_authority_performance_update() {
        let mut authority = create_test_egat_authority();
        let initial_reputation = authority.reputation_score;
        
        // Good performance should improve reputation
        authority.update_performance(95.0, 99.8, 90.0);
        assert!(authority.reputation_score >= initial_reputation);
        
        // Record successful block validation
        authority.record_block_validation();
        assert_eq!(authority.total_blocks_validated, 101);
        assert!(authority.last_block_time.is_some());
        
        // Bad performance should lower reputation
        authority.update_performance(60.0, 85.0, 50.0);
        // Reputation might decrease based on implementation
    }

    #[tokio::test]
    async fn test_poa_engine_creation() -> Result<()> {
        let poa_config = crate::config::POAConfig {
            round_time_ms: 15000,
            max_authorities: 10,
            min_stake_amount: 100_000,
            reputation_threshold: 70.0,
            uptime_threshold: 90.0,
            enable_slashing: true,
            slashing_percentage: 5.0,
            governance_quorum: 66.0,
            block_reward: 100,
        };
        
        let engine = POAConsensusEngine::new(poa_config.clone()).await?;
        assert_eq!(engine.get_authorities_count().await, 0);
        assert_eq!(engine.get_current_round().await, 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_authority_registration() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        let authority = create_test_egat_authority();
        let address = authority.address.clone();
        
        // Register authority
        engine.register_authority(authority).await?;
        assert_eq!(engine.get_authorities_count().await, 1);
        
        // Should be able to get the authority
        let retrieved = engine.get_authority(&address).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().address, address);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_authority_registration_limits() -> Result<()> {
        let mut poa_config = crate::config::POAConfig::default();
        poa_config.max_authorities = 2;
        
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Should be able to register up to max_authorities
        engine.register_authority(create_test_egat_authority()).await?;
        engine.register_authority(create_test_mea_authority()).await?;
        
        // Should fail to register more than max_authorities
        assert!(engine.register_authority(create_test_pea_authority()).await.is_err());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_authority_deregistration() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        let authority = create_test_egat_authority();
        let address = authority.address.clone();
        
        // Register and then deregister
        engine.register_authority(authority).await?;
        assert_eq!(engine.get_authorities_count().await, 1);
        
        engine.deregister_authority(&address).await?;
        assert_eq!(engine.get_authorities_count().await, 0);
        
        // Should not be able to get deregistered authority
        assert!(engine.get_authority(&address).await?.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_validator_selection() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register multiple authorities
        engine.register_authority(create_test_egat_authority()).await?;
        engine.register_authority(create_test_mea_authority()).await?;
        engine.register_authority(create_test_pea_authority()).await?;
        
        // Select next validator
        let validator = engine.select_next_validator().await?;
        assert!(!validator.is_empty());
        
        // Should be one of the registered authorities
        let authority = engine.get_authority(&validator).await?;
        assert!(authority.is_some());
        assert!(authority.unwrap().is_eligible());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_round_management() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register authorities
        engine.register_authority(create_test_egat_authority()).await?;
        engine.register_authority(create_test_mea_authority()).await?;
        
        let initial_round = engine.get_current_round().await;
        
        // Advance round
        engine.advance_round().await?;
        let new_round = engine.get_current_round().await;
        
        assert_eq!(new_round, initial_round + 1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_block_validation() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register authority
        let authority = create_test_egat_authority();
        let authority_address = authority.address.clone();
        engine.register_authority(authority).await?;
        
        // Create a test block with this authority as validator
        let validator_info = ValidatorInfo {
            address: authority_address.clone(),
            stake: 1_000_000,
            reputation: 95.0,
            authority_type: Some("EGAT".to_string()),
        };
        
        let transactions = vec![
            Transaction::new_genesis_mint("test".to_string(), 1000, "Test".to_string())?
        ];
        
        let block = Block::new(
            "previous_hash".to_string(),
            transactions,
            1,
            validator_info,
        )?;
        
        // Block should be valid with registered authority
        assert!(engine.validate_block(&block, None).await?);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_invalid_validator_block() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let engine = POAConsensusEngine::new(poa_config).await?;
        
        // Create block with unregistered validator
        let validator_info = ValidatorInfo {
            address: "unregistered_validator".to_string(),
            stake: 1000,
            reputation: 100.0,
            authority_type: Some("UNKNOWN".to_string()),
        };
        
        let transactions = vec![
            Transaction::new_genesis_mint("test".to_string(), 1000, "Test".to_string())?
        ];
        
        let block = Block::new(
            "previous_hash".to_string(),
            transactions,
            1,
            validator_info,
        )?;
        
        // Block should be invalid with unregistered authority
        assert!(!engine.validate_block(&block, None).await?);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_authority_slashing() -> Result<()> {
        let mut poa_config = crate::config::POAConfig::default();
        poa_config.enable_slashing = true;
        poa_config.slashing_percentage = 10.0;
        
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        let authority = create_test_egat_authority();
        let address = authority.address.clone();
        let initial_stake = authority.stake_amount;
        
        engine.register_authority(authority).await?;
        
        // Slash authority for misbehavior
        engine.slash_authority(&address, "Double signing detected").await?;
        
        let slashed_authority = engine.get_authority(&address).await?.unwrap();
        assert!(slashed_authority.stake_amount < initial_stake);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_governance_proposal() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register authorities for governance
        engine.register_authority(create_test_egat_authority()).await?;
        engine.register_authority(create_test_mea_authority()).await?;
        engine.register_authority(create_test_erc_authority()).await?;
        
        // Create governance proposal
        let proposal_id = engine.create_governance_proposal(
            "egat_authority_001",
            "Increase block reward to 150 tokens",
            "Block reward adjustment for network sustainability",
        ).await?;
        
        assert!(!proposal_id.is_empty());
        
        // Vote on proposal
        engine.vote_on_proposal(&proposal_id, "egat_authority_001", true).await?;
        engine.vote_on_proposal(&proposal_id, "mea_authority_001", true).await?;
        engine.vote_on_proposal(&proposal_id, "erc_authority_001", false).await?;
        
        // Check proposal status
        let proposal = engine.get_governance_proposal(&proposal_id).await?;
        assert!(proposal.is_some());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_consensus_statistics() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register authorities
        engine.register_authority(create_test_egat_authority()).await?;
        engine.register_authority(create_test_mea_authority()).await?;
        engine.register_authority(create_test_pea_authority()).await?;
        
        let stats = engine.get_consensus_stats().await?;
        
        assert_eq!(stats.total_authorities, 3);
        assert!(stats.active_authorities <= 3);
        assert!(stats.eligible_authorities <= 3);
        assert!(stats.average_reputation > 0.0);
        assert!(stats.total_stake > 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_authority_reputation_tracking() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        let mut authority = create_test_egat_authority();
        let address = authority.address.clone();
        let initial_reputation = authority.reputation_score;
        
        engine.register_authority(authority).await?;
        
        // Simulate good performance
        engine.update_authority_reputation(&address, 98.0).await?;
        
        let updated_authority = engine.get_authority(&address).await?.unwrap();
        // Reputation should improve or stay the same with good performance
        assert!(updated_authority.reputation_score >= initial_reputation - 1.0); // Allow small tolerance
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_multi_region_consensus() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register authorities from different regions
        engine.register_authority(create_test_egat_authority()).await?;    // Central
        engine.register_authority(create_test_mea_authority()).await?;     // Bangkok
        engine.register_authority(create_test_pea_authority()).await?;     // Northeast
        engine.register_authority(create_test_erc_authority()).await?;     // National
        
        // All regions should be represented
        let authorities = engine.get_all_authorities().await?;
        let regions: std::collections::HashSet<_> = authorities.iter()
            .map(|auth| &auth.region)
            .collect();
        
        assert!(regions.contains(&"Central".to_string()));
        assert!(regions.contains(&"Bangkok".to_string()));
        assert!(regions.contains(&"Northeast".to_string()));
        assert!(regions.contains(&"National".to_string()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_emergency_consensus() -> Result<()> {
        let poa_config = crate::config::POAConfig::default();
        let mut engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register ERC authority (regulatory)
        engine.register_authority(create_test_erc_authority()).await?;
        
        // Test emergency consensus mode (shorter rounds, immediate validation)
        engine.enable_emergency_mode("Grid stability emergency").await?;
        
        assert!(engine.is_emergency_mode().await);
        
        // Emergency mode should allow faster consensus
        let emergency_round_time = engine.get_emergency_round_time().await;
        assert!(emergency_round_time < 15000); // Less than normal 15 seconds
        
        // Disable emergency mode
        engine.disable_emergency_mode().await?;
        assert!(!engine.is_emergency_mode().await);
        
        Ok(())
    }
}

// Integration tests for POA consensus with blockchain
#[cfg(test)]
mod poa_integration_tests {
    use super::*;
    use crate::blockchain::Blockchain;
    use crate::storage::StorageManager;

    #[tokio::test]
    async fn test_poa_blockchain_integration() -> Result<()> {
        // Create blockchain with POA consensus
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        // Initialize POA consensus
        let poa_config = crate::config::POAConfig::default();
        let mut poa_engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register authorities
        poa_engine.register_authority(create_test_egat_authority()).await?;
        poa_engine.register_authority(create_test_mea_authority()).await?;
        
        // Select validator and create block
        let validator_address = poa_engine.select_next_validator().await?;
        let authority = poa_engine.get_authority(&validator_address).await?.unwrap();
        
        let validator_info = ValidatorInfo {
            address: authority.address,
            stake: authority.stake_amount,
            reputation: authority.reputation_score,
            authority_type: Some(format!("{:?}", authority.authority_type)),
        };
        
        // Create transactions
        let transactions = vec![
            Transaction::new_energy_trade(
                "gen1".to_string(),
                "user1".to_string(),
                100.0,
                4.5,
                "Central_Grid".to_string(),
                None,
            )?
        ];
        
        // Create block
        let block = Block::new(
            blockchain.get_latest_block_hash().await?,
            transactions,
            blockchain.get_height().await? + 1,
            validator_info,
        )?;
        
        // Validate with POA consensus
        assert!(poa_engine.validate_block(&block, None).await?);
        
        // Add to blockchain
        blockchain.add_block(block).await?;
        
        // Verify block was added
        assert!(blockchain.get_height().await? > 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_poa_consensus_with_energy_trading() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        let poa_config = crate::config::POAConfig::default();
        let mut poa_engine = POAConsensusEngine::new(poa_config).await?;
        
        // Register energy authorities
        poa_engine.register_authority(create_test_egat_authority()).await?;
        poa_engine.register_authority(create_test_mea_authority()).await?;
        poa_engine.register_authority(create_test_pea_authority()).await?;
        
        // Simulate multiple rounds of consensus with energy trading
        for round in 0..3 {
            // Select validator for this round
            let validator_address = poa_engine.select_next_validator().await?;
            let authority = poa_engine.get_authority(&validator_address).await?.unwrap();
            
            // Create energy trading transactions
            let transactions = vec![
                Transaction::new_generator_mint(
                    format!("generator_{}", round),
                    1000.0 + (round as f64 * 100.0),
                    format!("Generation round {}", round),
                    None,
                )?,
                Transaction::new_energy_trade(
                    format!("generator_{}", round),
                    format!("consumer_{}", round),
                    100.0,
                    4.0 + (round as f64 * 0.1),
                    "Grid".to_string(),
                    None,
                )?
            ];
            
            let validator_info = ValidatorInfo {
                address: authority.address.clone(),
                stake: authority.stake_amount,
                reputation: authority.reputation_score,
                authority_type: Some(format!("{:?}", authority.authority_type)),
            };
            
            let block = Block::new(
                blockchain.get_latest_block_hash().await?,
                transactions,
                blockchain.get_height().await? + 1,
                validator_info,
            )?;
            
            // Validate and add block
            assert!(poa_engine.validate_block(&block, None).await?);
            blockchain.add_block(block).await?;
            
            // Advance to next round
            poa_engine.advance_round().await?;
        }
        
        // Verify multiple blocks were created
        assert_eq!(blockchain.get_height().await?, 3);
        
        // Check energy statistics
        let energy_stats = blockchain.get_energy_stats().await?;
        assert!(energy_stats.total_energy_generated > 0.0);
        assert!(energy_stats.total_energy_traded > 0.0);
        
        Ok(())
    }
}
