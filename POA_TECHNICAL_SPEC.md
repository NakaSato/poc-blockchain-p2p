# GridTokenX POA Technical Specification

## 🔧 Technical Implementation Specification

### File Structure
```
src/
├── consensus/
│   ├── mod.rs
│   ├── poa/
│   │   ├── mod.rs
│   │   ├── authority_registry.rs
│   │   ├── consensus_engine.rs
│   │   ├── governance.rs
│   │   ├── reputation.rs
│   │   └── validator_selection.rs
│   └── traits.rs
├── poa_config.rs
└── api/
    └── poa_endpoints.rs
```

## 📋 Implementation Checklist

### Phase 1: Core Infrastructure ✅

#### 1.1 Authority Registry (`src/consensus/poa/authority_registry.rs`)
```rust
// Data structures for managing authorities
pub struct AuthorityRegistry {
    // Authority storage and management
    active_authorities: RwLock<HashMap<String, Authority>>,
    pending_authorities: RwLock<HashMap<String, PendingAuthority>>,
    revoked_authorities: RwLock<HashMap<String, RevokedAuthority>>,
    authority_order: RwLock<Vec<String>>,
    config: POAConfig,
}

// Implementation methods
impl AuthorityRegistry {
    pub async fn register_authority(&self, registration: AuthorityRegistration) -> Result<()>;
    pub async fn activate_authority(&self, address: &str) -> Result<()>;
    pub async fn revoke_authority(&self, address: &str, reason: &str) -> Result<()>;
    pub async fn get_active_authorities(&self) -> Vec<Authority>;
    pub async fn update_authority_performance(&self, address: &str, metrics: PerformanceMetrics) -> Result<()>;
}
```

#### 1.2 POA Consensus Engine (`src/consensus/poa/consensus_engine.rs`)
```rust
pub struct POAConsensusEngine {
    authority_registry: Arc<AuthorityRegistry>,
    reputation_tracker: Arc<ReputationTracker>,
    governance: Arc<AuthorityGovernance>,
    blockchain: Arc<RwLock<Blockchain>>,
    config: POAConfig,
    current_state: RwLock<POAState>,
}

pub struct POAState {
    current_authority_index: usize,
    last_block_time: Option<DateTime<Utc>>,
    missed_blocks: u64,
    consensus_round: u64,
}

impl POAConsensusEngine {
    pub async fn start_consensus_loop(&self) -> Result<()>;
    pub async fn propose_block(&self) -> Result<Block>;
    pub async fn validate_block(&self, block: &Block) -> Result<ValidationResult>;
    pub async fn handle_block_timeout(&self) -> Result<()>;
}
```

### Phase 2: Authority Management

#### 2.1 Authority Types and Validation
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThaiAuthorityType {
    EGAT,           // Electricity Generating Authority of Thailand
    MEA,            // Metropolitan Electricity Authority  
    PEA,            // Provincial Electricity Authority
    ERC,            // Energy Regulatory Commission
    PowerProducer,  // Licensed power generation companies
    GridOperator,   // Technical grid management
}

pub struct AuthorityCredentials {
    license_number: String,
    organization_name: String,
    authority_type: ThaiAuthorityType,
    public_key: Vec<u8>,
    certificate_hash: String,
    endorsement_signatures: Vec<AuthorityEndorsement>,
}

impl AuthorityCredentials {
    pub fn verify_thai_energy_license(&self) -> Result<bool>;
    pub fn validate_endorsements(&self) -> Result<bool>;
    pub fn check_organization_registry(&self) -> Result<bool>;
}
```

#### 2.2 Round-Robin Validator Selection
```rust
impl ValidatorSelection for POAConsensusEngine {
    async fn select_next_authority(&self) -> Result<String> {
        let registry = self.authority_registry.get_active_authorities().await;
        let current_height = self.get_blockchain_height().await?;
        
        // Calculate next authority based on block height
        let authority_index = (current_height as usize) % registry.len();
        let selected_authority = &registry[authority_index].address;
        
        // Verify authority health before selection
        if self.reputation_tracker.is_authority_healthy(selected_authority).await? {
            Ok(selected_authority.clone())
        } else {
            // Skip unhealthy authority and select next
            self.select_fallback_authority(authority_index).await
        }
    }
    
