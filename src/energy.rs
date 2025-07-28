//! GridTokenX Energy Trading Module
//!
//! This module implements the energy trading system for the GridTokenX blockchain,
//! including order matching, grid management, and energy market operations.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blockchain::{Blockchain, Transaction};
use crate::config::GridConfig;

/// Energy trading system manager
#[derive(Debug)]
pub struct EnergyTrading {
    blockchain: Arc<RwLock<Blockchain>>,
    order_book: RwLock<EnergyOrderBook>,
    trading_engine: RwLock<TradingEngine>,
}

/// Grid manager for monitoring and control
#[derive(Debug)]
pub struct GridManager {
    config: GridConfig,
    grid_status: RwLock<GridStatus>,
    monitoring_active: RwLock<bool>,
}

/// Energy order book
#[derive(Debug, Default)]
pub struct EnergyOrderBook {
    buy_orders: Vec<EnergyOrder>,
    sell_orders: Vec<EnergyOrder>,
    matched_trades: Vec<MatchedTrade>,
}

/// Trading engine for order matching
#[derive(Debug, Default)]
pub struct TradingEngine {
    matching_algorithm: MatchingAlgorithm,
    price_discovery: PriceDiscovery,
}

/// Individual energy order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyOrder {
    pub id: String,
    pub trader_address: String,
    pub order_type: OrderType,
    pub energy_amount: f64,
    pub price_per_kwh: u64,
    pub energy_source: Option<String>,
    pub grid_location: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: OrderStatus,
}

/// Order types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Active,
    PartiallyFilled,
    Filled,
    Cancelled,
    Expired,
}

/// Matched energy trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedTrade {
    pub id: String,
    pub buy_order_id: String,
    pub sell_order_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: u64,
    pub total_value: u64,
    pub matched_at: DateTime<Utc>,
    pub buyer_address: String,
    pub seller_address: String,
}

/// Grid status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStatus {
    pub frequency: f64,
    pub voltage_stability: f64,
    pub load_balance: f64,
    pub congestion_level: f64,
    pub renewable_percentage: f64,
    pub total_generation: f64,
    pub total_consumption: f64,
    pub last_updated: DateTime<Utc>,
}

/// Energy trading metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnergyMetrics {
    pub total_energy_traded: f64,
    pub active_orders: u64,
    pub completed_trades: u64,
    pub average_price: f64,
    pub price_volatility: f64,
}

/// Order matching algorithm
#[derive(Debug, Clone)]
pub enum MatchingAlgorithm {
    PriceTimePriority,
    ProRata,
    LocationPreference,
}

/// Price discovery mechanism
#[derive(Debug, Clone)]
pub struct PriceDiscovery {
    last_trade_price: u64,
    weighted_average_price: u64,
    bid_ask_spread: u64,
}

impl Default for MatchingAlgorithm {
    fn default() -> Self {
        Self::PriceTimePriority
    }
}

impl Default for PriceDiscovery {
    fn default() -> Self {
        Self {
            last_trade_price: 0,
            weighted_average_price: 0,
            bid_ask_spread: 0,
        }
    }
}

impl EnergyTrading {
    /// Create new energy trading system
    pub async fn new(blockchain: Arc<RwLock<Blockchain>>) -> Result<Self> {
        Ok(Self {
            blockchain,
            order_book: RwLock::new(EnergyOrderBook::default()),
            trading_engine: RwLock::new(TradingEngine::default()),
        })
    }

    /// Start the trading engine
    pub async fn start_trading_engine(&self) -> Result<()> {
        tracing::info!("Starting energy trading engine");

        // Start order matching loop
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            self.process_order_matching().await?;
        }
    }

    /// Submit a new energy order
    pub async fn submit_order(&self, order: EnergyOrder) -> Result<String> {
        let mut order_book = self.order_book.write().await;

        match order.order_type {
            OrderType::Buy => order_book.buy_orders.push(order.clone()),
            OrderType::Sell => order_book.sell_orders.push(order.clone()),
        }

        tracing::info!("Energy order submitted: {}", order.id);
        Ok(order.id)
    }

    /// Cancel an energy order
    pub async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let mut order_book = self.order_book.write().await;

        // Remove from buy orders
        order_book.buy_orders.retain(|order| order.id != order_id);

        // Remove from sell orders
        order_book.sell_orders.retain(|order| order.id != order_id);

        tracing::info!("Energy order cancelled: {}", order_id);
        Ok(())
    }

    /// Process order matching
    async fn process_order_matching(&self) -> Result<()> {
        let mut order_book = self.order_book.write().await;
        let mut trading_engine = self.trading_engine.write().await;

        // Simple price-time priority matching
        let mut matches = Vec::new();

        for buy_order in &order_book.buy_orders {
            for sell_order in &order_book.sell_orders {
                if buy_order.price_per_kwh >= sell_order.price_per_kwh
                    && buy_order.grid_location == sell_order.grid_location
                {
                    let trade_amount = buy_order.energy_amount.min(sell_order.energy_amount);
                    let trade_price = sell_order.price_per_kwh; // Seller's price

                    let matched_trade = MatchedTrade {
                        id: uuid::Uuid::new_v4().to_string(),
                        buy_order_id: buy_order.id.clone(),
                        sell_order_id: sell_order.id.clone(),
                        energy_amount: trade_amount,
                        price_per_kwh: trade_price,
                        total_value: (trade_amount * trade_price as f64) as u64,
                        matched_at: Utc::now(),
                        buyer_address: buy_order.trader_address.clone(),
                        seller_address: sell_order.trader_address.clone(),
                    };

                    matches.push(matched_trade);
                    break;
                }
            }
        }

        // Process matches
        for matched_trade in matches {
            order_book.matched_trades.push(matched_trade.clone());

            // Remove or update matched orders
            order_book
                .buy_orders
                .retain(|order| order.id != matched_trade.buy_order_id);
            order_book
                .sell_orders
                .retain(|order| order.id != matched_trade.sell_order_id);

            tracing::info!(
                "Energy trade matched: {} kWh at {} tokens/kWh",
                matched_trade.energy_amount,
                matched_trade.price_per_kwh
            );
        }

        Ok(())
    }

    /// Get energy trading metrics
    pub async fn get_metrics(&self) -> Result<EnergyMetrics> {
        let order_book = self.order_book.read().await;

        let total_energy_traded = order_book
            .matched_trades
            .iter()
            .map(|trade| trade.energy_amount)
            .sum();

        let active_orders = (order_book.buy_orders.len() + order_book.sell_orders.len()) as u64;
        let completed_trades = order_book.matched_trades.len() as u64;

        let average_price = if completed_trades > 0 {
            order_book
                .matched_trades
                .iter()
                .map(|trade| trade.price_per_kwh as f64)
                .sum::<f64>()
                / completed_trades as f64
        } else {
            0.0
        };

        Ok(EnergyMetrics {
            total_energy_traded,
            active_orders,
            completed_trades,
            average_price,
            price_volatility: 0.0, // Would calculate from price history
        })
    }
}

