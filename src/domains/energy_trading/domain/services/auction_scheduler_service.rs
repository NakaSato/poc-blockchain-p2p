//! Auction Scheduler Service
//!
//! Implements Periodic Uniform-Price Double Auction (UPDA) scheduling
//! and market clearing functionality for the energy trading platform.

use crate::shared::domain::errors::DomainError;
use crate::domains::energy_trading::domain::{
    aggregates::{OrderBook, PriceLevel},
    entities::{EnergyOrder, EnergyTrade},
    value_objects::{TradeId, EnergyAmount, PricePerKwh},
    services::dynamic_pricing_service::{DynamicPricingService, PriceSignal},
};
use chrono::{DateTime, Utc, Duration, Timelike};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

/// Auction schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionConfig {
    /// Auction interval in minutes (e.g., 15, 30, 60)
    pub interval_minutes: i64,
    /// Market clearing start time (e.g., "09:00")
    pub market_open_time: String,
    /// Market clearing end time (e.g., "17:00") 
    pub market_close_time: String,
    /// Minimum order volume for auction participation
    pub min_order_volume: f64,
    /// Maximum price deviation from indicative price (%)
    pub max_price_deviation: f64,
}

impl Default for AuctionConfig {
    fn default() -> Self {
        Self {
            interval_minutes: 30,
            market_open_time: "06:00".to_string(),
            market_close_time: "22:00".to_string(),
            min_order_volume: 1.0,
            max_price_deviation: 0.50, // 50%
        }
    }
}

/// Auction round result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub market_name: String,
    pub auction_id: String,
    pub clearing_time: DateTime<Utc>,
    pub clearing_price: f64,
    pub total_volume: f64,
    pub matched_trades: Vec<EnergyTrade>,
    pub unmatched_buy_orders: Vec<TradeId>,
    pub unmatched_sell_orders: Vec<TradeId>,
    pub price_signal_before: Option<PriceSignal>,
    pub price_signal_after: Option<PriceSignal>,
}

/// Auction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuctionStatus {
    Scheduled,
    InProgress,
    Completed,
    Failed(String),
}

/// Scheduled auction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledAuction {
    pub auction_id: String,
    pub market_name: String,
    pub scheduled_time: DateTime<Utc>,
    pub status: AuctionStatus,
    pub created_at: DateTime<Utc>,
}

/// Auction scheduler service for UPDA implementation
pub struct AuctionSchedulerService {
    order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
    pricing_service: Arc<DynamicPricingService>,
    config: AuctionConfig,
    scheduled_auctions: RwLock<Vec<ScheduledAuction>>,
    auction_history: RwLock<Vec<AuctionResult>>,
}

impl AuctionSchedulerService {
    pub fn new(
        order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
        pricing_service: Arc<DynamicPricingService>,
        config: AuctionConfig,
    ) -> Self {
        Self {
            order_books,
            pricing_service,
            config,
            scheduled_auctions: RwLock::new(Vec::new()),
            auction_history: RwLock::new(Vec::new()),
        }
    }

    /// Schedule next auction for a market
    pub async fn schedule_next_auction(&self, market_name: &str) -> Result<ScheduledAuction, DomainError> {
        let now = Utc::now();
        let next_auction_time = self.calculate_next_auction_time(now);
        
        let auction = ScheduledAuction {
            auction_id: format!("auction_{}_{}", market_name, next_auction_time.timestamp()),
            market_name: market_name.to_string(),
            scheduled_time: next_auction_time,
            status: AuctionStatus::Scheduled,
            created_at: now,
        };

        {
            let mut auctions = self.scheduled_auctions.write().await;
            auctions.push(auction.clone());
        }

        info!(
            "Scheduled auction {} for market {} at {}",
            auction.auction_id, market_name, next_auction_time
        );

        Ok(auction)
    }

