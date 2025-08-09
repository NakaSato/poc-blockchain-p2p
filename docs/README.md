# GridTokenX Blockchain

A revolutionary blockchain-based platform that enables peer-to-peer energy trading in Thailand's electricity market. Built on a hybrid architecture combining traditional and decentralized systems, GridTokenX facilitates efficient energy distribution while promoting renewable energy adoption and grid stability.

## 🌟 Key Features

- **Peer-to-Peer Energy Trading**: Direct energy transactions between producers and consumers
- **1:1 Token-Energy Ratio**: Stable token economics with 1 kWh = 1 Token
- **Grid Integration**: Real-time grid management and congestion control
- **Renewable Energy Focus**: Carbon tracking and sustainability metrics
- **Governance System**: Community-driven decision making
- **Regulatory Compliance**: Full compliance with Thai energy regulations

## 🏗️ Architecture

GridTokenX is built using **Domain-Driven Design (DDD)** principles with a modular architecture that combines:

### Consensus Mechanisms
- **Proof of Authority (PoA)**: Primary consensus for energy trading validation
- **Proof of Stake (PoS)**: For regular transactions and network governance  
- **Authority Nodes**: Integration with Thai energy authorities (EGAT, MEA, PEA)

### DDD Architecture Layers
- **Domain Layer**: Core business logic and energy trading rules
- **Application Layer**: CQRS with command/query handlers and event buses
- **Infrastructure Layer**: Storage, networking, and external system integrations
- **Shared Kernel**: Common types, errors, and cross-cutting concerns

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- Git
- At least 4GB RAM
- 10GB free disk space

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/gridtokenx/blockchain.git
   cd blockchain
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Initialize the blockchain**
   ```bash
   ./target/release/gridtokenx-node init
   ```

4. **Start the node**
   ```bash
   ./target/release/gridtokenx-node start
   ```

### Docker Setup

```bash
# Build Docker image
docker build -t gridtokenx-node .

# Run node
docker run -d --name gridtokenx \
  -p 8080:8080 \
  -p 9000:9000 \
  -v gridtokenx-data:/app/data \
  gridtokenx-node
```

## ⚙️ Configuration

The node can be configured using the `config.toml` file. Key sections include:

### Node Configuration
```toml
[node]
node_id = "gridtokenx-node-001"
node_type = "validator"  # validator, trader, observer, grid_operator, authority
```

### Network Settings
```toml
[network]
network_name = "gridtokenx-testnet"
network_id = 1001
max_peers = 50
```

### Energy Trading
```toml
[energy]
energy_token_ratio = 1.0  # 1 kWh = 1 Token
min_trade_amount = 0.1
max_trade_amount = 10000.0
```

### Thai Market Integration
```toml
[thai_market]
[thai_market.peak_hours]
start_hour = 18  # 6 PM
end_hour = 22    # 10 PM
pricing_multiplier = 1.5
```

## 🔌 API Reference

GridTokenX provides a comprehensive REST API for interacting with the blockchain.

### Base URL
```
http://localhost:8080/api/v1/
```

### Core Endpoints

#### Node Status
```bash
GET /status
```

#### Blockchain Operations
```bash
GET /blocks/{height}           # Get block by height
GET /blocks/hash/{hash}        # Get block by hash
POST /transactions             # Submit transaction
GET /transactions/{id}         # Get transaction details
```

#### Energy Trading
```bash
POST /energy/orders           # Submit energy order
GET /energy/orders            # Get active orders
GET /energy/stats             # Get trading statistics
GET /grid/status              # Get grid status
```

#### Account Management
```bash
GET /accounts/{address}       # Get account info
GET /accounts/{address}/balance  # Get account balance
```

### Example: Submit Energy Order

```bash
curl -X POST http://localhost:8080/api/v1/energy/orders \
  -H "Content-Type: application/json" \
  -d '{
    "order_type": "sell",
    "energy_amount": 100.0,
    "price_per_kwh": 5000,
    "energy_source": "solar",
    "grid_location": "BKK-01-SUB001",
    "expiration_hours": 24
  }'
```

## 🏛️ Governance

GridTokenX features a robust governance system allowing token holders to:

- Propose network parameter changes
- Vote on energy pricing regulations
- Manage energy authorities
- Respond to grid emergencies

### Creating a Proposal

