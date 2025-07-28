//! GridTokenX Configuration Module
//!
//! This module handles all configuration settings for the GridTokenX blockchain node,
//! including network settings, consensus parameters, and Thai energy market specifics.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// Main node configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Unique node identifier
    pub node_id: String,
    /// Node type (validator, trader, observer)
    pub node_type: NodeType,
    /// Network configuration
    pub network: NetworkConfig,
    /// P2P networking settings
    pub p2p: P2PConfig,
    /// API server settings
    pub api: ApiConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Consensus engine settings
    pub consensus: ConsensusConfig,
    /// Energy trading specific settings
    pub energy: EnergyConfig,
    /// Grid integration settings
    pub grid: GridConfig,
    /// Governance settings
    pub governance: GovernanceConfig,
    /// Security settings
    pub security: SecurityConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Thai market specific settings
    pub thai_market: ThaiMarketConfig,
}

/// Node types in the GridTokenX network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    /// Full validator node that participates in consensus
    Validator,
    /// Energy trader node (buy/sell energy)
    Trader,
    /// Observer node (read-only)
    Observer,
    /// Grid operator node (manages grid stability)
    GridOperator,
    /// Authority node (regulatory oversight)
    Authority,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Network name (mainnet, testnet, devnet)
    pub network_name: String,
    /// Network ID
    pub network_id: u64,
    /// Genesis block hash
    pub genesis_hash: String,
    /// Chain ID for transaction replay protection
    pub chain_id: u64,
    /// Bootstrap nodes for initial connection
    pub bootstrap_nodes: Vec<String>,
    /// Maximum number of peers
    pub max_peers: usize,
}

/// P2P networking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfig {
    /// Local listening address
    pub listen_addr: String,
    /// External address for NAT traversal
    pub external_addr: Option<String>,
    /// P2P port
    pub port: u16,
    /// Maximum inbound connections
    pub max_inbound_connections: usize,
    /// Maximum outbound connections
    pub max_outbound_connections: usize,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Gossip protocol settings
    pub gossip: GossipConfig,
}

/// Gossip protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipConfig {
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    /// Message TTL in seconds
    pub message_ttl: u64,
    /// Maximum message size
    pub max_message_size: usize,
    /// Mesh network maintenance interval
    pub mesh_maintenance_interval: u64,
}

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// API server host
    pub host: String,
    /// API server port
    pub port: u16,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Rate limiting settings
    pub rate_limit: RateLimitConfig,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
    /// Enable rate limiting
    pub enabled: bool,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert_file: String,
    /// Private key file path
    pub key_file: String,
    /// Enable TLS
    pub enabled: bool,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type (rocksdb, redis, memory)
    pub backend: StorageBackend,
    /// Data directory path
    pub path: String,
    /// Cache size in MB
    pub cache_size: usize,
    /// Enable compression
    pub compression: bool,
    /// Backup settings
    pub backup: BackupConfig,
    /// Pruning settings
    pub pruning: PruningConfig,
}

/// Storage backend types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    RocksDB,
    Redis,
    Memory,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    pub enabled: bool,
    /// Backup interval in hours
    pub interval_hours: u64,
    /// Backup directory
    pub backup_dir: String,
    /// Number of backups to keep
    pub retention_count: usize,
}

/// Pruning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruningConfig {
    /// Enable pruning
    pub enabled: bool,
    /// Blocks to keep (0 = keep all)
    pub keep_blocks: u64,
    /// Pruning interval in blocks
    pub pruning_interval: u64,
}

/// Consensus engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Consensus algorithm (PoS, PoW, PoA)
    pub algorithm: ConsensusAlgorithm,
    /// Block time target in seconds
    pub block_time: u64,
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Gas limit per block
    pub gas_limit: u64,
    /// Minimum validator stake
    pub min_validator_stake: u64,
    /// Validator selection parameters
    pub validator: ValidatorConfig,
    /// Proof of Work settings (if applicable)
    pub pow: Option<PoWConfig>,
}

