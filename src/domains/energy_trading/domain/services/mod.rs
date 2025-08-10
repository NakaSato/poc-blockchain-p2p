//! Energy Trading Domain Services
//!
//! This module contains domain services for energy trading business logic.

pub mod energy_trading_service;
pub mod dynamic_pricing_service;
pub mod auction_scheduler_service;

pub use energy_trading_service::{EnergyTradingDomainService, PricingStrategy};
pub use dynamic_pricing_service::{DynamicPricingService, PricingConfig, PriceSignal};
pub use auction_scheduler_service::{AuctionSchedulerService, AuctionConfig, AuctionResult, ScheduledAuction};
