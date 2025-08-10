# GridTokenX Token Creation Guide

## Overview

In the GridTokenX blockchain, tokens are created through several mechanisms tied directly to energy production and trading. The system follows a **1:1 ratio** where **1 kWh = 1 Token**, creating stable token economics directly tied to real energy value.

## Token Creation Mechanisms

### 1. Genesis Token Minting

The initial token supply is created through genesis blocks when the blockchain is initialized:

```rust
// Genesis mint transaction for initial token supply
let genesis_tx = Transaction::new_genesis_mint(
    "authority_address".to_string(),
    1_000_000, // 1 million tokens
    "Initial token allocation for energy authorities".to_string()
)?;
```

**Who can create genesis tokens:**
- System initialization only
- Thai energy authorities (EGAT, MEA, PEA, ERC)
- One-time allocation during network launch

### 2. Energy Production Tokens

Tokens are automatically minted when energy is produced and verified:

```rust
// Energy production creates tokens through IoT measurement
let production_tx = Transaction::new(
    TransactionType::EnergyMeasurement {
        device_id: "solar_panel_001".to_string(),
        energy_consumed: 0.0,
        energy_produced: 100.0, // 100 kWh produced = 100 tokens minted
        instantaneous_power: 5.0,
        energy_source: "Solar".to_string(),
        location: "Bangkok_North".to_string(),
        timestamp: Utc::now(),
        quality_metrics: Some(EnergyQualityMetrics::default()),
    },
    "producer_address".to_string(),
    None,
    0, // No fee for production
    nonce,
)?;
```

**Energy-to-Token Conversion:**
```rust
/// Convert energy to tokens (1 kWh = 1,000,000 micro-tokens for precision)
pub fn energy_to_tokens(kwh: f64) -> u64 {
    (kwh * 1_000_000.0) as u64
}

/// Convert tokens back to energy
pub fn tokens_to_energy(tokens: u64) -> f64 {
    tokens as f64 / 1_000_000.0
}
```

### 3. Carbon Credit Bonus Tokens

Additional tokens are minted based on renewable energy production:

```rust
/// Calculate carbon credits and bonus tokens for renewable energy
pub fn calculate_carbon_credits(energy_kwh: f64, source_type: &str) -> f64 {
    match source_type.to_lowercase().as_str() {
        "solar" => energy_kwh * 0.5,      // 0.5 bonus tokens per kWh
        "wind" => energy_kwh * 0.6,       // 0.6 bonus tokens per kWh  
        "hydro" => energy_kwh * 0.4,      // 0.4 bonus tokens per kWh
        "biomass" => energy_kwh * 0.3,    // 0.3 bonus tokens per kWh
        "geothermal" => energy_kwh * 0.7, // 0.7 bonus tokens per kWh
        _ => 0.0,                         // No bonus for non-renewable
    }
}
```

### 4. Authority Mining Rewards

Validator authorities receive tokens for block validation:

```rust
// Block validation rewards for PoA authorities
let validation_reward = BlockReward {
    validator_address: "authority_001".to_string(),
    block_reward: 50, // 50 tokens per block
    consensus_bonus: 10, // 10 tokens for consensus participation
    uptime_bonus: 5, // 5 tokens for high availability
};
```

## Token Allocation Examples

### Example 1: Solar Farm Token Creation

```rust
use gridtokenx_blockchain::{Transaction, TransactionType, EnergySource};

// Solar farm produces 1000 kWh
let solar_production = Transaction::new(
    TransactionType::EnergyMeasurement {
        device_id: "solar_farm_bangkok_001".to_string(),
        energy_consumed: 0.0,
        energy_produced: 1000.0, // 1000 kWh
        instantaneous_power: 50.0,
        energy_source: "Solar".to_string(),
        location: "Bangkok_North_Grid".to_string(),
        timestamp: Utc::now(),
        quality_metrics: Some(EnergyQualityMetrics {
            frequency: 50.0,
            voltage: 22000.0,
            power_factor: 0.95,
            thd: 2.0,
            reliability_score: 98,
        }),
    },
    "solar_farm_001".to_string(),
    None,
    0,
    1,
)?;

// Results in:
// - 1,000 base tokens (1 token per kWh)
// - 500 carbon credit bonus tokens (0.5 per kWh for solar)
// - Total: 1,500 tokens created
```

### Example 2: Energy Trading Token Transfer