/// Consensus algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    /// Proof of Stake
    PoS,
    /// Proof of Work
    PoW,
    /// Proof of Authority
    PoA,
    /// Hybrid (PoS + PoW for energy validation)
    Hybrid,
}

/// Validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    /// Maximum number of validators
    pub max_validators: usize,
    /// Validator rotation period in blocks
    pub rotation_period: u64,
    /// Slashing parameters
    pub slashing: SlashingConfig,
    /// Reward distribution
    pub rewards: RewardConfig,
}

/// Slashing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingConfig {
    /// Slash for double signing
    pub double_sign_slash_rate: f64,
    /// Slash for downtime
    pub downtime_slash_rate: f64,
    /// Downtime threshold in blocks
    pub downtime_threshold: u64,
}

/// Reward configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardConfig {
    /// Block reward in tokens
    pub block_reward: u64,
    /// Transaction fee sharing with validators
    pub fee_share_rate: f64,
    /// Energy trading incentive rate
    pub energy_incentive_rate: f64,
}

/// Proof of Work configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoWConfig {
    /// Initial difficulty
    pub initial_difficulty: u64,
    /// Difficulty adjustment period
    pub difficulty_adjustment: u64,
    /// Target block time for PoW
    pub pow_block_time: u64,
}

/// Energy trading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConfig {
    /// Energy-to-token conversion rate
    pub energy_token_ratio: f64,
    /// Minimum trade amount in kWh
    pub min_trade_amount: f64,
    /// Maximum trade amount in kWh
    pub max_trade_amount: f64,
    /// Order book settings
    pub order_book: OrderBookConfig,
    /// Carbon credit settings
    pub carbon_credits: CarbonCreditConfig,
    /// Price limits
    pub price_limits: PriceLimitConfig,
}

/// Order book configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookConfig {
    /// Maximum orders per trader
    pub max_orders_per_trader: usize,
    /// Order expiration time in hours
    pub default_expiration_hours: u64,
    /// Minimum order size in kWh
    pub min_order_size: f64,
    /// Matching algorithm parameters
    pub matching: MatchingConfig,
}

/// Order matching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingConfig {
    /// Price tolerance for matching (%)
    pub price_tolerance: f64,
    /// Time preference for matching
    pub time_preference: bool,
    /// Location preference weight
    pub location_preference_weight: f64,
}

/// Carbon credit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonCreditConfig {
    /// Enable carbon credit tracking
    pub enabled: bool,
    /// Carbon credit rates by energy source
    pub credit_rates: std::collections::HashMap<String, f64>,
    /// Verification requirements
    pub verification_required: bool,
}

/// Price limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLimitConfig {
    /// Minimum price per kWh in tokens
    pub min_price: u64,
    /// Maximum price per kWh in tokens
    pub max_price: u64,
    /// Daily price change limit (%)
    pub daily_change_limit: f64,
}

/// Grid integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    /// Grid operator endpoints
    pub grid_operators: Vec<GridOperatorConfig>,
    /// SCADA integration settings
    pub scada: ScadaConfig,
    /// Smart meter integration
    pub smart_meters: SmartMeterConfig,
    /// Grid stability monitoring
    pub stability_monitoring: StabilityConfig,
}

/// Grid operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridOperatorConfig {
    /// Operator name (EGAT, MEA, PEA)
    pub name: String,
    /// API endpoint
    pub endpoint: String,
    /// Authentication credentials
    pub credentials: Option<String>,
    /// Region coverage
    pub regions: Vec<String>,
}

/// SCADA system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScadaConfig {
    /// Enable SCADA integration
    pub enabled: bool,
    /// SCADA protocol (Modbus, DNP3, IEC 61850)
    pub protocol: String,
    /// Connection settings
    pub connection: ScadaConnectionConfig,
}

/// SCADA connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScadaConnectionConfig {
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Unit ID
    pub unit_id: u8,
    /// Timeout in seconds
    pub timeout: u64,
}

