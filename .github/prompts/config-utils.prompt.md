---
mode: edit
---

# GridTokenX Configuration and Utilities Development Prompt

You are developing the configuration management and utility modules for GridTokenX - Thailand's energy trading blockchain platform ensuring flexible deployment and robust utility functions.

## Configuration Management Overview

The configuration system (`src/config.rs`) provides:
- **Environment-Specific Settings**: Development, staging, production configurations
- **Authority Integration**: EGAT, MEA, PEA connection parameters
- **Performance Tuning**: Blockchain, network, and storage optimization
- **Security Configuration**: Cryptographic settings and access controls
- **Thai Market Compliance**: Regulatory and operational parameters

## Configuration Structure

### Main Configuration Schema
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node: NodeSettings,
    pub blockchain: BlockchainConfig,
    pub consensus: ConsensusConfig,
    pub energy_trading: EnergyTradingConfig,
    pub network: NetworkConfig,
    pub storage: StorageConfig,
    pub api: ApiConfig,
    pub governance: GovernanceConfig,
    pub thai_authorities: ThaiAuthorityConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSettings {
    pub node_id: String,
    pub node_type: NodeType,
    pub environment: Environment,
    pub region: ThaiRegion,
    pub grid_zone: GridZone,
    pub data_directory: String,
    pub max_connections: u32,
    pub enable_mining: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    AuthorityNode { authority_type: AuthorityType },
    ValidatorNode { stake_amount: u64 },
    TradingNode { participant_id: String },
    ObserverNode,
    ArchiveNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}
```

### Thai Authority Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiAuthorityConfig {
    pub egat: AuthorityNodeConfig,
    pub mea: AuthorityNodeConfig,
    pub pea: AuthorityNodeConfig,
    pub nepo: RegulatoryConfig,
    pub erc: RegulatoryConfig,
    pub grid_integration: GridIntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityNodeConfig {
    pub enabled: bool,
    pub endpoints: Vec<String>,
    pub api_keys: HashMap<String, String>,
    pub certificates: CertificateConfig,
    pub backup_endpoints: Vec<String>,
    pub heartbeat_interval: Duration,
    pub timeout: Duration,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridIntegrationConfig {
    pub scada_endpoints: Vec<String>,
    pub ems_integration: bool,               // Energy Management System
    pub real_time_data_feed: bool,
    pub emergency_protocols: EmergencyProtocolConfig,
    pub grid_codes: GridCodeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCodeConfig {
    pub frequency_tolerance: f64,            // Hz deviation allowed
    pub voltage_tolerance: f64,              // % voltage deviation
    pub power_factor_min: f64,              // Minimum power factor
    pub harmonic_distortion_max: f64,       // Maximum THD
    pub response_time_max: Duration,         // Max response time to grid signals
}
```

