---
mode: edit
---

# GridTokenX Governance System Development Prompt

You are developing the governance module for GridTokenX - Thailand's energy trading blockchain platform implementing decentralized autonomous organization (DAO) features for community-driven decision making.

## Governance Architecture Overview

The governance system (`src/governance.rs`) implements:
- **Hybrid Governance**: Combination of community voting and authority oversight
- **Thai Legal Compliance**: Integration with Thai regulatory framework
- **Stakeholder Representation**: Balanced participation across all network participants
- **Emergency Protocols**: Rapid decision-making for grid stability

## Governance Structure

### Participant Categories
```rust
pub enum GovernanceParticipant {
    AuthorityNode {
        authority_type: AuthorityType,
        voting_power: u64,              // Based on grid responsibility
        veto_power: bool,               // Can veto proposals affecting grid stability
    },
    Validator {
        stake_amount: u64,
        performance_score: f64,
        voting_power: u64,              // Based on stake and performance
    },
    EnergyProducer {
        production_capacity: f64,       // MW capacity
        renewable_percentage: f64,
        voting_power: u64,              // Based on production and renewables
    },
    EnergyConsumer {
        consumption_history: f64,       // Historical consumption
        grid_contribution: f64,         // Demand response participation
        voting_power: u64,
    },
    CommunityMember {
        token_holdings: u64,
        participation_score: f64,
        voting_power: u64,              // Based on holdings and participation
    },
}
```

### Voting Power Calculation
```rust
impl GovernanceParticipant {
    pub fn calculate_voting_power(&self) -> u64 {
        match self {
            AuthorityNode { authority_type, .. } => {
                match authority_type {
                    AuthorityType::EGAT => 10000,  // Highest voting power for transmission operator
                    AuthorityType::MEA | AuthorityType::PEA => 8000,  // Distribution operators
                    AuthorityType::NEPO => 6000,   // Policy oversight
                    AuthorityType::ERC => 7000,    // Market regulation
                }
            },
            Validator { stake_amount, performance_score, .. } => {
                (stake_amount as f64 * performance_score * 0.1) as u64
            },
            EnergyProducer { production_capacity, renewable_percentage, .. } => {
                let base_power = (production_capacity * 100.0) as u64;
                let renewable_bonus = (renewable_percentage * base_power as f64 * 0.5) as u64;
                base_power + renewable_bonus
            },
            EnergyConsumer { consumption_history, grid_contribution, .. } => {
                let base_power = (consumption_history * 10.0) as u64;
                let contribution_bonus = (grid_contribution * 1000.0) as u64;
                base_power + contribution_bonus
            },
            CommunityMember { token_holdings, participation_score, .. } => {
                (token_holdings as f64 * participation_score) as u64
            },
        }
    }
}
```

## Proposal Types and Categories

### Infrastructure Proposals
```rust
pub enum InfrastructureProposal {
    GridUpgrade {
        location: GridLocation,
        upgrade_type: UpgradeType,
        cost_estimate: u64,
        implementation_timeline: Duration,
        authority_endorsement: Vec<AuthorityType>,
    },
    NewTransmissionLine {
        start_point: GridCoordinate,
        end_point: GridCoordinate,
        capacity: f64,              // MW capacity
        environmental_impact: EnvironmentalImpact,
    },
    SmartMeterDeployment {
        target_region: Region,
        meter_specifications: MeterSpecs,
        rollout_schedule: RolloutSchedule,
    },
}
```

### Market Rule Proposals
```rust
pub enum MarketRuleProposal {
    PricingMechanism {
        new_algorithm: PricingAlgorithm,
        impact_analysis: MarketImpactAnalysis,
        transition_period: Duration,
    },
    TradingHours {
        new_hours: TradingHours,
        rationale: String,
        affected_participants: Vec<ParticipantType>,
    },
    CongestionManagement {
        new_rules: CongestionRules,
        penalty_structure: PenaltyStructure,
        exemptions: Vec<ExemptionCriteria>,
    },
    RenewableIncentives {
        incentive_structure: IncentiveStructure,
        eligibility_criteria: EligibilityCriteria,
        budget_allocation: u64,
    },
}
```

### Technical Proposals
```rust
pub enum TechnicalProposal {
    ProtocolUpgrade {
        version: String,
        changes: Vec<ProtocolChange>,
        backward_compatibility: bool,
        testing_results: TestingResults,
        security_audit: SecurityAudit,
    },
    ConsensusParameterChange {
        parameter: ConsensusParameter,
        current_value: String,
        proposed_value: String,
        justification: String,
    },
    SecurityUpdate {
        vulnerability_description: String,
        severity_level: SeverityLevel,
        proposed_fix: SecurityFix,
        emergency_flag: bool,
    },
}
```

