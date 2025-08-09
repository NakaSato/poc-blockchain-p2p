//! Shared Application Infrastructure
//!
//! This module contains shared application layer components including
//! command and query buses, and application service patterns.

pub mod command_bus;
pub mod query_bus;
pub mod event_bus;

// Re-export commonly used types
pub use command_bus::{Command, CommandHandler, CommandBus};
pub use query_bus::{Query, QueryHandler, QueryBus};
pub use event_bus::{IntegrationEvent, IntegrationEventHandler, IntegrationEventBus};
