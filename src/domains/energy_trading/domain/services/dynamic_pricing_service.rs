//! Dynamic Pricing Service
//!
//! Provides real-time price discovery and indicative pricing signals
//! for the energy trading platform based on supply-demand ratios.

use crate::shared::domain::errors::DomainError;
use crate::domains::energy_trading::domain::{
    aggregates::OrderBook,
    value_objects::{PricePerKwh, EnergyAmount},
};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Configuration for dynamic pricing algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    /// Base price when supply equals demand (THB per kWh)
    pub p_balance: f64,
    /// Price range parameter (min/max multiplier)
    pub p_con: f64,
    /// Sensitivity parameter for price curve
    pub k: f64,
    /// Update interval in seconds
    pub update_interval: u64,
    /// Minimum ratio to prevent log(0)
    pub min_ratio: f64,
}

impl Default for PricingConfig {
    fn default() -> Self {
        Self {
            p_balance: 4.0,  // 4 THB per kWh base price
            p_con: 2.0,      // ±2x price range
            k: 1.0,          // Standard sensitivity
            update_interval: 300, // 5 minutes
            min_ratio: 0.01, // Prevent division by zero
        }
    }
}

/// Price signal with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSignal {
    pub market_name: String,
    pub indicative_price: f64,
    pub supply_demand_ratio: f64,
    pub total_supply: f64,
    pub total_demand: f64,
    pub calculated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

/// Dynamic pricing service for real-time price discovery
pub struct DynamicPricingService {
    order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
    config: PricingConfig,
    price_signals: RwLock<HashMap<String, PriceSignal>>,
    price_history: RwLock<Vec<PriceSignal>>,
}

impl DynamicPricingService {
    pub fn new(
        order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
        config: PricingConfig,
    ) -> Self {
        Self {
            order_books,
            config,
            price_signals: RwLock::new(HashMap::new()),
            price_history: RwLock::new(Vec::new()),
        }
    }

    /// Calculate indicative price for a market using dynamic pricing formula
    /// pt = (π/2) * pcon * tan⁻¹(k * ln(Rt)) + pbalance
    pub async fn calculate_indicative_price(&self, market_name: &str) -> Result<f64, DomainError> {
        let ratio = self.get_supply_demand_ratio(market_name).await?;
        
        // Ensure ratio is within safe bounds
        let safe_ratio = ratio.max(self.config.min_ratio);
        
        // Apply dynamic pricing formula
        let ln_ratio = safe_ratio.ln();
        let tan_arg = self.config.k * ln_ratio;
        let tan_result = tan_arg.tanh(); // Use tanh for stability instead of tan
        
        let price = (std::f64::consts::PI / 2.0) * self.config.p_con * tan_result + self.config.p_balance;
        
        // Ensure price is positive
        let final_price = price.max(0.01);
        
        info!(
            "Calculated indicative price for {}: {:.4} THB/kWh (ratio: {:.4})",
            market_name, final_price, safe_ratio
        );
        
        Ok(final_price)
    }

    /// Get current supply-to-demand ratio for a market
    pub async fn get_supply_demand_ratio(&self, market_name: &str) -> Result<f64, DomainError> {
        let order_books = self.order_books.read().await;
        
        let order_book = order_books
            .get(market_name)
            .ok_or_else(|| DomainError::aggregate_not_found(format!("Market {} not found", market_name)))?;
        
        let market_depth = order_book.get_market_depth();
        
        // Calculate total supply (sell orders) and demand (buy orders)
        let total_supply = market_depth.total_sell_volume;
        let total_demand = market_depth.total_buy_volume;
        
        if total_demand <= 0.0 {
            warn!("No demand in market {}, using minimum ratio", market_name);
            return Ok(self.config.min_ratio);
        }
        
        let ratio = total_supply / total_demand;
        
        info!(
            "Market {} - Supply: {:.2} kWh, Demand: {:.2} kWh, Ratio: {:.4}",
            market_name, total_supply, total_demand, ratio
        );
        
        Ok(ratio)
    }

    /// Update price signal for a market and store in history
    pub async fn update_price_signal(&self, market_name: &str) -> Result<PriceSignal, DomainError> {
        let indicative_price = self.calculate_indicative_price(market_name).await?;
        let ratio = self.get_supply_demand_ratio(market_name).await?;
        
        let order_books = self.order_books.read().await;
        let order_book = order_books
            .get(market_name)
            .ok_or_else(|| DomainError::aggregate_not_found(format!("Market {} not found", market_name)))?;
        
        let market_depth = order_book.get_market_depth();
        let now = Utc::now();
        
        let price_signal = PriceSignal {
            market_name: market_name.to_string(),
            indicative_price,
            supply_demand_ratio: ratio,
            total_supply: market_depth.total_sell_volume,
            total_demand: market_depth.total_buy_volume,
            calculated_at: now,
            valid_until: now + chrono::Duration::seconds(self.config.update_interval as i64),
        };
        
        // Store current signal
        {
            let mut signals = self.price_signals.write().await;
            signals.insert(market_name.to_string(), price_signal.clone());
        }
        
        // Add to history
        {
            let mut history = self.price_history.write().await;
            history.push(price_signal.clone());
            
            // Keep last 1000 signals
            if history.len() > 1000 {
                history.remove(0);
            }
        }
        
        info!(
            "Updated price signal for {}: {:.4} THB/kWh",
            market_name, indicative_price
        );
        
        Ok(price_signal)
    }

