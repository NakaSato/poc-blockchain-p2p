---
mode: edit
---

# GridTokenX Energy Trading System Development Prompt

You are developing the energy trading module for GridTokenX - Thailand's peer-to-peer energy trading blockchain platform.

## Energy Trading System Overview

The energy trading system (`src/energy.rs`) implements real-time energy market operations with blockchain integration:

### Core Components
1. **EnergyTrading**: Main trading system coordinator
2. **GridManager**: Grid monitoring and congestion management
3. **EnergyOrderBook**: Buy/sell order matching system
4. **TradingEngine**: Advanced order matching algorithms

## Energy Market Mechanics

### Order Types
```rust
pub enum OrderType {
    Market,           // Execute immediately at market price
    Limit(u64),      // Execute at specific price or better
    GridBalancing,   // System-generated grid balancing orders
    Emergency,       // Critical grid stability orders
}
```

### Energy Units and Pricing
- **Base Unit**: 1 kWh = 1 GridToken (GT)
- **Pricing**: Dynamic based on supply/demand and grid conditions
- **Time Slots**: 15-minute trading intervals aligned with grid operations
- **Location**: Grid node-specific pricing for congestion management

### Order Matching Algorithm
1. **Price-Time Priority**: Best price first, then earliest timestamp
2. **Grid Constraints**: Respect transmission capacity limits
3. **Authority Override**: Grid operators can insert emergency orders
4. **Renewable Priority**: Bonus matching for certified renewable energy

## Thai Energy Market Integration

### Authority Integration
- **EGAT**: Transmission system operator, wholesale market oversight
- **MEA/PEA**: Distribution operators, retail market participation
- **NEPO**: Policy compliance and market monitoring
- **ERC**: Regulatory oversight and dispute resolution

### Market Compliance
```rust
pub struct MarketCompliance {
    pub energy_act_compliance: bool,    // Energy Trading Act B.E. 2562
    pub erc_registration: String,       // ERC license number
    pub nepo_reporting: bool,          // NEPO data submission
    pub grid_code_compliance: bool,    // Technical grid standards
}
```

### Time-of-Use Pricing
- **Peak Hours**: 09:00-22:00 (higher rates)
- **Off-Peak Hours**: 22:00-09:00 (lower rates)
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
