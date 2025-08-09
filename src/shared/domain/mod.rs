//! Shared Domain Infrastructure
//!
//! This module contains shared domain concepts, events, and value objects
//! that are used across multiple bounded contexts in the GridTokenX system.

pub mod events;
pub mod value_objects;
pub mod errors;
pub mod repository;

// Re-export commonly used types
pub use events::{DomainEvent, EventStore, EventPublisher};
pub use value_objects::{ValueObject, Hash, Signature, Timestamp, Amount, Address};
pub use errors::{DomainError, DomainResult};
pub use repository::{Repository, AggregateRoot};