```bash
# Using the CLI tool
./target/release/gridtokenx-cli governance propose \
  --title "Increase renewable energy incentives" \
  --description "Proposal to increase carbon credit rates for solar energy" \
  --type parameter-change \
  --parameter "carbon_credits.solar" \
  --new-value "0.7"
```

## 🏛️ Domain-Driven Design (DDD) Architecture

GridTokenX implements a robust DDD architecture that provides clear separation of concerns and maintainable code structure:

### 🎯 Shared Kernel
- **Domain Errors**: Centralized error handling with `DomainError`
- **Domain Events**: Event-driven architecture with `DomainEvent` trait
- **Repository Pattern**: Abstract data access with `AggregateRoot` and `Repository`
- **CQRS Buses**: Command, Query, and Event buses for clean application flow

### 🔄 Bounded Contexts

#### Energy Trading Domain
- **Value Objects**: `TradeId`, `TraderId`, `EnergyAmount`, `PricePerKwh`, `TradingWindow`
- **Entities**: `EnergyOrder` and `EnergyTrade` with complete lifecycle management
- **Aggregates**: `OrderBook` aggregate ensuring trading invariants
- **Domain Services**: `EnergyTradingDomainService` containing core business logic

### 📋 Application Patterns
- **Command Handlers**: Process trading commands with validation
- **Event Sourcing**: Track all domain events for audit and replay
- **Repository Pattern**: Clean data access abstraction
- **Anti-Corruption Layer**: Protect domain from external dependencies

### ⚙️ Migration Strategy
The project uses the **Strangler Fig pattern** to gradually migrate from legacy code to DDD:
- ✅ **Phase 1**: Shared kernel and domain foundation - COMPLETED
- ✅ **Phase 2**: Energy trading domain implementation - COMPLETED  
- 🔄 **Phase 3**: Grid management and governance domains - IN PROGRESS
- 📋 **Phase 4**: Complete legacy system retirement - PLANNED

## 🔋 Energy Trading

### Order Types

1. **Buy Orders**: Request to purchase energy
2. **Sell Orders**: Offer to sell energy
3. **Matched Trades**: Automatically matched orders

### Energy Sources

- Solar (0.5 carbon credits/kWh)
- Wind (0.6 carbon credits/kWh)
- Hydro (0.4 carbon credits/kWh)
- Biomass (0.3 carbon credits/kWh)
- Geothermal (0.7 carbon credits/kWh)

### Grid Integration

GridTokenX integrates with Thailand's power grid through:

- **SCADA Systems**: Real-time monitoring
- **Smart Meters**: Automated readings
- **Grid Operators**: EGAT, MEA, PEA integration

## 🔐 Security

### Cryptographic Features

- **Ed25519 Signatures**: Fast and secure digital signatures
- **SHA256 Hashing**: Proven cryptographic hash function
- **PBKDF2 Key Derivation**: Secure key generation

### Network Security

- **DDoS Protection**: Rate limiting and connection management
- **Peer Reputation**: Dynamic peer scoring system
- **Message Authentication**: All network messages are signed

## 🔧 Development

### Building from Source

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Run tests (including DDD domain tests)
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Check code quality
cargo clippy
cargo fmt --check

