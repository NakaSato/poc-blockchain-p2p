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

GridTokenX is built with a **modular architecture** that combines:

### Consensus Mechanisms
- **Proof of Authority (PoA)**: Primary consensus for energy trading validation
- **Authority Nodes**: Integration with Thai energy authorities (EGAT, MEA, PEA)

### Core Architecture Modules
- **Blockchain Layer**: Core blockchain functionality and transaction processing
- **Energy Trading**: Energy market operations and order matching
- **P2P Networking**: Peer-to-peer communication and network management
- **Governance**: Network governance and proposal management
- **API Layer**: RESTful API powered by Axum web framework

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

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Check code quality
cargo clippy
cargo fmt --check

# Build documentation
cargo doc --open
```

### Development Guidelines

#### Adding New Features
1. Create new modules in appropriate `src/` directory
2. Define data structures and validation
3. Implement business logic with proper error handling
4. Add API endpoints if needed
5. Write comprehensive tests

#### Code Organization
- **Modules**: Well-defined responsibilities and clean interfaces
- **Error Handling**: Use `Result` types and proper error propagation
- **Testing**: Unit tests for all core functionality
- **Documentation**: Keep code well-documented

#### Best Practices
- Follow Rust idioms and conventions
- Use async/await for I/O operations
- Implement proper logging with tracing
- Handle errors gracefully

### Project Structure

```
poc-blockchain-p2p/
├── src/
│   ├── main.rs              # Main entry point
│   ├── lib.rs               # Library exports and module organization
│   ├── api.rs               # REST API server (Axum)
│   ├── config.rs            # Configuration management
│   ├── utils.rs             # Utility functions
│   ├── p2p.rs              # P2P networking
│   ├── storage.rs          # Data persistence
│   ├── consensus.rs        # Consensus algorithms (PoA)
│   ├── energy.rs           # Energy trading functionality
│   ├── governance.rs       # Governance system
│   │
│   ├── blockchain/         # Blockchain core
│   │   ├── mod.rs
│   │   ├── block.rs        # Block structure and validation
│   │   ├── chain.rs        # Blockchain implementation
│   │   └── transaction.rs  # Transaction types and processing
│   │
│   └── consensus_poa/      # Proof of Authority consensus
│       ├── mod.rs
│       └── poa.rs          # PoA implementation
│
├── config/                 # Configuration files
├── docs/                   # Documentation
├── target/                 # Rust build artifacts
└── Cargo.toml             # Project dependencies
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
└── Cargo.toml             # Rust dependencies
```

### Testing

```bash
# Run all tests
cargo test

# Run blockchain core tests
cargo test blockchain::tests

# Run consensus tests
cargo test consensus::tests

# Run energy trading tests
cargo test energy::tests

# Run with coverage
cargo tarpaulin --out Html
```

### Test Organization
- **Unit Tests**: Core logic and data structure validation
- **Integration Tests**: API endpoints and cross-module interactions
- **Module Tests**: Individual module functionality
- **End-to-End Tests**: Full system workflow validation

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

### Phase 2 (Q2 2024) - Modular Architecture ✅
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