    async fn select_fallback_authority(&self, failed_index: usize) -> Result<String> {
        let registry = self.authority_registry.get_active_authorities().await;
        
        // Try next 3 authorities in sequence
        for i in 1..=3 {
            let next_index = (failed_index + i) % registry.len();
            let candidate = &registry[next_index].address;
            
            if self.reputation_tracker.is_authority_healthy(candidate).await? {
                // Log authority fallback
                tracing::warn!("Authority fallback: {} -> {}", failed_index, next_index);
                return Ok(candidate.clone());
            }
        }
        
        Err(anyhow!("No healthy authorities available"))
    }
}
```

### Phase 3: Block Production and Validation

#### 3.1 Authority Block Production
```rust
impl BlockProduction for POAConsensusEngine {
    async fn create_authority_block(&self, authority_address: &str) -> Result<Block> {
        // Get pending transactions
        let transactions = self.get_pending_transactions(1000).await?;
        
        // Filter and validate energy transactions
        let validated_txs = self.validate_energy_transactions(&transactions).await?;
        
        // Create block header
        let previous_block = self.blockchain.read().await.get_latest_block().await?;
        let block_height = previous_block.height + 1;
        
        let mut block = Block {
            height: block_height,
            previous_hash: previous_block.hash.clone(),
            timestamp: Utc::now(),
            transactions: validated_txs,
            merkle_root: String::new(),
            authority_signature: None,
            proposer_address: authority_address.to_string(),
        };
        
        // Calculate merkle root
        block.merkle_root = self.calculate_merkle_root(&block.transactions)?;
        
        // Sign block with authority private key
        let signature = self.sign_block_as_authority(&block, authority_address).await?;
        block.authority_signature = Some(signature);
        
        Ok(block)
    }
    
    async fn sign_block_as_authority(&self, block: &Block, authority: &str) -> Result<String> {
        let authority_info = self.authority_registry.get_authority(authority).await?;
        
        // Create block hash for signing
        let block_hash = self.calculate_block_hash_for_signing(block)?;
        
        // Sign with authority's private key
        let signature = self.crypto_sign(&block_hash, &authority_info.private_key)?;
        
        Ok(hex::encode(signature))
    }
}
```

#### 3.2 Block Validation
```rust
impl BlockValidation for POAConsensusEngine {
    async fn validate_poa_block(&self, block: &Block) -> Result<ValidationResult> {
        let mut validation = ValidationResult::new();
        
        // 1. Verify block structure
        validation.add_check("structure", self.validate_block_structure(block)?);
        
        // 2. Verify authority signature
        validation.add_check("authority_signature", self.validate_authority_signature(block).await?);
        
        // 3. Verify authority turn
        validation.add_check("authority_turn", self.validate_authority_turn(block).await?);
        
        // 4. Verify transaction validity
        validation.add_check("transactions", self.validate_block_transactions(block).await?);
        
        // 5. Verify energy trading compliance
        validation.add_check("energy_compliance", self.validate_energy_compliance(block).await?);
        
        Ok(validation)
    }
    
    async fn validate_authority_signature(&self, block: &Block) -> Result<bool> {
        let signature = block.authority_signature.as_ref()
            .ok_or_else(|| anyhow!("Missing authority signature"))?;
            
        let expected_authority = self.get_expected_authority_for_height(block.height).await?;
        
        if block.proposer_address != expected_authority.address {
            return Ok(false);
        }
        
        let block_hash = self.calculate_block_hash_for_signing(block)?;
        let signature_bytes = hex::decode(signature)?;
        
        self.crypto_verify(&block_hash, &signature_bytes, &expected_authority.public_key)
    }
    
    async fn validate_authority_turn(&self, block: &Block) -> Result<bool> {
        let authorities = self.authority_registry.get_active_authorities().await;
        let expected_index = (block.height as usize) % authorities.len();
        let expected_authority = &authorities[expected_index];
        
        Ok(block.proposer_address == expected_authority.address)
    }
}
```

### Phase 4: Governance System

#### 4.1 Authority Governance
```rust
pub struct AuthorityGovernance {
    proposals: RwLock<HashMap<String, GovernanceProposal>>,
    voting_records: RwLock<HashMap<String, VotingRecord>>,
    config: GovernanceConfig,
    authority_registry: Arc<AuthorityRegistry>,
}

#[derive(Debug, Clone)]
pub enum GovernanceProposal {
    AddAuthority {
        candidate: AuthorityRegistration,
        proposer: String,
        votes: HashMap<String, ProposalVote>,
        created_at: DateTime<Utc>,
        voting_deadline: DateTime<Utc>,
    },
    RemoveAuthority {
        target: String,
        reason: String,
        evidence: Vec<EvidenceItem>,
        proposer: String,
        votes: HashMap<String, ProposalVote>,
        created_at: DateTime<Utc>,
        voting_deadline: DateTime<Utc>,
    },
    UpdateParameters {
        parameter_changes: HashMap<String, serde_json::Value>,
        proposer: String,
        votes: HashMap<String, ProposalVote>,
        created_at: DateTime<Utc>,
        voting_deadline: DateTime<Utc>,
    },
}