/// Smart meter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartMeterConfig {
    /// Enable smart meter integration
    pub enabled: bool,
    /// Communication protocol
    pub protocol: String,
    /// Reading interval in seconds
    pub reading_interval: u64,
    /// Data validation settings
    pub validation: MeterValidationConfig,
}

/// Meter data validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterValidationConfig {
    /// Maximum reading deviation (%)
    pub max_deviation: f64,
    /// Minimum reading interval
    pub min_interval: u64,
    /// Enable anomaly detection
    pub anomaly_detection: bool,
}

/// Grid stability monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityConfig {
    /// Monitor frequency stability
    pub monitor_frequency: bool,
    /// Monitor voltage stability
    pub monitor_voltage: bool,
    /// Alert thresholds
    pub thresholds: StabilityThresholds,
}

/// Grid stability thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityThresholds {
    /// Frequency deviation limit (Hz)
    pub frequency_deviation: f64,
    /// Voltage deviation limit (%)
    pub voltage_deviation: f64,
    /// Load imbalance threshold (%)
    pub load_imbalance: f64,
}

/// Governance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    /// Enable governance features
    pub enabled: bool,
    /// Voting period in days
    pub voting_period_days: u32,
    /// Quorum requirements
    pub quorum_threshold: f64,
    /// Proposal thresholds
    pub proposal: ProposalConfig,
}

/// Governance proposal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalConfig {
    /// Minimum stake to create proposal
    pub min_stake_to_propose: u64,
    /// Proposal fee in tokens
    pub proposal_fee: u64,
    /// Execution delay in days
    pub execution_delay_days: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Cryptographic settings
    pub crypto: CryptoConfig,
    /// Network security
    pub network_security: NetworkSecurityConfig,
    /// Access control
    pub access_control: AccessControlConfig,
}

/// Cryptographic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Signature algorithm (Ed25519, ECDSA)
    pub signature_algorithm: String,
    /// Hash algorithm (SHA256, Blake2b)
    pub hash_algorithm: String,
    /// Key derivation settings
    pub key_derivation: KeyDerivationConfig,
}

/// Key derivation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    /// KDF algorithm
    pub algorithm: String,
    /// Iteration count
    pub iterations: u32,
    /// Salt length
    pub salt_length: usize,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Enable DDoS protection
    pub ddos_protection: bool,
    /// IP whitelist
    pub ip_whitelist: Vec<String>,
    /// IP blacklist
    pub ip_blacklist: Vec<String>,
    /// Connection limits
    pub connection_limits: ConnectionLimits,
}

/// Connection limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLimits {
    /// Max connections per IP
    pub max_connections_per_ip: usize,
    /// Connection rate limit
    pub connection_rate_limit: u32,
    /// Ban duration in seconds
    pub ban_duration: u64,
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Enable API authentication
    pub require_auth: bool,
    /// JWT settings
    pub jwt: JwtConfig,
    /// Role-based access control
    pub rbac: RbacConfig,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT secret key
    pub secret_key: String,
    /// Token expiration in hours
    pub expiration_hours: u64,
    /// Issuer name
    pub issuer: String,
}

/// Role-based access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfig {
    /// Enable RBAC
    pub enabled: bool,
    /// Default user role
    pub default_role: String,
    /// Role permissions
    pub roles: std::collections::HashMap<String, Vec<String>>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, pretty)
    pub format: String,
    /// Log to file
    pub file_logging: FileLoggingConfig,
    /// Metrics collection
    pub metrics: MetricsConfig,
}

/// File logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLoggingConfig {
    /// Enable file logging
    pub enabled: bool,
    /// Log file path
    pub file_path: String,
    /// Log rotation settings
    pub rotation: LogRotationConfig,
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Maximum file size in MB
    pub max_size_mb: u64,
    /// Number of files to keep
    pub keep_files: usize,
    /// Rotation interval
    pub rotation_interval: String,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Collection interval in seconds
    pub collection_interval: u64,
}

