//! Shared Module
//!
//! This module contains shared infrastructure and domain components
//! that are used across multiple bounded contexts.

pub mod domain;
pub mod application;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{
    events::{DomainEvent, EventBus},
    value_objects::ValueObject,
    errors::DomainError,
    repository::{Repository, AggregateRoot},
};

pub use application::{
    command_bus::{Command, CommandHandler, CommandBus},
    query_bus::{Query, QueryHandler, QueryBus},
    event_bus::{IntegrationEvent, IntegrationEventHandler, IntegrationEventBus},
};

pub use infrastructure::{
    storage::{StorageProvider, InMemoryStorage, FileSystemStorage},
    network::{NetworkProvider, P2PNetworkAdapter, NetworkMessage},
    logging::{Logger, StructuredLogger, LogLevel},
};
