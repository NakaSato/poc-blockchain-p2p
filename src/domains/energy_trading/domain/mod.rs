//! Energy Trading Domain Layer
//!
//! Contains the core business logic for energy trading.

pub mod value_objects;
pub mod entities;
pub mod aggregates;
pub mod services;

pub use value_objects::{
    TradeId, TraderId, EnergyAmount, PricePerKwh, TradeType, TradeStatus, TradingWindow
};
pub use entities::{
    EnergyOrder, EnergyTrade, SettlementStatus,
    EnergyOrderCreatedEvent, EnergyOrderCancelledEvent, EnergyOrderFilledEvent,
    EnergyTradeExecutedEvent, EnergyDeliveredEvent, PaymentCompletedEvent, TradeSettledEvent,
};
pub use aggregates::{
    OrderBook, MarketDepth, PriceLevel, TradesExecutedEvent
};
pub use services::{
    EnergyTradingDomainService, PricingStrategy
};
