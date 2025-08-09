---
mode: edit
type: domain-specific
domain: energy-trading
priority: critical
tags: [energy-trading, market, orders, grid, thailand, ddd, rust]
---

# 🏪 GridTokenX Energy Trading System Development Guide

> **Powering Thailand's Energy Marketplace**  
> Expert guidance for developing the core energy trading domain that revolutionizes peer-to-peer energy exchange.

## 🎯 Domain Focus: Energy Trading

You are developing the **Energy Trading Domain** - the heart of GridTokenX that enables secure, efficient, and regulatory-compliant energy transactions across Thailand's electrical grid.

### 🏗️ **Domain-Driven Architecture**
```
src/domains/energy_trading/
├── domain/
│   ├── entities/           # EnergyOrder, Trade, Market, Trader
│   ├── value_objects/      # Price, EnergyAmount, TradingWindow, GridLocation
│   ├── aggregates/         # OrderBook, TradingSession, MarketDepth
│   └── services/          # OrderMatchingService, PriceDiscoveryService
├── application/
│   ├── commands/          # PlaceOrder, CancelOrder, ExecuteTrade
│   ├── queries/           # GetMarketDepth, GetOrderHistory, GetPricing
│   └── services/          # EnergyTradingApplicationService
└── infrastructure/
    ├── persistence/       # OrderRepository, TradeRepository
    ├── grid_integration/  # GridStateAdapter, AuthorityNotification
    └── pricing/           # PricingEngineAdapter, MarketDataProvider
```

## ⚡ Energy Trading Business Logic

### 🔄 **Order Lifecycle Management**
#### **Order States & Transitions**
```
🔄 Order Lifecycle:
📝 Created → ✅ Validated → 📋 Active → ⚡ Executing → ✨ Completed
              ↓             ↓           ↓
              ❌ Rejected   🚫 Cancelled 🔄 Partially Filled
```

#### **Order Types & Business Rules**
| Order Type | Use Case | Priority | Grid Impact |
|------------|----------|----------|-------------|
| **Market** | Immediate execution | High | Real-time grid response |
| **Limit** | Price-specific trades | Normal | Planned grid allocation |
| **GridBalancing** | System stability | Critical | Automatic grid correction |
| **Emergency** | Crisis response | Highest | Override all constraints |

### ⚖️ **Energy Conservation & Validation**
```
🔬 Energy Physics Enforcement:
├── 🔋 Energy cannot be created or destroyed
├── ⚡ Total input = Total output + Transmission losses
├── 🌍 Grid capacity constraints must be respected
├── 📊 Real-time energy balance validation
└── 🚨 Reject trades violating conservation laws
```

### 💰 **Pricing & Market Dynamics**

#### **Value Objects**
- **Price**: Satoshi per kWh with min/max bounds validation
- **EnergyAmount**: kWh with precision and realistic limits
- **GridLocation**: Thai grid coordinates with distance calculations
- **TradingWindow**: 15-minute intervals aligned with grid operations

#### **Dynamic Pricing Factors**
```
💱 Price Discovery Algorithm:
├── 📊 Supply/Demand ratio (primary factor)
├── 🕘 Time-of-use patterns (peak vs off-peak)
├── 🌍 Geographic congestion (transmission costs)
├── 🌱 Renewable energy premiums/discounts
├── 🏛️ Authority-mandated pricing floors/ceilings
└── ⚡ Real-time grid stability requirements
```

## 🇹🇭 Thai Energy Market Integration

### 🏛️ **Authority Coordination**
| Authority | Role | Trading Privileges | Validation Requirements |
|-----------|------|-------------------|-------------------------|
| **EGAT** | Transmission operator | Emergency grid orders, wholesale oversight | Multi-signature validation |
| **MEA** | Bangkok distribution | Metro area order validation | Geographic bounds check |
| **PEA** | Provincial distribution | Rural/provincial oversight | License verification |
| **ERC** | Market regulator | Dispute resolution, compliance | Regulatory compliance check |

### 📊 **Market Compliance & Regulatory Integration**

