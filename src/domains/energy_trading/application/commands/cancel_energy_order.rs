//! Cancel Energy Order Command
//!
//! Command for cancelling energy orders.

use crate::shared::{
    domain::errors::DomainError,
    application::{Command, CommandHandler},
};
use crate::domains::energy_trading::domain::{
    services::EnergyTradingDomainService,
    value_objects::TradeId,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelEnergyOrderCommand {
    pub order_id: String,
    pub market_name: String,
    pub requester_id: String, // For authorization
}

impl Command for CancelEnergyOrderCommand {
    type Result = CancelEnergyOrderResult;
    
    fn command_type(&self) -> &'static str {
        "CancelEnergyOrder"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelEnergyOrderResult {
    pub order_id: String,
    pub market_name: String,
    pub cancelled_at: DateTime<Utc>,
    pub success: bool,
    pub message: String,
}

pub struct CancelEnergyOrderHandler {
    energy_trading_service: Arc<EnergyTradingDomainService>,
}

impl CancelEnergyOrderHandler {
    pub fn new(energy_trading_service: Arc<EnergyTradingDomainService>) -> Self {
        Self {
            energy_trading_service,
        }
    }
}

#[async_trait]
impl CommandHandler<CancelEnergyOrderCommand> for CancelEnergyOrderHandler {
    async fn handle(&self, command: CancelEnergyOrderCommand) -> Result<CancelEnergyOrderResult, DomainError> {
        // Convert command to domain value objects
        let order_id = TradeId::new(command.order_id.clone())?;
        
        // Execute domain logic
        let result = self.energy_trading_service
            .cancel_order(&order_id, &command.market_name)
            .await;
        
        let cancelled_at = Utc::now();
        
        match result {
            Ok(_) => Ok(CancelEnergyOrderResult {
                order_id: command.order_id,
                market_name: command.market_name,
                cancelled_at,
                success: true,
                message: "Order cancelled successfully".to_string(),
            }),
            Err(err) => Ok(CancelEnergyOrderResult {
                order_id: command.order_id,
                market_name: command.market_name,
                cancelled_at,
                success: false,
                message: format!("Failed to cancel order: {}", err),
            }),
        }
    }
}
