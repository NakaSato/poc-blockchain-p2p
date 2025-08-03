# GridTokenX Blockchain Codebase Documentation

## Overview

GridTokenX is a revolutionary blockchain-based platform for peer-to-peer energy trading in Thailand's electricity market. The codebase implements a hybrid consensus mechanism with authority nodes, real-time energy trading, and grid integration.

## Project Structure

```
src/
├── lib.rs                          # Main library entry point
├── main.rs                         # Binary executable entry point
├── config.rs                       # Configuration management
├── storage.rs                      # Data persistence layer
├── utils.rs                        # Utility functions
├── api.rs                          # REST API and WebSocket services
├── consensus.rs                    # Consensus mechanism implementation
├── energy.rs                       # Energy trading system
├── governance.rs                   # DAO governance system
├── p2p.rs                          # Peer-to-peer networking
├── blockchain/                     # Core blockchain components
│   ├── mod.rs                     # Blockchain module exports
│   ├── block.rs                   # Block structure and validation
│   ├── chain.rs                   # Blockchain management
│   └── transaction.rs             # Transaction types and validation
├── consensus/                      # Consensus mechanism components
│   ├── mod.rs                     # Consensus module exports
│   └── poa.rs                     # Proof of Authority implementation
└── *_tests.rs                     # Test files for various modules
```

---

## Core Files Explanation

### 1. `src/lib.rs` - Library Entry Point

**Purpose**: Main library interface that re-exports commonly used types and provides the public API for the GridTokenX blockchain library.

**Key Components**:
- **Module Declarations**: Exposes all major modules (blockchain, config, storage, utils)
- **Re-exports**: Provides convenient access to frequently used types
- **Library Metadata**: Version, name, and description constants
- **Basic Tests**: Simple integration tests for library functionality

**Key Exports**:
```rust
pub use blockchain::{Block, Blockchain, Transaction, TransactionType};
pub use config::NodeConfig;
pub use storage::StorageManager;
pub use utils::{crypto, EnergyConversion, ThaiEnergyMarket, Utils};
```

**Usage Example**:
```rust
use gridtokenx_blockchain::{Blockchain, NodeConfig, StorageManager};
```

---

### 2. `src/main.rs` - Executable Entry Point

**Purpose**: Command-line interface for running GridTokenX blockchain nodes with support for different node types and operations.

**Key Features**:
- **CLI Interface**: Uses `clap` for command-line argument parsing
- **Multiple Commands**: Start node, initialize blockchain, show status, import/export data
- **Node Types**: Supports validator, trader, and observer nodes
- **Configuration Loading**: Reads TOML configuration files
- **Logging Setup**: Configurable logging with tracing

**Available Commands**:
- `start`: Launch a blockchain node with specified configuration
- `init`: Initialize a new blockchain with genesis block
- `status`: Display current node and blockchain status
- `import`: Import blockchain data from file
- `export`: Export blockchain data to file

**Command Example**:
```bash
./gridtokenx-node start --config config.toml --mining --node-type validator
```

---

### 3. `src/config.rs` - Configuration Management

**Purpose**: Comprehensive configuration system for all aspects of the GridTokenX node, including Thai energy market specific settings.

**Key Configuration Sections**:

#### Core Node Settings
- **NodeConfig**: Main configuration structure
- **NodeType**: Validator, Trader, Observer, Authority, Archive
- **Environment**: Development, Testing, Staging, Production

#### Network & P2P Configuration
- **NetworkConfig**: Blockchain network parameters
- **P2PConfig**: libp2p networking settings (ports, discovery, protocols)
- **ApiConfig**: REST API and WebSocket settings

#### Thai Energy Market Integration
- **ThaiMarketConfig**: Market hours, holidays, regional settings
- **GridConfig**: Grid integration, SCADA connectivity, emergency protocols
- **EnergyConfig**: Trading parameters, pricing algorithms, settlement

#### Authority Integration
- **AuthorityConfig**: EGAT, MEA, PEA connection settings
- **ConsensusConfig**: PoA parameters, validator settings, finality rules

#### Security & Performance
- **SecurityConfig**: Encryption, authentication, rate limiting
- **StorageConfig**: Database settings, backup, retention policies
- **LoggingConfig**: Log levels, destinations, rotation

**Example Configuration Loading**:
```rust
let config = NodeConfig::load_from_file("config.toml")?;
let thai_settings = &config.thai_market;
```

---

### 4. `src/storage.rs` - Data Persistence Layer

**Purpose**: High-performance storage layer using RocksDB for blockchain data, energy trading records, and grid information.

**Key Components**:

#### StorageManager
- **Database Management**: RocksDB instance with column families
- **Performance Optimization**: Write batching, compression, caching
- **Backup & Recovery**: Automated backups, point-in-time recovery

#### Column Families Organization
- **Blocks**: Block data indexed by height and hash
- **Transactions**: Transaction data with multiple indexes
- **Energy Orders**: Active and historical energy trading orders
- **Grid State**: Real-time grid monitoring data
- **Authorities**: Authority node information and signatures
- **Governance**: DAO proposals and voting records

