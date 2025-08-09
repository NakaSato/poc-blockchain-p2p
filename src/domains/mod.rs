//! Bounded Contexts
//!
//! This module contains all the bounded contexts (domains) in the GridTokenX system.
//! Each bounded context represents a specific business domain with its own
//! ubiquitous language and clear boundaries.

pub mod energy_trading;

// Re-export main domain types for convenience
pub use energy_trading::{
    // Core trading types
    EnergyOrder, EnergyTrade, OrderBook, EnergyTradingDomainService,
    
    // Value objects
    TradeId, TraderId, EnergyAmount, PricePerKwh, TradeType, TradeStatus, TradingWindow,
    
    // Application layer
    PlaceEnergyOrderCommand, PlaceEnergyOrderHandler,
    CancelEnergyOrderCommand, CancelEnergyOrderHandler,
    GetMarketDepthQuery, GetMarketDepthHandler,
};

// Future domains to be implemented:
// pub mod grid_management;
// pub mod governance;
// pub mod blockchain_infrastructure;
// pub mod account_management;
// pub mod network;