### Regulatory Proposals
```rust
pub enum RegulatoryProposal {
    ComplianceUpdate {
        regulation_reference: String,
        required_changes: Vec<ComplianceChange>,
        implementation_deadline: DateTime<Utc>,
        authority_mandate: AuthorityMandate,
    },
    LicensingRequirement {
        participant_type: ParticipantType,
        license_criteria: LicenseCriteria,
        verification_process: VerificationProcess,
    },
    ReportingRequirement {
        data_type: DataType,
        reporting_frequency: ReportingFrequency,
        recipient_authorities: Vec<AuthorityType>,
    },
}
```

## Voting Mechanisms

### Voting Methods
```rust
pub enum VotingMethod {
    SimpleMajority,                     // >50% of voting power
    SuperMajority(f64),                // Specified percentage (e.g., 67%)
    QuorumBased {
        quorum_percentage: f64,         // Minimum participation required
        approval_percentage: f64,       // Approval threshold
    },
    AuthorityConsensus {
        required_authorities: Vec<AuthorityType>,
        authority_threshold: f64,       // % of authorities required
    },
    StakeholderConsensus {
        category_thresholds: HashMap<ParticipantType, f64>,
    },
}

pub enum VoteChoice {
    Approve,
    Reject,
    Abstain,
    ConditionalApproval {
        conditions: Vec<Condition>,
    },
}
```

### Delegation System
```rust
pub struct VoteDelegation {
    pub delegator: String,              // Address of token holder
    pub delegate: String,               // Address of delegate
    pub delegation_scope: DelegationScope,
    pub expiry: Option<DateTime<Utc>>,
    pub revocable: bool,
}

pub enum DelegationScope {
    AllProposals,
    Category(ProposalCategory),
    SpecificProposal(String),
    ConditionalDelegation {
        conditions: Vec<DelegationCondition>,
    },
}
```

## Proposal Lifecycle

### Proposal Submission
```rust
impl Governance {
    pub async fn submit_proposal(
        &self,
        proposer: &str,
        proposal: Proposal,
        stake_amount: u64,
    ) -> Result<ProposalId> {
        // Verify proposer eligibility
        self.verify_proposer_eligibility(proposer, &proposal).await?;
        
        // Validate proposal format and content
        self.validate_proposal(&proposal).await?;
        
        // Lock stake as proposal bond
        self.lock_proposal_stake(proposer, stake_amount).await?;
        
        // Submit for initial review
        let proposal_id = self.generate_proposal_id();
        let proposal_record = ProposalRecord {
            id: proposal_id.clone(),
            proposer: proposer.to_string(),
            proposal,
            stake_amount,
            status: ProposalStatus::UnderReview,
            submission_time: Utc::now(),
        };
        
        self.store_proposal(proposal_record).await?;
        
        // Notify relevant authorities for review
        self.notify_authorities_for_review(&proposal_id).await?;
        
        Ok(proposal_id)
    }
}
```

### Authority Review Process
```rust
pub struct AuthorityReview {
    pub authority: AuthorityType,
    pub reviewer: String,
    pub review_status: ReviewStatus,
    pub comments: String,
    pub regulatory_compliance: ComplianceStatus,
    pub grid_impact_assessment: Option<GridImpactAssessment>,
    pub review_timestamp: DateTime<Utc>,
}

pub enum ReviewStatus {
    Approved,
    Rejected { reason: String },
    RequiresModification { suggested_changes: Vec<String> },
    EscalatedToHigherAuthority,
}
```

### Voting Period Management
```rust
impl Governance {
    pub async fn initiate_voting(&self, proposal_id: &str) -> Result<()> {
        let proposal = self.get_proposal(proposal_id).await?;
        
        // Calculate voting period based on proposal type
        let voting_period = self.calculate_voting_period(&proposal.proposal);
        
        // Determine eligible voters
        let eligible_voters = self.get_eligible_voters(&proposal.proposal).await?;
        
        // Create voting session
        let voting_session = VotingSession {
            proposal_id: proposal_id.to_string(),
            start_time: Utc::now(),
            end_time: Utc::now() + voting_period,
            eligible_voters,
            voting_method: self.determine_voting_method(&proposal.proposal),
            required_quorum: self.calculate_required_quorum(&proposal.proposal),
        };
        
        self.store_voting_session(voting_session).await?;
        
        // Notify all eligible voters
        self.notify_voters(proposal_id).await?;
        
        Ok(())
    }
}
```