    /// Get current price signal for a market
    pub async fn get_current_price_signal(&self, market_name: &str) -> Option<PriceSignal> {
        let signals = self.price_signals.read().await;
        signals.get(market_name).cloned()
    }

    /// Get price history for a market
    pub async fn get_price_history(&self, market_name: &str, limit: usize) -> Vec<PriceSignal> {
        let history = self.price_history.read().await;
        history
            .iter()
            .filter(|signal| signal.market_name == market_name)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Start automatic price signal updates
    pub async fn start_price_updates(&self) -> Result<(), DomainError> {
        let interval = self.config.update_interval;
        
        info!("Starting dynamic pricing updates every {} seconds", interval);
        
        // In a real implementation, this would run in a background task
        // For now, we'll update all known markets
        let order_books = self.order_books.read().await;
        let markets: Vec<String> = order_books.keys().cloned().collect();
        drop(order_books);
        
        for market_name in markets {
            if let Err(e) = self.update_price_signal(&market_name).await {
                warn!("Failed to update price signal for {}: {}", market_name, e);
            }
        }
        
        Ok(())
    }

    /// Get pricing configuration
    pub fn get_config(&self) -> &PricingConfig {
        &self.config
    }

    /// Update pricing configuration
    pub async fn update_config(&mut self, new_config: PricingConfig) {
        self.config = new_config;
        info!("Updated pricing configuration");
    }

    /// Calculate theoretical price for given supply/demand volumes
    pub fn calculate_theoretical_price(&self, supply: f64, demand: f64) -> f64 {
        if demand <= 0.0 {
            return self.config.p_balance;
        }
        
        let ratio = (supply / demand).max(self.config.min_ratio);
        let ln_ratio = ratio.ln();
        let tan_arg = self.config.k * ln_ratio;
        let tan_result = tan_arg.tanh();
        
        let price = (std::f64::consts::PI / 2.0) * self.config.p_con * tan_result + self.config.p_balance;
        price.max(0.01)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::energy_trading::domain::{
        aggregates::OrderBook,
        entities::EnergyOrder,
        value_objects::{TraderId, TradeType, EnergyAmount, PricePerKwh, TradingWindow},
    };
    use chrono::Duration;

    #[tokio::test]
    async fn test_dynamic_pricing_calculation() {
        let order_books = Arc::new(RwLock::new(HashMap::new()));
        let config = PricingConfig::default();
        let pricing_service = DynamicPricingService::new(order_books.clone(), config);

        // Test theoretical price calculation
        let price_equal = pricing_service.calculate_theoretical_price(100.0, 100.0);
        assert!((price_equal - 4.0).abs() < 0.1); // Should be close to base price

        let price_high_demand = pricing_service.calculate_theoretical_price(50.0, 100.0);
        assert!(price_high_demand > 4.0); // Price should increase with high demand

        let price_high_supply = pricing_service.calculate_theoretical_price(200.0, 100.0);
        assert!(price_high_supply < 4.0); // Price should decrease with high supply
    }

    #[tokio::test]
    async fn test_supply_demand_ratio_calculation() {
        let order_books = Arc::new(RwLock::new(HashMap::new()));
        let config = PricingConfig::default();
        let pricing_service = DynamicPricingService::new(order_books.clone(), config);

        // Create a test market with orders
        let mut order_book = OrderBook::new("test_market".to_string());
        
        // Add some test orders
        let trader_id = TraderId::new("trader_001".to_string()).unwrap();
        let energy_amount = EnergyAmount::new(100.0).unwrap();
        let price = PricePerKwh::new(4.0).unwrap();
        let now = Utc::now();
        let window = TradingWindow::new(now, now + Duration::hours(1)).unwrap();

        let buy_order = EnergyOrder::new(
            trader_id.clone(),
            TradeType::Buy,
            energy_amount.clone(),
            price.clone(),
            window.clone(),
        ).unwrap();

        let sell_order = EnergyOrder::new(
            trader_id,
            TradeType::Sell,
            energy_amount,
            price,
            window,
        ).unwrap();

        order_book.add_order(buy_order).unwrap();
        order_book.add_order(sell_order).unwrap();

        // Add to service
        {
            let mut books = order_books.write().await;
            books.insert("test_market".to_string(), order_book);
        }

        // Test ratio calculation
        let ratio = pricing_service.get_supply_demand_ratio("test_market").await.unwrap();
        assert!((ratio - 1.0).abs() < 0.01); // Should be approximately 1.0 for equal supply/demand
    }
}
