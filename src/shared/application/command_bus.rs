//! Command Bus Pattern Implementation
//!
//! Provides infrastructure for handling commands in a CQRS architecture.

use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

/// Trait for commands in CQRS pattern
pub trait Command: Send + Sync + 'static {
    type Result: Send + Sync;
    fn command_type(&self) -> &'static str;
}

/// Trait for command handlers
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(&self, command: C) -> Result<C::Result, DomainError>;
}

/// Command bus for dispatching commands to their handlers
pub struct CommandBus {
    handlers: Arc<RwLock<HashMap<&'static str, Box<dyn CommandDispatcher>>>>,
}

trait CommandDispatcher: Send + Sync {
    fn dispatch(&self, command: Box<dyn std::any::Any + Send>) 
        -> Pin<Box<dyn Future<Output = Result<Box<dyn std::any::Any + Send>, DomainError>> + Send + '_>>;
}

struct TypedCommandDispatcher<C: Command, H: CommandHandler<C>> {
    handler: H,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: Command, H: CommandHandler<C>> CommandDispatcher for TypedCommandDispatcher<C, H> {
    fn dispatch(&self, command: Box<dyn std::any::Any + Send>) 
        -> Pin<Box<dyn Future<Output = Result<Box<dyn std::any::Any + Send>, DomainError>> + Send + '_>> {
        Box::pin(async move {
            let command = *command.downcast::<C>().map_err(|_| {
                DomainError::invalid_operation("Failed to downcast command")
            })?;
            
            let result = self.handler.handle(command).await?;
            Ok(Box::new(result) as Box<dyn std::any::Any + Send>)
        })
    }
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_handler<C: Command, H: CommandHandler<C> + 'static>(&self, handler: H) {
        let dispatcher = TypedCommandDispatcher {
            handler,
            _phantom: std::marker::PhantomData,
        };
        
        let mut handlers = self.handlers.write().await;
        // Use a sample command to get the type name
        let type_name = std::any::type_name::<C>();
        let type_name_str = type_name.to_string().into_boxed_str();
        let type_name_static = Box::leak(type_name_str);
        handlers.insert(type_name_static, Box::new(dispatcher));
    }

    pub async fn send<C: Command>(&self, command: C) -> Result<C::Result, DomainError> {
        let command_type = command.command_type();
        let handlers = self.handlers.read().await;
        
        let dispatcher = handlers.get(command_type)
            .ok_or_else(|| DomainError::InvalidOperation { 
                message: format!("No handler registered for command type: {}", command_type)
            })?;
        
        let result = dispatcher.dispatch(Box::new(command)).await?;
        let result = *result.downcast::<C::Result>().map_err(|_| {
            DomainError::InvalidOperation { 
                message: "Failed to downcast command result".to_string() 
            }
        })?;
        
        Ok(result)
    }
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Example command implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEnergyTradeCommand {
    pub trader_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub trade_type: String,
}

impl Command for CreateEnergyTradeCommand {
    type Result = String; // Trade ID
    
    fn command_type(&self) -> &'static str {
        "CreateEnergyTrade"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessGovernanceVoteCommand {
    pub proposal_id: String,
    pub voter_id: String,
    pub vote: bool,
    pub voting_power: u64,
}

impl Command for ProcessGovernanceVoteCommand {
    type Result = bool; // Success
    
    fn command_type(&self) -> &'static str {
        "ProcessGovernanceVote"
    }
}
