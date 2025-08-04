//! GridTokenX Transaction Module
//!
//! This module implements various transaction types for the GridTokenX blockchain,
//! including energy trading, governance, and standard token transactions.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Main transaction structure for GridTokenX blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique transaction identifier
    pub id: String,
    /// Transaction type and data
    pub transaction_type: TransactionType,
    /// Sender's address
    pub from: String,
    /// Receiver's address (optional for some transaction types)
    pub to: Option<String>,
    /// Transaction fee in tokens
    pub fee: u64,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Digital signature
    pub signature: String,
    /// Transaction nonce (prevents replay attacks)
    pub nonce: u64,
    /// Gas limit for transaction execution
    pub gas_limit: u64,
    /// Gas price in tokens
    pub gas_price: u64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Different types of transactions supported by GridTokenX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    /// Standard token transfer
    TokenTransfer {
        amount: u64,
        message: Option<String>,
    },
    /// Energy trading transaction
    EnergyTrade(EnergyTransaction),
    /// Governance-related transaction
    Governance(GovernanceTransaction),
    /// Genesis block mint transaction
    GenesisMint { amount: u64, description: String },
    /// Authority registration
    AuthorityRegistration {
        authority_name: String,
        description: String,
        authority_type: AuthorityType,
    },
    /// Smart contract deployment
    ContractDeployment {
        bytecode: Vec<u8>,
        constructor_args: Vec<String>,
    },
    /// Smart contract execution
    ContractExecution {
        contract_address: String,
        method: String,
        args: Vec<String>,
    },
    /// IoT device energy measurement
    EnergyMeasurement {
        device_id: String,
        energy_consumed: f64,
        energy_produced: f64,
        instantaneous_power: f64,
        energy_source: String,
        location: String,
        timestamp: DateTime<Utc>,
        quality_metrics: Option<EnergyQualityMetrics>,
    },
    /// IoT device registration
    DeviceRegistration {
        device_id: String,
        device_type: String,
        location: String,
        grid_operator: String,
        capabilities: Vec<String>,
        firmware_version: Option<String>,
    },
}

/// Energy trading specific transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTransaction {
    /// Amount of energy in kWh
    pub energy_amount: f64,
    /// Price per kWh in tokens
    pub price_per_kwh: u64,
    /// Total transaction value in tokens
    pub total_value: u64,
    /// Energy source type (solar, wind, grid, etc.)
    pub energy_source: EnergySource,
    /// Delivery time window
    pub delivery_window: DeliveryWindow,
    /// Grid location information
    pub grid_location: GridLocation,
    /// Carbon credits generated/transferred
    pub carbon_credits: f64,
    /// Quality metrics
    pub quality_metrics: EnergyQualityMetrics,
    /// Regulatory compliance data
    pub compliance_data: ComplianceData,
    /// Order type (buy/sell/match)
    pub order_type: EnergyOrderType,
}

/// Types of energy sources in Thai energy market
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnergySource {
    /// Solar photovoltaic
    Solar,
    /// Wind power
    Wind,
    /// Hydroelectric
    Hydro,
    /// Biomass
    Biomass,
    /// Geothermal
    Geothermal,
    /// Natural gas
    NaturalGas,
    /// Coal
    Coal,
    /// Nuclear
    Nuclear,
    /// Grid mix (unknown source)
    GridMix,
    /// Battery storage
    Battery,
}

/// Energy delivery time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryWindow {
    /// Start time for energy delivery
    pub start_time: DateTime<Utc>,
    /// End time for energy delivery
    pub end_time: DateTime<Utc>,
    /// Flexibility in minutes (±)
    pub flexibility_minutes: u32,
}

/// Grid location information for energy trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridLocation {
    /// Thai provincial code
    pub province_code: String,
    /// Distribution area code
    pub distribution_area: String,
    /// Substation identifier
    pub substation_id: String,
    /// Voltage level (kV)
    pub voltage_level: f64,
    /// GPS coordinates
    pub coordinates: Option<(f64, f64)>,
}

/// Energy quality and grid stability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyQualityMetrics {
    /// Frequency stability (Hz)
    pub frequency: f64,
    /// Voltage stability (V)
    pub voltage: f64,
    /// Power factor
    pub power_factor: f64,
    /// Total harmonic distortion (%)
    pub thd: f64,
    /// Reliability score (0-100)
    pub reliability_score: u8,
}