    /// Execute a periodic uniform-price double auction (UPDA)
    pub async fn clear_market(&self, market_name: &str) -> Result<AuctionResult, DomainError> {
        let auction_id = format!("auction_{}_{}", market_name, Utc::now().timestamp());
        
        info!("Starting market clearing for {} (auction: {})", market_name, auction_id);

        // Get price signal before auction
        let price_signal_before = self.pricing_service.get_current_price_signal(market_name).await;

        // Get all orders for the market
        let (buy_orders, sell_orders) = {
            let order_books = self.order_books.read().await;
            let order_book = order_books
                .get(market_name)
                .ok_or_else(|| DomainError::aggregate_not_found(format!("Market {} not found", market_name)))?;
            
            let market_depth = order_book.get_market_depth();
            (market_depth.buy_orders, market_depth.sell_orders)
        };

        // Sort orders for UPDA: buy orders by price descending, sell orders by price ascending
        let mut sorted_buy_orders = buy_orders;
        let mut sorted_sell_orders = sell_orders;
        
        sorted_buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal));
        sorted_sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));

        // Find clearing price and volume using UPDA algorithm
        let (clearing_price, clearing_volume, matched_trades) = 
            self.execute_upda_algorithm(&sorted_buy_orders, &sorted_sell_orders).await?;

        // Update price signal after auction
        self.pricing_service.update_price_signal(market_name).await.ok();
        let price_signal_after = self.pricing_service.get_current_price_signal(market_name).await;

        // Create auction result
        let auction_result = AuctionResult {
            market_name: market_name.to_string(),
            auction_id: auction_id.clone(),
            clearing_time: Utc::now(),
            clearing_price,
            total_volume: clearing_volume,
            matched_trades: matched_trades.clone(),
            unmatched_buy_orders: Vec::new(), // TODO: Track unmatched orders
            unmatched_sell_orders: Vec::new(),
            price_signal_before,
            price_signal_after,
        };

        // Store in history
        {
            let mut history = self.auction_history.write().await;
            history.push(auction_result.clone());
            
            // Keep last 1000 auctions
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        info!(
            "Market clearing completed for {}: price={:.4} THB/kWh, volume={:.2} kWh, trades={}",
            market_name, clearing_price, clearing_volume, matched_trades.len()
        );

        Ok(auction_result)
    }

    /// Execute the UPDA (Uniform Price Double Auction) algorithm
    async fn execute_upda_algorithm(
        &self,
        buy_orders: &[PriceLevel], 
        sell_orders: &[PriceLevel], 
    ) -> Result<(f64, f64, Vec<EnergyTrade>), DomainError> {
        let mut cumulative_demand = 0.0;
        let mut cumulative_supply = 0.0;
        let mut clearing_price = 0.0;
        let mut clearing_volume = 0.0;

        // Build demand and supply curves
        let mut demand_curve = Vec::new();
        for price_level in buy_orders {
            cumulative_demand += price_level.volume;
            demand_curve.push((price_level.price, cumulative_demand));
        }

        let mut supply_curve = Vec::new();
        for price_level in sell_orders {
            cumulative_supply += price_level.volume;
            supply_curve.push((price_level.price, cumulative_supply));
        }

        // Find intersection point (clearing price and volume)
        for (buy_price, buy_volume) in &demand_curve {
            for (sell_price, sell_volume) in &supply_curve {
                if buy_price >= sell_price && *buy_volume >= clearing_volume && *sell_volume >= clearing_volume {
                    clearing_price = (*buy_price + *sell_price) / 2.0; // Uniform price
                    clearing_volume = buy_volume.min(*sell_volume);
                    break;
                }
            }
        }

        // Create matched trades at clearing price
        let matched_trades = self.create_matched_trades_at_clearing_price(
            buy_orders,
            sell_orders,
            clearing_price,
            clearing_volume,
        ).await?;

        Ok((clearing_price, clearing_volume, matched_trades))
    }

    /// Create matched trades at the uniform clearing price
    async fn create_matched_trades_at_clearing_price(
        &self,
        _buy_orders: &[PriceLevel],
        _sell_orders: &[PriceLevel],
        clearing_price: f64,
        clearing_volume: f64,
    ) -> Result<Vec<EnergyTrade>, DomainError> {
        // This is a simplified implementation
        // In reality, we would match specific buy and sell orders
        // and create individual trades for each match
        
        let trades = Vec::new(); // Placeholder
        
        info!(
            "Created {} trades at clearing price {:.4} for volume {:.2}",
            trades.len(), clearing_price, clearing_volume
        );

        Ok(trades)
    }

    /// Calculate next auction time based on interval
    fn calculate_next_auction_time(&self, from_time: DateTime<Utc>) -> DateTime<Utc> {
        let interval = Duration::minutes(self.config.interval_minutes);
        
        // Round up to next interval boundary
        let minutes_since_hour = from_time.minute() as i64;
        let minutes_to_next_interval = self.config.interval_minutes - 
            (minutes_since_hour % self.config.interval_minutes);
        
        from_time + Duration::minutes(minutes_to_next_interval)
    }

    /// Check if market is open for trading
    pub fn is_market_open(&self, time: DateTime<Utc>) -> bool {
        // Simplified implementation - in reality would consider holidays, etc.
        let hour = time.hour();
        hour >= 6 && hour <= 22 // 6 AM to 10 PM
    }

    /// Get scheduled auctions for a market
    pub async fn get_scheduled_auctions(&self, market_name: &str) -> Vec<ScheduledAuction> {
        let auctions = self.scheduled_auctions.read().await;
        auctions
            .iter()
            .filter(|auction| auction.market_name == market_name)
            .cloned()
            .collect()
    }

    /// Get auction history for a market
    pub async fn get_auction_history(&self, market_name: &str, limit: usize) -> Vec<AuctionResult> {
        let history = self.auction_history.read().await;
        history
            .iter()
            .filter(|result| result.market_name == market_name)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Start automatic auction scheduling
    pub async fn start_auction_scheduler(&self) -> Result<(), DomainError> {
        info!("Starting auction scheduler with {}-minute intervals", self.config.interval_minutes);
        
        // In a real implementation, this would run as a background service
        // For now, we'll just schedule the next round for all markets
        let order_books = self.order_books.read().await;
        let markets: Vec<String> = order_books.keys().cloned().collect();
        drop(order_books);

        for market_name in markets {
            if let Err(e) = self.schedule_next_auction(&market_name).await {
                warn!("Failed to schedule auction for {}: {}", market_name, e);
            }
        }

        Ok(())
    }

    /// Update auction configuration
    pub async fn update_config(&mut self, new_config: AuctionConfig) {
        self.config = new_config;
        info!("Updated auction configuration");
    }

    /// Get current configuration
    pub fn get_config(&self) -> &AuctionConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::energy_trading::domain::services::dynamic_pricing_service::{
        DynamicPricingService, PricingConfig
    };

    #[tokio::test]
    async fn test_auction_scheduling() {
        let order_books = Arc::new(RwLock::new(HashMap::new()));
        let pricing_config = PricingConfig::default();
        let pricing_service = Arc::new(DynamicPricingService::new(order_books.clone(), pricing_config));
        let auction_config = AuctionConfig::default();
        let scheduler = AuctionSchedulerService::new(order_books, pricing_service, auction_config);

        // Test auction scheduling
        let result = scheduler.schedule_next_auction("test_market").await;
        assert!(result.is_ok());

        let auction = result.unwrap();
        assert_eq!(auction.market_name, "test_market");
        assert!(matches!(auction.status, AuctionStatus::Scheduled));
    }

    #[test]
    fn test_next_auction_time_calculation() {
        let order_books = Arc::new(RwLock::new(HashMap::new()));
        let pricing_config = PricingConfig::default();
        let pricing_service = Arc::new(DynamicPricingService::new(order_books.clone(), pricing_config));
        let auction_config = AuctionConfig { interval_minutes: 30, ..Default::default() };
        let scheduler = AuctionSchedulerService::new(order_books, pricing_service, auction_config);

        let test_time = Utc::now().with_minute(15).unwrap().with_second(0).unwrap();
        let next_time = scheduler.calculate_next_auction_time(test_time);
        
        assert_eq!(next_time.minute(), 30);
    }

    #[test]
    fn test_market_hours() {
        let order_books = Arc::new(RwLock::new(HashMap::new()));
        let pricing_config = PricingConfig::default();
        let pricing_service = Arc::new(DynamicPricingService::new(order_books.clone(), pricing_config));
        let auction_config = AuctionConfig::default();
        let scheduler = AuctionSchedulerService::new(order_books, pricing_service, auction_config);

        let morning = Utc::now().with_hour(8).unwrap();
        let evening = Utc::now().with_hour(20).unwrap();
        let midnight = Utc::now().with_hour(2).unwrap();

        assert!(scheduler.is_market_open(morning));
        assert!(scheduler.is_market_open(evening));
        assert!(!scheduler.is_market_open(midnight));
    }
}