/// Thai energy market specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiMarketConfig {
    /// ERC (Energy Regulatory Commission) settings
    pub erc_compliance: ERCComplianceConfig,
    /// Peak hour settings
    pub peak_hours: PeakHoursConfig,
    /// Regional settings
    pub regions: RegionalConfig,
    /// Currency settings
    pub currency: CurrencyConfig,
}

/// ERC compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCComplianceConfig {
    /// Enable ERC compliance checks
    pub enabled: bool,
    /// ERC API endpoint
    pub erc_api_endpoint: String,
    /// Compliance check interval
    pub check_interval_hours: u64,
}

/// Peak hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakHoursConfig {
    /// Peak hours start (24-hour format)
    pub start_hour: u8,
    /// Peak hours end (24-hour format)
    pub end_hour: u8,
    /// Peak hour pricing multiplier
    pub pricing_multiplier: f64,
    /// Weekend peak hours
    pub weekend_peak_enabled: bool,
}

/// Regional configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalConfig {
    /// Default region
    pub default_region: String,
    /// Regional pricing differences
    pub regional_multipliers: std::collections::HashMap<String, f64>,
    /// Grid zones
    pub grid_zones: Vec<String>,
}

/// Currency configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConfig {
    /// Base currency (THB)
    pub base_currency: String,
    /// Exchange rate source
    pub exchange_rate_source: String,
    /// Rate update interval in hours
    pub rate_update_interval: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: Uuid::new_v4().to_string(),
            node_type: NodeType::Validator,
            network: NetworkConfig::default(),
            p2p: P2PConfig::default(),
            api: ApiConfig::default(),
            storage: StorageConfig::default(),
            consensus: ConsensusConfig::default(),
            energy: EnergyConfig::default(),
            grid: GridConfig::default(),
            governance: GovernanceConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
            thai_market: ThaiMarketConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_name: "gridtokenx-testnet".to_string(),
            network_id: 1001,
            genesis_hash: String::new(),
            chain_id: 1001,
            bootstrap_nodes: vec![],
            max_peers: 50,
        }
    }
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            listen_addr: "/ip4/0.0.0.0/tcp/9000".to_string(),
            external_addr: None,
            port: 9000,
            max_inbound_connections: 25,
            max_outbound_connections: 25,
            connection_timeout: 30,
            enable_mdns: true,
            gossip: GossipConfig::default(),
        }
    }
}

impl Default for GossipConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: 1,
            message_ttl: 300,
            max_message_size: 1048576, // 1MB
            mesh_maintenance_interval: 5,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
            request_timeout: 30,
            rate_limit: RateLimitConfig::default(),
            tls: None,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
            enabled: true,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::RocksDB,
            path: "./data".to_string(),
            cache_size: 128, // 128MB
            compression: true,
            backup: BackupConfig::default(),
            pruning: PruningConfig::default(),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_hours: 24,
            backup_dir: "./backups".to_string(),
            retention_count: 7,
        }
    }
}

impl Default for PruningConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            keep_blocks: 0, // Keep all blocks by default
            pruning_interval: 1000,
        }
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            algorithm: ConsensusAlgorithm::Hybrid,
            block_time: 10,          // 10 seconds for energy trading
            max_block_size: 1048576, // 1MB
            gas_limit: 10_000_000,
            min_validator_stake: 100_000,
            validator: ValidatorConfig::default(),
            pow: Some(PoWConfig::default()),
        }
    }
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_validators: 21,
            rotation_period: 1000,
            slashing: SlashingConfig::default(),
            rewards: RewardConfig::default(),
        }
    }
}

impl Default for SlashingConfig {
    fn default() -> Self {
        Self {
            double_sign_slash_rate: 0.05, // 5%
            downtime_slash_rate: 0.01,    // 1%
            downtime_threshold: 100,      // blocks
        }
    }
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            block_reward: 50_000,       // 50k tokens per block
            fee_share_rate: 0.8,        // 80% to validators
            energy_incentive_rate: 0.1, // 10% bonus for energy trades
        }
    }
}

