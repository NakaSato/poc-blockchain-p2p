//! DDD Implementation Tests
//!
//! Tests for the Domain-Driven Design implementation of energy trading.

#[cfg(test)]
mod tests {
    use crate::domains::energy_trading::{
        EnergyTradingDomainService, TraderId, EnergyAmount, PricePerKwh, 
        TradeType, TradingWindow, PlaceEnergyOrderCommand, PlaceEnergyOrderHandler
    };
    use crate::shared::application::{CommandHandler};
    use chrono::{Utc, Duration};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_energy_trading_domain_service_creation() {
        let service = EnergyTradingDomainService::new();
        assert!(service.is_ok());
        
        let service = service.unwrap();
        assert_eq!(service.min_trade_amount().value(), 0.1);
        assert_eq!(service.max_trade_amount().value(), 1000.0);
    }

    #[tokio::test]
    async fn test_place_energy_order_command() {
        // Create domain service
        let service = Arc::new(EnergyTradingDomainService::new().unwrap());
        let handler = PlaceEnergyOrderHandler::new(service);
        
        // Create command
        let now = Utc::now();
        let command = PlaceEnergyOrderCommand {
            trader_id: "trader_001".to_string(),
            order_type: "Buy".to_string(),
            energy_amount: 100.0,
            price_per_kwh: 4.5,
            trading_window_start: now,
            trading_window_end: now + Duration::hours(1),
            market_name: "thailand_central".to_string(),
        };
        
        // Execute command
        let result = handler.handle(command).await;
        assert!(result.is_ok());
        
        let order_result = result.unwrap();
        assert_eq!(order_result.order_type, "Buy");
        assert_eq!(order_result.energy_amount, 100.0);
        assert_eq!(order_result.price_per_kwh, 4.5);
        assert_eq!(order_result.status, "Active");
    }

    #[tokio::test]
    async fn test_energy_trading_value_objects() {
        // Test TradeId
        let trade_id = TraderId::new("trader_123".to_string());
        assert!(trade_id.is_ok());
        assert_eq!(trade_id.unwrap().value(), "trader_123");
        
        // Test EnergyAmount
        let energy_amount = EnergyAmount::new(50.5);
        assert!(energy_amount.is_ok());
        assert_eq!(energy_amount.unwrap().value(), 50.5);
        
        // Test invalid energy amount
        let invalid_energy = EnergyAmount::new(-10.0);
        assert!(invalid_energy.is_err());
        
        // Test PricePerKwh
        let price = PricePerKwh::new(3.75);
        assert!(price.is_ok());
        assert_eq!(price.unwrap().value(), 3.75);
        
        // Test TradeType
        let buy_type = TradeType::from_string("Buy");
        assert!(buy_type.is_ok());
        assert_eq!(buy_type.unwrap(), TradeType::Buy);
        
        let sell_type = TradeType::from_string("Sell");
        assert!(sell_type.is_ok());
        assert_eq!(sell_type.unwrap(), TradeType::Sell);
        
        // Test TradingWindow
        let now = Utc::now();
        let window = TradingWindow::new(now, now + Duration::hours(2));
        assert!(window.is_ok());
        
        // Test invalid window (end before start)
        let invalid_window = TradingWindow::new(now, now - Duration::hours(1));
        assert!(invalid_window.is_err());
    }

    #[tokio::test]
    async fn test_order_book_functionality() {
        use crate::domains::energy_trading::{OrderBook, EnergyOrder};
        
        let mut order_book = OrderBook::new("test_market".to_string());
        
        // Create a buy order
        let trader_id = TraderId::new("buyer_001".to_string()).unwrap();
        let energy_amount = EnergyAmount::new(100.0).unwrap();
        let price = PricePerKwh::new(4.0).unwrap();
        let now = Utc::now();
        let window = TradingWindow::new(now, now + Duration::hours(1)).unwrap();
        
        let buy_order = EnergyOrder::new(
            trader_id,
            TradeType::Buy,
            energy_amount,
            price,
            window,
        ).unwrap();
        
        // Add order to book
        let trades = order_book.add_order(buy_order);
        assert!(trades.is_ok());
        assert!(trades.unwrap().is_empty()); // No matching orders yet
        
        // Check market depth
        let depth = order_book.get_market_depth();
        assert_eq!(depth.buy_orders.len(), 1);
        assert_eq!(depth.sell_orders.len(), 0);
        assert_eq!(depth.total_buy_volume, 100.0);
    }

    #[tokio::test]
    async fn test_domain_events() {
        use crate::domains::energy_trading::EnergyOrderCreatedEvent;
        use crate::shared::domain::events::DomainEvent;
        
        let event = EnergyOrderCreatedEvent {
            event_id: uuid::Uuid::new_v4(),
            order_id: "order_123".to_string(),
            trader_id: "trader_456".to_string(),
            order_type: "Buy".to_string(),
            energy_amount: 50.0,
            price_per_kwh: 4.25,
            occurred_at: Utc::now(),
            aggregate_version: 1,
        };
        
        assert_eq!(event.event_type(), "EnergyOrderCreated");
        assert_eq!(event.aggregate_id(), "order_123");
        assert_eq!(event.aggregate_version(), 1);
        
        // Test event serialization
        let event_data = event.event_data();
        assert!(event_data.is_object());
    }

    #[tokio::test]
    async fn test_business_rules_validation() {
        let service = EnergyTradingDomainService::new().unwrap();
        
        // Test minimum energy amount violation
        let result = service.place_order(
            TraderId::new("trader_001".to_string()).unwrap(),
            TradeType::Buy,
            EnergyAmount::new(0.05).unwrap(), // Below minimum
            PricePerKwh::new(4.0).unwrap(),
            TradingWindow::new(Utc::now(), Utc::now() + Duration::hours(1)).unwrap(),
            "test_market".to_string(),
        ).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("below minimum"));
        
        // Test maximum energy amount violation  
        let result = service.place_order(
            TraderId::new("trader_001".to_string()).unwrap(),
            TradeType::Buy,
            EnergyAmount::new(2000.0).unwrap(), // Above maximum
            PricePerKwh::new(4.0).unwrap(),
            TradingWindow::new(Utc::now(), Utc::now() + Duration::hours(1)).unwrap(),
            "test_market".to_string(),
        ).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum"));
        
        // Test invalid price range
        let result = service.place_order(
            TraderId::new("trader_001".to_string()).unwrap(),
            TradeType::Buy,
            EnergyAmount::new(100.0).unwrap(),
            PricePerKwh::new(100.0).unwrap(), // Outside reasonable range
            TradingWindow::new(Utc::now(), Utc::now() + Duration::hours(1)).unwrap(),
            "test_market".to_string(),
        ).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("outside reasonable range"));
    }
}