#### **Compliance Validation**
- ✅ Energy Trading Act B.E. 2562 (2019) compliance
- ✅ ERC license verification for all participants  
- ✅ NEPO real-time reporting integration
- ✅ Thai Grid Code technical standards
- ✅ Authority-mandated pricing constraints

#### **Time-of-Use Integration**
```
⏰ Thai Market Timing:
🌅 Peak Hours: 09:00-22:00 (premium rates)
🌙 Off-Peak: 22:00-09:00 (discounted rates)
🌡️ Seasonal: Mar-May hot season (surge pricing)
📍 Regional: Zone-specific congestion pricing
```

## 🏗️ Domain Services & Application Layer

### 🔧 **Domain Services** (`domain/services/`)

#### **OrderMatchingService**
```
🎯 Matching Algorithm:
├── 1️⃣ Price-time priority (best price wins)
├── 2️⃣ Grid constraint validation (capacity limits)
├── 3️⃣ Authority override handling (emergency orders)
├── 4️⃣ Renewable energy prioritization (green bonus)
└── 5️⃣ Geographic optimization (transmission efficiency)
```

#### **PriceDiscoveryService**
- Real-time supply/demand analysis
- Grid congestion impact calculation
- Authority pricing constraint enforcement
- Renewable energy premium/discount application

#### **GridConstraintService**
- Transmission capacity validation
- Grid stability impact assessment
- Authority emergency protocol integration
- Real-time grid state monitoring

### 🚀 **Application Services** (`application/services/`)

#### **EnergyTradingApplicationService**
- Order placement and validation orchestration
- Trade execution and settlement coordination
- Authority notification and compliance reporting
- Market data aggregation and distribution

### 📝 **Commands & Queries**

#### **Commands** (`application/commands/`)
- `PlaceEnergyOrder`: Create new buy/sell order with validation
- `CancelOrder`: Cancel active order with grid impact check
- `ExecuteTrade`: Complete matched trade with settlement
- `HandleGridEmergency`: Process authority emergency protocols

#### **Queries** (`application/queries/`)
- `GetMarketDepth`: Current buy/sell order book state
- `GetOrderHistory`: Trader's historical order activity
- `GetPricingData`: Real-time and historical pricing information
- `GetGridImpact`: Trading impact on grid stability

## 🔄 Trading Aggregates & Business Rules

### 🏪 **OrderBook Aggregate**
```
📊 OrderBook Responsibilities:
├── 📋 Maintain buy/sell order collections
├── ⚖️ Enforce price-time priority matching
├── 🚨 Validate grid capacity constraints
├── 💱 Execute automated trade matching
├── 📊 Update market depth calculations
└── 🔔 Emit trade execution events
```

### 💼 **TradingSession Aggregate**
- Session-based trading coordination (15-minute windows)
- Authority override and emergency protocol handling
- Cross-border energy transfer validation
- Settlement and blockchain transaction coordination

### 📈 **MarketDepth Aggregate**
- Real-time order book depth calculation
- Price impact analysis for large orders
- Liquidity assessment and market maker incentives
- Historical market data aggregation

## 🧪 Testing Strategy

### 🔬 **Domain Testing**
- **Energy Conservation Tests**: Verify all trades maintain energy balance
- **Grid Constraint Tests**: Validate capacity and stability limits
- **Authority Integration Tests**: Ensure proper privilege enforcement
- **Market Manipulation Tests**: Protect against unfair trading practices

### 🎭 **Integration Testing**
- **Real-time Grid Integration**: Live grid data validation
- **Authority System Integration**: MEA/PEA/EGAT coordination
- **Blockchain Settlement**: End-to-end trade recording
- **Performance Testing**: Peak hour trading load handling

## 📚 Key Implementation Patterns

### 🎯 **Repository Pattern**
- **OrderRepository**: Persistent order storage and querying
- **TradeRepository**: Trade history and settlement tracking
- **MarketDataRepository**: Historical pricing and market data

### 📨 **Domain Events**
- **OrderPlaced**: New order entered into system
- **OrderMatched**: Successful order matching
- **TradeExecuted**: Completed energy transfer
- **GridConstraintViolation**: Invalid trade attempt
- **AuthorityOverride**: Emergency authority intervention

