//! Domain Modules
//!
//! This module contains all bounded contexts in the GridTokenX system.
//! Each bounded context represents a cohesive business domain with its own
//! entities, value objects, aggregates, and services.

pub mod energy_trading;
pub mod governance;

// Re-export commonly used types from each domain
pub use energy_trading::{
    EnergyOrder, OrderBook, EnergyTrade,
    PlaceEnergyOrderCommand, PlaceEnergyOrderHandler,
    EnergyTradingDomainService,
};

pub use governance::{
    GovernanceToken, StakePosition, RECToken,
    StakingService, IncentiveMechanismService, RECMarketplaceService,
    TokenAmount, VotingPower,
};

// Future domains to be implemented:
// pub mod grid_management;
// pub mod governance;
// pub mod blockchain_infrastructure;
// pub mod account_management;
// pub mod network;