## Emergency Governance Protocols

### Emergency Decision Making
```rust
pub struct EmergencyProtocol {
    pub trigger_conditions: Vec<EmergencyTrigger>,
    pub decision_authority: EmergencyAuthority,
    pub notification_requirements: NotificationRequirements,
    pub review_process: EmergencyReviewProcess,
}

pub enum EmergencyTrigger {
    GridInstability {
        frequency_deviation: f64,       // Hz deviation from 50 Hz
        voltage_deviation: f64,         // % deviation from nominal
        cascade_failure_risk: f64,     // Risk assessment score
    },
    CyberSecurityIncident {
        severity: SecuritySeverity,
        affected_systems: Vec<SystemComponent>,
        threat_level: ThreatLevel,
    },
    MarketManipulation {
        suspicious_activity: SuspiciousActivity,
        market_impact: MarketImpact,
        evidence_strength: f64,
    },
    RegulatoryCompliance {
        violation_type: ViolationType,
        authority_directive: AuthorityDirective,
        compliance_deadline: DateTime<Utc>,
    },
}
```

### Rapid Response Mechanism
```rust
impl Governance {
    pub async fn execute_emergency_decision(
        &self,
        emergency_type: EmergencyTrigger,
        decision: EmergencyDecision,
        authority_signature: AuthoritySignature,
    ) -> Result<()> {
        // Verify emergency authority
        self.verify_emergency_authority(&authority_signature, &emergency_type).await?;
        
        // Log emergency decision
        let emergency_record = EmergencyRecord {
            trigger: emergency_type,
            decision: decision.clone(),
            authority: authority_signature.authority_type,
            timestamp: Utc::now(),
            justification: decision.justification.clone(),
        };
        
        self.store_emergency_record(emergency_record).await?;
        
        // Execute decision immediately
        self.execute_decision(&decision).await?;
        
        // Schedule post-emergency review
        self.schedule_emergency_review(&decision).await?;
        
        // Notify all stakeholders
        self.broadcast_emergency_notification(&decision).await?;
        
        Ok(())
    }
}
```

## Governance Analytics and Transparency

### Participation Metrics
```rust
pub struct GovernanceMetrics {
    pub total_proposals: u64,
    pub proposals_by_category: HashMap<ProposalCategory, u64>,
    pub voter_turnout: f64,
    pub voter_turnout_by_category: HashMap<ParticipantType, f64>,
    pub proposal_success_rate: f64,
    pub average_voting_period: Duration,
    pub authority_participation: HashMap<AuthorityType, f64>,
    pub delegation_rate: f64,
}

impl Governance {
    pub async fn generate_transparency_report(&self) -> Result<TransparencyReport> {
        let metrics = self.calculate_governance_metrics().await?;
        
        TransparencyReport {
            reporting_period: self.get_current_period(),
            governance_metrics: metrics,
            active_proposals: self.get_active_proposals().await?,
            recent_decisions: self.get_recent_decisions().await?,
            emergency_actions: self.get_emergency_actions().await?,
            regulatory_compliance_status: self.check_regulatory_compliance().await?,
        }
    }
}
```

### Public Accessibility
```rust
pub struct PublicGovernanceInterface {
    pub proposal_viewer: ProposalViewer,
    pub voting_interface: VotingInterface,
    pub transparency_dashboard: TransparencyDashboard,
    pub regulatory_compliance_tracker: ComplianceTracker,
}

impl PublicGovernanceInterface {
    pub async fn get_proposal_details(&self, proposal_id: &str) -> Result<PublicProposalView> {
        let proposal = self.proposal_viewer.get_proposal(proposal_id).await?;
        
        // Filter sensitive information
        let public_view = PublicProposalView {
            id: proposal.id,
            title: proposal.title,
            category: proposal.category,
            description: proposal.description,
            status: proposal.status,
            voting_progress: self.calculate_voting_progress(&proposal).await?,
            authority_reviews: self.get_public_reviews(&proposal).await?,
            implementation_timeline: proposal.implementation_timeline,
        };
        
        Ok(public_view)
    }
}
```

When implementing governance features, ensure transparency, inclusivity, and regulatory compliance while maintaining the ability to respond rapidly to grid emergencies and market disruptions.