# Build documentation
cargo doc --open
```

### DDD Development Guidelines

#### Adding New Domains
1. Create new bounded context in `src/domains/`
2. Define domain entities and value objects
3. Implement aggregates with business invariants
4. Add domain services for complex business logic
5. Create application command/query handlers
6. Write comprehensive domain tests

#### Domain Layer Rules
- **Entities**: Must have identity and lifecycle
- **Value Objects**: Immutable with validation
- **Aggregates**: Enforce business invariants
- **Domain Services**: Stateless business logic
- **Events**: Capture domain state changes

#### Application Layer Patterns
- Use CQRS for read/write separation
- Implement command handlers for writes
- Use query handlers for reads  
- Publish domain events for integration

### Project Structure

```
poc-blockchain-p2p/
├── src/
│   ├── main.rs              # Main entry point
│   ├── lib.rs               # Library exports and DDD module organization
│   ├── api.rs               # REST API server
│   ├── config.rs            # Configuration management
│   ├── utils.rs             # Utility functions
│   ├── p2p.rs              # P2P networking
│   ├── storage.rs          # Data persistence
│   ├── consensus.rs        # Consensus algorithms (PoA)
│   ├── energy.rs           # Legacy energy trading (being migrated)
│   ├── governance.rs       # Governance system
│   │
│   ├── shared/             # 🎯 DDD Shared Kernel
│   │   ├── mod.rs
│   │   ├── domain/         # Domain primitives
│   │   │   ├── errors.rs   # Domain error types
│   │   │   ├── events.rs   # Domain events
│   │   │   ├── repository.rs # Repository patterns
│   │   │   └── value_objects.rs # Shared value objects
│   │   ├── application/    # Application layer patterns
│   │   │   ├── command_bus.rs # CQRS command bus
│   │   │   ├── query_bus.rs   # CQRS query bus
│   │   │   └── event_bus.rs   # Event-driven architecture
│   │   └── infrastructure/ # Infrastructure abstractions
│   │       ├── logging.rs  # Logging abstractions
│   │       ├── network.rs  # Network abstractions
│   │       └── storage.rs  # Storage abstractions
│   │
│   ├── domains/            # 🎯 DDD Bounded Contexts
│   │   ├── mod.rs
│   │   └── energy_trading/ # Energy Trading Domain
│   │       ├── mod.rs
│   │       ├── tests.rs    # Domain tests
│   │       ├── domain/     # Domain layer
│   │       │   ├── mod.rs
│   │       │   ├── value_objects.rs # Trade IDs, amounts, prices
│   │       │   ├── entities/       # Domain entities
│   │       │   │   ├── mod.rs
│   │       │   │   ├── energy_order.rs # Energy order entity
│   │       │   │   └── energy_trade.rs # Energy trade entity
│   │       │   ├── aggregates/     # Aggregate roots
│   │       │   │   ├── mod.rs
│   │       │   │   └── order_book.rs # Order book aggregate
│   │       │   └── services/       # Domain services
│   │       │       ├── mod.rs
│   │       │       └── energy_trading_service.rs
│   │       ├── application/        # Application layer
│   │       │   ├── mod.rs
│   │       │   └── commands/       # Command handlers
│   │       │       ├── mod.rs
│   │       │       └── place_energy_order.rs
│   │       └── infrastructure/     # Infrastructure layer
│   │           ├── mod.rs
│   │           └── repositories/   # Repository implementations
│   │
│   ├── blockchain/         # Core blockchain logic
│   │   ├── mod.rs
│   │   ├── block.rs        # Block structure
│   │   ├── chain.rs        # Blockchain management
│   │   └── transaction.rs  # Transaction types
│   │
│   ├── consensus_poa/      # Proof of Authority consensus
│   │   ├── mod.rs
│   │   └── poa.rs         # PoA implementation
│   │
│   └── scaling/           # Scaling solutions
│       ├── mod.rs
│       ├── sharding.rs    # Sharding implementation
│       └── sharding_complex.rs # Advanced sharding
│
├── config/                 # Environment-specific configs
│   ├── egat.toml          # EGAT authority configuration
│   ├── erc.toml           # ERC authority configuration  
│   ├── mea.toml           # MEA authority configuration
│   └── pea.toml           # PEA authority configuration
├── docs/                  # Documentation
├── config.toml            # Default configuration
├── Cargo.toml             # Rust dependencies
└── DDD_MIGRATION_PLAN.md  # DDD migration documentation
```

### Testing

```bash
# Run all tests
cargo test

# Run specific domain tests
cargo test domains::energy_trading::tests

# Run shared kernel tests  
cargo test shared::

# Run blockchain core tests
cargo test blockchain::tests

# Run with coverage
cargo tarpaulin --out Html

