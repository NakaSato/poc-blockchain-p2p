---
mode: edit
---

# GridTokenX P2P Network Development Prompt

You are developing the peer-to-peer networking layer for GridTokenX - Thailand's energy trading blockchain platform using libp2p for robust, decentralized communication.

## Network Architecture Overview

The P2P network (`src/p2p.rs`) implements a multi-layered networking system designed for:
- **High Availability**: 99.99% uptime for critical energy trading
- **Low Latency**: <100ms message propagation for real-time energy trading
- **Scalability**: Support for 10,000+ nodes across Thailand
- **Security**: Encrypted communication with identity verification

## libp2p Stack Configuration

### Transport Layer
```rust
use libp2p::{
    tcp, noise, yamux, gossipsub, kad, mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    identity::Keypair,
    PeerId, Multiaddr,
};

#[derive(NetworkBehaviour)]
pub struct GridTokenXBehaviour {
    pub gossipsub: gossipsub::Behaviour,      // Message broadcasting
    pub kademlia: kad::Behaviour<MemoryStore>, // Peer discovery and routing
    pub mdns: mdns::tokio::Behaviour,         // Local network discovery
}
```

### Network Protocols
1. **TCP**: Primary transport for reliability
2. **Noise**: Encryption and authentication
3. **Yamux**: Connection multiplexing
4. **GossipSub**: Message dissemination
5. **Kademlia DHT**: Peer discovery and content routing
6. **mDNS**: Local peer discovery

## Node Types and Roles

### Authority Nodes (EGAT, MEA, PEA)
```rust
pub struct AuthorityNodeConfig {
    pub node_type: AuthorityType,
    pub static_addresses: Vec<Multiaddr>,     // Well-known addresses
    pub priority_routing: bool,               // Preferential message routing
    pub emergency_broadcast: bool,            // Emergency message capabilities
    pub grid_data_authority: bool,           // Authoritative grid state
}
```

### Validator Nodes
- **Stake Validators**: High-stake participants with enhanced responsibilities
- **Energy Validators**: Nodes with verified energy production/consumption
- **Community Validators**: Standard validator nodes with minimum stake

### Trading Nodes
- **Producer Nodes**: Energy generators (solar, wind, traditional plants)
- **Consumer Nodes**: Energy buyers (homes, businesses, factories)
- **Trader Nodes**: Market makers and energy arbitrageurs

### Observer Nodes
- **Monitor Nodes**: Grid monitoring and analytics
- **Archive Nodes**: Full blockchain history storage
- **Research Nodes**: Academic and research institutions

## Message Types and Routing

### Critical Messages (Priority Routing)
```rust
pub enum CriticalMessage {
    GridEmergency {
        severity: EmergencyLevel,
        affected_zones: Vec<GridZone>,
        required_action: EmergencyAction,
        authority_signature: AuthoritySignature,
    },
    SystemShutdown {
        reason: ShutdownReason,
        grace_period: Duration,
        authority_signature: AuthoritySignature,
    },
}
```

### Trading Messages
```rust
pub enum TradingMessage {
    EnergyOrder {
        order_id: String,
        order_type: OrderType,
        price: u64,
        quantity: f64,
        location: GridZone,
        expiry: DateTime<Utc>,
    },
    OrderMatch {
        buy_order: String,
        sell_order: String,
        matched_price: u64,
        matched_quantity: f64,
        execution_time: DateTime<Utc>,
    },
    MarketData {
        zone: GridZone,
        current_price: u64,
        volume_24h: f64,
        grid_congestion: f64,
    },
}
```

### Consensus Messages
```rust
pub enum ConsensusMessage {
    BlockProposal {
        block: Block,
        proposer: PeerId,
        authority_endorsement: Option<AuthoritySignature>,
    },
    BlockVote {
        block_hash: String,
        vote: Vote,
        validator_signature: ValidatorSignature,
    },
    FinalityProof {
        block_hash: String,
        finality_signatures: Vec<AuthoritySignature>,
    },
}
```

## Network Topology and Optimization

### Geographic Distribution
```rust
pub struct NetworkTopology {
    pub regions: HashMap<Region, Vec<PeerId>>,
    pub authority_backbone: Vec<PeerId>,      // Authority node interconnections
    pub trading_clusters: Vec<TradingCluster>, // Regional trading groups
    pub redundancy_paths: Vec<RedundancyPath>, // Backup communication routes
}

pub enum Region {
    Bangkok,           // MEA territory
    Central,           // Central Thailand
    North,             // Northern provinces
    Northeast,         // Isan region
    East,              // Eastern seaboard
    West,              // Western provinces
    South,             // Southern Thailand
}
```

### Connection Management
- **Authority Connections**: Maintain persistent connections to all authority nodes
- **Trading Connections**: Dynamic connections based on trading activity
- **Regional Clusters**: Prefer connections within same grid regions
- **Redundancy**: Multiple path routing for critical messages

