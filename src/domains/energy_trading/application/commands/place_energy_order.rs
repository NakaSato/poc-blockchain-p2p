//! Place Energy Order Command
//!
//! Command for placing energy buy/sell orders in the trading system.

use crate::shared::{
    domain::errors::DomainError,
    application::{Command, CommandHandler},
};
use crate::domains::energy_trading::domain::{
    entities::EnergyOrder,
    services::EnergyTradingDomainService,
    value_objects::{TraderId, EnergyAmount, PricePerKwh, TradeType, TradingWindow},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceEnergyOrderCommand {
    pub trader_id: String,
    pub order_type: String, // "Buy" or "Sell"
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub trading_window_start: DateTime<Utc>,
    pub trading_window_end: DateTime<Utc>,
    pub market_name: String,
}

impl Command for PlaceEnergyOrderCommand {
    type Result = PlaceEnergyOrderResult;
    
    fn command_type(&self) -> &'static str {
        "PlaceEnergyOrder"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceEnergyOrderResult {
    pub order_id: String,
    pub trader_id: String,
    pub order_type: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub trades_executed: Vec<TradeExecutionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeExecutionResult {
    pub trade_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub total_price: f64,
    pub executed_at: DateTime<Utc>,
}

pub struct PlaceEnergyOrderHandler {
    energy_trading_service: Arc<EnergyTradingDomainService>,
}

impl PlaceEnergyOrderHandler {
    pub fn new(energy_trading_service: Arc<EnergyTradingDomainService>) -> Self {
        Self {
            energy_trading_service,
        }
    }
}

#[async_trait]
impl CommandHandler<PlaceEnergyOrderCommand> for PlaceEnergyOrderHandler {
    async fn handle(&self, command: PlaceEnergyOrderCommand) -> Result<PlaceEnergyOrderResult, DomainError> {
        // Convert command to domain value objects
        let trader_id = TraderId::new(command.trader_id)?;
        let order_type = TradeType::from_string(&command.order_type)?;
        let energy_amount = EnergyAmount::new(command.energy_amount)?;
        let price_per_kwh = PricePerKwh::new(command.price_per_kwh)?;
        let trading_window = TradingWindow::new(
            command.trading_window_start,
            command.trading_window_end,
        )?;
        
        // Execute domain logic
        let (order, trades) = self.energy_trading_service
            .place_order(
                trader_id,
                order_type.clone(),
                energy_amount,
                price_per_kwh,
                trading_window,
                command.market_name,
            )
            .await?;
        
        // Convert trades to result format
        let trades_executed: Vec<TradeExecutionResult> = trades
            .into_iter()
            .map(|trade| TradeExecutionResult {
                trade_id: trade.id().value().to_string(),
                buyer_id: trade.buyer_id().value().to_string(),
                seller_id: trade.seller_id().value().to_string(),
                energy_amount: trade.energy_amount().value(),
                price_per_kwh: trade.price_per_kwh().value(),
                total_price: trade.total_price(),
                executed_at: trade.executed_at(),
            })
            .collect();
        
        // Return result
        Ok(PlaceEnergyOrderResult {
            order_id: order.id().value().to_string(),
            trader_id: order.trader_id().value().to_string(),
            order_type: order.order_type().to_string(),
            energy_amount: order.energy_amount().value(),
            price_per_kwh: order.price_per_kwh().value(),
            status: order.status().to_string(),
            created_at: order.created_at(),
            trades_executed,
        })
    }
}