### 🔧 **Anti-Corruption Layer**
- **GridSystemAdapter**: Interface to Thai grid infrastructure
- **AuthorityNotificationAdapter**: Real-time authority integration
- **BlockchainAdapter**: Energy transaction recording
- **PricingEngineAdapter**: External market data integration

---

## 🎯 Implementation Focus Areas

> **Start with**: Order lifecycle and basic matching algorithm
> **Critical**: Grid constraint validation and authority integration
> **Remember**: Every trade must respect energy physics and Thai market regulations
- **Seasonal Adjustments**: Hot season (Mar-May) premium
- **Grid Congestion Multipliers**: Location-based pricing

## Grid Management Features

### Real-Time Monitoring
```rust
pub struct GridStatus {
    pub frequency: f64,              // Grid frequency (50 Hz target)
    pub voltage_levels: HashMap<String, f64>,
    pub power_flow: HashMap<String, PowerFlow>,
    pub congestion_points: Vec<CongestionPoint>,
    pub renewable_generation: f64,
    pub demand_forecast: DemandForecast,
}
```

### Congestion Management
- **Transmission Constraints**: Respect line capacity limits
- **Distribution Limits**: Local transformer and feeder capacity
- **Dynamic Pricing**: Higher prices in congested areas
- **Load Curtailment**: Emergency demand reduction protocols

### Grid Balancing
- **Frequency Regulation**: Automatic generation control (AGC)
- **Reserve Services**: Primary, secondary, and tertiary reserves
- **Demand Response**: Incentivized load shifting and reduction
- **Storage Integration**: Battery and pumped hydro coordination

## Performance Requirements

### Trading Performance
- **Order Processing**: <100ms for standard orders
- **Matching Speed**: 10,000+ orders/second during peak hours
- **Settlement**: Real-time for completed trades
- **Grid Updates**: <1 second propagation to all nodes

### Reliability Standards
- **Availability**: 99.99% uptime (critical infrastructure)
- **Data Integrity**: Zero tolerance for energy double-spending
- **Disaster Recovery**: <15 minutes RTO/RPO
- **Load Balancing**: Automatic failover between trading nodes

## Development Guidelines

### Energy Conservation Validation
```rust
impl EnergyTrading {
    pub fn validate_energy_conservation(&self, transactions: &[Transaction]) -> Result<()> {
        let total_generation: f64 = transactions.iter()
            .filter_map(|tx| tx.energy_output())
            .sum();
        let total_consumption: f64 = transactions.iter()
            .filter_map(|tx| tx.energy_input())
            .sum();
        
        if (total_generation - total_consumption).abs() > ENERGY_TOLERANCE {
            return Err(anyhow!("Energy conservation violation"));
        }
        Ok(())
    }
}
```

### Error Handling Patterns
- **Grid Violations**: Reject transactions that violate grid constraints
- **Market Abuse**: Detect and prevent market manipulation
- **Authority Conflicts**: Resolve competing authority directives
- **System Failures**: Graceful degradation with emergency protocols

### Testing Strategy
- **Market Simulation**: Historical Thai energy market data replay
- **Grid Stress Testing**: Extreme load and generation scenarios
- **Authority Integration**: Mock EGAT/MEA/PEA system testing
- **Performance Testing**: Peak trading hour load simulation

## Renewable Energy Integration

### Certificate System
```rust
pub struct RenewableCertificate {
    pub energy_source: EnergySource,     // Solar, Wind, Hydro, Biomass
    pub generation_time: DateTime<Utc>,
    pub location: GridLocation,
    pub carbon_offset: f64,              // kg CO2 equivalent
    pub certification_authority: String,
}
```

### Carbon Tracking
- **Emission Factors**: Source-specific CO2 emissions per kWh
- **Carbon Credits**: Tradeable carbon offset certificates
- **Sustainability Metrics**: Real-time grid carbon intensity
- **Reporting**: Automated ESG compliance reporting

When implementing energy trading features, prioritize grid stability, regulatory compliance, and real-time performance while ensuring fair market operations and promoting renewable energy adoption.
