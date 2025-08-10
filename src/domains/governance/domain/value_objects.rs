//! Governance Domain Value Objects
//!
//! Core value objects for the governance domain including tokens,
//! stakes, RECs, and voting power.

use crate::shared::domain::errors::DomainError;
use chrono::Duration;
use serde::{Serialize, Deserialize};
use std::fmt;

/// Token amount with precision handling
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TokenAmount {
    // Stored as smallest unit (e.g., wei for ETH-like tokens)
    amount_smallest_unit: u64,
    decimals: u8,
}

impl TokenAmount {
    pub fn new(amount: f64, decimals: u8) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::InvalidValue("Token amount cannot be negative".to_string()));
        }
        
        let multiplier = 10_u64.pow(decimals as u32);
        let amount_smallest_unit = (amount * multiplier as f64) as u64;
        
        Ok(Self {
            amount_smallest_unit,
            decimals,
        })
    }

    pub fn zero() -> Self {
        Self {
            amount_smallest_unit: 0,
            decimals: 18, // Default to 18 decimals like ETH
        }
    }

    pub fn value(&self) -> f64 {
        let multiplier = 10_u64.pow(self.decimals as u32);
        self.amount_smallest_unit as f64 / multiplier as f64
    }

    pub fn add(&self, other: &TokenAmount) -> Result<TokenAmount, DomainError> {
        if self.decimals != other.decimals {
            return Err(DomainError::InvalidOperation("Cannot add tokens with different decimals".to_string()));
        }
        
        Ok(TokenAmount {
            amount_smallest_unit: self.amount_smallest_unit + other.amount_smallest_unit,
            decimals: self.decimals,
        })
    }
}

impl fmt::Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6} tokens", self.value())
    }
}

/// Unique identifier for stakes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StakeId(String);

impl StakeId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::InvalidValue("Stake ID cannot be empty".to_string()));
        }
        Ok(Self(id))
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StakeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for RECs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RECId(String);

impl RECId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::InvalidValue("REC ID cannot be empty".to_string()));
        }
        Ok(Self(id))
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RECId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Voting power calculation based on stake
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VotingPower {
    base_tokens: TokenAmount,
    multiplier: f64,
    effective_power: u64,
}

impl VotingPower {
    pub fn new(staked_tokens: TokenAmount, lock_duration_days: u32) -> Result<Self, DomainError> {
        let multiplier = match lock_duration_days {
            0..=30 => 1.0,
            31..=90 => 1.25,
            91..=180 => 1.5,
            181..=365 => 2.0,
            _ => 2.5,
        };

        let effective_power = (staked_tokens.value() * multiplier) as u64;

        Ok(Self {
            base_tokens: staked_tokens,
            multiplier,
            effective_power,
        })
    }

    pub fn calculate_power(token_amount: &TokenAmount, lock_duration: &Duration) -> Result<Self, DomainError> {
        let lock_days = lock_duration.num_days() as u32;
        Self::new(token_amount.clone(), lock_days)
    }

    pub fn effective_power(&self) -> u64 {
        self.effective_power
    }

    pub fn base_tokens(&self) -> &TokenAmount {
        &self.base_tokens
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl fmt::Display for VotingPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} voting power ({}x multiplier)", self.effective_power, self.multiplier)
    }
}

/// Stable credit amount for energy trading
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StableCreditAmount {
    amount_cents: u64, // Stored in cents for precision
}

impl StableCreditAmount {
    pub fn new(amount: f64) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::InvalidValue("Credit amount cannot be negative".to_string()));
        }
        
        Ok(Self {
            amount_cents: (amount * 100.0) as u64,
        })
    }

    pub fn zero() -> Self {
        Self { amount_cents: 0 }
    }

    pub fn value(&self) -> f64 {
        self.amount_cents as f64 / 100.0
    }
}

impl fmt::Display for StableCreditAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.value())
    }
}

/// Energy source types for RECs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergySourceType {
    Solar,
    Wind,
    Hydro,
    Geothermal,
    Biomass,
    Nuclear,
    Other(String),
}

impl fmt::Display for EnergySourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnergySourceType::Solar => write!(f, "Solar"),
            EnergySourceType::Wind => write!(f, "Wind"),
            EnergySourceType::Hydro => write!(f, "Hydro"),
            EnergySourceType::Geothermal => write!(f, "Geothermal"),
            EnergySourceType::Biomass => write!(f, "Biomass"),
            EnergySourceType::Nuclear => write!(f, "Nuclear"),
            EnergySourceType::Other(source) => write!(f, "Other: {}", source),
        }
    }
}
