//! Order Book Aggregate
//!
//! Manages the order book for energy trading in a specific market.

use crate::shared::domain::{
    errors::DomainError,
    repository::AggregateRoot,
    events::DomainEvent,
};
use crate::domains::energy_trading::domain::{
    entities::{EnergyOrder, EnergyTrade},
    value_objects::{TradeId, TraderId, EnergyAmount, PricePerKwh, TradeType, TradeStatus},
};
use chrono::{DateTime, Utc};
use std::collections::{BTreeMap, VecDeque};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Market depth information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepth {
    pub buy_orders: Vec<PriceLevel>,
    pub sell_orders: Vec<PriceLevel>,
    pub spread: Option<f64>,
    pub total_buy_volume: f64,
    pub total_sell_volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: f64,
    pub volume: f64,
    pub order_count: usize,
}

/// Order Book Aggregate
pub struct OrderBook {
    id: String,
    market_name: String,
    buy_orders: BTreeMap<String, Vec<EnergyOrder>>, // Price -> Orders (sorted by time)
    sell_orders: BTreeMap<String, Vec<EnergyOrder>>, // Price -> Orders (sorted by time)
    last_updated: DateTime<Utc>,
    version: u64,
    uncommitted_events: VecDeque<Box<dyn DomainEvent>>,
}

impl OrderBook {
    pub fn new(market_name: String) -> Self {
        let id = format!("orderbook_{}", market_name);
        
        Self {
            id,
            market_name,
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            last_updated: Utc::now(),
            version: 1,
            uncommitted_events: VecDeque::new(),
        }
    }
    
    pub fn add_order(&mut self, order: EnergyOrder) -> Result<Vec<EnergyTrade>, DomainError> {
        let mut trades = Vec::new();
        
        match order.order_type() {
            TradeType::Buy => {
                trades = self.process_buy_order(order)?;
            }
            TradeType::Sell => {
                trades = self.process_sell_order(order)?;
            }
        }
        
        self.last_updated = Utc::now();
        self.version += 1;
        
        // Raise market updated event
        if !trades.is_empty() {
            self.raise_event(Box::new(TradesExecutedEvent {
                event_id: Uuid::new_v4(),
                order_book_id: self.id.clone(),
                market_name: self.market_name.clone(),
                trade_count: trades.len(),
                total_volume: trades.iter().map(|t| t.energy_amount().value()).sum(),
                occurred_at: self.last_updated,
                aggregate_version: self.version,
            }));
        }
        
        Ok(trades)
    }
    
