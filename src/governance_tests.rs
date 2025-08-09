//! Comprehensive tests for governance and voting functionality

use crate::governance::{GovernanceSystem, Proposal, Vote, ProposalType, ProposalStatus};
use crate::blockchain::{Transaction, Block};
use crate::consensus_poa::{Authority, ThaiAuthorityType};
use chrono::{Utc, Duration};
use anyhow::Result;

#[cfg(test)]
mod governance_tests {
    use super::*;

    fn create_test_proposal() -> Proposal {
        Proposal {
            id: "prop_001".to_string(),
            title: "Increase Block Reward".to_string(),
            description: "Increase block reward from 100 to 150 tokens to incentivize validators".to_string(),
            proposer: "egat_authority_001".to_string(),
            proposal_type: ProposalType::ParameterChange,
            target_parameter: Some("block_reward".to_string()),
            new_value: Some("150".to_string()),
            current_value: Some("100".to_string()),
            created_at: Utc::now(),
            voting_starts_at: Utc::now(),
            voting_ends_at: Utc::now() + Duration::days(7),
            execution_delay: Duration::days(1),
            status: ProposalStatus::Active,
            required_quorum: 66.0, // 66% quorum required
            approval_threshold: 50.0, // 50% approval required
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            total_voting_power: 0.0,
            executed_at: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    fn create_test_vote(proposal_id: &str, voter: &str, support: bool) -> Vote {
        Vote {
            id: format!("vote_{}_{}", proposal_id, voter),
            proposal_id: proposal_id.to_string(),
            voter: voter.to_string(),
            support,
            voting_power: 100.0,
            reason: Some("Supporting network improvement".to_string()),
            cast_at: Utc::now(),
            transaction_hash: Some("tx_hash_123".to_string()),
        }
    }

    fn create_test_authority(name: &str, auth_type: ThaiAuthorityType) -> Authority {
        Authority {
            address: format!("{}_authority_001", name.to_lowercase()),
            public_key: vec![1, 2, 3, 4],
            authority_type: auth_type,
            license_number: format!("{}-2024-001", name.to_uppercase()),
            organization: format!("{} Organization", name),
            stake_amount: 1_000_000,
            joined_at: Utc::now(),
            reputation_score: 95.0,
            total_blocks_validated: 100,
            last_block_time: Some(Utc::now()),
            region: "Test Region".to_string(),
            is_active: true,
            uptime_percentage: 99.0,
            governance_participation: 85.0,
            energy_capacity_mw: 1000.0,
            grid_connections: vec!["Test_Grid".to_string()],
        }
    }

    #[tokio::test]
    async fn test_governance_system_creation() -> Result<()> {
        let config = crate::config::GovernanceConfig {
            proposal_deposit: 10_000,
            voting_period_days: 7,
            execution_delay_days: 1,
            quorum_threshold: 66.0,
            approval_threshold: 50.0,
            max_active_proposals: 10,
            enable_delegated_voting: true,
            min_voting_power: 1000.0,
        };

        let governance = GovernanceSystem::new(config).await?;
        assert_eq!(governance.get_active_proposals_count().await, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_proposal_creation() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register authority to propose
        let authority = create_test_authority("egat", ThaiAuthorityType::EGAT);
        governance.register_voter(authority).await?;

        let proposal = create_test_proposal();
        let proposal_id = proposal.id.clone();

        // Submit proposal
        governance.submit_proposal(proposal).await?;
        assert_eq!(governance.get_active_proposals_count().await, 1);

        // Retrieve proposal
        let retrieved = governance.get_proposal(&proposal_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Increase Block Reward");

        Ok(())
    }

    #[tokio::test]
    async fn test_proposal_validation() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register authority
        let authority = create_test_authority("egat", ThaiAuthorityType::EGAT);
        governance.register_voter(authority).await?;

        // Test invalid proposal (empty title)
        let mut invalid_proposal = create_test_proposal();
        invalid_proposal.title = String::new();
        
        assert!(governance.submit_proposal(invalid_proposal).await.is_err());

        // Test invalid proposal (voting ends before starts)
        let mut invalid_timing = create_test_proposal();
        invalid_timing.voting_ends_at = invalid_timing.voting_starts_at - Duration::hours(1);
        
        assert!(governance.submit_proposal(invalid_timing).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_voting_process() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register multiple authorities
        let authorities = vec![
            create_test_authority("egat", ThaiAuthorityType::EGAT),
            create_test_authority("mea", ThaiAuthorityType::MEA),
            create_test_authority("pea", ThaiAuthorityType::PEA),
            create_test_authority("erc", ThaiAuthorityType::ERC),
        ];

        for authority in authorities {
            governance.register_voter(authority).await?;
        }

        // Submit proposal
        let proposal = create_test_proposal();
        let proposal_id = proposal.id.clone();
        governance.submit_proposal(proposal).await?;

        // Cast votes
        let vote1 = create_test_vote(&proposal_id, "egat_authority_001", true);
        let vote2 = create_test_vote(&proposal_id, "mea_authority_001", true);
        let vote3 = create_test_vote(&proposal_id, "pea_authority_001", false);

        governance.cast_vote(vote1).await?;
        governance.cast_vote(vote2).await?;
        governance.cast_vote(vote3).await?;

        // Check proposal vote counts
        let updated_proposal = governance.get_proposal(&proposal_id).await?.unwrap();
        assert_eq!(updated_proposal.votes_for, 2);
        assert_eq!(updated_proposal.votes_against, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_voting_power_calculation() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register authorities with different voting power
        let mut egat = create_test_authority("egat", ThaiAuthorityType::EGAT);
        let mut mea = create_test_authority("mea", ThaiAuthorityType::MEA);

        egat.stake_amount = 2_000_000; // Higher stake
        egat.reputation_score = 98.0;  // Higher reputation
        mea.stake_amount = 1_000_000;
        mea.reputation_score = 85.0;

        governance.register_voter(egat.clone()).await?;
        governance.register_voter(mea.clone()).await?;

        let egat_power = governance.calculate_voting_power(&egat).await?;
        let mea_power = governance.calculate_voting_power(&mea).await?;

        // EGAT should have higher voting power
        assert!(egat_power > mea_power);
        assert!(egat_power > 0.0);
        assert!(mea_power > 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_quorum_requirements() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register 4 authorities
        for (i, auth_type) in [
            ThaiAuthorityType::EGAT,
            ThaiAuthorityType::MEA,
            ThaiAuthorityType::PEA,
            ThaiAuthorityType::ERC,
        ].iter().enumerate() {
            let authority = create_test_authority(&format!("auth{}", i), auth_type.clone());
            governance.register_voter(authority).await?;
        }

        // Submit proposal
        let proposal = create_test_proposal();
        let proposal_id = proposal.id.clone();
        governance.submit_proposal(proposal).await?;

        // Vote with only 2 out of 4 authorities (50% participation)
        let vote1 = create_test_vote(&proposal_id, "auth0_authority_001", true);
        let vote2 = create_test_vote(&proposal_id, "auth1_authority_001", true);

        governance.cast_vote(vote1).await?;
        governance.cast_vote(vote2).await?;

        // Should not meet 66% quorum requirement
        assert!(!governance.check_quorum_met(&proposal_id).await?);

        // Add one more vote to meet quorum
        let vote3 = create_test_vote(&proposal_id, "auth2_authority_001", false);
        governance.cast_vote(vote3).await?;

        // Should now meet quorum (75% participation)
        assert!(governance.check_quorum_met(&proposal_id).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_proposal_execution() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register authorities
        for i in 0..4 {
            let authority = create_test_authority(&format!("auth{}", i), ThaiAuthorityType::EGAT);
            governance.register_voter(authority).await?;
        }

        // Submit proposal
        let mut proposal = create_test_proposal();
        proposal.voting_ends_at = Utc::now() - Duration::hours(1); // Voting period ended
        let proposal_id = proposal.id.clone();
        governance.submit_proposal(proposal).await?;

        // Cast majority votes in favor
        for i in 0..3 {
            let vote = create_test_vote(&proposal_id, &format!("auth{}_authority_001", i), true);
            governance.cast_vote(vote).await?;
        }

        // Execute proposal
        governance.execute_proposal(&proposal_id).await?;

        let executed_proposal = governance.get_proposal(&proposal_id).await?.unwrap();
        assert_eq!(executed_proposal.status, ProposalStatus::Executed);
        assert!(executed_proposal.executed_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_proposal_rejection() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register authorities
        for i in 0..4 {
            let authority = create_test_authority(&format!("auth{}", i), ThaiAuthorityType::MEA);
            governance.register_voter(authority).await?;
        }

        // Submit proposal
        let mut proposal = create_test_proposal();
        proposal.voting_ends_at = Utc::now() - Duration::hours(1);
        let proposal_id = proposal.id.clone();
        governance.submit_proposal(proposal).await?;

        // Cast majority votes against
        for i in 0..3 {
            let vote = create_test_vote(&proposal_id, &format!("auth{}_authority_001", i), false);
            governance.cast_vote(vote).await?;
        }

        // Finalize proposal (should be rejected)
        governance.finalize_proposal(&proposal_id).await?;

        let rejected_proposal = governance.get_proposal(&proposal_id).await?.unwrap();
        assert_eq!(rejected_proposal.status, ProposalStatus::Rejected);

        Ok(())
    }

    #[tokio::test]
    async fn test_delegated_voting() -> Result<()> {
        let mut config = crate::config::GovernanceConfig::default();
        config.enable_delegated_voting = true;

        let mut governance = GovernanceSystem::new(config).await?;

        // Register authorities
        let delegator = create_test_authority("delegator", ThaiAuthorityType::PEA);
        let delegate = create_test_authority("delegate", ThaiAuthorityType::ERC);

        governance.register_voter(delegator.clone()).await?;
        governance.register_voter(delegate.clone()).await?;

        // Set up delegation
        governance.delegate_voting_power(
            &delegator.address,
            &delegate.address,
            50.0, // Delegate 50% of voting power
        ).await?;

        // Submit proposal
        let proposal = create_test_proposal();
        let proposal_id = proposal.id.clone();
        governance.submit_proposal(proposal).await?;

        // Delegate votes on behalf of delegator
        let vote = create_test_vote(&proposal_id, &delegate.address, true);
        governance.cast_delegated_vote(vote, &delegator.address).await?;

        // Check that delegated voting power was used
        let proposal_state = governance.get_proposal(&proposal_id).await?.unwrap();
        assert!(proposal_state.total_voting_power > 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_proposal_types() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        let authority = create_test_authority("egat", ThaiAuthorityType::EGAT);
        governance.register_voter(authority).await?;

        // Test different proposal types
        let proposals = vec![
            Proposal {
                id: "param_change".to_string(),
                title: "Parameter Change".to_string(),
                description: "Change block time".to_string(),
                proposer: "egat_authority_001".to_string(),
                proposal_type: ProposalType::ParameterChange,
                target_parameter: Some("block_time".to_string()),
                new_value: Some("12".to_string()),
                current_value: Some("15".to_string()),
                ..create_test_proposal()
            },
            Proposal {
                id: "authority_add".to_string(),
                title: "Add New Authority".to_string(),
                description: "Add new energy authority".to_string(),
                proposer: "egat_authority_001".to_string(),
                proposal_type: ProposalType::AuthorityManagement,
                target_parameter: Some("new_authority_address".to_string()),
                new_value: Some("new_auth_001".to_string()),
                current_value: None,
                ..create_test_proposal()
            },
            Proposal {
                id: "upgrade".to_string(),
                title: "Protocol Upgrade".to_string(),
                description: "Upgrade consensus mechanism".to_string(),
                proposer: "egat_authority_001".to_string(),
                proposal_type: ProposalType::ProtocolUpgrade,
                target_parameter: Some("consensus_version".to_string()),
                new_value: Some("2.0".to_string()),
                current_value: Some("1.0".to_string()),
                ..create_test_proposal()
            },
        ];

        for proposal in proposals {
            assert!(governance.submit_proposal(proposal).await.is_ok());
        }

        assert_eq!(governance.get_active_proposals_count().await, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_emergency_proposals() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register ERC (regulatory authority)
        let erc_authority = create_test_authority("erc", ThaiAuthorityType::ERC);
        governance.register_voter(erc_authority).await?;

        // Create emergency proposal
        let mut emergency_proposal = create_test_proposal();
        emergency_proposal.id = "emergency_001".to_string();
        emergency_proposal.title = "Emergency Grid Shutdown".to_string();
        emergency_proposal.description = "Emergency shutdown due to grid instability".to_string();
        emergency_proposal.proposal_type = ProposalType::Emergency;
        emergency_proposal.voting_ends_at = Utc::now() + Duration::hours(2); // Shorter voting period
        emergency_proposal.required_quorum = 51.0; // Lower quorum for emergency
        emergency_proposal.proposer = "erc_authority_001".to_string();

        // Emergency proposals should be accepted
        governance.submit_emergency_proposal(emergency_proposal).await?;

        assert_eq!(governance.get_active_proposals_count().await, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_governance_statistics() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        // Register authorities
        for i in 0..3 {
            let authority = create_test_authority(&format!("auth{}", i), ThaiAuthorityType::EGAT);
            governance.register_voter(authority).await?;
        }

        // Submit multiple proposals
        for i in 0..5 {
            let mut proposal = create_test_proposal();
            proposal.id = format!("prop_{:03}", i);
            proposal.title = format!("Proposal {}", i);
            governance.submit_proposal(proposal).await?;
        }

        // Cast some votes
        for i in 0..3 {
            let vote = create_test_vote("prop_000", &format!("auth{}_authority_001", i), true);
            governance.cast_vote(vote).await?;
        }

        let stats = governance.get_governance_statistics().await?;
        assert_eq!(stats.total_proposals, 5);
        assert_eq!(stats.active_proposals, 5);
        assert_eq!(stats.total_voters, 3);
        assert!(stats.total_votes > 0);
        assert!(stats.participation_rate > 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_voting_history() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        let authority = create_test_authority("egat", ThaiAuthorityType::EGAT);
        let voter_address = authority.address.clone();
        governance.register_voter(authority).await?;

        // Submit multiple proposals and vote
        for i in 0..3 {
            let mut proposal = create_test_proposal();
            proposal.id = format!("prop_{:03}", i);
            governance.submit_proposal(proposal.clone()).await?;

            let vote = create_test_vote(&proposal.id, &voter_address, i % 2 == 0);
            governance.cast_vote(vote).await?;
        }

        // Check voting history
        let voting_history = governance.get_voter_history(&voter_address).await?;
        assert_eq!(voting_history.len(), 3);

        // Check specific vote
        let first_vote = &voting_history[0];
        assert_eq!(first_vote.proposal_id, "prop_000");
        assert_eq!(first_vote.voter, voter_address);

        Ok(())
    }

    #[tokio::test]
    async fn test_proposal_cancellation() -> Result<()> {
        let config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(config).await?;

        let authority = create_test_authority("egat", ThaiAuthorityType::EGAT);
        governance.register_voter(authority).await?;

        // Submit proposal
        let proposal = create_test_proposal();
        let proposal_id = proposal.id.clone();
        governance.submit_proposal(proposal).await?;

        // Proposer should be able to cancel their own proposal
        governance.cancel_proposal(&proposal_id, "egat_authority_001").await?;

        let cancelled_proposal = governance.get_proposal(&proposal_id).await?.unwrap();
        assert_eq!(cancelled_proposal.status, ProposalStatus::Cancelled);

        Ok(())
    }
}

#[cfg(test)]
mod governance_integration_tests {
    use super::*;
    use crate::blockchain::Blockchain;
    use crate::storage::StorageManager;

    #[tokio::test]
    async fn test_governance_with_blockchain() -> Result<()> {
        // Create blockchain and governance system
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        let gov_config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(gov_config).await?;

        // Register authority
        let authority = create_test_authority("egat", ThaiAuthorityType::EGAT);
        governance.register_voter(authority.clone()).await?;

        // Submit governance proposal as transaction
        let proposal = create_test_proposal();
        let governance_tx = Transaction::new_governance_proposal(
            authority.address.clone(),
            proposal.title.clone(),
            proposal.description.clone(),
            serde_json::to_string(&proposal)?,
        )?;

        blockchain.add_pending_transaction(governance_tx).await?;

        // Create block with governance transaction
        let pending = blockchain.get_pending_transactions().await;
        let validator_info = crate::blockchain::ValidatorInfo {
            address: authority.address,
            stake: authority.stake_amount,
            reputation: authority.reputation_score,
            authority_type: Some(format!("{:?}", authority.authority_type)),
        };

        let block = Block::new(
            blockchain.get_latest_block_hash().await?,
            pending,
            blockchain.get_height().await? + 1,
            validator_info,
        )?;

        blockchain.add_block(block).await?;

        // Verify governance transaction was included
        assert!(blockchain.get_height().await? > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_multi_authority_governance() -> Result<()> {
        let gov_config = crate::config::GovernanceConfig::default();
        let mut governance = GovernanceSystem::new(gov_config).await?;

        // Register all Thai energy authorities
        let authorities = vec![
            ("egat", ThaiAuthorityType::EGAT),
            ("mea", ThaiAuthorityType::MEA),
            ("pea", ThaiAuthorityType::PEA),
            ("erc", ThaiAuthorityType::ERC),
        ];

        for (name, auth_type) in authorities {
            let authority = create_test_authority(name, auth_type);
            governance.register_voter(authority).await?;
        }

        // Submit proposal that affects all authorities
        let proposal = Proposal {
            id: "network_upgrade".to_string(),
            title: "National Grid Upgrade".to_string(),
            description: "Upgrade national grid infrastructure for better energy trading".to_string(),
            proposer: "erc_authority_001".to_string(), // ERC proposes
            proposal_type: ProposalType::ProtocolUpgrade,
            target_parameter: Some("grid_protocol".to_string()),
            new_value: Some("v2.0".to_string()),
            current_value: Some("v1.0".to_string()),
            ..create_test_proposal()
        };

        governance.submit_proposal(proposal.clone()).await?;

        // All authorities participate in voting
        let votes = vec![
            ("egat_authority_001", true),   // EGAT supports
            ("mea_authority_001", true),    // MEA supports
            ("pea_authority_001", false),   // PEA opposes
            ("erc_authority_001", true),    // ERC supports (proposer)
        ];

        for (voter, support) in votes {
            let vote = create_test_vote(&proposal.id, voter, support);
            governance.cast_vote(vote).await?;
        }

        // Check results - should pass with 3/4 support
        let final_proposal = governance.get_proposal(&proposal.id).await?.unwrap();
        assert_eq!(final_proposal.votes_for, 3);
        assert_eq!(final_proposal.votes_against, 1);

        // Should meet quorum and approval threshold
        assert!(governance.check_quorum_met(&proposal.id).await?);
        assert!(governance.check_approval_threshold_met(&proposal.id).await?);

        Ok(())
    }
}
