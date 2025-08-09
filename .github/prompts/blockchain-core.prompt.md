---
mode: edit
type: domain-specific
domain: blockchain-infrastructure
priority: high
tags: [blockchain, blocks, transactions, chain, consensus, rust, ddd]
---

# ⛓️ GridTokenX Blockchain Core Development Guide

> **Building the Foundation**  
> Expert guidance for developing the blockchain infrastructure that powers Thailand's energy trading revolution.

## 🎯 Domain Focus: Blockchain Infrastructure

You are developing the **Blockchain Infrastructure Domain** - the foundational layer that ensures secure, transparent, and immutable energy trading records while maintaining Thailand's grid stability.

### 🏗️ **Domain-Driven Architecture**
```
src/domains/blockchain_infrastructure/
├── domain/
│   ├── entities/           # Block, Transaction, Chain
│   ├── value_objects/      # Hash, Signature, Timestamp
│   ├── aggregates/         # BlockchainState, TransactionPool
│   └── services/          # ValidationService, ConsensusService
├── application/
│   ├── commands/          # CreateBlock, ValidateTransaction
│   ├── queries/           # GetBlock, GetTransactionHistory
│   └── services/          # BlockchainApplicationService
└── infrastructure/
    ├── persistence/       # BlockStorage, TransactionRepository
    ├── networking/        # BlockPropagation, PeerSync
    └── consensus/         # PoAConsensusAdapter
```

## 🔋 Energy-Centric Blockchain Design

### ⚡ **Energy Transaction Types**
| Type | Purpose | Energy Impact | Authority Role |
|------|---------|---------------|----------------|
| **EnergyTrade** | P2P energy exchange | Direct energy transfer | Validation |
| **GridBalance** | System stabilization | Grid correction | Authority-initiated |
| **RenewableCert** | Green energy proof | Certificate tracking | MEA/PEA certification |
| **EmergencyGrid** | Crisis response | Emergency protocols | EGAT override |

### 🏗️ **Block Structure Design**
#### **Block Entity Design** (`src/domains/blockchain_infrastructure/domain/entities/block.rs`)
```
🔸 BlockHeader
├── 🔗 previous_hash: Hash
├── 📊 merkle_root: Hash  
├── ⏰ timestamp: Timestamp
├── 🌐 grid_state_hash: Hash
├── ✍️ authority_signatures: Vec<AuthoritySignature>
└── 📈 energy_metrics: EnergyMetrics

🔸 BlockBody
├── 💱 energy_transactions: Vec<EnergyTransaction>
├── 🏛️ governance_transactions: Vec<GovernanceTransaction>
├── ⚡ grid_transactions: Vec<GridTransaction>
└── 🌱 renewable_certificates: Vec<RenewableCertificate>
```

#### **Transaction Value Objects** (`src/domains/blockchain_infrastructure/domain/value_objects/`)
- **TransactionId**: Unique immutable identifier
- **EnergyAmount**: kWh with validation (must be positive, realistic bounds)
- **GridToken**: 1:1 ratio with energy, precision handling
- **AuthoritySignature**: Multi-signature validation for authority nodes
- **GridLocation**: Thai grid coordinate system integration

## 🔄 Business Rules & Domain Logic

### ⚖️ **Energy Conservation Validation**
```
Business Rule: Total Energy In = Total Energy Out + Transmission Losses
├── 🔍 Pre-block validation: Sum all input/output energy
├── 📊 Grid loss calculation: Based on transmission distance/load
├── ✅ Conservation check: Equation must balance within tolerance
└── 🚨 Rejection: Block invalid if conservation violated
```

### 🏛️ **Authority Privileges & Validation**
| Authority | Privileges | Validation Requirements |
|-----------|------------|-------------------------|
| **EGAT** | Emergency grid protocols, transmission management | Multi-sig + timestamp validation |
| **MEA** | Bangkok distribution, renewable certificates | Geographic bounds check |
| **PEA** | Provincial distribution, rural grid management | License verification |

### ⚡ **Performance & Consensus Rules**
- **Block Time**: 10 seconds (optimized for energy trading speed)
- **Block Size**: Max 1MB (balance throughput vs propagation)
- **Finality**: 3 confirmations (energy trading certainty)
- **Fork Resolution**: Authority consensus with grid state priority

