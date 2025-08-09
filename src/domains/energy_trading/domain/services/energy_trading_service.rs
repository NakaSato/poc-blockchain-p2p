//! Energy Trading Domain Service
//!
//! Contains core business logic for energy trading operations.

use crate::shared::domain::errors::DomainError;
use crate::domains::energy_trading::domain::{
    entities::{EnergyOrder, EnergyTrade},
    aggregates::OrderBook,
    value_objects::{TradeId, TraderId, EnergyAmount, PricePerKwh, TradeType, TradingWindow},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Market pricing strategies
#[derive(Debug, Clone)]
pub enum PricingStrategy {
    FixedSpread(f64),
    DynamicMarket,
    TimeOfUse,
}

/// Energy Trading Domain Service
pub struct EnergyTradingDomainService {
    order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
    pricing_strategy: PricingStrategy,
    min_trade_amount: EnergyAmount,
    max_trade_amount: EnergyAmount,
}

impl EnergyTradingDomainService {
    pub fn new() -> Result<Self, DomainError> {
        Ok(Self {
            order_books: Arc::new(RwLock::new(HashMap::new())),
            pricing_strategy: PricingStrategy::DynamicMarket,
            min_trade_amount: EnergyAmount::new(0.1)?, // 0.1 kWh minimum
            max_trade_amount: EnergyAmount::new(1000.0)?, // 1000 kWh maximum
        })
    }
    
    pub async fn place_order(
        &self,
        trader_id: TraderId,
        order_type: TradeType,
        energy_amount: EnergyAmount,
        price_per_kwh: PricePerKwh,
        trading_window: TradingWindow,
        market_name: String,
    ) -> Result<(EnergyOrder, Vec<EnergyTrade>), DomainError> {
        // Business rule validation
        self.validate_order_constraints(&energy_amount, &price_per_kwh, &trading_window)?;
        
        // Create the order
        let order = EnergyOrder::new(
            trader_id,
            order_type,
            energy_amount,
            price_per_kwh,
            trading_window,
        )?;
        
        // Process order through the order book
        let trades = {
            let mut order_books = self.order_books.write().await;
            let order_book = order_books
                .entry(market_name.clone())
                .or_insert_with(|| OrderBook::new(market_name));
            
            order_book.add_order(order.clone())?
        };
        
        Ok((order, trades))
    }
    
    pub async fn cancel_order(
        &self,
        order_id: &TradeId,
        market_name: &str,
    ) -> Result<(), DomainError> {
        let mut order_books = self.order_books.write().await;
        
        if let Some(order_book) = order_books.get_mut(market_name) {
            order_book.cancel_order(order_id)?;
            Ok(())
        } else {
            Err(DomainError::aggregate_not_found(
                format!("Market not found: {}", market_name)
            ))
        }
    }
    
    pub async fn get_market_depth(&self, market_name: &str) -> Result<crate::domains::energy_trading::domain::aggregates::MarketDepth, DomainError> {
        let order_books = self.order_books.read().await;
        
        if let Some(order_book) = order_books.get(market_name) {
            Ok(order_book.get_market_depth())
        } else {
            Err(DomainError::aggregate_not_found(
                format!("Market not found: {}", market_name)
            ))
        }
    }
    
    pub async fn calculate_optimal_price(
        &self,
        order_type: &TradeType,
        energy_amount: &EnergyAmount,
        market_name: &str,
    ) -> Result<PricePerKwh, DomainError> {
        match self.pricing_strategy {
            PricingStrategy::FixedSpread(spread) => {
                self.calculate_fixed_spread_price(order_type, energy_amount, spread, market_name).await
            }
            PricingStrategy::DynamicMarket => {
                self.calculate_dynamic_market_price(order_type, energy_amount, market_name).await
            }
            PricingStrategy::TimeOfUse => {
                self.calculate_time_of_use_price(order_type, energy_amount).await
            }
        }
    }
    
    fn validate_order_constraints(
        &self,
        energy_amount: &EnergyAmount,
        price_per_kwh: &PricePerKwh,
        trading_window: &TradingWindow,
    ) -> Result<(), DomainError> {
        // Check energy amount bounds
        if energy_amount.value() < self.min_trade_amount.value() {
            return Err(DomainError::business_rule_violation(
                format!("Energy amount {} is below minimum {}", 
                    energy_amount.value(), 
                    self.min_trade_amount.value())
            ));
        }
        
        if energy_amount.value() > self.max_trade_amount.value() {
            return Err(DomainError::business_rule_violation(
                format!("Energy amount {} exceeds maximum {}", 
                    energy_amount.value(), 
                    self.max_trade_amount.value())
            ));
        }
        
        // Check price bounds (reasonable market prices in Thai context)
        if price_per_kwh.value() < 1.0 || price_per_kwh.value() > 50.0 {
            return Err(DomainError::business_rule_violation(
                format!("Price {} GTX/kWh is outside reasonable range (1-50)", 
                    price_per_kwh.value())
            ));
        }
        
        // Check trading window validity
        if !trading_window.is_valid() {
            return Err(DomainError::business_rule_violation(
                "Trading window is invalid"
            ));
        }
        
        Ok(())
    }
    
    async fn calculate_fixed_spread_price(
        &self,
        order_type: &TradeType,
        _energy_amount: &EnergyAmount,
        spread: f64,
        market_name: &str,
    ) -> Result<PricePerKwh, DomainError> {
        let market_depth = self.get_market_depth(market_name).await?;
        
        let base_price = if let Some(best_buy) = market_depth.buy_orders.first() {
            if let Some(best_sell) = market_depth.sell_orders.first() {
                (best_buy.price + best_sell.price) / 2.0
            } else {
                best_buy.price
            }
        } else if let Some(best_sell) = market_depth.sell_orders.first() {
            best_sell.price
        } else {
            // No orders in book, use default price
            4.5 // Default GTX/kWh price for Thai energy market
        };
        
        let adjusted_price = match order_type {
            TradeType::Buy => base_price - spread,
            TradeType::Sell => base_price + spread,
        };
        
        PricePerKwh::new(adjusted_price.max(1.0))
    }
    
    async fn calculate_dynamic_market_price(
        &self,
        order_type: &TradeType,
        energy_amount: &EnergyAmount,
        market_name: &str,
    ) -> Result<PricePerKwh, DomainError> {
        let market_depth = self.get_market_depth(market_name).await?;
        
        // Calculate price based on market depth and order size
        let base_price = if let Some(spread) = market_depth.spread {
            let mid_price = if let (Some(best_buy), Some(best_sell)) = 
                (market_depth.buy_orders.first(), market_depth.sell_orders.first()) {
                (best_buy.price + best_sell.price) / 2.0
            } else {
                4.5 // Default price
            };
            
            // Adjust for order size impact
            let size_impact = (energy_amount.value() / 100.0) * 0.1; // 0.1% impact per 100 kWh
            
            match order_type {
                TradeType::Buy => mid_price + (spread / 4.0) + size_impact,
                TradeType::Sell => mid_price - (spread / 4.0) - size_impact,
            }
        } else {
            4.5 // Default price when no spread available
        };
        
        PricePerKwh::new(base_price.max(1.0))
    }
    
    async fn calculate_time_of_use_price(
        &self,
        _order_type: &TradeType,
        _energy_amount: &EnergyAmount,
    ) -> Result<PricePerKwh, DomainError> {
        use chrono::Timelike;
        
        let now = chrono::Utc::now();
        let hour = now.hour();
        
        // Thai energy market time-of-use pricing
        let base_price = match hour {
            6..=8 | 18..=22 => 6.0,   // Peak hours
            9..=16 => 4.5,            // Standard hours  
            _ => 3.5,                 // Off-peak hours
        };
        
        PricePerKwh::new(base_price)
    }
    
    pub async fn validate_trade_compliance(
        &self,
        trade: &EnergyTrade,
    ) -> Result<bool, DomainError> {
        // Business compliance rules for Thai energy market
        
        // Check trade amount limits
        if trade.energy_amount().value() < self.min_trade_amount.value() 
            || trade.energy_amount().value() > self.max_trade_amount.value() {
            return Ok(false);
        }
        
        // Check price reasonableness
        if trade.price_per_kwh().value() < 1.0 || trade.price_per_kwh().value() > 50.0 {
            return Ok(false);
        }
        
        // Check total trade value limits
        if trade.total_price() > 50000.0 { // 50,000 GTX limit per trade
            return Ok(false);
        }
        
        // All checks passed
        Ok(true)
    }
    
    // Getters
    pub fn min_trade_amount(&self) -> &EnergyAmount {
        &self.min_trade_amount
    }
    
    pub fn max_trade_amount(&self) -> &EnergyAmount {
        &self.max_trade_amount
    }
    
    pub fn pricing_strategy(&self) -> &PricingStrategy {
        &self.pricing_strategy
    }
}
