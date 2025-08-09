//! Repository and Aggregate Root Patterns
//!
//! This module provides base traits for implementing the Repository pattern
//! and Aggregate Root pattern in domain-driven design.

use async_trait::async_trait;
use crate::shared::domain::{DomainResult, DomainEvent};
use crate::shared::domain::events::EventEnvelope;
use std::collections::VecDeque;

/// Base trait for aggregate roots
pub trait AggregateRoot {
    /// Type of the aggregate's identifier
    type Id: Clone + PartialEq + std::fmt::Debug + Send + Sync;
    
    /// Get the aggregate's unique identifier
    fn id(&self) -> &Self::Id;
    
    /// Get the current version of the aggregate
    fn version(&self) -> u64;
    
    /// Get uncommitted events
    fn uncommitted_events(&self) -> &VecDeque<Box<dyn DomainEvent>>;
    
    /// Mark events as committed
    fn mark_events_as_committed(&mut self);
    
    /// Apply an event to the aggregate
    fn apply_event(&mut self, event: Box<dyn DomainEvent>);
    
    /// Raise a new domain event
    fn raise_event(&mut self, event: Box<dyn DomainEvent>);
}

/// Generic repository trait for aggregate persistence
#[async_trait]
pub trait Repository<T: AggregateRoot>: Send + Sync {
    /// Find an aggregate by its ID
    async fn find_by_id(&self, id: &T::Id) -> DomainResult<Option<T>>;
    
    /// Get an aggregate by its ID (fails if not found)
    async fn get_by_id(&self, id: &T::Id) -> DomainResult<T> {
        self.find_by_id(id)
            .await?
            .ok_or_else(|| crate::shared::domain::DomainError::aggregate_not_found(
                format!("{:?}", id)
            ))
    }
    
    /// Save an aggregate
    async fn save(&self, aggregate: &mut T) -> DomainResult<()>;
    
    /// Delete an aggregate
    async fn delete(&self, id: &T::Id) -> DomainResult<()>;
    
    /// Check if an aggregate exists
    async fn exists(&self, id: &T::Id) -> DomainResult<bool> {
        Ok(self.find_by_id(id).await?.is_some())
    }
}

/// Base aggregate root implementation
pub struct BaseAggregateRoot<Id> {
    id: Id,
    version: u64,
    uncommitted_events: VecDeque<Box<dyn DomainEvent>>,
}

impl<Id: Clone + PartialEq + std::fmt::Debug + Send + Sync> BaseAggregateRoot<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            version: 0,
            uncommitted_events: VecDeque::new(),
        }
    }
    
    pub fn from_events(id: Id, events: Vec<EventEnvelope>) -> DomainResult<Self> {
        let mut aggregate = Self::new(id);
        
        for envelope in events {
            // Deserialize and apply each event
            // This would need to be implemented by specific aggregates
            aggregate.version = envelope.aggregate_version;
        }
        
        Ok(aggregate)
    }
}

impl<Id: Clone + PartialEq + std::fmt::Debug + Send + Sync> AggregateRoot for BaseAggregateRoot<Id> {
    type Id = Id;
    
    fn id(&self) -> &Self::Id {
        &self.id
    }
    
    fn version(&self) -> u64 {
        self.version
    }
    
    fn uncommitted_events(&self) -> &VecDeque<Box<dyn DomainEvent>> {
        &self.uncommitted_events
    }
    
    fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
    
    fn apply_event(&mut self, event: Box<dyn DomainEvent>) {
        // Apply the event to change aggregate state
        // This should be overridden by specific aggregates
        self.version += 1;
    }
    
    fn raise_event(&mut self, event: Box<dyn DomainEvent>) {
        // Just add to uncommitted events - applying will be handled elsewhere
        self.uncommitted_events.push_back(event);
    }
}

/// Unit of Work pattern for managing aggregate transactions
pub struct UnitOfWork {
    aggregates: Vec<Box<dyn AggregateRoot<Id = String>>>,
}

impl UnitOfWork {
    pub fn new() -> Self {
        Self {
            aggregates: Vec::new(),
        }
    }
    
    pub fn register<T: AggregateRoot<Id = String> + 'static>(&mut self, aggregate: T) {
        self.aggregates.push(Box::new(aggregate));
    }
    
    pub async fn commit(&mut self) -> DomainResult<()> {
        // Collect all uncommitted events
        let mut all_events = Vec::new();
        
        for aggregate in &self.aggregates {
            for event in aggregate.uncommitted_events() {
                all_events.push(event.clone());
            }
        }
        
        // Publish events in a transaction
        // This would integrate with the event store and publisher
        
        // Mark all events as committed
        for aggregate in &mut self.aggregates {
            aggregate.mark_events_as_committed();
        }
        
        Ok(())
    }
}

impl Default for UnitOfWork {
    fn default() -> Self {
        Self::new()
    }
}
