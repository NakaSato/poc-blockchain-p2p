//! Energy Trading Value Objects
//!
//! Contains value objects specific to energy trading domain.

use crate::shared::domain::{value_objects::ValueObject, errors::DomainError};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

/// Trade identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeId(String);

impl ValueObject for TradeId {}

impl TradeId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::invalid_value("Trade ID cannot be empty"));
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

impl FromStr for TradeId {
    type Err = DomainError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl std::fmt::Display for TradeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Trader identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraderId(String);

impl ValueObject for TraderId {}

impl TraderId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::invalid_value("Trader ID cannot be empty"));
        }
        Ok(Self(id))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for TraderId {
    type Err = DomainError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl std::fmt::Display for TraderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Energy amount in kWh
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EnergyAmount(f64);

impl ValueObject for EnergyAmount {}

impl EnergyAmount {
    pub fn new(amount: f64) -> Result<Self, DomainError> {
        if amount <= 0.0 {
            return Err(DomainError::invalid_value("Energy amount must be positive"));
        }
        if !amount.is_finite() {
            return Err(DomainError::invalid_value("Energy amount must be a valid number"));
        }
        Ok(Self(amount))
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }
    
    pub fn add(&self, other: &EnergyAmount) -> Result<EnergyAmount, DomainError> {
        EnergyAmount::new(self.0 + other.0)
    }
    
    pub fn subtract(&self, other: &EnergyAmount) -> Result<EnergyAmount, DomainError> {
        EnergyAmount::new(self.0 - other.0)
    }
    
    pub fn multiply(&self, factor: f64) -> Result<EnergyAmount, DomainError> {
        EnergyAmount::new(self.0 * factor)
    }
}

impl std::fmt::Display for EnergyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} kWh", self.0)
    }
}

/// Price per kWh
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PricePerKwh(f64);

impl ValueObject for PricePerKwh {}

impl PricePerKwh {
    pub fn new(price: f64) -> Result<Self, DomainError> {
        if price < 0.0 {
            return Err(DomainError::invalid_value("Price per kWh cannot be negative"));
        }
        if price.is_infinite() || price.is_nan() {
            return Err(DomainError::invalid_value("Price per kWh must be a valid number"));
        }
        Ok(Self(price))
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }
    
    pub fn calculate_total_price(&self, amount: &EnergyAmount) -> f64 {
        self.0 * amount.value()
    }
}

impl std::fmt::Display for PricePerKwh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.4}/kWh", self.0)
    }
}

/// Trade type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeType {
    Buy,
    Sell,
}

impl ValueObject for TradeType {}

impl TradeType {
    pub fn is_buy(&self) -> bool {
        matches!(self, TradeType::Buy)
    }
    
    pub fn is_sell(&self) -> bool {
        matches!(self, TradeType::Sell)
    }
    
    pub fn opposite(&self) -> TradeType {
        match self {
            TradeType::Buy => TradeType::Sell,
            TradeType::Sell => TradeType::Buy,
        }
    }
    
    pub fn from_string(s: &str) -> Result<Self, DomainError> {
        Self::from_str(s)
    }
}

impl std::fmt::Display for TradeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeType::Buy => write!(f, "BUY"),
            TradeType::Sell => write!(f, "SELL"),
        }
    }
}

impl FromStr for TradeType {
    type Err = DomainError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BUY" | "Buy" => Ok(TradeType::Buy),
            "SELL" | "Sell" => Ok(TradeType::Sell),
            _ => Err(DomainError::invalid_value(format!("Invalid trade type: {}", s))),
        }
    }
}

/// Trade status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeStatus {
    Active,          // Order is active and available for matching
    Pending,         // Trade is pending execution
    PartiallyFilled, // Order is partially filled
    Filled,          // Order is completely filled  
    Matched,         // Trade has been matched
    Executed,        // Trade has been executed
    Settled,         // Trade has been settled
    Cancelled,       // Trade/Order has been cancelled
    Failed,          // Trade/Order has failed
}

impl ValueObject for TradeStatus {}

impl TradeStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, TradeStatus::Active | TradeStatus::Pending | TradeStatus::PartiallyFilled | TradeStatus::Matched | TradeStatus::Executed)
    }
    
    pub fn is_final(&self) -> bool {
        matches!(self, TradeStatus::Filled | TradeStatus::Settled | TradeStatus::Cancelled | TradeStatus::Failed)
    }
    
    pub fn can_cancel(&self) -> bool {
        matches!(self, TradeStatus::Active | TradeStatus::Pending | TradeStatus::PartiallyFilled)
    }
    
    pub fn can_fill(&self) -> bool {
        matches!(self, TradeStatus::Active | TradeStatus::PartiallyFilled)
    }
    
    pub fn can_transition_to(&self, new_status: &TradeStatus) -> bool {
        match (self, new_status) {
            // From Active order state
            (TradeStatus::Active, TradeStatus::PartiallyFilled) => true,
            (TradeStatus::Active, TradeStatus::Filled) => true,
            (TradeStatus::Active, TradeStatus::Cancelled) => true,
            (TradeStatus::Active, TradeStatus::Matched) => true,
            
            // From PartiallyFilled state
            (TradeStatus::PartiallyFilled, TradeStatus::Filled) => true,
            (TradeStatus::PartiallyFilled, TradeStatus::Cancelled) => true,
            
            // From trade states
            (TradeStatus::Pending, TradeStatus::Matched) => true,
            (TradeStatus::Pending, TradeStatus::Cancelled) => true,
            (TradeStatus::Matched, TradeStatus::Executed) => true,
            (TradeStatus::Matched, TradeStatus::Failed) => true,
            (TradeStatus::Executed, TradeStatus::Settled) => true,
            (TradeStatus::Executed, TradeStatus::Failed) => true,
            
            // From Filled to settlement
            (TradeStatus::Filled, TradeStatus::Settled) => true,
            (TradeStatus::Filled, TradeStatus::Failed) => true,
            
            _ => false,
        }
    }
}

impl std::fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeStatus::Active => write!(f, "ACTIVE"),
            TradeStatus::Pending => write!(f, "PENDING"),
            TradeStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            TradeStatus::Filled => write!(f, "FILLED"),
            TradeStatus::Matched => write!(f, "MATCHED"),
            TradeStatus::Executed => write!(f, "EXECUTED"),
            TradeStatus::Settled => write!(f, "SETTLED"),
            TradeStatus::Cancelled => write!(f, "CANCELLED"),
            TradeStatus::Failed => write!(f, "FAILED"),
        }
    }
}

/// Trading window for time-based trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingWindow {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
}

impl ValueObject for TradingWindow {}

impl TradingWindow {
    pub fn new(start_time: chrono::DateTime<chrono::Utc>, end_time: chrono::DateTime<chrono::Utc>) -> Result<Self, DomainError> {
        if start_time >= end_time {
            return Err(DomainError::invalid_value("Start time must be before end time")); 
        }
        
        Ok(Self { start_time, end_time })
    }
    
    pub fn duration(&self) -> chrono::Duration {
        self.end_time - self.start_time
    }
    
    pub fn is_valid(&self) -> bool {
        self.start_time < self.end_time
    }
    
    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now();
        now >= self.start_time && now <= self.end_time
    }
    
    pub fn overlaps_with(&self, other: &TradingWindow) -> bool {
        self.start_time < other.end_time && self.end_time > other.start_time
    }
}
