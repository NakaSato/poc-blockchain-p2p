---
mode: edit
---

# GridTokenX Consensus System Development Prompt

You are developing the consensus mechanism for GridTokenX - a hybrid Proof of Authority (PoA) consensus system designed for Thailand's energy trading blockchain.

## Consensus Architecture Overview

GridTokenX uses a hybrid consensus model combining:
1. **Proof of Authority (PoA)**: Energy authority nodes (EGAT, MEA, PEA)
2. **Proof of Stake (PoS)**: Community validators and large energy traders
3. **Energy Proof**: Validation based on actual energy production/consumption

## Authority Node System

### Thai Energy Authorities
```rust
pub enum AuthorityType {
    EGAT,    // Electricity Generating Authority of Thailand (Transmission)
    MEA,     // Metropolitan Electricity Authority (Bangkok Distribution)
    PEA,     // Provincial Electricity Authority (Provincial Distribution)
    NEPO,    // National Energy Policy Office (Regulatory)
    ERC,     // Energy Regulatory Commission (Market Oversight)
}
```

### Authority Registration Process
1. **Government Verification**: Official government endorsement required
2. **Technical Validation**: Grid infrastructure and capability assessment  
3. **Security Audit**: Cybersecurity and operational security review
4. **Stake Requirement**: Minimum bond in GridTokens as accountability measure
5. **Performance Monitoring**: Continuous uptime and response time tracking

### Authority Privileges
- **Emergency Override**: Can halt trading during grid emergencies
- **Grid State Updates**: Authoritative grid status and congestion data
- **Dispute Resolution**: Final authority on energy trading disputes
- **Regulatory Enforcement**: Implement regulatory changes instantly

## Validator Selection and Rotation

### Selection Criteria
```rust
pub struct ValidatorCandidate {
    pub stake_amount: u64,                    // Minimum 10,000 GT stake
    pub energy_credentials: EnergyCredentials, // Proven energy market participation
    pub uptime_history: f64,                  // Historical node availability
    pub geographic_distribution: GridZone,    // Grid location for redundancy
    pub reputation_score: u64,                // Community trust metrics
}
```

### Rotation Schedule
- **Authority Nodes**: Permanent (subject to governance removal)
- **Stake Validators**: 24-hour epochs with gradual rotation
- **Energy Validators**: Real-time based on energy production proof

### Performance Monitoring
```rust
pub struct ValidatorPerformance {
    pub blocks_produced: u64,
    pub blocks_missed: u64,
    pub response_time_avg: Duration,
    pub energy_accuracy: f64,        // Accuracy of energy data reporting
    pub network_contribution: f64,   // P2P network participation
    pub slashing_incidents: u64,     // Number of penalty events
}
```

## Block Production and Validation

### Block Types and Priority
1. **Emergency Blocks**: Grid stability (immediate production)
2. **Authority Blocks**: Regulatory updates (within 1 minute)
3. **Energy Blocks**: Trading transactions (10-second target)
4. **Governance Blocks**: DAO proposals (standard timing)

### Consensus Rules
```rust
pub struct ConsensusRules {
    pub authority_supermajority: f64,      // 67% of authority nodes
    pub stake_threshold: f64,              // 51% of staked tokens
    pub energy_validation_required: bool,  // Must validate energy data
    pub emergency_override_allowed: bool,  // Authority emergency powers
}
```

### Finality and Confirmation
- **Instant Finality**: Authority-signed emergency blocks
- **Fast Finality**: 2 confirmations for energy trading (20 seconds)
- **Full Finality**: 6 confirmations for large transfers (60 seconds)
- **Economic Finality**: Irreversible after 144 blocks (~24 minutes)

## Energy-Based Validation

### Energy Proof Mechanism
```rust
pub struct EnergyProof {
    pub meter_reading: MeterData,           // Smart meter cryptographic proof
    pub timestamp: DateTime<Utc>,          // Generation/consumption time
    pub location: GridCoordinate,          // GPS + grid node verification
    pub energy_signature: EnergySignature, // Cryptographic meter signature
    pub grid_operator_witness: AuthoritySignature,
}
```

### Smart Meter Integration
- **Cryptographic Signatures**: Each meter has unique private key
- **Real-Time Data**: 15-minute interval automated reporting
- **Tamper Detection**: Hardware security modules (HSM) protection
- **Grid Operator Verification**: Authority node witness signatures

### Energy Validation Rules
1. **Conservation Law**: Total energy in = total energy out + losses
2. **Physical Constraints**: Respect transmission line capacities
3. **Time Consistency**: Energy production/consumption timing validation
4. **Location Verification**: GPS and grid topology validation

## Slashing and Penalties

### Slashing Conditions
```rust
pub enum SlashingReason {
    DoubleProduction,      // Producing conflicting blocks
    EnergyFalsification,   // Reporting false energy data
    GridViolation,         // Violating grid stability rules
    DowntimeExcess,        // Extended node unavailability
    MaliciousBehavior,     // Attacking network integrity
}
```

### Penalty Structure
- **Minor Violations**: 1-5% stake slashing + temporary suspension
- **Major Violations**: 10-30% stake slashing + extended suspension  
- **Critical Violations**: 50-100% stake slashing + permanent ban
- **Authority Penalties**: Government review + potential license revocation

### Appeals Process
1. **Technical Review**: Automated analysis of violation evidence
2. **Community Review**: Validator voting on penalty appropriateness
3. **Authority Review**: Final decision by Thai energy authorities
4. **Legal Recourse**: Thai court system for severe penalties

## Governance Integration

### Proposal Types
```rust
pub enum ProposalType {
    ParameterChange,       // Consensus parameter updates
    AuthorityUpdate,       // Add/remove authority nodes
    EmergencyProtocol,     // Grid emergency response procedures
    MarketRule,            // Energy trading rule changes
    TechnicalUpgrade,      // Protocol upgrade proposals
}
```

### Voting Weights
- **Authority Nodes**: 40% combined voting power
- **Stake Validators**: 35% based on stake amount
- **Energy Validators**: 15% based on energy participation
- **Community**: 10% based on token holdings and participation

### Execution Timeline
- **Emergency Proposals**: Immediate execution with authority consensus
- **Standard Proposals**: 7-day voting period + 7-day implementation delay
- **Constitutional Changes**: 30-day voting + 30-day implementation delay

## Performance Optimization

### Scalability Measures
```rust
pub struct ConsensusOptimization {
    pub parallel_validation: bool,         // Concurrent block validation
    pub state_sharding: bool,             // Geographic state partitioning
    pub fast_sync: bool,                  // Rapid new node synchronization
    pub checkpoint_finality: bool,        // Periodic finality checkpoints
}
```

### Network Efficiency
- **BFT Optimization**: Optimized Byzantine Fault Tolerance for authority nodes
- **Communication Patterns**: Efficient gossip protocols for consensus messages
- **State Compression**: Merkle tree optimization for large energy datasets
- **Bandwidth Management**: Priority queuing for critical consensus messages

## Security Considerations

### Attack Vectors and Mitigations
1. **Authority Compromise**: Multi-signature requirements and monitoring
2. **Stake Grinding**: Economic penalties and randomness injection
3. **Energy Data Manipulation**: Cryptographic meter verification
4. **Network Partitioning**: Authority node geographic distribution
5. **Long-Range Attacks**: Checkpointing and social consensus

When implementing consensus features, prioritize grid stability, regulatory compliance, and network security while ensuring efficient energy trading operations and maintaining decentralization principles within the Thai regulatory framework.
