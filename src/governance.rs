//! GridTokenX Governance Module
//!
//! This module implements the governance system for the GridTokenX blockchain,
//! including proposal creation, voting mechanisms, and execution of governance decisions.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blockchain::{Blockchain, Transaction};

/// Governance system manager
#[derive(Debug)]
pub struct GovernanceSystem {
    blockchain: Arc<RwLock<Blockchain>>,
    proposals: RwLock<HashMap<String, Proposal>>,
    voting_power: RwLock<HashMap<String, VotingPower>>,
    governance_config: GovernanceConfig,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub proposal_type: ProposalType,
    pub created_at: DateTime<Utc>,
    pub voting_start: DateTime<Utc>,
    pub voting_end: DateTime<Utc>,
    pub execution_delay: u64, // seconds
    pub status: ProposalStatus,
    pub votes: HashMap<String, Vote>,
    pub execution_data: Vec<u8>,
    pub quorum_required: f64,
    pub approval_threshold: f64,
}

/// Types of governance proposals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// Change network parameters
    ParameterChange {
        parameter: String,
        current_value: String,
        new_value: String,
    },
    /// Upgrade smart contracts
    ContractUpgrade {
        contract_address: String,
        new_bytecode: Vec<u8>,
    },
    /// Add or remove energy authorities
    AuthorityManagement {
        action: AuthorityAction,
        authority_address: String,
        authority_type: String,
    },
    /// Emergency response measures
    Emergency {
        emergency_type: EmergencyType,
        response_actions: Vec<String>,
        duration_hours: u64,
    },
    /// Energy pricing regulations
    PricingRegulation {
        min_price: u64,
        max_price: u64,
        peak_hour_multiplier: f64,
        effective_date: DateTime<Utc>,
    },
    /// Grid infrastructure funding
    InfrastructureFunding {
        project_name: String,
        funding_amount: u64,
        recipient: String,
        milestones: Vec<String>,
    },
    /// Carbon credit policies
    CarbonCreditPolicy {
        policy_type: String,
        credit_rates: HashMap<String, f64>,
        verification_requirements: Vec<String>,
    },
}

/// Authority management actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityAction {
    Add,
    Remove,
    Suspend,
    Reinstate,
    UpdatePermissions,
}

/// Emergency types requiring governance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyType {
    GridFailure,
    CyberAttack,
    NaturalDisaster,
    MarketManipulation,
    RegulatoryViolation,
    SystemOutage,
}

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
    Cancelled,
}

/// Individual vote on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub choice: VoteChoice,
    pub voting_power: u64,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

/// Vote choices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// Voting power information for an address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingPower {
    pub address: String,
    pub power: u64,
    pub source: VotingPowerSource,
    pub delegation: Option<Delegation>,
    pub last_updated: DateTime<Utc>,
}

/// Sources of voting power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VotingPowerSource {
    TokenStake(u64),
    EnergyStake(f64),
    AuthorityStatus(String),
    Delegation(u64),
}

/// Vote delegation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub delegate_to: String,
    pub delegated_power: u64,
    pub delegation_start: DateTime<Utc>,
    pub delegation_end: Option<DateTime<Utc>>,
}

/// Governance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub voting_period_days: u32,
    pub execution_delay_days: u32,
    pub quorum_threshold: f64,
    pub approval_threshold: f64,
    pub min_stake_to_propose: u64,
    pub proposal_fee: u64,
    pub max_proposals_per_address: u32,
}

/// Governance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceMetrics {
    pub total_proposals: u64,
    pub active_proposals: u64,
    pub executed_proposals: u64,
    pub total_voters: u64,
    pub average_turnout: f64,
    pub total_voting_power: u64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            voting_period_days: 7,
            execution_delay_days: 2,
            quorum_threshold: 0.5,           // 50% quorum
            approval_threshold: 0.6,         // 60% approval
            min_stake_to_propose: 1_000_000, // 1M tokens
            proposal_fee: 10_000,            // 10k tokens
            max_proposals_per_address: 5,
        }
    }
}