impl Default for PoWConfig {
    fn default() -> Self {
        Self {
            initial_difficulty: 1000,
            difficulty_adjustment: 144, // blocks
            pow_block_time: 60,         // 1 minute for PoW components
        }
    }
}

impl Default for EnergyConfig {
    fn default() -> Self {
        Self {
            energy_token_ratio: 1.0,    // 1 kWh = 1 Token
            min_trade_amount: 0.1,      // 0.1 kWh minimum
            max_trade_amount: 10_000.0, // 10 MWh maximum
            order_book: OrderBookConfig::default(),
            carbon_credits: CarbonCreditConfig::default(),
            price_limits: PriceLimitConfig::default(),
        }
    }
}

impl Default for OrderBookConfig {
    fn default() -> Self {
        Self {
            max_orders_per_trader: 100,
            default_expiration_hours: 24,
            min_order_size: 0.1,
            matching: MatchingConfig::default(),
        }
    }
}

impl Default for MatchingConfig {
    fn default() -> Self {
        Self {
            price_tolerance: 5.0, // 5% price tolerance
            time_preference: true,
            location_preference_weight: 0.1,
        }
    }
}

impl Default for CarbonCreditConfig {
    fn default() -> Self {
        let mut credit_rates = std::collections::HashMap::new();
        credit_rates.insert("solar".to_string(), 0.5);
        credit_rates.insert("wind".to_string(), 0.6);
        credit_rates.insert("hydro".to_string(), 0.4);
        credit_rates.insert("biomass".to_string(), 0.3);

        Self {
            enabled: true,
            credit_rates,
            verification_required: true,
        }
    }
}

impl Default for PriceLimitConfig {
    fn default() -> Self {
        Self {
            min_price: 1_000,         // 1 token per kWh minimum
            max_price: 10_000,        // 10 tokens per kWh maximum
            daily_change_limit: 50.0, // 50% daily change limit
        }
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            grid_operators: vec![
                GridOperatorConfig {
                    name: "EGAT".to_string(),
                    endpoint: "https://api.egat.co.th".to_string(),
                    credentials: None,
                    regions: vec!["central".to_string(), "northern".to_string()],
                },
                GridOperatorConfig {
                    name: "MEA".to_string(),
                    endpoint: "https://api.mea.or.th".to_string(),
                    credentials: None,
                    regions: vec!["bangkok".to_string(), "metro".to_string()],
                },
                GridOperatorConfig {
                    name: "PEA".to_string(),
                    endpoint: "https://api.pea.co.th".to_string(),
                    credentials: None,
                    regions: vec!["provincial".to_string()],
                },
            ],
            scada: ScadaConfig::default(),
            smart_meters: SmartMeterConfig::default(),
            stability_monitoring: StabilityConfig::default(),
        }
    }
}

impl Default for ScadaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            protocol: "modbus".to_string(),
            connection: ScadaConnectionConfig::default(),
        }
    }
}

impl Default for ScadaConnectionConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 502,
            unit_id: 1,
            timeout: 30,
        }
    }
}

impl Default for SmartMeterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            protocol: "mqtt".to_string(),
            reading_interval: 60, // 1 minute
            validation: MeterValidationConfig::default(),
        }
    }
}

impl Default for MeterValidationConfig {
    fn default() -> Self {
        Self {
            max_deviation: 10.0, // 10% deviation allowed
            min_interval: 30,    // 30 seconds minimum
            anomaly_detection: true,
        }
    }
}

impl Default for StabilityConfig {
    fn default() -> Self {
        Self {
            monitor_frequency: true,
            monitor_voltage: true,
            thresholds: StabilityThresholds::default(),
        }
    }
}

impl Default for StabilityThresholds {
    fn default() -> Self {
        Self {
            frequency_deviation: 0.5, // ±0.5 Hz
            voltage_deviation: 5.0,   // ±5%
            load_imbalance: 10.0,     // 10% imbalance
        }
    }
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            voting_period_days: 7,
            quorum_threshold: 0.5, // 50% quorum
            proposal: ProposalConfig::default(),
        }
    }
}

