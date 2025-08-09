//! Query Bus Pattern Implementation
//!
//! Provides infrastructure for handling queries in a CQRS architecture.

use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

/// Trait for queries in CQRS pattern
pub trait Query: Send + Sync + 'static {
    type Result: Send + Sync;
    fn query_type(&self) -> &'static str;
}

/// Trait for query handlers
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    async fn handle(&self, query: Q) -> Result<Q::Result, DomainError>;
}

/// Query bus for dispatching queries to their handlers
pub struct QueryBus {
    handlers: Arc<RwLock<HashMap<&'static str, Box<dyn QueryDispatcher>>>>,
}

trait QueryDispatcher: Send + Sync {
    fn dispatch(&self, query: Box<dyn std::any::Any + Send>) 
        -> Pin<Box<dyn Future<Output = Result<Box<dyn std::any::Any + Send>, DomainError>> + Send + '_>>;
}

struct TypedQueryDispatcher<Q: Query, H: QueryHandler<Q>> {
    handler: H,
    _phantom: std::marker::PhantomData<Q>,
}

#[async_trait]
impl<Q: Query, H: QueryHandler<Q>> QueryDispatcher for TypedQueryDispatcher<Q, H> {
    fn dispatch(&self, query: Box<dyn std::any::Any + Send>) 
        -> Pin<Box<dyn Future<Output = Result<Box<dyn std::any::Any + Send>, DomainError>> + Send + '_>> {
        Box::pin(async move {
            let query = *query.downcast::<Q>().map_err(|_| {
                DomainError::invalid_operation("Failed to downcast query")
            })?;
            
            let result = self.handler.handle(query).await?;
            Ok(Box::new(result) as Box<dyn std::any::Any + Send>)
        })
    }
}

impl QueryBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_handler<Q: Query, H: QueryHandler<Q> + 'static>(&self, handler: H) {
        let dispatcher = TypedQueryDispatcher {
            handler,
            _phantom: std::marker::PhantomData,
        };
        
        let mut handlers = self.handlers.write().await;
        let type_name = std::any::type_name::<Q>();
        handlers.insert(Box::leak(type_name.to_string().into_boxed_str()), Box::new(dispatcher));
    }

    pub async fn ask<Q: Query>(&self, query: Q) -> Result<Q::Result, DomainError> {
        let query_type = query.query_type();
        let handlers = self.handlers.read().await;
        
        let dispatcher = handlers.get(query_type)
            .ok_or_else(|| DomainError::invalid_operation(format!("No handler registered for query type: {}", query_type)))?;
        
        let result = dispatcher.dispatch(Box::new(query)).await?;
        let result = *result.downcast::<Q::Result>().map_err(|_| {
            DomainError::invalid_operation("Failed to downcast query result")
        })?;
        
        Ok(result)
    }
}

impl Default for QueryBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Example query implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEnergyTradeQuery {
    pub trade_id: String,
}

impl Query for GetEnergyTradeQuery {
    type Result = Option<EnergyTradeView>;
    
    fn query_type(&self) -> &'static str {
        "GetEnergyTrade"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActiveTradesQuery {
    pub trader_id: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Query for GetActiveTradesQuery {
    type Result = Vec<EnergyTradeView>;
    
    fn query_type(&self) -> &'static str {
        "GetActiveTrades"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGovernanceProposalQuery {
    pub proposal_id: String,
}

impl Query for GetGovernanceProposalQuery {
    type Result = Option<GovernanceProposalView>;
    
    fn query_type(&self) -> &'static str {
        "GetGovernanceProposal"
    }
}

/// View models for query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradeView {
    pub id: String,
    pub trader_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub trade_type: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposalView {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer_id: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub voting_deadline: chrono::DateTime<chrono::Utc>,
}
