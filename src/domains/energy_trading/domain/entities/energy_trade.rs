//! Energy Trade Entity
//!
//! Represents a completed trade between a buyer and seller.

use crate::shared::domain::{
    errors::DomainError,
    repository::AggregateRoot,
    events::DomainEvent,
};
use crate::domains::energy_trading::domain::value_objects::{
    TradeId, TraderId, EnergyAmount, PricePerKwh
};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Settlement status for a trade
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementStatus {
    Pending,
    EnergyDelivered,
    PaymentCompleted,
    Settled,
    Failed,
}

impl std::fmt::Display for SettlementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettlementStatus::Pending => write!(f, "Pending"),
            SettlementStatus::EnergyDelivered => write!(f, "EnergyDelivered"),
            SettlementStatus::PaymentCompleted => write!(f, "PaymentCompleted"),
            SettlementStatus::Settled => write!(f, "Settled"),
            SettlementStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Energy Trade Entity
pub struct EnergyTrade {
    id: TradeId,
    buyer_id: TraderId,
    seller_id: TraderId,
    energy_amount: EnergyAmount,
    price_per_kwh: PricePerKwh,
    total_price: f64,
    settlement_status: SettlementStatus,
    executed_at: DateTime<Utc>,
    settled_at: Option<DateTime<Utc>>,
    version: u64,
    uncommitted_events: VecDeque<Box<dyn DomainEvent>>,
}

impl EnergyTrade {
    pub fn new(
        buyer_id: TraderId,
        seller_id: TraderId,
        energy_amount: EnergyAmount,
        price_per_kwh: PricePerKwh,
    ) -> Result<Self, DomainError> {
        // Business rule validation
        Self::validate_trade_parameters(&buyer_id, &seller_id, &energy_amount, &price_per_kwh)?;
        
        let id = TradeId::generate();
        let executed_at = Utc::now();
        let total_price = energy_amount.value() * price_per_kwh.value();
        
        let mut trade = Self {
            id: id.clone(),
            buyer_id: buyer_id.clone(),
            seller_id: seller_id.clone(),
            energy_amount: energy_amount.clone(),
            price_per_kwh: price_per_kwh.clone(),
            total_price,
            settlement_status: SettlementStatus::Pending,
            executed_at,
            settled_at: None,
            version: 1,
            uncommitted_events: VecDeque::new(),
        };
        
        // Raise domain event
        trade.raise_event(Box::new(EnergyTradeExecutedEvent {
            event_id: Uuid::new_v4(),
            trade_id: id.value().to_string(),
            buyer_id: buyer_id.value().to_string(),
            seller_id: seller_id.value().to_string(),
            energy_amount: energy_amount.value(),
            price_per_kwh: price_per_kwh.value(),
            total_price,
            executed_at,
            aggregate_version: 1,
        }));
        
        Ok(trade)
    }
    
    pub fn mark_energy_delivered(&mut self) -> Result<(), DomainError> {
        match self.settlement_status {
            SettlementStatus::Pending => {
                self.settlement_status = SettlementStatus::EnergyDelivered;
                self.version += 1;
                
                self.raise_event(Box::new(EnergyDeliveredEvent {
                    event_id: Uuid::new_v4(),
                    trade_id: self.id.value().to_string(),
                    buyer_id: self.buyer_id.value().to_string(),
                    seller_id: self.seller_id.value().to_string(),
                    energy_amount: self.energy_amount.value(),
                    occurred_at: Utc::now(),
                    aggregate_version: self.version,
                }));
                
                self.check_settlement();
                Ok(())
            }
            _ => Err(DomainError::business_rule_violation(
                "Energy can only be delivered when trade is pending"
            )),
        }
    }
    
    pub fn mark_payment_completed(&mut self) -> Result<(), DomainError> {
        match self.settlement_status {
            SettlementStatus::Pending | SettlementStatus::EnergyDelivered => {
                let previous_status = self.settlement_status.clone();
                self.settlement_status = SettlementStatus::PaymentCompleted;
                self.version += 1;
                
                self.raise_event(Box::new(PaymentCompletedEvent {
                    event_id: Uuid::new_v4(),
                    trade_id: self.id.value().to_string(),
                    buyer_id: self.buyer_id.value().to_string(),
                    seller_id: self.seller_id.value().to_string(),
                    total_price: self.total_price,
                    occurred_at: Utc::now(),
                    aggregate_version: self.version,
                }));
                
                // Check if we can settle now
                if previous_status == SettlementStatus::EnergyDelivered {
                    self.settle();
                }
                
                Ok(())
            }
            _ => Err(DomainError::business_rule_violation(
                "Payment can only be completed when trade is pending or energy delivered"
            )),
        }
    }
    
    fn settle(&mut self) {
        self.settlement_status = SettlementStatus::Settled;
        self.settled_at = Some(Utc::now());
        self.version += 1;
        
        self.raise_event(Box::new(TradeSettledEvent {
            event_id: Uuid::new_v4(),
            trade_id: self.id.value().to_string(),
            buyer_id: self.buyer_id.value().to_string(),
            seller_id: self.seller_id.value().to_string(),
            energy_amount: self.energy_amount.value(),
            total_price: self.total_price,
            settled_at: self.settled_at.unwrap(),
            aggregate_version: self.version,
        }));
    }
    