impl AuthorityGovernance {
    pub async fn submit_proposal(&self, proposal: GovernanceProposal) -> Result<String> {
        let proposal_id = Uuid::new_v4().to_string();
        
        // Validate proposer authority
        self.validate_proposer_authority(&proposal).await?;
        
        // Store proposal
        self.proposals.write().await.insert(proposal_id.clone(), proposal);
        
        // Broadcast to network
        self.broadcast_proposal(&proposal_id).await?;
        
        Ok(proposal_id)
    }
    
    pub async fn cast_vote(&self, proposal_id: &str, voter: &str, vote: ProposalVote) -> Result<()> {
        // Verify voter is active authority
        if !self.authority_registry.is_active_authority(voter).await? {
            return Err(anyhow!("Voter is not an active authority"));
        }
        
        let mut proposals = self.proposals.write().await;
        let proposal = proposals.get_mut(proposal_id)
            .ok_or_else(|| anyhow!("Proposal not found"))?;
            
        // Add vote to proposal
        match proposal {
            GovernanceProposal::AddAuthority { votes, .. } => {
                votes.insert(voter.to_string(), vote);
            },
            GovernanceProposal::RemoveAuthority { votes, .. } => {
                votes.insert(voter.to_string(), vote);
            },
            GovernanceProposal::UpdateParameters { votes, .. } => {
                votes.insert(voter.to_string(), vote);
            },
        }
        
        // Check if proposal reached decision threshold
        self.evaluate_proposal_completion(proposal_id).await?;
        
        Ok(())
    }
    
    async fn evaluate_proposal_completion(&self, proposal_id: &str) -> Result<()> {
        let proposals = self.proposals.read().await;
        let proposal = proposals.get(proposal_id).unwrap();
        
        let total_authorities = self.authority_registry.get_active_authority_count().await?;
        let votes_cast = self.count_votes(proposal);
        
        // Check if voting period expired or enough votes collected
        let voting_complete = match proposal {
            GovernanceProposal::AddAuthority { voting_deadline, .. } => {
                Utc::now() > *voting_deadline || votes_cast >= total_authorities
            },
            _ => Utc::now() > self.get_proposal_deadline(proposal) || votes_cast >= total_authorities,
        };
        
        if voting_complete {
            self.finalize_proposal(proposal_id).await?;
        }
        
        Ok(())
    }
}
```

### Phase 5: Performance Monitoring

#### 5.1 Reputation System
```rust
pub struct ReputationTracker {
    performance_data: RwLock<HashMap<String, AuthorityPerformance>>,
    reputation_config: ReputationConfig,
}

pub struct AuthorityPerformance {
    blocks_proposed: u64,
    blocks_missed: u64,
    average_response_time: Duration,
    last_activity: DateTime<Utc>,
    uptime_percentage: f64,
    community_reports: Vec<CommunityReport>,
    reputation_score: f64,
}

impl ReputationTracker {
    pub async fn update_authority_performance(&self, authority: &str, metrics: PerformanceUpdate) -> Result<()> {
        let mut performance_data = self.performance_data.write().await;
        
        let performance = performance_data.entry(authority.to_string())
            .or_insert_with(|| AuthorityPerformance::default());
            
        match metrics {
            PerformanceUpdate::BlockProposed { response_time } => {
                performance.blocks_proposed += 1;
                performance.update_response_time(response_time);
                performance.last_activity = Utc::now();
            },
            PerformanceUpdate::BlockMissed => {
                performance.blocks_missed += 1;
            },
            PerformanceUpdate::CommunityReport { report } => {
                performance.community_reports.push(report);
            },
        }
        
        // Recalculate reputation score
        performance.reputation_score = self.calculate_reputation_score(performance).await?;
        
        Ok(())
    }
    
    async fn calculate_reputation_score(&self, performance: &AuthorityPerformance) -> Result<f64> {
        let config = &self.reputation_config;
        
        // Calculate component scores
        let success_rate = if performance.blocks_proposed + performance.blocks_missed > 0 {
            performance.blocks_proposed as f64 / (performance.blocks_proposed + performance.blocks_missed) as f64
        } else {
            1.0 // No blocks attempted yet
        };
        
        let response_score = {
            let max_acceptable_response = Duration::from_secs(5);
            let response_ratio = performance.average_response_time.as_secs_f64() / max_acceptable_response.as_secs_f64();
            (1.0 - response_ratio.min(1.0)).max(0.0)
        };
        
        let uptime_score = performance.uptime_percentage / 100.0;
        
        let community_score = self.calculate_community_score(&performance.community_reports).await?;
        
        // Weighted average
        let reputation = success_rate * config.success_rate_weight +
                        response_score * config.response_time_weight +
                        uptime_score * config.uptime_weight +
                        community_score * config.community_weight;
                        
        Ok(reputation.min(1.0).max(0.0))
    }
    
