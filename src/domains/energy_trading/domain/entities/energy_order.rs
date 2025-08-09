//! Energy Order Entity
//!
//! Represents an energy buy or sell order in the trading system.

use crate::shared::domain::{
    errors::DomainError,
    repository::AggregateRoot,
    events::DomainEvent,
};
use crate::domains::energy_trading::domain::value_objects::{
    TradeId, TraderId, EnergyAmount, PricePerKwh, TradeType, TradeStatus, TradingWindow
};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Energy Order Entity
pub struct EnergyOrder {
    id: TradeId,
    trader_id: TraderId,
    order_type: TradeType,
    energy_amount: EnergyAmount,
    remaining_amount: EnergyAmount,
    price_per_kwh: PricePerKwh,
    trading_window: TradingWindow,
    status: TradeStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    version: u64,
    uncommitted_events: VecDeque<Box<dyn DomainEvent>>,
}

impl Clone for EnergyOrder {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            trader_id: self.trader_id.clone(),
            order_type: self.order_type.clone(),
            energy_amount: self.energy_amount.clone(),
            remaining_amount: self.remaining_amount.clone(),
            price_per_kwh: self.price_per_kwh.clone(),
            trading_window: self.trading_window.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            version: self.version,
            uncommitted_events: VecDeque::new(), // Don't clone events
        }
    }
}

impl EnergyOrder {
    pub fn new(
        trader_id: TraderId,
        order_type: TradeType,
        energy_amount: EnergyAmount,
        price_per_kwh: PricePerKwh,
        trading_window: TradingWindow,
    ) -> Result<Self, DomainError> {
        // Business rule validation
        Self::validate_order_parameters(&energy_amount, &price_per_kwh, &trading_window)?;
        
        let id = TradeId::generate();
        let now = Utc::now();
        
        let mut order = Self {
            id: id.clone(),
            trader_id: trader_id.clone(),
            order_type: order_type.clone(),
            energy_amount: energy_amount.clone(),
            remaining_amount: energy_amount.clone(),
            price_per_kwh: price_per_kwh.clone(),
            trading_window: trading_window.clone(),
            status: TradeStatus::Active,
            created_at: now,
            updated_at: now,
            version: 1,
            uncommitted_events: VecDeque::new(),
        };
        
        // Raise domain event
        order.raise_event(Box::new(EnergyOrderCreatedEvent {
            event_id: Uuid::new_v4(),
            order_id: id.value().to_string(),
            trader_id: trader_id.value().to_string(),
            order_type: order_type.to_string(),
            energy_amount: energy_amount.value(),
            price_per_kwh: price_per_kwh.value(),
            occurred_at: now,
            aggregate_version: 1,
        }));
        
        Ok(order)
    }
    
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        match self.status {
            TradeStatus::Active | TradeStatus::PartiallyFilled => {
                self.status = TradeStatus::Cancelled;
                self.updated_at = Utc::now();
                self.version += 1;
                
                self.raise_event(Box::new(EnergyOrderCancelledEvent {
                    event_id: Uuid::new_v4(),
                    order_id: self.id.value().to_string(),
                    trader_id: self.trader_id.value().to_string(),
                    occurred_at: self.updated_at,
                    aggregate_version: self.version,
                }));
                
                Ok(())
            }
            _ => Err(DomainError::business_rule_violation(
                "Order cannot be cancelled in current status"
            )),
        }
    }
    
    pub fn fill_partially(&mut self, filled_amount: EnergyAmount) -> Result<(), DomainError> {
        if filled_amount.value() > self.remaining_amount.value() {
            return Err(DomainError::business_rule_violation(
                "Fill amount cannot exceed remaining amount"
            ));
        }
        
        let new_remaining = EnergyAmount::new(
            self.remaining_amount.value() - filled_amount.value()
        )?;
        
        self.remaining_amount = new_remaining.clone();
        self.updated_at = Utc::now();
        self.version += 1;
        
        // Update status based on remaining amount
        if new_remaining.value() == 0.0 {
            self.status = TradeStatus::Filled;
        } else {
            self.status = TradeStatus::PartiallyFilled;
        }
        
        self.raise_event(Box::new(EnergyOrderFilledEvent {
            event_id: Uuid::new_v4(),
            order_id: self.id.value().to_string(),
            trader_id: self.trader_id.value().to_string(),
            filled_amount: filled_amount.value(),
            remaining_amount: new_remaining.value(),
            new_status: self.status.to_string(),
            occurred_at: self.updated_at,
            aggregate_version: self.version,
        }));
        
        Ok(())
    }
    
    fn validate_order_parameters(
        energy_amount: &EnergyAmount,
        price_per_kwh: &PricePerKwh,
        trading_window: &TradingWindow,
    ) -> Result<(), DomainError> {
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
        
        if !trading_window.is_valid() {
            return Err(DomainError::business_rule_violation(
                "Trading window must be valid"
            ));
        }
        
        Ok(())
    }
    
    // Getters
    pub fn id(&self) -> &TradeId { &self.id }
    pub fn trader_id(&self) -> &TraderId { &self.trader_id }
    pub fn order_type(&self) -> &TradeType { &self.order_type }
    pub fn energy_amount(&self) -> &EnergyAmount { &self.energy_amount }
    pub fn remaining_amount(&self) -> &EnergyAmount { &self.remaining_amount }
    pub fn price_per_kwh(&self) -> &PricePerKwh { &self.price_per_kwh }
    pub fn trading_window(&self) -> &TradingWindow { &self.trading_window }
    pub fn status(&self) -> &TradeStatus { &self.status }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
}

impl AggregateRoot for EnergyOrder {
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
        // For now, we use state-based aggregates
    }
    
    fn raise_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push_back(event);
    }
}

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyOrderCreatedEvent {
    pub event_id: Uuid,
    pub order_id: String,
    pub trader_id: String,
    pub order_type: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub occurred_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for EnergyOrderCreatedEvent {
    fn event_type(&self) -> &'static str {
        "EnergyOrderCreated"
    }
    
    fn aggregate_id(&self) -> String {
        self.order_id.clone()
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
pub struct EnergyOrderCancelledEvent {
    pub event_id: Uuid,
    pub order_id: String,
    pub trader_id: String,
    pub occurred_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for EnergyOrderCancelledEvent {
    fn event_type(&self) -> &'static str {
        "EnergyOrderCancelled"
    }
    
    fn aggregate_id(&self) -> String {
        self.order_id.clone()
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
pub struct EnergyOrderFilledEvent {
    pub event_id: Uuid,
    pub order_id: String,
    pub trader_id: String,
    pub filled_amount: f64,
    pub remaining_amount: f64,
    pub new_status: String,
    pub occurred_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for EnergyOrderFilledEvent {
    fn event_type(&self) -> &'static str {
        "EnergyOrderFilled"
    }
    
    fn aggregate_id(&self) -> String {
        self.order_id.clone()
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

impl std::fmt::Debug for EnergyOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnergyOrder")
            .field("id", &self.id)
            .field("trader_id", &self.trader_id)
            .field("order_type", &self.order_type)
            .field("energy_amount", &self.energy_amount)
            .field("price_per_kwh", &self.price_per_kwh)
            .field("status", &self.status)
            .field("trading_window", &self.trading_window)
            .field("remaining_amount", &self.remaining_amount)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("version", &self.version)
            .field("uncommitted_events_count", &self.uncommitted_events.len())
            .finish()
    }
}