# Run DDD domain tests specifically
cargo test test_energy_trading_domain_service_creation
cargo test test_place_energy_order_command
```

### Test Organization
- **Unit Tests**: Domain logic and value object validation
- **Integration Tests**: Cross-domain interactions and API endpoints
- **Domain Tests**: Business rule validation and aggregate behavior
- **Repository Tests**: Data persistence and retrieval patterns

## 🚀 Deployment

### Production Deployment

1. **System Requirements**
   - CPU: 4+ cores
   - RAM: 8GB+
   - Storage: 100GB+ SSD
   - Network: 100Mbps+

2. **Security Hardening**
   ```bash
   # Generate secure node keys
   ./target/release/gridtokenx-node generate-keys
   
   # Set up firewall
   ufw allow 8080/tcp  # API port
   ufw allow 9000/tcp  # P2P port
   ```

3. **Monitoring Setup**
   ```bash
   # Prometheus metrics
   curl http://localhost:8080/metrics
   
   # Health check
   curl http://localhost:8080/api/v1/status
   ```

### Docker Compose

```yaml
version: '3.8'
services:
  gridtokenx-node:
    image: gridtokenx-node:latest
    ports:
      - "8080:8080"
      - "9000:9000"
    volumes:
      - ./data:/app/data
      - ./config.toml:/app/config.toml
    environment:
      - RUST_LOG=info
    restart: unless-stopped

  prometheus:
    image: prom/prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
```

## 📊 Monitoring

### Metrics

GridTokenX exposes Prometheus metrics at `/metrics`:

- `gridtokenx_blockchain_height` - Current blockchain height
- `gridtokenx_energy_trades_total` - Total energy trades
- `gridtokenx_network_peers` - Connected peers
- `gridtokenx_consensus_rounds` - Consensus rounds completed

### Logging

Logs are structured and can be configured for different outputs:

```toml
[logging]
level = "info"
format = "json"  # or "pretty"

[logging.file_logging]
enabled = true
file_path = "./logs/gridtokenx.log"
```

## 🤝 Contributing

We welcome contributions to GridTokenX! Please read our [contributing guidelines](CONTRIBUTING.md) before submitting PRs.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

### Code Style

- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new features

## 📜 License

GridTokenX is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🆘 Support

### Documentation

- [API Documentation](docs/api.md)
- [Energy Trading Guide](docs/energy-trading.md)
- [Node Operation Manual](docs/node-operation.md)
- [Governance Guide](docs/governance.md)

### Community

- **Discord**: [GridTokenX Community](https://discord.gg/gridtokenx)
- **Telegram**: [@gridtokenx](https://t.me/gridtokenx)
- **Twitter**: [@GridTokenX](https://twitter.com/GridTokenX)

### Issues and Bugs

Please report issues on our [GitHub Issues](https://github.com/gridtokenx/blockchain/issues) page.

### Professional Support

For enterprise support and custom implementations, contact: enterprise@gridtokenx.com

## 🗓️ Roadmap

### Phase 1 (Q1 2024) - Foundation ✅
- Core blockchain implementation
- Basic energy trading
- P2P networking
- Web API

### Phase 2 (Q2 2024) - DDD Architecture ✅
- Domain-Driven Design implementation
- Shared kernel and bounded contexts
- Energy trading domain with CQRS
- Event-driven architecture
- Comprehensive test suite

### Phase 3 (Q3 2024) - Integration 🔄
- Thai grid operator integration
- Smart contract platform
- Mobile applications
- Regulatory compliance tools
- Additional domain contexts (Grid, Governance)

### Phase 4 (Q4 2024) - Scaling
- Sharding implementation
- Cross-chain bridges
- Advanced analytics
- AI-powered grid optimization

### Phase 5 (Q1 2025) - Ecosystem
- DeFi integrations
- Carbon credit marketplace
- IoT device integration
- International expansion

## 📈 Performance

### Benchmarks

- **Transaction Throughput**: 1,000+ TPS
- **Block Time**: 10 seconds
- **Energy Order Matching**: <100ms
- **Network Latency**: <500ms (Thailand)

### Optimization

- Asynchronous processing with Tokio
- Efficient storage with RocksDB
- Optimized serialization with bincode
- Connection pooling for APIs

## 🌐 Ecosystem

### Partners

- **EGAT**: Electricity Generating Authority of Thailand
- **MEA**: Metropolitan Electricity Authority
- **PEA**: Provincial Electricity Authority
- **ERC**: Energy Regulatory Commission

### Integrations

- **Smart Meters**: AMI-compliant devices
- **Solar Inverters**: SolarEdge, Huawei, SMA
- **Battery Systems**: Tesla Powerwall, LG Chem
- **EV Chargers**: ChargePoint, EVBox

---

**GridTokenX** - Powering Thailand's Energy Future 🇹🇭⚡

For the latest updates, visit our [website](https://gridtokenx.com) or follow us on [GitHub](https://github.com/gridtokenx/blockchain).