    pub async fn is_authority_healthy(&self, authority: &str) -> Result<bool> {
        let performance_data = self.performance_data.read().await;
        let performance = performance_data.get(authority)
            .ok_or_else(|| anyhow!("Authority performance not found"))?;
            
        let min_reputation = self.reputation_config.min_healthy_reputation;
        let max_inactive_time = self.reputation_config.max_inactive_duration;
        
        let is_active = performance.last_activity > Utc::now() - max_inactive_time;
        let has_good_reputation = performance.reputation_score >= min_reputation;
        
        Ok(is_active && has_good_reputation)
    }
}
```

### Phase 6: Configuration

#### 6.1 POA Configuration Structure
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POAConfig {
    pub enabled: bool,
    pub block_time_seconds: u64,
    pub authority_timeout_seconds: u64,
    pub max_authorities: usize,
    pub min_authorities: usize,
    pub governance: GovernanceConfig,
    pub reputation: ReputationConfig,
    pub initial_authorities: Vec<InitialAuthority>,
    pub thai_compliance: ThaiComplianceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub voting_period_blocks: u64,
    pub approval_threshold_percent: f64,
    pub minimum_proposer_reputation: f64,
    pub proposal_deposit_amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationConfig {
    pub success_rate_weight: f64,
    pub response_time_weight: f64,
    pub uptime_weight: f64,
    pub community_weight: f64,
    pub min_healthy_reputation: f64,
    pub max_inactive_duration_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiComplianceConfig {
    pub egat_integration: bool,
    pub mea_integration: bool,
    pub pea_integration: bool,
    pub erc_reporting: bool,
    pub license_verification_endpoint: String,
    pub grid_stability_monitoring: bool,
}
```

## 🧪 Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_authority_registration() {
        // Test authority registration process
    }
    
    #[tokio::test]
    async fn test_round_robin_selection() {
        // Test validator selection algorithm
    }
    
    #[tokio::test]
    async fn test_block_validation() {
        // Test POA block validation
    }
    
    #[tokio::test]
    async fn test_governance_voting() {
        // Test governance proposal and voting
    }
    
    #[tokio::test]
    async fn test_reputation_calculation() {
        // Test reputation scoring system
    }
    
    #[tokio::test]
    async fn test_authority_fallback() {
        // Test fallback when authority is unhealthy
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_full_poa_consensus_cycle() {
        // Test complete consensus cycle with multiple authorities
    }
    
    #[tokio::test]
    async fn test_authority_governance_workflow() {
        // Test end-to-end governance process
    }
    
    #[tokio::test]
    async fn test_thai_energy_compliance() {
        // Test Thai energy market specific features
    }
}
```

### Performance Tests
```rust
#[cfg(test)]
mod performance_tests {
    #[tokio::test]
    async fn test_block_production_speed() {
        // Measure block production performance
    }
    
    #[tokio::test]
    async fn test_authority_selection_performance() {
        // Measure validator selection speed
    }
    
    #[tokio::test]
    async fn test_concurrent_governance_voting() {
        // Test voting system under load
    }
}
```

## 📚 API Endpoints

### Authority Management
- `POST /api/v1/poa/register-authority` - Register new authority
- `GET /api/v1/poa/authorities` - List all authorities
- `GET /api/v1/poa/authorities/{address}` - Get authority details
- `POST /api/v1/poa/authorities/{address}/revoke` - Revoke authority

### Governance
- `POST /api/v1/poa/governance/proposals` - Submit governance proposal
- `GET /api/v1/poa/governance/proposals` - List active proposals
- `POST /api/v1/poa/governance/proposals/{id}/vote` - Cast vote
- `GET /api/v1/poa/governance/proposals/{id}` - Get proposal details

### Monitoring
- `GET /api/v1/poa/performance/{address}` - Authority performance metrics
- `GET /api/v1/poa/reputation/{address}` - Authority reputation score
- `GET /api/v1/poa/health` - Overall POA health status

This technical specification provides the detailed implementation roadmap for the GridTokenX POA consensus system.