impl GridManager {
    /// Create new grid manager
    pub async fn new(config: GridConfig) -> Result<Self> {
        Ok(Self {
            config,
            grid_status: RwLock::new(GridStatus::default()),
            monitoring_active: RwLock::new(false),
        })
    }

    /// Start grid monitoring
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut monitoring = self.monitoring_active.write().await;
        *monitoring = true;
        drop(monitoring);

        tracing::info!("Starting grid monitoring");

        loop {
            let monitoring = self.monitoring_active.read().await;
            if !*monitoring {
                break;
            }
            drop(monitoring);

            self.update_grid_status().await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }

        Ok(())
    }

    /// Stop grid monitoring
    pub async fn stop_monitoring(&self) {
        let mut monitoring = self.monitoring_active.write().await;
        *monitoring = false;
        tracing::info!("Grid monitoring stopped");
    }

    /// Update grid status
    async fn update_grid_status(&self) -> Result<()> {
        let mut status = self.grid_status.write().await;

        // Simulate grid measurements (in real implementation, would read from SCADA/smart meters)
        status.frequency = 50.0 + (rand::random::<f64>() - 0.5) * 0.2; // 49.9-50.1 Hz
        status.voltage_stability = 95.0 + rand::random::<f64>() * 5.0; // 95-100%
        status.load_balance = 85.0 + rand::random::<f64>() * 15.0; // 85-100%
        status.congestion_level = rand::random::<f64>() * 20.0; // 0-20%
        status.renewable_percentage = 30.0 + rand::random::<f64>() * 40.0; // 30-70%
        status.total_generation = 1000.0 + rand::random::<f64>() * 500.0; // MW
        status.total_consumption = status.total_generation * (0.95 + rand::random::<f64>() * 0.1);
        status.last_updated = Utc::now();

        Ok(())
    }

    /// Get current grid status
    pub async fn get_grid_status(&self) -> GridStatus {
        self.grid_status.read().await.clone()
    }

    /// Check if grid is stable
    pub async fn is_grid_stable(&self) -> bool {
        let status = self.grid_status.read().await;

        status.frequency >= 49.8
            && status.frequency <= 50.2
            && status.voltage_stability >= 90.0
            && status.load_balance >= 80.0
            && status.congestion_level <= 25.0
    }
}

impl Default for GridStatus {
    fn default() -> Self {
        Self {
            frequency: 50.0,
            voltage_stability: 95.0,
            load_balance: 90.0,
            congestion_level: 5.0,
            renewable_percentage: 35.0,
            total_generation: 1200.0,
            total_consumption: 1150.0,
            last_updated: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::StorageManager;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_energy_trading_creation() {
        let storage = Arc::new(StorageManager::new_memory().await.unwrap());
        let blockchain = Arc::new(RwLock::new(
            crate::blockchain::Blockchain::new(storage).await.unwrap(),
        ));

        let energy_trading = EnergyTrading::new(blockchain).await.unwrap();
        let metrics = energy_trading.get_metrics().await.unwrap();

        assert_eq!(metrics.total_energy_traded, 0.0);
        assert_eq!(metrics.active_orders, 0);
    }

    #[tokio::test]
    async fn test_grid_manager_creation() {
        let config = GridConfig::default();
        let grid_manager = GridManager::new(config).await.unwrap();

        let status = grid_manager.get_grid_status().await;
        assert_eq!(status.frequency, 50.0);
        assert!(grid_manager.is_grid_stable().await);
    }

    #[test]
    fn test_energy_order_serialization() {
        let order = EnergyOrder {
            id: "test-order".to_string(),
            trader_address: "trader123".to_string(),
            order_type: OrderType::Buy,
            energy_amount: 100.0,
            price_per_kwh: 5000,
            energy_source: Some("solar".to_string()),
            grid_location: "BKK-01-SUB001".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            status: OrderStatus::Active,
        };

        let serialized = serde_json::to_string(&order).unwrap();
        let deserialized: EnergyOrder = serde_json::from_str(&serialized).unwrap();

        assert_eq!(order.id, deserialized.id);
        assert_eq!(order.energy_amount, deserialized.energy_amount);
    }
}