/// Regulatory compliance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceData {
    /// ERC (Energy Regulatory Commission) approval
    pub erc_approved: bool,
    /// MEA/PEA registration number
    pub utility_registration: Option<String>,
    /// Environmental impact assessment
    pub environmental_compliance: bool,
    /// Safety certifications
    pub safety_certifications: Vec<String>,
    /// Renewable energy certificate (REC)
    pub rec_certificate: Option<String>,
}

/// Energy order types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnergyOrderType {
    /// Buy energy order
    Buy,
    /// Sell energy order
    Sell,
    /// Matched trade
    Match {
        buy_order_id: String,
        sell_order_id: String,
    },
}

/// Governance transaction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceTransaction {
    /// Submit a new proposal
    ProposalSubmission {
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_period_days: u32,
        execution_delay_days: u32,
    },
    /// Vote on a proposal
    Vote {
        proposal_id: String,
        vote: VoteChoice,
        voting_power: u64,
        reason: Option<String>,
    },
    /// Execute approved proposal
    ProposalExecution {
        proposal_id: String,
        execution_data: Vec<u8>,
    },
    /// Delegate voting power
    VotingDelegation {
        delegate_to: String,
        voting_power: u64,
        duration_blocks: u64,
    },
}

/// Types of governance proposals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// Change network parameters
    ParameterChange {
        parameter: String,
        new_value: String,
    },
    /// Upgrade smart contracts
    ContractUpgrade {
        contract_address: String,
        new_bytecode: Vec<u8>,
    },
    /// Add/remove energy authorities
    AuthorityManagement {
        action: AuthorityAction,
        authority_address: String,
    },
    /// Emergency response measures
    Emergency {
        emergency_type: EmergencyType,
        response_actions: Vec<String>,
    },
    /// Energy pricing regulations
    PricingRegulation {
        min_price: u64,
        max_price: u64,
        peak_hour_multiplier: f64,
    },
}

/// Voting choices for governance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// Authority management actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityAction {
    Add,
    Remove,
    Suspend,
    Reinstate,
}

/// Emergency types requiring governance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyType {
    GridFailure,
    CyberAttack,
    NaturalDisaster,
    MarketManipulation,
    RegulatoryViolation,
}

