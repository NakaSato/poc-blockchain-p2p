//! Domain Events Infrastructure
//!
//! This module provides the foundation for domain events in the GridTokenX system,
//! enabling loose coupling between bounded contexts and supporting event-driven architecture.

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Base trait for all domain events
pub trait DomainEvent: Send + Sync + 'static {
    /// Unique identifier for the event type
    fn event_type(&self) -> &'static str;
    
    /// ID of the aggregate that generated this event
    fn aggregate_id(&self) -> String;
    
    /// When the event occurred
    fn occurred_at(&self) -> DateTime<Utc>;
    
    /// Event payload as JSON
    fn event_data(&self) -> serde_json::Value;
    
    /// Version of the aggregate when event was generated
    fn aggregate_version(&self) -> u64;
    
    /// Unique identifier for this event instance
    fn event_id(&self) -> Uuid;
}

/// Event envelope containing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: String,
    pub aggregate_version: u64,
    pub occurred_at: DateTime<Utc>,
    pub event_data: serde_json::Value,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

impl EventEnvelope {
    pub fn new<E: DomainEvent>(event: &E) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event.event_type().to_string(),
            aggregate_id: event.aggregate_id(),
            aggregate_version: event.aggregate_version(),
            occurred_at: event.occurred_at(),
            event_data: event.event_data(),
            correlation_id: None,
            causation_id: None,
        }
    }
    
    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    pub fn with_causation(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
}

/// Event store for persisting and retrieving domain events
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Save events for an aggregate
    async fn save_events(
        &self,
        aggregate_id: &str,
        events: Vec<EventEnvelope>,
        expected_version: u64,
    ) -> Result<()>;
    
    /// Load events for an aggregate
    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<EventEnvelope>>;
    
    /// Load events from a specific version
    async fn load_events_from_version(
        &self,
        aggregate_id: &str,
        from_version: u64,
    ) -> Result<Vec<EventEnvelope>>;
    
    /// Get all events of a specific type
    async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<EventEnvelope>>;
    
    /// Get events in chronological order
    async fn get_events_since(&self, since: DateTime<Utc>) -> Result<Vec<EventEnvelope>>;
}

/// Event publisher for distributing events to handlers
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish a single event
    async fn publish(&self, event: EventEnvelope) -> Result<()>;
    
    /// Publish multiple events
    async fn publish_batch(&self, events: Vec<EventEnvelope>) -> Result<()>;
}

/// Event handler for processing domain events
#[async_trait]
pub trait EventHandler<E: DomainEvent>: Send + Sync {
    /// Handle a domain event
    async fn handle(&self, event: &E) -> Result<()>;
    
    /// Event types this handler is interested in
    fn handles(&self) -> Vec<&'static str>;
}

/// Event bus for managing event subscriptions and dispatch
pub struct EventBus {
    handlers: RwLock<HashMap<String, Vec<Arc<dyn EventHandlerErased>>>>,
    publisher: Arc<dyn EventPublisher>,
}

#[async_trait]
trait EventHandlerErased: Send + Sync {
    async fn handle_erased(&self, event: &EventEnvelope) -> Result<()>;
}

impl EventBus {
    pub fn new(publisher: Arc<dyn EventPublisher>) -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            publisher,
        }
    }
    
    /// Subscribe a handler to specific event types
    pub async fn subscribe<H, E>(&self, handler: Arc<H>)
    where
        H: EventHandler<E> + 'static,
        E: DomainEvent + for<'de> Deserialize<'de> + 'static,
    {
        let mut handlers = self.handlers.write().await;
        
        for event_type in handler.handles() {
            let erased_handler = Arc::new(TypedEventHandler::<H, E>::new(handler.clone()));
            handlers
                .entry(event_type.to_string())
                .or_insert_with(Vec::new)
                .push(erased_handler);
        }
    }
    
    /// Publish an event to all registered handlers
    pub async fn publish<E: DomainEvent>(&self, event: E) -> Result<()> {
        let envelope = EventEnvelope::new(&event);
        
        // Persist the event
        self.publisher.publish(envelope.clone()).await?;
        
        // Dispatch to handlers
        self.dispatch(envelope).await?;
        
        Ok(())
    }
    
    /// Dispatch an event envelope to handlers
    async fn dispatch(&self, envelope: EventEnvelope) -> Result<()> {
        let handlers = self.handlers.read().await;
        
        if let Some(event_handlers) = handlers.get(&envelope.event_type) {
            for handler in event_handlers {
                if let Err(e) = handler.handle_erased(&envelope).await {
                    tracing::error!(
                        "Failed to handle event {} for aggregate {}: {}",
                        envelope.event_type,
                        envelope.aggregate_id,
                        e
                    );
                    // Continue processing other handlers
                }
            }
        }
        
        Ok(())
    }
}