#### Storage Operations
- **Atomic Transactions**: ACID compliance for complex operations
- **Indexing**: Multi-dimensional indexing for efficient queries
- **Compression**: LZ4 compression for space efficiency
- **Metrics**: Performance monitoring and alerting

**Example Usage**:
```rust
let storage = StorageManager::new("./data").await?;
let block = storage.get_block_by_height(1000).await?;
storage.store_energy_order(&order).await?;
```

---

### 5. `src/blockchain/` - Core Blockchain Components

#### `blockchain/mod.rs` - Module Organization
**Purpose**: Centralizes blockchain functionality and exports core types.

**Key Exports**:
- Block, Blockchain, Transaction types
- BlockchainConfig with Thai energy market parameters
- BlockchainStats for monitoring and analytics
- Account structures for energy trading participants

#### `blockchain/block.rs` - Block Structure
**Purpose**: Defines block structure optimized for energy trading with grid state integration.

**Key Features**:
- **Energy-Specific Fields**: Total energy traded, renewable percentage
- **Grid Integration**: Grid state hash, congestion levels
- **Authority Signatures**: Multi-signature support for EGAT/MEA/PEA
- **Merkle Trees**: Efficient transaction verification
- **Validation Rules**: Energy conservation, grid constraints

**Block Structure**:
```rust
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub total_energy: f64,           // Total energy in this block
    pub grid_state_hash: String,     // Grid state after block
    pub authority_signatures: Vec<AuthoritySignature>,
}
```

#### `blockchain/transaction.rs` - Transaction Types
**Purpose**: Defines various transaction types for energy trading, governance, and grid operations.

**Transaction Types**:
- **EnergyTransaction**: Energy production, consumption, trading
- **GovernanceTransaction**: DAO proposals, voting, authority management
- **AuthorityTransaction**: Grid state updates, emergency actions
- **StandardTransaction**: Token transfers, smart contracts

**Energy Transaction Example**:
```rust
pub struct EnergyTransaction {
    pub transaction_id: String,
    pub energy_amount: f64,          // kWh
    pub energy_type: EnergyType,     // Solar, Wind, etc.
    pub grid_location: GridLocation,
    pub timestamp: DateTime<Utc>,
    pub meter_reading: MeterReading,
    pub renewable_certificate: Option<RenewableCertificate>,
}
```

#### `blockchain/chain.rs` - Blockchain Management
**Purpose**: Core blockchain operations including validation, consensus, and state management.

**Key Features**:
- **Chain Validation**: Energy conservation, authority consensus
- **State Management**: Account balances, energy credits
- **Fork Resolution**: Authority-based finality
- **Performance Optimization**: Parallel validation, caching

---

### 6. `src/energy.rs` - Energy Trading System

**Purpose**: Implements the sophisticated energy trading platform with real-time order matching and grid integration.

**Key Components**:

#### EnergyTrading Manager
- **Order Management**: Submit, cancel, modify energy orders
- **Matching Engine**: Price-time priority with grid constraints
- **Settlement**: Automatic settlement and clearing
- **Risk Management**: Position limits, credit checks

#### GridManager
- **Real-Time Monitoring**: Grid frequency, voltage, power flows
- **Congestion Management**: Dynamic pricing based on grid constraints
- **Emergency Response**: Automatic load shedding, generation dispatch
- **Integration**: SCADA systems, EMS connectivity

#### Market Operations
- **Order Types**: Market, limit, stop, grid-balancing orders
- **Time Slots**: 15-minute trading intervals
- **Pricing**: Dynamic pricing with congestion multipliers
- **Clearing**: Continuous and periodic clearing mechanisms

**Trading Example**:
```rust
let sell_order = EnergyOrder {
    order_type: OrderType::Limit(150), // 150 tokens per kWh
    energy_amount: 100.0,              // 100 kWh
    grid_zone: GridZone::Bangkok,
    renewable_only: true,
};
energy_trading.submit_order(sell_order).await?;
```

---

### 7. `src/consensus.rs` - Consensus Mechanism

**Purpose**: Implements hybrid Proof of Authority (PoA) consensus with Thai energy authority integration.

**Key Features**:

#### Authority Management
- **EGAT/MEA/PEA Integration**: Official energy authority nodes
- **Authority Registration**: Government verification process
- **Signature Validation**: Multi-signature consensus
- **Emergency Override**: Grid stability emergency powers

#### Validator System
- **Stake-Based Selection**: Economic incentives for validators
- **Performance Monitoring**: Uptime, response time tracking
- **Slashing Conditions**: Penalties for malicious behavior
- **Rotation Schedule**: Regular validator set updates

#### Consensus Rules
- **Block Production**: Authority nodes have priority
- **Finality**: Immediate finality for emergency blocks
- **Energy Validation**: Smart meter data verification
- **Grid Constraints**: Respect transmission capacity limits

---

### 8. `src/governance.rs` - DAO Governance System

**Purpose**: Decentralized governance system for community decision-making with regulatory compliance.

**Key Features**:

#### Proposal System
- **Proposal Types**: Infrastructure, market rules, technical upgrades
- **Submission Process**: Stake-based proposal submission
- **Review Process**: Authority review for regulatory compliance
- **Voting Mechanisms**: Weighted voting based on stake and participation

#### Stakeholder Participation
- **Voting Weights**: Based on energy participation, stake, reputation
- **Delegation**: Vote delegation with scope limitations
- **Quorum Requirements**: Minimum participation thresholds
- **Authority Veto**: Regulatory compliance override powers

#### Emergency Governance
- **Emergency Protocols**: Rapid response for grid stability
- **Authority Powers**: EGAT emergency decision authority
- **Transparency**: Public audit trail of all decisions

---

### 9. `src/p2p.rs` - Peer-to-Peer Networking

**Purpose**: libp2p-based networking layer optimized for energy trading and grid data propagation.

**Key Features**:

#### Network Protocols
- **GossipSub**: Message broadcasting for trading and grid data
- **Kademlia DHT**: Peer discovery and content routing
- **Noise Protocol**: Encrypted communication
- **mDNS**: Local network discovery

#### Message Types
- **Trading Messages**: Order submissions, matches, settlements
- **Grid Messages**: Real-time grid state updates
- **Consensus Messages**: Block proposals, votes, finality
- **Emergency Messages**: Grid emergency notifications

#### Performance Optimization
- **Priority Routing**: Critical messages get priority
- **Geographic Clustering**: Regional peer grouping
- **Bandwidth Management**: Rate limiting and compression
- **Redundancy**: Multiple path routing for reliability

---

### 10. `src/api.rs` - REST API and WebSocket Services

**Purpose**: External interface for energy trading, grid monitoring, and blockchain interaction.

**Key Endpoints**:

#### Blockchain API
- **GET /api/v1/blocks/{height}**: Retrieve block information
- **GET /api/v1/transactions/{hash}**: Transaction details
- **POST /api/v1/transactions**: Submit new transactions
- **GET /api/v1/status**: Blockchain and node status

#### Energy Trading API
- **POST /api/v1/orders**: Submit energy orders
- **GET /api/v1/orders**: Query energy orders
- **DELETE /api/v1/orders/{id}**: Cancel orders
- **GET /api/v1/market-data**: Real-time market data

#### Grid Management API
- **GET /api/v1/grid/status**: Current grid status
- **POST /api/v1/grid/data**: Submit grid measurements (authority only)
- **GET /api/v1/grid/forecast**: Grid demand/supply forecast

#### WebSocket Feeds
- **ws://host/ws/trading**: Real-time trading updates
- **ws://host/ws/grid**: Grid status updates
- **ws://host/ws/blocks**: New block notifications

---

### 11. `src/utils.rs` - Utility Functions

**Purpose**: Common utility functions for cryptography, energy calculations, and Thai market operations.

**Key Modules**:

#### Cryptographic Utils
- **Key Generation**: Ed25519 keypair generation
- **Digital Signatures**: Message signing and verification
- **Hashing**: SHA-256 hashing utilities
- **Authority Signatures**: Special authority signature handling

#### Energy Conversion Utils
- **Unit Conversion**: kWh ↔ tokens, MW ↔ kW conversions
- **Carbon Calculations**: CO2 emissions per energy source
- **Renewable Certificates**: REC value calculations

#### Thai Market Utils
- **Trading Hours**: Thai market operating hours
- **Holiday Detection**: Thai public holiday calendar
- **Seasonal Pricing**: Hot/cool season multipliers
- **Grid Zone Validation**: Thai grid zone verification

---

## Test Files

### Unit Tests
- **`*_tests.rs`**: Comprehensive unit tests for each module
- **Integration Tests**: End-to-end trading lifecycle tests
- **Performance Tests**: Load testing for peak trading hours
- **Compliance Tests**: Thai energy regulation compliance

### Test Coverage Areas
- **Energy Conservation**: Validation of energy balance laws
- **Authority Integration**: EGAT/MEA/PEA system integration
- **Market Operations**: Order matching and settlement
- **Grid Stability**: Emergency response procedures
- **Security**: Cryptographic verification and attack resistance

---

## Configuration Files

### `config.toml` - Main Configuration
Contains all node settings, Thai market parameters, and authority integration details.

### Environment-Specific Configs
- **Development**: Fast block times, debug logging
- **Testing**: In-memory storage, mock authorities
- **Production**: Full security, authority integration, monitoring

---

## Key Design Principles

### 1. Energy Conservation
All transactions must respect the fundamental law: Energy In = Energy Out + Losses

### 2. Thai Regulatory Compliance
Full compliance with Energy Trading Act B.E. 2562 (2019) and ERC regulations

### 3. Grid Stability Priority
Grid stability takes precedence over trading optimization

### 4. Real-Time Performance
Sub-second response times for critical grid operations

### 5. Authority Integration
Seamless integration with EGAT, MEA, and PEA systems

### 6. Transparency
Public audit trail for all market operations and governance decisions

---

This codebase represents a sophisticated blockchain platform specifically designed for Thailand's energy market, combining cutting-edge blockchain technology with real-world energy trading requirements and regulatory compliance.
