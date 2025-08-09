//! Energy Trading Application Queries
//!
//! This module contains all queries for energy trading read operations.

pub mod get_market_depth;

pub use get_market_depth::{
    GetMarketDepthQuery,
    GetMarketDepthResult,
    GetMarketDepthHandler,
};