```rust
// Energy sell order - transfers existing tokens
let sell_order = Transaction::new_energy_trade(
    "producer_address".to_string(),
    "buyer_address".to_string(),
    EnergyTransaction {
        energy_amount: 100.0,
        price_per_kwh: 3500, // 3.5 tokens per kWh
        total_value: 350000, // 350,000 micro-tokens (350 tokens)
        energy_source: EnergySource::Solar,
        delivery_window: DeliveryWindow {
            start_time: Utc::now() + Duration::hours(1),
            end_time: Utc::now() + Duration::hours(2),
            flexibility_minutes: 15,
        },
        grid_location: GridLocation {
            province_code: "BKK".to_string(),
            distribution_area: "MEA_01".to_string(),
            substation_id: "SUB_001".to_string(),
            voltage_level: 22.0,
            coordinates: Some((13.7563, 100.5018)),
        },
        carbon_credits: 50.0,
        quality_metrics: EnergyQualityMetrics::default(),
        compliance_data: ComplianceData::default(),
        order_type: EnergyOrderType::Sell,
    },
    100, // Transaction fee
    2,
)?;
```

## Token Supply Management

### Current Configuration

```toml
# config.toml
[energy]
# Energy to token conversion rate (1 kWh = 1 Token)
energy_token_ratio = 1.0
# Minimum trade amount in kWh
min_trade_amount = 0.1
# Maximum trade amount in kWh  
max_trade_amount = 10000.0

[consensus.rewards]
# Block validation reward
block_reward = 50
# Consensus participation bonus
consensus_bonus = 10
# Authority uptime bonus
uptime_bonus = 5
```

### Supply Economics

1. **Base Token Supply**: Created through energy production
2. **Carbon Credit Multiplier**: Bonus tokens for renewable energy
3. **Validator Rewards**: Tokens for network security
4. **Transaction Fees**: Tokens burned or redistributed

### Energy Source Token Multipliers

| Energy Source | Base Tokens | Carbon Bonus | Total per kWh |
|---------------|-------------|--------------|---------------|
| Solar         | 1.0         | 0.5          | 1.5 tokens   |
| Wind          | 1.0         | 0.6          | 1.6 tokens   |
| Hydro         | 1.0         | 0.4          | 1.4 tokens   |
| Biomass       | 1.0         | 0.3          | 1.3 tokens   |
| Geothermal    | 1.0         | 0.7          | 1.7 tokens   |
| Natural Gas   | 1.0         | 0.0          | 1.0 tokens   |
| Coal          | 1.0         | 0.0          | 1.0 tokens   |

## API Usage for Token Creation

### Creating Energy Production Transaction

```bash
# Submit energy production data (creates tokens)
curl -X POST http://localhost:8080/api/v1/energy/production \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "solar_panel_001",
    "energy_produced": 50.0,
    "energy_source": "solar",
    "location": "Bangkok_North",
    "quality_metrics": {
      "frequency": 50.0,
      "voltage": 22000.0,
      "power_factor": 0.95
    }
  }'
```

### Checking Token Balance

```bash
# Get account balance
curl http://localhost:8080/api/v1/accounts/{address}/balance
```

### Energy Trading (Token Transfer)

```bash
# Submit energy sell order
curl -X POST http://localhost:8080/api/v1/energy/orders \
  -H "Content-Type: application/json" \
  -d '{
    "order_type": "sell",
    "energy_amount": 100.0,
    "price_per_kwh": 3500,
    "energy_source": "solar",
    "grid_location": "BKK-01-SUB001",
    "expiration_hours": 24
  }'
```

## Key Features

### 1. Real Energy Backing
- Every token represents real energy production
- 1:1 ratio maintains stable value
- Verified through IoT devices and smart meters

### 2. Renewable Energy Incentives
- Bonus tokens for renewable sources
- Carbon credit integration
- Environmental compliance rewards

### 3. Authority Validation
- Thai energy authorities validate production
- Regulatory compliance built-in
- Grid stability requirements

### 4. Transparent Supply
- All token creation is auditable
- Energy production data on blockchain
- Real-time supply tracking

## Security and Validation

### Production Verification
- IoT device authentication
- Smart meter integration
- Grid operator validation
- Quality metrics verification

### Token Integrity
- Cryptographic signatures (Ed25519)
- Multi-signature validation for large amounts
- Authority oversight for compliance
- Automatic auditing and reporting

This token creation system ensures that GridTokenX tokens are directly backed by real energy production while incentivizing renewable energy adoption and maintaining regulatory compliance with Thai energy market requirements.