/// Authority types in Thai energy sector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityType {
    /// Electricity Generating Authority of Thailand
    EGAT,
    /// Metropolitan Electricity Authority
    MEA,
    /// Provincial Electricity Authority
    PEA,
    /// Energy Regulatory Commission
    ERC,
    /// Department of Alternative Energy Development and Efficiency
    DEDE,
    /// Office of the National Energy Security Committee
    ONESEC,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(
        transaction_type: TransactionType,
        from: String,
        to: Option<String>,
        fee: u64,
        nonce: u64,
    ) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        Ok(Self {
            id,
            transaction_type,
            from,
            to,
            fee,
            timestamp,
            signature: String::new(), // Will be filled when signing
            nonce,
            gas_limit: 100_000, // Default gas limit
            gas_price: 1,       // Default gas price
            metadata: HashMap::new(),
        })
    }

    /// Create a genesis mint transaction
    pub fn new_genesis_mint(to: String, amount: u64, description: String) -> Result<Self> {
        Self::new(
            TransactionType::GenesisMint {
                amount,
                description,
            },
            "system".to_string(),
            Some(to),
            0, // No fee for genesis
            0, // Genesis nonce
        )
    }

    /// Create an authority registration transaction
    pub fn new_authority_registration(authority_name: String, description: String) -> Result<Self> {
        Self::new(
            TransactionType::AuthorityRegistration {
                authority_name,
                description,
                authority_type: AuthorityType::ERC, // Default
            },
            "system".to_string(),
            None,
            0,
            0,
        )
    }

    /// Create an energy trading transaction
    pub fn new_energy_trade(
        from: String,
        to: String,
        energy_tx: EnergyTransaction,
        fee: u64,
        nonce: u64,
    ) -> Result<Self> {
        Self::new(
            TransactionType::EnergyTrade(energy_tx),
            from,
            Some(to),
            fee,
            nonce,
        )
    }

    /// Create a governance vote transaction
    pub fn new_governance_vote(
        from: String,
        proposal_id: String,
        vote: VoteChoice,
        voting_power: u64,
        fee: u64,
        nonce: u64,
    ) -> Result<Self> {
        Self::new(
            TransactionType::Governance(GovernanceTransaction::Vote {
                proposal_id,
                vote,
                voting_power,
                reason: None,
            }),
            from,
            None,
            fee,
            nonce,
        )
    }

    /// Calculate transaction hash
    pub fn hash(&self) -> Result<String> {
        let serialized = bincode::serialize(self)
            .map_err(|e| anyhow!("Failed to serialize transaction: {}", e))?;

        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Get transaction size in bytes
    pub fn size(&self) -> Result<usize> {
        bincode::serialize(self)
            .map(|data| data.len())
            .map_err(|e| anyhow!("Failed to calculate transaction size: {}", e))
    }

    /// Validate transaction structure
    pub fn validate(&self) -> Result<()> {
        // Basic validation
        if self.id.is_empty() {
            return Err(anyhow!("Transaction ID cannot be empty"));
        }

        if self.from.is_empty() {
            return Err(anyhow!("Sender address cannot be empty"));
        }

        if self.gas_limit == 0 {
            return Err(anyhow!("Gas limit must be greater than zero"));
        }

        // Validate specific transaction types
        match &self.transaction_type {
            TransactionType::TokenTransfer { amount, .. } => {
                if *amount == 0 {
                    return Err(anyhow!("Token transfer amount must be greater than zero"));
                }
                if self.to.is_none() {
                    return Err(anyhow!("Token transfer must have a recipient"));
                }
            }
            TransactionType::EnergyTrade(energy_tx) => {
                self.validate_energy_transaction(energy_tx)?;
            }
            TransactionType::Governance(gov_tx) => {
                self.validate_governance_transaction(gov_tx)?;
            }
            _ => {} // Other types validated elsewhere
        }

        Ok(())
    }

    /// Validate energy transaction specifics
    fn validate_energy_transaction(&self, energy_tx: &EnergyTransaction) -> Result<()> {
        if energy_tx.energy_amount <= 0.0 {
            return Err(anyhow!("Energy amount must be positive"));
        }

        if energy_tx.price_per_kwh == 0 {
            return Err(anyhow!("Energy price must be greater than zero"));
        }

        if energy_tx.delivery_window.start_time >= energy_tx.delivery_window.end_time {
            return Err(anyhow!("Invalid delivery window"));
        }

        // Validate Thai market constraints
        if energy_tx.energy_amount > 10_000.0 {
            return Err(anyhow!("Energy amount exceeds maximum limit (10,000 kWh)"));
        }

        // Validate price ranges (example Thai market rates)
        if energy_tx.price_per_kwh < 1_000 || energy_tx.price_per_kwh > 10_000 {
            return Err(anyhow!(
                "Energy price outside acceptable range (1-10 tokens/kWh)"
            ));
        }

        Ok(())
    }

    /// Validate governance transaction specifics
    fn validate_governance_transaction(&self, gov_tx: &GovernanceTransaction) -> Result<()> {
        match gov_tx {
            GovernanceTransaction::ProposalSubmission {
                title,
                description,
                voting_period_days,
                ..
            } => {
                if title.is_empty() || description.is_empty() {
                    return Err(anyhow!("Proposal title and description cannot be empty"));
                }
                if *voting_period_days == 0 || *voting_period_days > 30 {
                    return Err(anyhow!("Voting period must be between 1-30 days"));
                }
            }
            GovernanceTransaction::Vote {
                proposal_id,
                voting_power,
                ..
            } => {
                if proposal_id.is_empty() {
                    return Err(anyhow!("Proposal ID cannot be empty"));
                }
                if *voting_power == 0 {
                    return Err(anyhow!("Voting power must be greater than zero"));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Sign transaction with private key
    pub fn sign(&mut self, private_key: &[u8]) -> Result<()> {
        // This would implement actual cryptographic signing
        // For now, we'll create a placeholder signature
        let tx_hash = self.hash()?;

        // In a real implementation, this would use ed25519 or similar
        let signature_data = format!("{}-{}", hex::encode(private_key), tx_hash);
        let mut hasher = Sha256::new();
        hasher.update(signature_data.as_bytes());
        self.signature = hex::encode(hasher.finalize());

        Ok(())
    }

    /// Verify transaction signature
    pub fn verify_signature(&self, public_key: &[u8]) -> Result<bool> {
        if self.signature.is_empty() {
            return Ok(false);
        }

        // This would implement actual signature verification
        // For now, we'll do a basic check
        Ok(self.signature.len() == 64) // SHA256 hex string length
    }

    /// Get total transaction cost (including fees and gas)
    pub fn get_total_cost(&self) -> u64 {
        let gas_cost = self.gas_limit * self.gas_price;
        self.fee + gas_cost
    }

    /// Check if transaction is energy-related
    pub fn is_energy_transaction(&self) -> bool {
        matches!(self.transaction_type, TransactionType::EnergyTrade(_))
    }

    /// Check if transaction is governance-related
    pub fn is_governance_transaction(&self) -> bool {
        matches!(self.transaction_type, TransactionType::Governance(_))
    }

    /// Get carbon credits impact of transaction
    pub fn get_carbon_impact(&self) -> f64 {
        match &self.transaction_type {
            TransactionType::EnergyTrade(energy_tx) => energy_tx.carbon_credits,
            _ => 0.0,
        }
    }
}

impl EnergyTransaction {
    /// Create a new energy buy order
    pub fn new_buy_order(
        energy_amount: f64,
        max_price_per_kwh: u64,
        delivery_window: DeliveryWindow,
        grid_location: GridLocation,
    ) -> Self {
        Self {
            energy_amount,
            price_per_kwh: max_price_per_kwh,
            total_value: (energy_amount * max_price_per_kwh as f64) as u64,
            energy_source: EnergySource::GridMix, // Buyer doesn't specify source
            delivery_window,
            grid_location,
            carbon_credits: 0.0,
            quality_metrics: EnergyQualityMetrics::default(),
            compliance_data: ComplianceData::default(),
            order_type: EnergyOrderType::Buy,
        }
    }

    /// Create a new energy sell order
    pub fn new_sell_order(
        energy_amount: f64,
        min_price_per_kwh: u64,
        energy_source: EnergySource,
        delivery_window: DeliveryWindow,
        grid_location: GridLocation,
    ) -> Self {
        let carbon_credits = match energy_source {
            EnergySource::Solar => energy_amount * 0.5,
            EnergySource::Wind => energy_amount * 0.6,
            EnergySource::Hydro => energy_amount * 0.4,
            _ => 0.0,
        };

        Self {
            energy_amount,
            price_per_kwh: min_price_per_kwh,
            total_value: (energy_amount * min_price_per_kwh as f64) as u64,
            energy_source,
            delivery_window,
            grid_location,
            carbon_credits,
            quality_metrics: EnergyQualityMetrics::default(),
            compliance_data: ComplianceData::default(),
            order_type: EnergyOrderType::Sell,
        }
    }
}

impl Default for EnergyQualityMetrics {
    fn default() -> Self {
        Self {
            frequency: 50.0,       // 50Hz standard in Thailand
            voltage: 220.0,        // 220V standard voltage
            power_factor: 0.95,    // Good power factor
            thd: 5.0,              // 5% THD is acceptable
            reliability_score: 85, // 85% reliability
        }
    }
}

impl Default for ComplianceData {
    fn default() -> Self {
        Self {
            erc_approved: false,
            utility_registration: None,
            environmental_compliance: false,
            safety_certifications: Vec::new(),
            rec_certificate: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            TransactionType::TokenTransfer {
                amount: 1000,
                message: Some("Test transfer".to_string()),
            },
            "sender".to_string(),
            Some("receiver".to_string()),
            10,
            1,
        )
        .unwrap();

        assert!(!tx.id.is_empty());
        assert_eq!(tx.from, "sender");
        assert_eq!(tx.to, Some("receiver".to_string()));
        assert_eq!(tx.fee, 10);
    }

    #[test]
    fn test_energy_transaction_validation() {
        let mut tx = Transaction::new_energy_trade(
            "producer".to_string(),
            "consumer".to_string(),
            EnergyTransaction::new_sell_order(
                100.0,
                5000,
                EnergySource::Solar,
                DeliveryWindow {
                    start_time: Utc::now(),
                    end_time: Utc::now() + chrono::Duration::hours(1),
                    flexibility_minutes: 15,
                },
                GridLocation {
                    province_code: "BKK".to_string(),
                    distribution_area: "MEA-01".to_string(),
                    substation_id: "SUB-001".to_string(),
                    voltage_level: 22.0,
                    coordinates: Some((13.7563, 100.5018)),
                },
            ),
            100,
            1,
        )
        .unwrap();

        assert!(tx.validate().is_ok());
        assert!(tx.is_energy_transaction());
        assert_eq!(tx.get_carbon_impact(), 50.0); // 100 kWh * 0.5 for solar
    }

    #[test]
    fn test_governance_transaction() {
        let tx = Transaction::new_governance_vote(
            "voter".to_string(),
            "proposal-123".to_string(),
            VoteChoice::Yes,
            1000,
            10,
            1,
        )
        .unwrap();

        assert!(tx.validate().is_ok());
        assert!(tx.is_governance_transaction());
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction::new(
            TransactionType::TokenTransfer {
                amount: 1000,
                message: None,
            },
            "sender".to_string(),
            Some("receiver".to_string()),
            10,
            1,
        )
        .unwrap();

        let hash1 = tx.hash().unwrap();
        let hash2 = tx.hash().unwrap();
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex string
    }
}