/// Wrapper to erase event handler types
struct TypedEventHandler<H, E> {
    handler: Arc<H>,
    _phantom: std::marker::PhantomData<E>,
}

impl<H, E> TypedEventHandler<H, E> {
    fn new(handler: Arc<H>) -> Self {
        Self {
            handler,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<H, E> EventHandlerErased for TypedEventHandler<H, E>
where
    H: EventHandler<E> + Send + Sync,
    E: DomainEvent + for<'de> Deserialize<'de> + Send + Sync,
{
    async fn handle_erased(&self, envelope: &EventEnvelope) -> Result<()> {
        // Deserialize the event data
        let event: E = serde_json::from_value(envelope.event_data.clone())?;
        
        // Handle the typed event
        self.handler.handle(&event).await
    }
}

/// In-memory event store implementation for testing
pub struct InMemoryEventStore {
    events: RwLock<HashMap<String, Vec<EventEnvelope>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn save_events(
        &self,
        aggregate_id: &str,
        events: Vec<EventEnvelope>,
        expected_version: u64,
    ) -> Result<()> {
        let mut store = self.events.write().await;
        let aggregate_events = store.entry(aggregate_id.to_string()).or_insert_with(Vec::new);
        
        // Check optimistic concurrency
        let current_version = aggregate_events.len() as u64;
        if current_version != expected_version {
            return Err(anyhow::anyhow!(
                "Concurrency conflict: expected version {}, but was {}",
                expected_version,
                current_version
            ));
        }
        
        // Append new events
        aggregate_events.extend(events);
        
        Ok(())
    }
    
    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<EventEnvelope>> {
        let events = self.events.read().await;
        Ok(events
            .get(aggregate_id)
            .map(|e| e.clone())
            .unwrap_or_default())
    }
    
    async fn load_events_from_version(
        &self,
        aggregate_id: &str,
        from_version: u64,
    ) -> Result<Vec<EventEnvelope>> {
        let events = self.events.read().await;
        Ok(events
            .get(aggregate_id)
            .map(|e| {
                e.iter()
                    .skip(from_version as usize)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }
    
    async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<EventEnvelope>> {
        let events = self.events.read().await;
        let mut matching_events = Vec::new();
        
        for aggregate_events in events.values() {
            for event in aggregate_events {
                if event.event_type == event_type {
                    matching_events.push(event.clone());
                }
            }
        }
        
        // Sort by occurred_at
        matching_events.sort_by_key(|e| e.occurred_at);
        
        Ok(matching_events)
    }
    
    async fn get_events_since(&self, since: DateTime<Utc>) -> Result<Vec<EventEnvelope>> {
        let events = self.events.read().await;
        let mut matching_events = Vec::new();
        
        for aggregate_events in events.values() {
            for event in aggregate_events {
                if event.occurred_at >= since {
                    matching_events.push(event.clone());
                }
            }
        }
        
        // Sort by occurred_at
        matching_events.sort_by_key(|e| e.occurred_at);
        
        Ok(matching_events)
    }
}

/// Simple in-memory event publisher for testing
pub struct InMemoryEventPublisher;

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: EventEnvelope) -> Result<()> {
        tracing::debug!("Publishing event: {} for aggregate: {}", 
                       event.event_type, event.aggregate_id);
        Ok(())
    }
    
    async fn publish_batch(&self, events: Vec<EventEnvelope>) -> Result<()> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }
}