### Energy Trading Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradingConfig {
    pub market_hours: MarketHours,
    pub order_matching: OrderMatchingConfig,
    pub pricing: PricingConfig,
    pub settlement: SettlementConfig,
    pub risk_management: RiskManagementConfig,
    pub renewable_incentives: RenewableIncentiveConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHours {
    pub day_ahead_cutoff: String,           // "14:00" - 2 PM cutoff for next day
    pub real_time_trading_hours: Vec<TradingWindow>,
    pub maintenance_windows: Vec<MaintenanceWindow>,
    pub holiday_schedule: Vec<String>,       // Thai holidays
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    pub base_price_range: PriceRange,
    pub congestion_multipliers: HashMap<GridZone, f64>,
    pub time_of_use_multipliers: TimeOfUsePricing,
    pub renewable_discount: f64,            // Discount for renewable energy
    pub carbon_credit_price: u64,           // Price per ton CO2 offset
    pub emergency_price_cap: u64,           // Maximum price during emergencies
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOfUsePricing {
    pub peak_hours: Vec<TimeRange>,         // 9 AM - 10 PM
    pub off_peak_hours: Vec<TimeRange>,     // 10 PM - 9 AM
    pub peak_multiplier: f64,               // 1.5x base price
    pub off_peak_multiplier: f64,           // 0.8x base price
    pub seasonal_adjustments: SeasonalPricing,
}
```

### Security Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption: EncryptionConfig,
    pub authentication: AuthenticationConfig,
    pub authorization: AuthorizationConfig,
    pub audit: AuditConfig,
    pub rate_limiting: RateLimitingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: String,                  // "AES-256-GCM"
    pub key_rotation_interval: Duration,
    pub certificate_path: String,
    pub private_key_path: String,
    pub ca_certificate_path: String,
    pub tls_version: String,                // "1.3"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub methods: Vec<AuthMethod>,
    pub session_timeout: Duration,
    pub max_login_attempts: u32,
    pub lockout_duration: Duration,
    pub two_factor_required: bool,
    pub authority_verification: AuthorityVerificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    ApiKey,
    Certificate,
    OAuth2,
    AuthoritySignature,
}
```

## Configuration Loading and Validation

### Configuration Manager
```rust
pub struct ConfigManager {
    config: NodeConfig,
    config_path: String,
    environment_overrides: HashMap<String, String>,
    secrets_manager: Option<SecretsManager>,
}

impl ConfigManager {
    pub fn new(config_path: &str, environment: Environment) -> Result<Self> {
        let mut config = Self::load_base_config(config_path)?;
        
        // Apply environment-specific overrides
        Self::apply_environment_overrides(&mut config, environment)?;
        
        // Load secrets from secure storage
        let secrets_manager = Self::initialize_secrets_manager(&config)?;
        Self::inject_secrets(&mut config, &secrets_manager)?;
        
        // Validate configuration
        Self::validate_config(&config)?;
        
        Ok(ConfigManager {
            config,
            config_path: config_path.to_string(),
            environment_overrides: HashMap::new(),
            secrets_manager,
        })
    }
    
    fn load_base_config(path: &str) -> Result<NodeConfig> {
        let config_content = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file {}: {}", path, e))?;
        
        // Support multiple config formats
        if path.ends_with(".toml") {
            toml::from_str(&config_content)
                .map_err(|e| anyhow!("Failed to parse TOML config: {}", e))
        } else if path.ends_with(".json") {
            serde_json::from_str(&config_content)
                .map_err(|e| anyhow!("Failed to parse JSON config: {}", e))
        } else {
            Err(anyhow!("Unsupported config file format"))
        }
    }
    
    fn validate_config(config: &NodeConfig) -> Result<()> {
        // Validate node settings
        if config.node.node_id.is_empty() {
            return Err(anyhow!("Node ID cannot be empty"));
        }
        
        // Validate Thai authority configurations
        if matches!(config.node.node_type, NodeType::AuthorityNode { .. }) {
            Self::validate_authority_config(config)?;
        }
        
        // Validate energy trading parameters
        Self::validate_energy_trading_config(&config.energy_trading)?;
        
        // Validate network configuration
        Self::validate_network_config(&config.network)?;
        
        // Validate security settings
        Self::validate_security_config(&config.security)?;
        
        Ok(())
    }
    
    fn validate_authority_config(config: &NodeConfig) -> Result<()> {
        if let NodeType::AuthorityNode { authority_type } = &config.node.node_type {
            let authority_config = match authority_type {
                AuthorityType::EGAT => &config.thai_authorities.egat,
                AuthorityType::MEA => &config.thai_authorities.mea,
                AuthorityType::PEA => &config.thai_authorities.pea,
            };
            
            if !authority_config.enabled {
                return Err(anyhow!("Authority node configured but disabled"));
            }
            
            if authority_config.endpoints.is_empty() {
                return Err(anyhow!("Authority node must have at least one endpoint"));
            }
            
            // Validate certificates exist
            Self::validate_certificate_files(&authority_config.certificates)?;
        }
        
        Ok(())
    }
}
```

### Environment-Specific Configurations
```rust
impl ConfigManager {
    fn apply_environment_overrides(config: &mut NodeConfig, env: Environment) -> Result<()> {
        match env {
            Environment::Development => {
                config.logging.level = "debug".to_string();
                config.api.cors_enabled = true;
                config.security.rate_limiting.enabled = false;
                config.storage.sync_mode = "async".to_string();
            },
            Environment::Testing => {
                config.storage.in_memory = true;
                config.network.bootstrap_nodes = vec!["127.0.0.1:8000".to_string()];
                config.consensus.block_time = Duration::from_secs(1); // Fast blocks for testing
            },
            Environment::Staging => {
                config.thai_authorities.egat.endpoints = vec![
                    "https://staging-egat-api.example.com".to_string()
                ];
                config.energy_trading.market_hours.day_ahead_cutoff = "14:00".to_string();
            },
            Environment::Production => {
                config.security.audit.enabled = true;
                config.logging.file_output = true;
                config.storage.backup_interval = Duration::from_hours(1);
                config.consensus.finality_threshold = 6;
            },
        }
        Ok(())
    }
    
    pub fn get_config(&self) -> &NodeConfig {
        &self.config
    }
    
    pub fn update_config<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut NodeConfig) -> Result<()>,
    {
        updater(&mut self.config)?;
        Self::validate_config(&self.config)?;
        self.save_config()?;
        Ok(())
    }
    
    pub fn reload_config(&mut self) -> Result<()> {
        *self = Self::new(&self.config_path, self.config.node.environment)?;
        Ok(())
    }
}
```

## Utility Functions and Helpers

### Cryptographic Utilities
```rust
pub mod crypto {
    use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
    use sha2::{Sha256, Digest};
    use rand::rngs::OsRng;
    
    pub struct CryptoUtils;
    
    impl CryptoUtils {
        pub fn generate_keypair() -> Keypair {
            Keypair::generate(&mut OsRng)
        }
        
        pub fn hash_data(data: &[u8]) -> String {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        
        pub fn sign_message(keypair: &Keypair, message: &[u8]) -> Signature {
            keypair.sign(message)
        }
        
        pub fn verify_signature(
            public_key: &PublicKey,
            message: &[u8],
            signature: &Signature,
        ) -> bool {
            public_key.verify(message, signature).is_ok()
        }
        
        pub fn create_authority_signature(
            authority_keypair: &Keypair,
            authority_type: AuthorityType,
            data: &[u8],
        ) -> AuthoritySignature {
            let timestamp = Utc::now();
            let signature_data = [
                data,
                authority_type.to_string().as_bytes(),
                timestamp.timestamp().to_be_bytes().as_slice(),
            ].concat();
            
            let signature = authority_keypair.sign(&signature_data);
            
            AuthoritySignature {
                authority_type,
                signature,
                public_key: authority_keypair.public,
                timestamp,
                data_hash: Self::hash_data(data),
            }
        }
        
        pub fn verify_authority_signature(
            authority_signature: &AuthoritySignature,
            data: &[u8],
            trusted_authorities: &HashMap<AuthorityType, PublicKey>,
        ) -> Result<bool> {
            // Verify authority is trusted
            let trusted_key = trusted_authorities.get(&authority_signature.authority_type)
                .ok_or_else(|| anyhow!("Authority not in trusted list"))?;
            
            if trusted_key != &authority_signature.public_key {
                return Ok(false);
            }
            
            // Verify data hash
            if Self::hash_data(data) != authority_signature.data_hash {
                return Ok(false);
            }
            
            // Reconstruct signature data
            let signature_data = [
                data,
                authority_signature.authority_type.to_string().as_bytes(),
                authority_signature.timestamp.timestamp().to_be_bytes().as_slice(),
            ].concat();
            
            Ok(Self::verify_signature(
                &authority_signature.public_key,
                &signature_data,
                &authority_signature.signature,
            ))
        }
    }
}
```

### Thai Energy Market Utilities
```rust
pub mod thai_energy_market {
    use chrono::{DateTime, Utc, TimeZone, Datelike};
    
    pub struct ThaiEnergyMarket;
    
    impl ThaiEnergyMarket {
        pub fn is_trading_day(date: DateTime<Utc>) -> bool {
            let bangkok_time = date.with_timezone(&chrono_tz::Asia::Bangkok);
            
            // Check if it's a weekend
            if matches!(bangkok_time.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun) {
                return false;
            }
            
            // Check Thai holidays
            Self::is_thai_holiday(bangkok_time.date_naive())
        }
        
        pub fn is_thai_holiday(date: chrono::NaiveDate) -> bool {
            let year = date.year();
            let month = date.month();
            let day = date.day();
            
            // Fixed holidays
            matches!((month, day), 
                (1, 1) |   // New Year's Day
                (12, 5) |  // King's Birthday
                (12, 10) | // Constitution Day
                (12, 31)   // New Year's Eve
            ) ||
            // Variable holidays (simplified - in practice would use proper calculation)
            Self::is_buddhist_holiday(date) ||
            Self::is_royal_holiday(date)
        }
        
        pub fn get_peak_hours() -> Vec<(u8, u8)> {
            // Peak hours in Thailand: 9 AM - 10 PM
            vec![(9, 22)]
        }
        
        pub fn calculate_time_of_use_multiplier(time: DateTime<Utc>) -> f64 {
            let bangkok_time = time.with_timezone(&chrono_tz::Asia::Bangkok);
            let hour = bangkok_time.hour();
            
            if Self::is_peak_hour(hour) {
                1.5 // Peak multiplier
            } else {
                0.8 // Off-peak multiplier
            }
        }
        
        pub fn is_peak_hour(hour: u32) -> bool {
            (9..=22).contains(&hour)
        }
        
        pub fn get_seasonal_multiplier(date: DateTime<Utc>) -> f64 {
            let bangkok_time = date.with_timezone(&chrono_tz::Asia::Bangkok);
            let month = bangkok_time.month();
            
            match month {
                3..=5 => 1.3,   // Hot season (March-May) - high AC demand
                6..=10 => 1.1,  // Rainy season (June-October) - moderate demand
                11..=2 => 0.9,  // Cool season (November-February) - lower demand
                _ => 1.0,
            }
        }
        
        pub fn validate_grid_zone(zone: &GridZone, region: ThaiRegion) -> bool {
            match (zone, region) {
                (GridZone::Bangkok, ThaiRegion::Central) => true,
                (GridZone::Central, ThaiRegion::Central) => true,
                (GridZone::North, ThaiRegion::North) => true,
                (GridZone::Northeast, ThaiRegion::Northeast) => true,
                (GridZone::East, ThaiRegion::East) => true,
                (GridZone::West, ThaiRegion::West) => true,
                (GridZone::South, ThaiRegion::South) => true,
                _ => false,
            }
        }
    }
}
```

### Energy Conversion Utilities
```rust
pub mod energy_conversion {
    pub struct EnergyConversion;
    
    impl EnergyConversion {
        pub fn kwh_to_tokens(kwh: f64) -> u64 {
            // 1 kWh = 1 Token (base ratio)
            (kwh * 1000.0) as u64 // Convert to milliTokens for precision
        }
        
        pub fn tokens_to_kwh(tokens: u64) -> f64 {
            tokens as f64 / 1000.0 // Convert from milliTokens
        }
        
        pub fn mw_to_kw(mw: f64) -> f64 {
            mw * 1000.0
        }
        
        pub fn calculate_carbon_credits(kwh: f64, energy_source: EnergySource) -> f64 {
            let emission_factor = match energy_source {
                EnergySource::Solar => 0.0,
                EnergySource::Wind => 0.0,
                EnergySource::Hydro => 0.024,      // kg CO2/kWh
                EnergySource::NaturalGas => 0.491, // kg CO2/kWh
                EnergySource::Coal => 0.820,       // kg CO2/kWh
                EnergySource::Nuclear => 0.012,    // kg CO2/kWh
                EnergySource::Biomass => 0.230,    // kg CO2/kWh
            };
            
            kwh * emission_factor
        }
        
        pub fn calculate_renewable_certificate_value(
            kwh: f64,
            source: EnergySource,
            market_premium: f64,
        ) -> u64 {
            let base_value = Self::kwh_to_tokens(kwh);
            let renewable_multiplier = match source {
                EnergySource::Solar => 1.2,
                EnergySource::Wind => 1.15,
                EnergySource::Hydro => 1.1,
                EnergySource::Biomass => 1.05,
                _ => 1.0,
            };
            
            ((base_value as f64) * renewable_multiplier * (1.0 + market_premium)) as u64
        }
    }
}
```

### Performance Monitoring Utilities
```rust
pub mod performance {
    use std::time::{Duration, Instant};
    use std::collections::VecDeque;
    
    pub struct PerformanceMonitor {
        metrics: HashMap<String, MetricCollector>,
        alert_thresholds: HashMap<String, f64>,
    }
    
    pub struct MetricCollector {
        values: VecDeque<(Instant, f64)>,
        max_size: usize,
    }
    
    impl MetricCollector {
        pub fn new(max_size: usize) -> Self {
            Self {
                values: VecDeque::new(),
                max_size,
            }
        }
        
        pub fn record(&mut self, value: f64) {
            let now = Instant::now();
            self.values.push_back((now, value));
            
            if self.values.len() > self.max_size {
                self.values.pop_front();
            }
        }
        
        pub fn average(&self, duration: Duration) -> Option<f64> {
            let cutoff = Instant::now() - duration;
            let recent_values: Vec<f64> = self.values
                .iter()
                .filter(|(timestamp, _)| *timestamp > cutoff)
                .map(|(_, value)| *value)
                .collect();
            
            if recent_values.is_empty() {
                None
            } else {
                Some(recent_values.iter().sum::<f64>() / recent_values.len() as f64)
            }
        }
        
        pub fn percentile(&self, p: f64, duration: Duration) -> Option<f64> {
            let cutoff = Instant::now() - duration;
            let mut recent_values: Vec<f64> = self.values
                .iter()
                .filter(|(timestamp, _)| *timestamp > cutoff)
                .map(|(_, value)| *value)
                .collect();
            
            if recent_values.is_empty() {
                return None;
            }
            
            recent_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = ((recent_values.len() as f64 - 1.0) * p) as usize;
            Some(recent_values[index])
        }
    }
    
    impl PerformanceMonitor {
        pub fn new() -> Self {
            Self {
                metrics: HashMap::new(),
                alert_thresholds: HashMap::new(),
            }
        }
        
        pub fn record_metric(&mut self, name: &str, value: f64) {
            self.metrics
                .entry(name.to_string())
                .or_insert_with(|| MetricCollector::new(1000))
                .record(value);
            
            // Check alert thresholds
            if let Some(threshold) = self.alert_thresholds.get(name) {
                if value > *threshold {
                    self.trigger_alert(name, value, *threshold);
                }
            }
        }
        
        pub fn set_alert_threshold(&mut self, metric: &str, threshold: f64) {
            self.alert_thresholds.insert(metric.to_string(), threshold);
        }
        
        fn trigger_alert(&self, metric: &str, value: f64, threshold: f64) {
            eprintln!("ALERT: {} = {} exceeds threshold {}", metric, value, threshold);
            // In production, would send to monitoring system
        }
        
        pub fn get_performance_summary(&self) -> PerformanceSummary {
            let mut summary = PerformanceSummary::new();
            
            for (name, collector) in &self.metrics {
                let avg = collector.average(Duration::from_minutes(5));
                let p95 = collector.percentile(0.95, Duration::from_minutes(5));
                let p99 = collector.percentile(0.99, Duration::from_minutes(5));
                
                summary.metrics.insert(name.clone(), MetricSummary {
                    average_5min: avg,
                    p95_5min: p95,
                    p99_5min: p99,
                });
            }
            
            summary
        }
    }
}
```

### Utility Integration Tests
```rust
#[cfg(test)]
mod utils_tests {
    use super::*;
    
    #[test]
    fn test_thai_energy_market_utilities() {
        use chrono::NaiveDate;
        
        // Test trading day detection
        let weekday = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(); // Monday
        let weekend = NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(); // Saturday
        
        assert!(ThaiEnergyMarket::is_trading_day(weekday.and_hms_opt(10, 0, 0).unwrap().and_utc()));
        assert!(!ThaiEnergyMarket::is_trading_day(weekend.and_hms_opt(10, 0, 0).unwrap().and_utc()));
        
        // Test peak hours
        assert!(ThaiEnergyMarket::is_peak_hour(15)); // 3 PM
        assert!(!ThaiEnergyMarket::is_peak_hour(23)); // 11 PM
        
        // Test seasonal multipliers
        let hot_season = NaiveDate::from_ymd_opt(2024, 4, 15).unwrap().and_hms_opt(12, 0, 0).unwrap().and_utc();
        let cool_season = NaiveDate::from_ymd_opt(2024, 12, 15).unwrap().and_hms_opt(12, 0, 0).unwrap().and_utc();
        
        assert!(ThaiEnergyMarket::get_seasonal_multiplier(hot_season) > 1.0);
        assert!(ThaiEnergyMarket::get_seasonal_multiplier(cool_season) < 1.0);
    }
    
    #[test]
    fn test_energy_conversion_utilities() {
        // Test kWh to token conversion
        let tokens = EnergyConversion::kwh_to_tokens(10.5);
        let kwh = EnergyConversion::tokens_to_kwh(tokens);
        
        assert!((kwh - 10.5).abs() < 0.001);
        
        // Test carbon credit calculation
        let solar_credits = EnergyConversion::calculate_carbon_credits(100.0, EnergySource::Solar);
        let coal_credits = EnergyConversion::calculate_carbon_credits(100.0, EnergySource::Coal);
        
        assert_eq!(solar_credits, 0.0);
        assert!(coal_credits > 80.0); // Coal produces significant emissions
    }
    
    #[test]
    fn test_cryptographic_utilities() {
        let keypair = CryptoUtils::generate_keypair();
        let message = b"test message";
        
        let signature = CryptoUtils::sign_message(&keypair, message);
        assert!(CryptoUtils::verify_signature(&keypair.public, message, &signature));
        
        // Test with wrong message
        let wrong_message = b"wrong message";
        assert!(!CryptoUtils::verify_signature(&keypair.public, wrong_message, &signature));
    }
}
```

When implementing configuration and utility features, ensure flexibility for different deployment environments, comprehensive validation of all settings, and robust utility functions that support the specific requirements of Thailand's energy market operations.
