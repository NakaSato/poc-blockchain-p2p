# Domain-Driven Design Architecture

This document describes the Domain-Driven Design (DDD) architecture implemented in GridTokenX blockchain platform.

## 🎯 Overview

GridTokenX uses DDD to manage complexity and maintain clear boundaries between different business concerns. The architecture ensures that business logic is well-encapsulated, testable, and maintainable.

## 🏗️ Architecture Layers

### Domain Layer
The core business logic layer containing:
- **Entities**: Objects with identity and lifecycle
- **Value Objects**: Immutable objects defined by their attributes
- **Aggregates**: Consistency boundaries for business operations
- **Domain Services**: Stateless business logic operations
- **Domain Events**: Significant business occurrences

### Application Layer
Orchestrates business operations:
- **Command Handlers**: Process write operations
- **Query Handlers**: Handle read operations
- **Event Handlers**: React to domain events
- **Application Services**: Coordinate domain operations

### Infrastructure Layer
Technical implementation details:
- **Repositories**: Data persistence implementations
- **External Services**: Third-party integrations
- **Messaging**: Event publishing and handling
- **Storage**: Database and file system access

### Shared Kernel
Common elements used across domains:
- **Base Types**: Common interfaces and traits
- **Error Handling**: Standardized error types
- **Event Infrastructure**: Event publishing mechanisms
- **Repository Patterns**: Data access abstractions

## 🔄 Bounded Contexts

### Energy Trading Domain

The primary business domain for peer-to-peer energy trading.

#### Value Objects
```rust
// Trade identification
pub struct TradeId(Uuid);
pub struct TraderId(Uuid);

// Energy measurements
pub struct EnergyAmount(f64); // kWh
pub struct PricePerKwh(f64);  // Satoshis per kWh

// Trading constraints
pub struct TradingWindow {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}
```

#### Entities
```rust
// Energy order with lifecycle
pub struct EnergyOrder {
    id: TradeId,
    trader_id: TraderId,
    order_type: TradeType,
    energy_amount: EnergyAmount,
    price_per_kwh: PricePerKwh,
    status: TradeStatus,
    // ... other fields
}

// Completed energy trade
pub struct EnergyTrade {
    id: TradeId,
    buyer_id: TraderId,
    seller_id: TraderId,
    energy_amount: EnergyAmount,
    settlement_status: SettlementStatus,
    // ... other fields
}
```

#### Aggregates
```rust
// Order book managing trading operations
pub struct OrderBook {
    id: OrderBookId,
    market_name: String,
    buy_orders: Vec<EnergyOrder>,
    sell_orders: Vec<EnergyOrder>,
    trades: Vec<EnergyTrade>,
    // ... business invariants
}
```

#### Domain Services
```rust
pub struct EnergyTradingDomainService {
    // Business rules and validations
    pub fn place_order(&self, ...) -> Result<(EnergyOrder, Vec<EnergyTrade>), DomainError>;
    pub fn match_orders(&self, ...) -> Result<Vec<EnergyTrade>, DomainError>;
    pub fn validate_trading_window(&self, ...) -> Result<(), DomainError>;
}
```

## 📋 CQRS Implementation

### Command Side (Writes)
Commands represent user intentions to change system state:

```rust
pub struct PlaceEnergyOrderCommand {
    pub trader_id: TraderId,
    pub order_type: TradeType,
    pub energy_amount: EnergyAmount,
    pub price_per_kwh: PricePerKwh,
    pub trading_window: TradingWindow,
    pub market_name: String,
}

pub struct PlaceEnergyOrderHandler {
    domain_service: Arc<EnergyTradingDomainService>,
}

impl CommandHandler<PlaceEnergyOrderCommand> for PlaceEnergyOrderHandler {
    async fn handle(&self, command: PlaceEnergyOrderCommand) 
        -> Result<CommandResult, DomainError>;
}
```

### Query Side (Reads)
Queries retrieve data without side effects:

```rust
pub struct GetActiveOrdersQuery {
    pub market_name: String,
    pub trader_id: Option<TraderId>,
}

pub struct GetActiveOrdersHandler {
    repository: Arc<dyn OrderBookRepository>,
}
```

### Event Side
Events capture what happened in the domain:

```rust
pub struct EnergyOrderPlacedEvent {
    pub order_id: TradeId,
    pub trader_id: TraderId,
    pub order_type: TradeType,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub occurred_at: DateTime<Utc>,
}
```

## 🔧 Repository Pattern