## 🏗️ Domain Services & Application Layer

### 🔧 **Domain Services** (`domain/services/`)

#### **BlockValidationService**
- Energy conservation law enforcement
- Grid constraint validation  
- Authority signature verification
- Transaction ordering and dependencies

#### **ConsensusService** 
- PoA consensus with authority nodes
- Grid state priority resolution
- Emergency protocol handling
- Cross-authority coordination

### 🚀 **Application Services** (`application/services/`)

#### **BlockchainApplicationService**
- Block creation and validation orchestration
- Transaction pool management
- Authority integration coordination  
- Grid state synchronization

### 📝 **Commands & Queries** (`application/commands/`, `application/queries/`)

#### **Commands**
- `CreateEnergyBlock`: New block with energy transactions
- `ValidateTransaction`: Pre-block transaction validation
- `HandleEmergencyProtocol`: Authority emergency response
- `ProcessGridUpdate`: Real-time grid state updates

#### **Queries**
- `GetBlockByHash`: Retrieve specific block data
- `GetTransactionHistory`: Energy trading audit trail
- `GetGridStateAtBlock`: Historical grid state
- `GetAuthorityTransactions`: Authority action tracking

## 🔒 Security & Validation Patterns

### 🛡️ **Cryptographic Standards**
- **Hash Function**: SHA-256 for block linking
- **Digital Signatures**: ECDSA for authority authentication
- **Merkle Trees**: Transaction integrity and efficient verification
- **Multi-Signature**: Authority coordination and override protection

### ⚖️ **Validation Layers**
1. **Syntax Validation**: Basic format and structure checks
2. **Semantic Validation**: Business rule enforcement
3. **Authority Validation**: Signature and privilege verification
4. **Energy Validation**: Conservation laws and grid constraints
5. **State Validation**: Consistency with current blockchain state

## 🧪 Testing Strategy

### 🔬 **Domain Testing**
- **Energy Conservation Tests**: Verify all scenarios maintain energy balance
- **Authority Privilege Tests**: Ensure proper access control
- **Grid Constraint Tests**: Validate grid stability requirements
- **Performance Tests**: Block creation and validation speed

### 🎭 **Integration Testing**
- **Authority Node Integration**: Real MEA/PEA/EGAT coordination
- **Grid State Synchronization**: Real-time grid data integration
- **Consensus Coordination**: Multi-authority decision making
- **Emergency Protocol Testing**: Crisis response validation

## 📚 Key Implementation Patterns

### 🔄 **Aggregate Pattern**
- **BlockchainState**: Maintains current state, handles state transitions
- **TransactionPool**: Manages pending transactions, ordering, validation
- **AuthorityCoordination**: Manages multi-authority consensus

### 🎯 **Repository Pattern**
- **BlockRepository**: Persistent block storage and retrieval
- **TransactionRepository**: Transaction indexing and querying
- **GridStateRepository**: Historical grid state management

### 📨 **Event Sourcing**
- **BlockCreated**: New block added to chain
- **TransactionValidated**: Transaction passed validation
- **AuthoritySignatureReceived**: Authority endorsement
- **GridStateUpdated**: Real-time grid changes

---

## 🎯 Implementation Focus Areas

> **Start with**: Energy conservation validation and authority integration
> **Priority**: Grid stability and real-time transaction processing  
> **Remember**: Every blockchain operation must respect energy physics and Thai grid requirements
- Use `anyhow::Result` for comprehensive error context
- Custom error types for energy trading violations
- Grid stability errors must trigger emergency protocols
- Graceful degradation during authority node failures

### Testing Requirements
- Unit tests for all blockchain operations
- Integration tests with mock grid data
- Load testing for peak energy trading hours
- Chaos engineering for authority node failures

## Thai Energy Market Integration

### Regulatory Compliance
- Energy Trading Act B.E. 2562 (2019) compliance
- NEPO (National Energy Policy Office) regulations
- ERC (Energy Regulatory Commission) oversight
- Real-time reporting to authorities

### Market Operations
- Day-ahead energy market integration
- Real-time balancing market participation  
- Ancillary services (frequency regulation, reserves)
- Renewable energy certificate trading

When implementing blockchain core features, ensure thread safety, efficient storage, and seamless integration with the energy trading system while maintaining regulatory compliance and grid stability.