impl Default for ProposalConfig {
    fn default() -> Self {
        Self {
            min_stake_to_propose: 1_000_000, // 1M tokens
            proposal_fee: 10_000,            // 10k tokens
            execution_delay_days: 2,         // 2 day delay
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            crypto: CryptoConfig::default(),
            network_security: NetworkSecurityConfig::default(),
            access_control: AccessControlConfig::default(),
        }
    }
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            signature_algorithm: "Ed25519".to_string(),
            hash_algorithm: "SHA256".to_string(),
            key_derivation: KeyDerivationConfig::default(),
        }
    }
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            algorithm: "PBKDF2".to_string(),
            iterations: 100_000,
            salt_length: 32,
        }
    }
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            ddos_protection: true,
            ip_whitelist: vec![],
            ip_blacklist: vec![],
            connection_limits: ConnectionLimits::default(),
        }
    }
}

impl Default for ConnectionLimits {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 10,
            connection_rate_limit: 100, // per minute
            ban_duration: 3600,         // 1 hour
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            require_auth: false,
            jwt: JwtConfig::default(),
            rbac: RbacConfig::default(),
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret_key: "default-jwt-secret-change-in-production".to_string(),
            expiration_hours: 24,
            issuer: "gridtokenx".to_string(),
        }
    }
}

impl Default for RbacConfig {
    fn default() -> Self {
        let mut roles = std::collections::HashMap::new();
        roles.insert(
            "admin".to_string(),
            vec![
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
                "validate".to_string(),
            ],
        );
        roles.insert(
            "trader".to_string(),
            vec!["read".to_string(), "trade".to_string()],
        );
        roles.insert("observer".to_string(), vec!["read".to_string()]);

        Self {
            enabled: false,
            default_role: "observer".to_string(),
            roles,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            file_logging: FileLoggingConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

impl Default for FileLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            file_path: "./logs/gridtokenx.log".to_string(),
            rotation: LogRotationConfig::default(),
        }
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 100,
            keep_files: 10,
            rotation_interval: "daily".to_string(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            collection_interval: 60, // 1 minute
        }
    }
}

impl Default for ThaiMarketConfig {
    fn default() -> Self {
        Self {
            erc_compliance: ERCComplianceConfig::default(),
            peak_hours: PeakHoursConfig::default(),
            regions: RegionalConfig::default(),
            currency: CurrencyConfig::default(),
        }
    }
}

impl Default for ERCComplianceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            erc_api_endpoint: "https://api.erc.or.th".to_string(),
            check_interval_hours: 24,
        }
    }
}

impl Default for PeakHoursConfig {
    fn default() -> Self {
        Self {
            start_hour: 18, // 6 PM
            end_hour: 22,   // 10 PM
            pricing_multiplier: 1.5,
            weekend_peak_enabled: false,
        }
    }
}

impl Default for RegionalConfig {
    fn default() -> Self {
        let mut regional_multipliers = std::collections::HashMap::new();
        regional_multipliers.insert("bangkok".to_string(), 1.2);
        regional_multipliers.insert("central".to_string(), 1.0);
        regional_multipliers.insert("northern".to_string(), 0.9);
        regional_multipliers.insert("northeastern".to_string(), 0.8);
        regional_multipliers.insert("southern".to_string(), 1.1);

        Self {
            default_region: "central".to_string(),
            regional_multipliers,
            grid_zones: vec![
                "zone1".to_string(),
                "zone2".to_string(),
                "zone3".to_string(),
                "zone4".to_string(),
            ],
        }
    }
}

impl Default for CurrencyConfig {
    fn default() -> Self {
        Self {
            base_currency: "THB".to_string(),
            exchange_rate_source: "https://api.exchangerate-api.com".to_string(),
            rate_update_interval: 1, // 1 hour
        }
    }
}