### Abstract Repository
```rust
pub trait Repository<T: AggregateRoot> {
    async fn save(&mut self, aggregate: &mut T) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &T::IdType) -> Result<Option<T>, DomainError>;
    async fn find_all(&self) -> Result<Vec<T>, DomainError>;
}

pub trait OrderBookRepository: Repository<OrderBook> {
    async fn find_by_market(&self, market: &str) -> Result<Option<OrderBook>, DomainError>;
    async fn find_active_orders(&self, market: &str) -> Result<Vec<EnergyOrder>, DomainError>;
}
```

### Concrete Implementation
```rust
pub struct RocksDbOrderBookRepository {
    db: Arc<RwLock<rocksdb::DB>>,
}

impl Repository<OrderBook> for RocksDbOrderBookRepository {
    async fn save(&mut self, order_book: &mut OrderBook) -> Result<(), DomainError> {
        // Persist aggregate and publish events
    }
}
```

## 🎯 Domain Events

### Event Infrastructure
```rust
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> String;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn event_data(&self) -> serde_json::Value;
}

pub trait EventBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), DomainError>;
    async fn subscribe<H>(&mut self, handler: H) -> Result<(), DomainError>
    where H: EventHandler + Send + Sync + 'static;
}
```

### Event Handlers
```rust
pub struct EnergyOrderEventHandler {
    notification_service: Arc<dyn NotificationService>,
}

impl EventHandler for EnergyOrderEventHandler {
    async fn handle(&self, event: Box<dyn DomainEvent>) -> Result<(), DomainError> {
        match event.event_type() {
            "EnergyOrderPlaced" => self.send_order_confirmation(event).await,
            "EnergyOrderMatched" => self.notify_trade_execution(event).await,
            _ => Ok(()),
        }
    }
}
```

## ✅ Testing Strategy

### Unit Tests
Test individual domain objects in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_amount_validation() {
        let result = EnergyAmount::new(-1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("positive"));
    }

    #[test]
    fn test_order_placement() {
        let service = EnergyTradingDomainService::new().unwrap();
        let result = service.place_order(
            // ... valid parameters
        ).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
Test cross-domain interactions:

```rust
#[tokio::test]
async fn test_order_matching_flow() {
    let mut order_book = OrderBook::new("test_market".to_string()).unwrap();
    
    // Place buy order
    let buy_command = PlaceEnergyOrderCommand { /* ... */ };
    let (buy_order, trades) = service.place_order(buy_command).await.unwrap();
    
    // Place matching sell order
    let sell_command = PlaceEnergyOrderCommand { /* ... */ };
    let (sell_order, matched_trades) = service.place_order(sell_command).await.unwrap();
    
    assert!(!matched_trades.is_empty());
}
```

### Domain Tests
Test business rules and invariants:

```rust
#[tokio::test]
async fn test_minimum_trade_amount_enforcement() {
    let service = EnergyTradingDomainService::new().unwrap();
    
    let result = service.place_order(
        TraderId::new(),
        TradeType::Buy,
        EnergyAmount::new(0.05).unwrap(), // Below minimum
        PricePerKwh::new(4.0).unwrap(),
        TradingWindow::new(Utc::now(), Utc::now() + Duration::hours(1)).unwrap(),
        "test_market".to_string(),
    ).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("below minimum"));
}
```

## 🚀 Benefits of DDD Implementation

### Business Alignment
- Clear separation between business logic and technical concerns
- Domain experts can understand and validate business rules
- Business requirements directly map to domain code

### Maintainability
- Clean architecture with well-defined boundaries
- Easy to modify business rules without affecting infrastructure
- Clear dependency directions (Domain ← Application ← Infrastructure)

### Testability
- Domain logic can be tested in isolation
- Mock external dependencies easily
- Business rules validation through unit tests

### Scalability
- Bounded contexts can evolve independently
- Clear interfaces enable distributed development
- Event-driven architecture supports loose coupling

## 📚 Further Reading

- [Domain-Driven Design by Eric Evans](https://www.oreilly.com/library/view/domain-driven-design-tackling/0321125215/)
- [Implementing Domain-Driven Design by Vaughn Vernon](https://www.oreilly.com/library/view/implementing-domain-driven-design/9780133039900/)
- [Building Microservices by Sam Newman](https://www.oreilly.com/library/view/building-microservices-2nd/9781492034018/)
- [GridTokenX DDD Migration Plan](../DDD_MIGRATION_PLAN.md)

## 🤝 Contributing to DDD Architecture

When contributing to the DDD architecture:

1. **Understand the Domain**: Learn the business rules before coding
2. **Follow Patterns**: Use established DDD patterns consistently
3. **Write Tests**: Test business logic thoroughly
4. **Document Decisions**: Explain complex business rules
5. **Respect Boundaries**: Don't create dependencies between domains

For questions about the DDD implementation, consult the domain experts or create an issue with the "architecture" label.
