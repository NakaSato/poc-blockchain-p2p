//! Integration Event Bus Implementation
//!
//! Handles integration events between bounded contexts.

use crate::shared::domain::{errors::DomainError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

/// Trait for integration events that cross bounded context boundaries
pub trait IntegrationEvent: Send + Sync + 'static + Clone {
    fn event_type(&self) -> &'static str;
    fn event_id(&self) -> String;
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc>;
}

/// Trait for integration event handlers
#[async_trait]
pub trait IntegrationEventHandler<E: IntegrationEvent>: Send + Sync {
    async fn handle(&self, event: E) -> Result<(), DomainError>;
}

/// Integration event bus for cross-context communication
pub struct IntegrationEventBus {
    handlers: Arc<RwLock<HashMap<&'static str, Vec<Box<dyn IntegrationEventDispatcher>>>>>,
}

trait IntegrationEventDispatcher: Send + Sync {
    fn dispatch(&self, event: Box<dyn std::any::Any + Send>) 
        -> Pin<Box<dyn Future<Output = Result<(), DomainError>> + Send + '_>>;
}

struct TypedIntegrationEventDispatcher<E: IntegrationEvent, H: IntegrationEventHandler<E>> {
    handler: H,
    _phantom: std::marker::PhantomData<E>,
}

#[async_trait]
impl<E: IntegrationEvent, H: IntegrationEventHandler<E>> IntegrationEventDispatcher for TypedIntegrationEventDispatcher<E, H> {
    fn dispatch(&self, event: Box<dyn std::any::Any + Send>) 
        -> Pin<Box<dyn Future<Output = Result<(), DomainError>> + Send + '_>> {
        Box::pin(async move {
            let event = *event.downcast::<E>().map_err(|_| {
                DomainError::invalid_operation("Failed to downcast integration event")
            })?;
            
            self.handler.handle(event).await
        })
    }
}

impl IntegrationEventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe<E: IntegrationEvent, H: IntegrationEventHandler<E> + 'static>(&self, handler: H) {
        let dispatcher = TypedIntegrationEventDispatcher {
            handler,
            _phantom: std::marker::PhantomData,
        };
        
        let event_type = std::any::type_name::<E>();
        let mut handlers = self.handlers.write().await;
        handlers.entry(event_type)
            .or_insert_with(Vec::new)
            .push(Box::new(dispatcher));
    }

    pub async fn publish<E: IntegrationEvent>(&self, event: E) -> Result<(), DomainError> {
        let event_type = event.event_type();
        let handlers = self.handlers.read().await;
        
        if let Some(event_handlers) = handlers.get(event_type) {
            for handler in event_handlers {
                // Clone the event for each handler
                let event_clone = Box::new(event.clone());
                if let Err(e) = handler.dispatch(event_clone).await {
                    // Log error but continue with other handlers
                    eprintln!("Integration event handler error: {}", e);
                }
            }
        }
        
        Ok(())
    }

    /// Publish multiple events as a batch
    pub async fn publish_batch<E: IntegrationEvent>(&self, events: Vec<E>) -> Result<(), DomainError> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }
}

impl Default for IntegrationEventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Common integration event implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradeCreatedIntegrationEvent {
    pub event_id: String,
    pub trade_id: String,
    pub trader_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub trade_type: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl IntegrationEvent for EnergyTradeCreatedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "EnergyTradeCreated"
    }
    
    fn event_id(&self) -> String {
        self.event_id.clone()
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceVoteProcessedIntegrationEvent {
    pub event_id: String,
    pub proposal_id: String,
    pub voter_id: String,
    pub vote: bool,
    pub voting_power: u64,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl IntegrationEvent for GovernanceVoteProcessedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "GovernanceVoteProcessed"
    }
    
    fn event_id(&self) -> String {
        self.event_id.clone()
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockAddedIntegrationEvent {
    pub event_id: String,
    pub block_hash: String,
    pub block_height: u64,
    pub transaction_count: usize,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl IntegrationEvent for BlockAddedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "BlockAdded"
    }
    
    fn event_id(&self) -> String {
        self.event_id.clone()
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}
