//! Energy Trading Domain Aggregates
//!
//! This module contains aggregates for the energy trading domain.

pub mod order_book;

pub use order_book::{OrderBook, MarketDepth, PriceLevel, TradesExecutedEvent};