impl NodeConfig {
    /// Load configuration from file
    pub fn load(config_path: &str) -> Result<Self> {
        if Path::new(config_path).exists() {
            let config_str = fs::read_to_string(config_path)
                .map_err(|e| anyhow!("Failed to read config file: {}", e))?;

            let config: NodeConfig = toml::from_str(&config_str)
                .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;

            // Validate configuration
            config.validate()?;

            Ok(config)
        } else {
            // Create default config and save it
            let default_config = Self::default();
            default_config.save(config_path)?;
            Ok(default_config)
        }
    }

    /// Save configuration to file
    pub fn save(&self, config_path: &str) -> Result<()> {
        let config_str = toml::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;

        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(config_path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;
        }

        fs::write(config_path, config_str)
            .map_err(|e| anyhow!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate network configuration
        if self.network.network_id == 0 {
            return Err(anyhow!("Network ID cannot be zero"));
        }

        // Validate P2P configuration
        if self.p2p.port == 0 {
            return Err(anyhow!("P2P port cannot be zero"));
        }

        // Validate API configuration
        if self.api.port == 0 {
            return Err(anyhow!("API port cannot be zero"));
        }

        // Validate storage configuration
        if self.storage.path.is_empty() {
            return Err(anyhow!("Storage path cannot be empty"));
        }

        // Validate consensus configuration
        if self.consensus.block_time == 0 {
            return Err(anyhow!("Block time cannot be zero"));
        }

        if self.consensus.max_block_size == 0 {
            return Err(anyhow!("Max block size cannot be zero"));
        }

        // Validate energy configuration
        if self.energy.energy_token_ratio <= 0.0 {
            return Err(anyhow!("Energy token ratio must be positive"));
        }

        // Validate Thai market configuration
        if self.thai_market.peak_hours.start_hour >= 24
            || self.thai_market.peak_hours.end_hour >= 24
        {
            return Err(anyhow!("Peak hours must be in 24-hour format"));
        }

        Ok(())
    }

    /// Get configuration for specific node type
    pub fn for_node_type(node_type: NodeType) -> Self {
        let mut config = Self::default();
        config.node_type = node_type.clone();

        match node_type {
            NodeType::Validator => {
                config.consensus.min_validator_stake = 100_000;
                config.p2p.max_inbound_connections = 50;
                config.p2p.max_outbound_connections = 50;
            }
            NodeType::Trader => {
                config.consensus.min_validator_stake = 0; // Traders don't need to stake
                config.energy.order_book.max_orders_per_trader = 500;
                config.api.rate_limit.requests_per_minute = 120;
            }
            NodeType::Observer => {
                config.consensus.min_validator_stake = 0;
                config.p2p.max_inbound_connections = 10;
                config.p2p.max_outbound_connections = 20;
                config.api.rate_limit.requests_per_minute = 30;
            }
            NodeType::GridOperator => {
                config.grid.stability_monitoring.monitor_frequency = true;
                config.grid.stability_monitoring.monitor_voltage = true;
                config.grid.scada.enabled = true;
            }
            NodeType::Authority => {
                config.governance.enabled = true;
                config.thai_market.erc_compliance.enabled = true;
                config.security.access_control.require_auth = true;
            }
        }

        config
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.network.network_name == "gridtokenx-mainnet"
    }

    /// Check if running in testnet mode
    pub fn is_testnet(&self) -> bool {
        self.network.network_name.contains("testnet")
    }

    /// Get effective peak hours pricing multiplier
    pub fn get_peak_hours_multiplier(&self) -> f64 {
        if self.thai_market.peak_hours.pricing_multiplier > 0.0 {
            self.thai_market.peak_hours.pricing_multiplier
        } else {
            1.0 // No peak hour pricing
        }
    }

    /// Get regional pricing multiplier for given region
    pub fn get_regional_multiplier(&self, region: &str) -> f64 {
        self.thai_market
            .regions
            .regional_multipliers
            .get(region)
            .copied()
            .unwrap_or(1.0)
    }

    /// Check if ERC compliance is required
    pub fn requires_erc_compliance(&self) -> bool {
        self.thai_market.erc_compliance.enabled && self.is_production()
    }
}