    fn process_buy_order(&mut self, mut buy_order: EnergyOrder) -> Result<Vec<EnergyTrade>, DomainError> {
        let mut trades = Vec::new();
        let buy_price = buy_order.price_per_kwh().value();
        
        // Find matching sell orders (price <= buy_price)
        let matching_prices: Vec<String> = self.sell_orders
            .keys()
            .filter_map(|price_str| {
                if let Ok(sell_price) = price_str.parse::<f64>() {
                    if sell_price <= buy_price {
                        Some(price_str.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        for price_str in matching_prices {
            if buy_order.remaining_amount().value() <= 0.0 {
                break;
            }
            
            if let Some(sell_orders) = self.sell_orders.get_mut(&price_str) {
                while !sell_orders.is_empty() && buy_order.remaining_amount().value() > 0.0 {
                    let mut sell_order = sell_orders.remove(0);
                    
                    // Create trade without borrowing self
                    let trade = Self::create_trade_static(&mut buy_order, &mut sell_order)?;
                    trades.push(trade);
                    
                    // Re-add sell order if partially filled
                    if sell_order.remaining_amount().value() > 0.0 {
                        sell_orders.insert(0, sell_order);
                    }
                }
                
                // Remove empty price levels
                if sell_orders.is_empty() {
                    self.sell_orders.remove(&price_str);
                }
            }
        }
        
        // Add remaining buy order to book if not fully filled
        if buy_order.remaining_amount().value() > 0.0 {
            let price_key = buy_order.price_per_kwh().value().to_string();
            self.buy_orders
                .entry(price_key)
                .or_insert_with(Vec::new)
                .push(buy_order);
        }
        
        Ok(trades)
    }
    
    fn process_sell_order(&mut self, mut sell_order: EnergyOrder) -> Result<Vec<EnergyTrade>, DomainError> {
        let mut trades = Vec::new();
        let sell_price = sell_order.price_per_kwh().value();
        
        // Find matching buy orders (price >= sell_price)
        let matching_prices: Vec<String> = self.buy_orders
            .keys()
            .filter_map(|price_str| {
                if let Ok(buy_price) = price_str.parse::<f64>() {
                    if buy_price >= sell_price {
                        Some(price_str.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        for price_str in matching_prices {
            if sell_order.remaining_amount().value() <= 0.0 {
                break;
            }
            
            if let Some(buy_orders) = self.buy_orders.get_mut(&price_str) {
                while !buy_orders.is_empty() && sell_order.remaining_amount().value() > 0.0 {
                    let mut buy_order = buy_orders.remove(0);
                    
                    let trade = Self::create_trade_static(&mut buy_order, &mut sell_order)?;
                    trades.push(trade);
                    
                    // Re-add buy order if partially filled
                    if buy_order.remaining_amount().value() > 0.0 {
                        buy_orders.insert(0, buy_order);
                    }
                }
                
                // Remove empty price levels
                if buy_orders.is_empty() {
                    self.buy_orders.remove(&price_str);
                }
            }
        }
        
        // Add remaining sell order to book if not fully filled
        if sell_order.remaining_amount().value() > 0.0 {
            let price_key = sell_order.price_per_kwh().value().to_string();
            self.sell_orders
                .entry(price_key)
                .or_insert_with(Vec::new)
                .push(sell_order);
        }
        
        Ok(trades)
    }
    
    fn create_trade_static(
        buy_order: &mut EnergyOrder,
        sell_order: &mut EnergyOrder,
    ) -> Result<EnergyTrade, DomainError> {
        let trade_amount = EnergyAmount::new(
            buy_order.remaining_amount().value()
                .min(sell_order.remaining_amount().value())
        )?;
        
        // Use seller's price for execution (price improvement for buyer)
        let execution_price = sell_order.price_per_kwh().clone();
        
        // Update order statuses
        buy_order.fill_partially(trade_amount.clone())?;
        sell_order.fill_partially(trade_amount.clone())?;
        
        // Create trade
        let trade = EnergyTrade::new(
            buy_order.trader_id().clone(),
            sell_order.trader_id().clone(),
            trade_amount,
            execution_price,
        )?;
        
        Ok(trade)
    }
    
    pub fn cancel_order(&mut self, order_id: &TradeId) -> Result<(), DomainError> {
        // Search in buy orders
        for (_, orders) in self.buy_orders.iter_mut() {
            if let Some(pos) = orders.iter().position(|order| order.id() == order_id) {
                let mut order = orders.remove(pos);
                order.cancel()?;
                self.last_updated = Utc::now();
                self.version += 1;
                return Ok(());
            }
        }
        
        // Search in sell orders
        for (_, orders) in self.sell_orders.iter_mut() {
            if let Some(pos) = orders.iter().position(|order| order.id() == order_id) {
                let mut order = orders.remove(pos);
                order.cancel()?;
                self.last_updated = Utc::now();
                self.version += 1;
                return Ok(());
            }
        }
        
        Err(DomainError::aggregate_not_found(format!("Order not found: {}", order_id)))
    }
    
    pub fn get_market_depth(&self) -> MarketDepth {
        let buy_levels = self.calculate_price_levels(&self.buy_orders, true);
        let sell_levels = self.calculate_price_levels(&self.sell_orders, false);
        
        let total_buy_volume: f64 = buy_levels.iter().map(|level| level.volume).sum();
        let total_sell_volume: f64 = sell_levels.iter().map(|level| level.volume).sum();
        
        let spread = if let (Some(best_buy), Some(best_sell)) = (buy_levels.first(), sell_levels.first()) {
            Some(best_sell.price - best_buy.price)
        } else {
            None
        };
        
        MarketDepth {
            buy_orders: buy_levels,
            sell_orders: sell_levels,
            spread,
            total_buy_volume,
            total_sell_volume,
        }
    }
    
    fn calculate_price_levels(&self, orders: &BTreeMap<String, Vec<EnergyOrder>>, is_buy: bool) -> Vec<PriceLevel> {
        let mut levels = Vec::new();
        
        for (price_str, order_list) in orders {
            if let Ok(price) = price_str.parse::<f64>() {
                let volume: f64 = order_list.iter()
                    .map(|order| order.remaining_amount().value())
                    .sum();
                
                if volume > 0.0 {
                    levels.push(PriceLevel {
                        price,
                        volume,
                        order_count: order_list.len(),
                    });
                }
            }
        }
        
        // Sort by price (descending for buy, ascending for sell)
        levels.sort_by(|a, b| {
            if is_buy {
                b.price.partial_cmp(&a.price).unwrap()
            } else {
                a.price.partial_cmp(&b.price).unwrap()
            }
        });
        
        levels
    }
    
    // Getters
    pub fn id(&self) -> &str { &self.id }
    pub fn market_name(&self) -> &str { &self.market_name }
    pub fn last_updated(&self) -> DateTime<Utc> { self.last_updated }
}

impl AggregateRoot for OrderBook {
    type Id = String;
    
    fn id(&self) -> &Self::Id {
        &self.id
    }
    
    fn version(&self) -> u64 {
        self.version
    }
    
    fn uncommitted_events(&self) -> &VecDeque<Box<dyn DomainEvent>> {
        &self.uncommitted_events
    }
    
    fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
    
    fn apply_event(&mut self, _event: Box<dyn DomainEvent>) {
        // Event sourcing implementation would go here
    }
    
    fn raise_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push_back(event);
    }
}

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradesExecutedEvent {
    pub event_id: Uuid,
    pub order_book_id: String,
    pub market_name: String,
    pub trade_count: usize,
    pub total_volume: f64,
    pub occurred_at: DateTime<Utc>,
    pub aggregate_version: u64,
}

impl DomainEvent for TradesExecutedEvent {
    fn event_type(&self) -> &'static str {
        "TradesExecuted"
    }
    
    fn aggregate_id(&self) -> String {
        self.order_book_id.clone()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn aggregate_version(&self) -> u64 {
        self.aggregate_version
    }
    
    fn event_id(&self) -> Uuid {
        self.event_id
    }
}
