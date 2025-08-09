//! Energy Trading Bounded Context
//!
//! This module contains the complete energy trading domain implementation
//! following Domain-Driven Design principles.

pub mod domain;
pub mod application;

#[cfg(test)]
mod tests;

// Re-export domain types for easy access
pub use domain::{
    // Value Objects
    TradeId, TraderId, EnergyAmount, PricePerKwh, TradeType, TradeStatus, TradingWindow,
    
    // Entities
    EnergyOrder, EnergyTrade, SettlementStatus,
    
    // Aggregates
    OrderBook, MarketDepth, PriceLevel,
    
    // Services
    EnergyTradingDomainService, PricingStrategy,
    
    // Events
    EnergyOrderCreatedEvent, EnergyOrderCancelledEvent, EnergyOrderFilledEvent,
    EnergyTradeExecutedEvent, EnergyDeliveredEvent, PaymentCompletedEvent, TradeSettledEvent,
    TradesExecutedEvent,
};

// Re-export application types
pub use application::{
    // Commands
    PlaceEnergyOrderCommand, PlaceEnergyOrderResult, PlaceEnergyOrderHandler,
    CancelEnergyOrderCommand, CancelEnergyOrderResult, CancelEnergyOrderHandler,
    TradeExecutionResult,
    
    // Queries
    GetMarketDepthQuery, GetMarketDepthResult, GetMarketDepthHandler,
};