### Bandwidth Optimization
```rust
pub struct BandwidthManagement {
    pub priority_queues: HashMap<MessagePriority, VecDeque<Message>>,
    pub rate_limiting: HashMap<PeerId, RateLimiter>,
    pub compression: CompressionConfig,
    pub batching: BatchingConfig,
}
```

## Security and Authentication

### Node Identity Verification
```rust
pub struct NodeIdentity {
    pub peer_id: PeerId,
    pub node_type: NodeType,
    pub credentials: NodeCredentials,
    pub reputation: ReputationScore,
    pub energy_certificates: Vec<EnergyCertificate>,
}

pub struct NodeCredentials {
    pub government_license: Option<String>,    // For authority nodes
    pub energy_license: Option<String>,       // For energy producers
    pub stake_proof: Option<StakeProof>,      // For validators
    pub grid_connection_proof: GridConnectionProof,
}
```

### Message Authentication
- **Digital Signatures**: All messages signed with node private keys
- **Authority Verification**: Special verification for authority node messages
- **Replay Protection**: Nonce and timestamp verification
- **Integrity Checks**: Cryptographic hashes for message integrity

### Anti-Spam and Rate Limiting
```rust
pub struct SecurityPolicy {
    pub max_messages_per_second: u32,
    pub max_connections_per_ip: u32,
    pub reputation_threshold: f64,
    pub blacklist: HashSet<PeerId>,
    pub emergency_mode: bool,
}
```

## Discovery and Bootstrapping

### Bootstrap Nodes
- **Authority Nodes**: EGAT, MEA, PEA nodes as bootstrap points
- **University Nodes**: Academic institutions as neutral bootstrap nodes
- **Community Nodes**: Well-known community validator nodes
- **Geographic Distribution**: Bootstrap nodes in each major region

### Peer Discovery Strategy
```rust
impl PeerDiscovery {
    pub async fn discover_trading_peers(&self, grid_zone: GridZone) -> Vec<PeerId> {
        // Prioritize peers in same grid zone for lower latency
        // Use Kademlia DHT for peer discovery
        // Filter by node type and reputation
    }
    
    pub async fn discover_validators(&self) -> Vec<PeerId> {
        // Find active validator nodes
        // Verify stake and credentials
        // Prioritize by uptime and performance
    }
}
```

### Network Health Monitoring
```rust
pub struct NetworkHealth {
    pub connected_peers: u32,
    pub authority_connectivity: f64,          // % of authority nodes connected
    pub regional_connectivity: HashMap<Region, f64>,
    pub average_latency: Duration,
    pub message_success_rate: f64,
    pub network_partition_risk: f64,
}
```

## Performance Optimization

### Message Propagation
- **Eager Push**: Immediate forwarding of critical messages
- **Lazy Pull**: On-demand retrieval of non-critical data
- **Mesh Optimization**: Dynamic mesh topology for efficiency
- **Scoring System**: Peer scoring for optimal routing

### Connection Pooling
```rust
pub struct ConnectionPool {
    pub persistent_connections: HashMap<NodeType, Vec<Connection>>,
    pub connection_cache: LruCache<PeerId, Connection>,
    pub connection_metrics: ConnectionMetrics,
}
```

### Load Balancing
- **Regional Load Distribution**: Distribute load across regions
- **Authority Load Balancing**: Spread requests across authority nodes
- **Trading Session Management**: Manage peak trading hour loads
- **Graceful Degradation**: Reduce functionality during high load

## Fault Tolerance and Recovery

### Network Partitioning
```rust
pub struct PartitionRecovery {
    pub partition_detection: PartitionDetector,
    pub healing_strategy: HealingStrategy,
    pub authority_coordination: AuthorityCoordination,
    pub state_reconciliation: StateReconciliation,
}
```

### Failure Handling
1. **Node Failures**: Automatic reconnection and peer replacement
2. **Network Splits**: Authority-coordinated healing protocols
3. **Authority Isolation**: Emergency protocols for authority connectivity
4. **Cascading Failures**: Circuit breakers and graceful degradation

### Data Synchronization
- **Fast Sync**: Rapid synchronization for new nodes
- **Incremental Sync**: Efficient updates for existing nodes
- **Conflict Resolution**: Authority-mediated conflict resolution
- **Checkpoint Sync**: Periodic state checkpoints for efficiency

## Monitoring and Analytics

### Network Metrics
```rust
pub struct NetworkMetrics {
    pub message_throughput: f64,              // Messages per second
    pub latency_percentiles: LatencyStats,    // P50, P95, P99 latencies
    pub bandwidth_utilization: f64,          // Network bandwidth usage
    pub peer_churn_rate: f64,               // Peer connection/disconnection rate
    pub regional_health: HashMap<Region, RegionHealth>,
}
```

### Performance Dashboards
- **Real-Time Network Map**: Visual representation of network topology
- **Authority Node Status**: Health and connectivity of authority nodes
- **Trading Activity Heatmap**: Geographic trading activity visualization
- **Alert System**: Automated alerts for network issues

When implementing P2P networking features, prioritize reliability, security, and performance while ensuring seamless integration with Thailand's energy grid infrastructure and regulatory requirements.