impl GovernanceSystem {
    /// Create new governance system
    pub async fn new(blockchain: Arc<RwLock<Blockchain>>) -> Result<Self> {
        Ok(Self {
            blockchain,
            proposals: RwLock::new(HashMap::new()),
            voting_power: RwLock::new(HashMap::new()),
            governance_config: GovernanceConfig::default(),
        })
    }

    /// Start governance loop
    pub async fn start_governance_loop(&self) -> Result<()> {
        tracing::info!("Starting governance system");

        loop {
            self.process_proposals().await?;
            self.update_voting_power().await?;
            self.execute_passed_proposals().await?;

            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }

    /// Submit a new proposal
    pub async fn submit_proposal(
        &self,
        title: String,
        description: String,
        proposer: String,
        proposal_type: ProposalType,
    ) -> Result<String> {
        // Validate proposer has enough stake
        let voting_power = self.get_voting_power(&proposer).await;
        if voting_power < self.governance_config.min_stake_to_propose {
            return Err(anyhow!(
                "Insufficient stake to propose. Required: {}, Available: {}",
                self.governance_config.min_stake_to_propose,
                voting_power
            ));
        }

        // Check proposal limits
        let proposals = self.proposals.read().await;
        let proposer_proposals = proposals
            .values()
            .filter(|p| p.proposer == proposer && p.status == ProposalStatus::Active)
            .count();

        if proposer_proposals >= self.governance_config.max_proposals_per_address as usize {
            return Err(anyhow!(
                "Maximum active proposals per address exceeded: {}",
                self.governance_config.max_proposals_per_address
            ));
        }
        drop(proposals);

        let proposal_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let voting_start = now + chrono::Duration::hours(24); // 24 hour delay
        let voting_end =
            voting_start + chrono::Duration::days(self.governance_config.voting_period_days as i64);

        let proposal = Proposal {
            id: proposal_id.clone(),
            title,
            description,
            proposer,
            proposal_type,
            created_at: now,
            voting_start,
            voting_end,
            execution_delay: self.governance_config.execution_delay_days as u64 * 24 * 3600,
            status: ProposalStatus::Active,
            votes: HashMap::new(),
            execution_data: Vec::new(),
            quorum_required: self.governance_config.quorum_threshold,
            approval_threshold: self.governance_config.approval_threshold,
        };

        let mut proposals = self.proposals.write().await;
        proposals.insert(proposal_id.clone(), proposal);

        tracing::info!("New proposal submitted: {}", proposal_id);
        Ok(proposal_id)
    }

    /// Vote on a proposal
    pub async fn vote(
        &self,
        proposal_id: String,
        voter: String,
        choice: VoteChoice,
        reason: Option<String>,
    ) -> Result<()> {
        let mut proposals = self.proposals.write().await;
        let proposal = proposals
            .get_mut(&proposal_id)
            .ok_or_else(|| anyhow!("Proposal not found: {}", proposal_id))?;

        // Check if proposal is in voting period
        let now = Utc::now();
        if now < proposal.voting_start || now > proposal.voting_end {
            return Err(anyhow!("Proposal not in voting period"));
        }

        if proposal.status != ProposalStatus::Active {
            return Err(anyhow!("Proposal is not active"));
        }

        // Get voter's voting power
        let voting_power = self.get_voting_power(&voter).await;
        if voting_power == 0 {
            return Err(anyhow!("No voting power available"));
        }

        // Record vote
        let vote = Vote {
            voter: voter.clone(),
            choice,
            voting_power,
            reason,
            timestamp: now,
            signature: String::new(), // Would implement actual signature
        };

        proposal.votes.insert(voter, vote);

        tracing::info!("Vote recorded for proposal {}: {:?}", proposal_id, choice);
        Ok(())
    }

    /// Process all proposals (check for completion, execution, etc.)
    async fn process_proposals(&self) -> Result<()> {
        let mut proposals = self.proposals.write().await;
        let now = Utc::now();

        for proposal in proposals.values_mut() {
            if proposal.status == ProposalStatus::Active && now > proposal.voting_end {
                // Calculate results
                let (total_votes, yes_votes) = self.calculate_vote_results(proposal);
                let total_voting_power = self.get_total_voting_power().await;

                let quorum = total_votes as f64 / total_voting_power as f64;
                let approval = if total_votes > 0 {
                    yes_votes as f64 / total_votes as f64
                } else {
                    0.0
                };

                if quorum >= proposal.quorum_required && approval >= proposal.approval_threshold {
                    proposal.status = ProposalStatus::Passed;
                    tracing::info!("Proposal {} passed", proposal.id);
                } else {
                    proposal.status = ProposalStatus::Rejected;
                    tracing::info!("Proposal {} rejected", proposal.id);
                }
            }
        }

        Ok(())
    }

    /// Execute passed proposals
    async fn execute_passed_proposals(&self) -> Result<()> {
        let mut proposals = self.proposals.write().await;
        let now = Utc::now();

        for proposal in proposals.values_mut() {
            if proposal.status == ProposalStatus::Passed {
                let execution_time = proposal.voting_end
                    + chrono::Duration::seconds(proposal.execution_delay as i64);

                if now >= execution_time {
                    match self.execute_proposal(proposal).await {
                        Ok(()) => {
                            proposal.status = ProposalStatus::Executed;
                            tracing::info!("Proposal {} executed successfully", proposal.id);
                        }
                        Err(e) => {
                            tracing::error!("Failed to execute proposal {}: {}", proposal.id, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Execute a specific proposal
    async fn execute_proposal(&self, proposal: &Proposal) -> Result<()> {
        match &proposal.proposal_type {
            ProposalType::ParameterChange {
                parameter,
                new_value,
                ..
            } => {
                tracing::info!("Executing parameter change: {} = {}", parameter, new_value);
                // Would implement actual parameter changes
            }
            ProposalType::AuthorityManagement {
                action,
                authority_address,
                authority_type,
            } => {
                tracing::info!(
                    "Executing authority management: {:?} {} ({})",
                    action,
                    authority_address,
                    authority_type
                );
                // Would implement authority management
            }
            ProposalType::Emergency {
                emergency_type,
                response_actions,
                ..
            } => {
                tracing::info!(
                    "Executing emergency response: {:?} with {} actions",
                    emergency_type,
                    response_actions.len()
                );
                // Would implement emergency responses
            }
            ProposalType::PricingRegulation {
                min_price,
                max_price,
                peak_hour_multiplier,
                effective_date,
            } => {
                tracing::info!(
                    "Executing pricing regulation: {}-{} tokens/kWh, peak multiplier: {}, effective: {}",
                    min_price,
                    max_price,
                    peak_hour_multiplier,
                    effective_date
                );
                // Would implement pricing changes
            }
            _ => {
                tracing::info!("Executing proposal type: {:?}", proposal.proposal_type);
            }
        }

        Ok(())
    }

    /// Calculate vote results for a proposal
    fn calculate_vote_results(&self, proposal: &Proposal) -> (u64, u64) {
        let mut total_votes = 0;
        let mut yes_votes = 0;

        for vote in proposal.votes.values() {
            total_votes += vote.voting_power;
            if vote.choice == VoteChoice::Yes {
                yes_votes += vote.voting_power;
            }
        }

        (total_votes, yes_votes)
    }

    /// Get voting power for an address
    pub async fn get_voting_power(&self, address: &str) -> u64 {
        let voting_power = self.voting_power.read().await;
        voting_power.get(address).map(|vp| vp.power).unwrap_or(0)
    }

    /// Update voting power for all addresses
    async fn update_voting_power(&self) -> Result<()> {
        let blockchain = self.blockchain.read().await;
        let mut voting_power = self.voting_power.write().await;

        // Simplified - base voting power on token balance
        // In real implementation, would consider staking, delegation, etc.
        let all_accounts = blockchain.load_accounts().await?;

        for (address, account) in all_accounts.iter() {
            let power = VotingPower {
                address: address.clone(),
                power: account.token_balance / 1000, // 1 vote per 1000 tokens
                source: VotingPowerSource::TokenStake(account.token_balance),
                delegation: None,
                last_updated: Utc::now(),
            };

            voting_power.insert(address.clone(), power);
        }

        Ok(())
    }

    /// Get total voting power in the system
    async fn get_total_voting_power(&self) -> u64 {
        let voting_power = self.voting_power.read().await;
        voting_power.values().map(|vp| vp.power).sum()
    }

    /// Get all active proposals
    pub async fn get_active_proposals(&self) -> Vec<Proposal> {
        let proposals = self.proposals.read().await;
        proposals
            .values()
            .filter(|p| p.status == ProposalStatus::Active)
            .cloned()
            .collect()
    }

    /// Get proposal by ID
    pub async fn get_proposal(&self, proposal_id: &str) -> Option<Proposal> {
        let proposals = self.proposals.read().await;
        proposals.get(proposal_id).cloned()
    }

    /// Get governance metrics
    pub async fn get_metrics(&self) -> GovernanceMetrics {
        let proposals = self.proposals.read().await;
        let voting_power = self.voting_power.read().await;

        let total_proposals = proposals.len() as u64;
        let active_proposals = proposals
            .values()
            .filter(|p| p.status == ProposalStatus::Active)
            .count() as u64;
        let executed_proposals = proposals
            .values()
            .filter(|p| p.status == ProposalStatus::Executed)
            .count() as u64;

        let total_voters = voting_power.len() as u64;
        let total_voting_power = voting_power.values().map(|vp| vp.power).sum();

        // Calculate average turnout
        let average_turnout = if total_proposals > 0 {
            let total_votes: u64 = proposals.values().map(|p| p.votes.len() as u64).sum();
            total_votes as f64 / (total_proposals * total_voters) as f64
        } else {
            0.0
        };

        GovernanceMetrics {
            total_proposals,
            active_proposals,
            executed_proposals,
            total_voters,
            average_turnout,
            total_voting_power,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::StorageManager;

    #[tokio::test]
    async fn test_governance_system_creation() {
        let storage = Arc::new(StorageManager::new_memory().await.unwrap());
        let blockchain = Arc::new(RwLock::new(
            crate::blockchain::Blockchain::new(storage).await.unwrap(),
        ));

        let governance = GovernanceSystem::new(blockchain).await.unwrap();
        let metrics = governance.get_metrics().await;

        assert_eq!(metrics.total_proposals, 0);
        assert_eq!(metrics.active_proposals, 0);
    }

    #[test]
    fn test_proposal_serialization() {
        let proposal = Proposal {
            id: "test-proposal".to_string(),
            title: "Test Proposal".to_string(),
            description: "A test proposal".to_string(),
            proposer: "proposer123".to_string(),
            proposal_type: ProposalType::ParameterChange {
                parameter: "test_param".to_string(),
                current_value: "old".to_string(),
                new_value: "new".to_string(),
            },
            created_at: Utc::now(),
            voting_start: Utc::now(),
            voting_end: Utc::now() + chrono::Duration::days(7),
            execution_delay: 172800, // 2 days
            status: ProposalStatus::Active,
            votes: HashMap::new(),
            execution_data: Vec::new(),
            quorum_required: 0.5,
            approval_threshold: 0.6,
        };

        let serialized = serde_json::to_string(&proposal).unwrap();
        let deserialized: Proposal = serde_json::from_str(&serialized).unwrap();

        assert_eq!(proposal.id, deserialized.id);
        assert_eq!(proposal.title, deserialized.title);
    }

    #[test]
    fn test_vote_serialization() {
        let vote = Vote {
            voter: "voter123".to_string(),
            choice: VoteChoice::Yes,
            voting_power: 1000,
            reason: Some("Good proposal".to_string()),
            timestamp: Utc::now(),
            signature: "signature123".to_string(),
        };

        let serialized = serde_json::to_string(&vote).unwrap();
        let deserialized: Vote = serde_json::from_str(&serialized).unwrap();

        assert_eq!(vote.voter, deserialized.voter);
        assert_eq!(vote.choice, deserialized.choice);
        assert_eq!(vote.voting_power, deserialized.voting_power);
    }
}