    fn check_settlement(&mut self) {
        if self.settlement_status == SettlementStatus::EnergyDelivered {
            // In real system, check if payment is also completed
            // For now, auto-settle after energy delivery
            self.settle();
        }
    }
    
    fn validate_trade_parameters(
        buyer_id: &TraderId,
        seller_id: &TraderId,
        energy_amount: &EnergyAmount,
        price_per_kwh: &PricePerKwh,
    ) -> Result<(), DomainError> {
        if buyer_id == seller_id {
            return Err(DomainError::business_rule_violation(
                "Buyer and seller cannot be the same"
            ));
        }
        
        if energy_amount.value() <= 0.0 {
            return Err(DomainError::business_rule_violation(
                "Energy amount must be positive"
            ));
        }
        
        if price_per_kwh.value() <= 0.0 {
            return Err(DomainError::business_rule_violation(
                "Price must be positive"
            ));
        }
        
        Ok(())
    }
    
    // Getters
    pub fn id(&self) -> &TradeId { &self.id }
    pub fn buyer_id(&self) -> &TraderId { &self.buyer_id }
    pub fn seller_id(&self) -> &TraderId { &self.seller_id }
    pub fn energy_amount(&self) -> &EnergyAmount { &self.energy_amount }
    pub fn price_per_kwh(&self) -> &PricePerKwh { &self.price_per_kwh }
    pub fn total_price(&self) -> f64 { self.total_price }
    pub fn settlement_status(&self) -> &SettlementStatus { &self.settlement_status }
    pub fn executed_at(&self) -> DateTime<Utc> { self.executed_at }
    pub fn settled_at(&self) -> Option<DateTime<Utc>> { self.settled_at }
}

impl AggregateRoot for EnergyTrade {
    type Id = TradeId;
    
    fn id(&self) -> &Self::Id {
        &self.id
    }
    
    fn version(&self) -> u64 {
        self.version
    }
    
    fn uncommitted_events(&self) -> &VecDeque<Box<dyn DomainEvent>> {
        &self.uncommitted_events
    }
    
    fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
    
    fn apply_event(&mut self, _event: Box<dyn DomainEvent>) {
        // Event sourcing implementation would go here
    }
    
    fn raise_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push_back(event);
    }
}

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradeExecutedEvent {
    pub event_id: Uuid,
    pub trade_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub total_price: f64,
    pub executed_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for EnergyTradeExecutedEvent {
    fn event_type(&self) -> &'static str {
        "EnergyTradeExecuted"
    }
    
    fn aggregate_id(&self) -> String {
        self.trade_id.clone()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.executed_at
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn aggregate_version(&self) -> u64 {
        self.aggregate_version
    }
    
    fn event_id(&self) -> Uuid {
        self.event_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyDeliveredEvent {
    pub event_id: Uuid,
    pub trade_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub energy_amount: f64,
    pub occurred_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for EnergyDeliveredEvent {
    fn event_type(&self) -> &'static str {
        "EnergyDelivered"
    }
    
    fn aggregate_id(&self) -> String {
        self.trade_id.clone()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn aggregate_version(&self) -> u64 {
        self.aggregate_version
    }
    
    fn event_id(&self) -> Uuid {
        self.event_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCompletedEvent {
    pub event_id: Uuid,
    pub trade_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub total_price: f64,
    pub occurred_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for PaymentCompletedEvent {
    fn event_type(&self) -> &'static str {
        "PaymentCompleted"
    }
    
    fn aggregate_id(&self) -> String {
        self.trade_id.clone()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn aggregate_version(&self) -> u64 {
        self.aggregate_version
    }
    
    fn event_id(&self) -> Uuid {
        self.event_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSettledEvent {
    pub event_id: Uuid,
    pub trade_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub energy_amount: f64,
    pub total_price: f64,
    pub settled_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for TradeSettledEvent {
    fn event_type(&self) -> &'static str {
        "TradeSettled"
    }
    
    fn aggregate_id(&self) -> String {
        self.trade_id.clone()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.settled_at
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn aggregate_version(&self) -> u64 {
        self.aggregate_version
    }
    
    fn event_id(&self) -> Uuid {
        self.event_id
    }
}

use std::fmt;

impl fmt::Debug for EnergyTrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnergyTrade")
            .field("id", &self.id)
            .field("buyer_id", &self.buyer_id)
            .field("seller_id", &self.seller_id)
            .field("energy_amount", &self.energy_amount)
            .field("price_per_kwh", &self.price_per_kwh)
            .field("total_price", &self.total_price)
            .field("settlement_status", &self.settlement_status)
            .field("executed_at", &self.executed_at)
            .field("settled_at", &self.settled_at)
            .field("version", &self.version)
            .field("uncommitted_events_count", &self.uncommitted_events.len())
            .finish()
    }
}
