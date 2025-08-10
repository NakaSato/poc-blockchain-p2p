#!/bin/bash

# GridTokenX Token Creation Examples
# This script demonstrates various ways to create tokens in the GridTokenX system

echo "🌟 GridTokenX Token Creation Examples"
echo "====================================="

# Configuration
NODE_URL="http://localhost:8080/api/v1"
PRIVATE_KEY="your_private_key_here"
DEVICE_ID="solar_panel_001"
PRODUCER_ADDRESS="0x1234567890abcdef1234567890abcdef12345678"

echo ""
echo "1. 🔆 Solar Energy Production (Creates Tokens)"
echo "---------------------------------------------"

# Example 1: Solar energy production creates tokens automatically
curl -X POST "${NODE_URL}/energy/production" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${PRIVATE_KEY}" \
  -d '{
    "device_id": "solar_farm_bangkok_001",
    "energy_produced": 1000.0,
    "energy_consumed": 0.0,
    "energy_source": "solar",
    "location": "Bangkok_North_Grid",
    "instantaneous_power": 50.0,
    "quality_metrics": {
      "frequency": 50.0,
      "voltage": 22000.0,
      "power_factor": 0.95,
      "thd": 2.0,
      "reliability_score": 98
    },
    "grid_location": {
      "province_code": "BKK",
      "distribution_area": "MEA_01", 
      "substation_id": "SUB_001",
      "voltage_level": 22.0,
      "coordinates": [13.7563, 100.5018]
    }
  }' | jq '.'

echo ""
echo "Result: 1,000 base tokens + 500 solar bonus tokens = 1,500 total tokens created"

echo ""
echo "2. 💨 Wind Energy Production"
echo "---------------------------"

curl -X POST "${NODE_URL}/energy/production" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${PRIVATE_KEY}" \
  -d '{
    "device_id": "wind_turbine_001",
    "energy_produced": 500.0,
    "energy_consumed": 0.0,
    "energy_source": "wind",
    "location": "Nakhon_Ratchasima",
    "instantaneous_power": 25.0,
    "quality_metrics": {
      "frequency": 50.1,
      "voltage": 22000.0,
      "power_factor": 0.92,
      "thd": 3.0,
      "reliability_score": 96
    }
  }' | jq '.'

echo ""
echo "Result: 500 base tokens + 300 wind bonus tokens = 800 total tokens created"

echo ""
echo "3. 🔋 Energy Trading (Token Transfer)"
echo "------------------------------------"

# Example: Sell energy order (transfers existing tokens)
curl -X POST "${NODE_URL}/energy/orders" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${PRIVATE_KEY}" \
  -d '{
    "order_type": "sell",
    "trader_address": "'${PRODUCER_ADDRESS}'",
    "energy_amount": 100.0,
    "price_per_kwh": 3500,
    "energy_source": "solar",
    "grid_location": "BKK-01-SUB001",
    "expiration_hours": 24,
    "min_trade_amount": 10.0
  }' | jq '.'

echo ""
echo "Result: 100 kWh offered for sale at 3.5 tokens/kWh (350 tokens total)"

echo ""
echo "4. 🏛️ Authority Token Allocation"
echo "-------------------------------"

# Example: Authority allocation (genesis or governance-approved)
curl -X POST "${NODE_URL}/governance/mint" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${PRIVATE_KEY}" \
  -d '{
    "recipient": "'${PRODUCER_ADDRESS}'",
    "amount": 10000,
    "reason": "Initial allocation for registered solar farm",
    "authority_type": "MEA",
    "approval_reference": "MEA-2025-SF-001"
  }' | jq '.'

echo ""
echo "Result: 10,000 tokens allocated by MEA authority"

echo ""
echo "5. 📊 Check Token Balance"
echo "------------------------"

curl -X GET "${NODE_URL}/accounts/${PRODUCER_ADDRESS}/balance" \
  -H "Authorization: Bearer ${PRIVATE_KEY}" | jq '.'

echo ""
echo "6. 📈 Get Energy Trading Statistics"
echo "----------------------------------"

curl -X GET "${NODE_URL}/energy/stats" | jq '.'

echo ""
echo "7. 🌱 Carbon Credits Report"
echo "--------------------------"

curl -X GET "${NODE_URL}/carbon/credits/${PRODUCER_ADDRESS}" | jq '.'

echo ""
echo "8. ⚡ Grid Status and Token Supply"
echo "---------------------------------"

curl -X GET "${NODE_URL}/grid/status" | jq '.'
curl -X GET "${NODE_URL}/tokens/supply" | jq '.'

echo ""
echo "Token Creation Summary:"
echo "======================="
echo "✅ Base Tokens: 1 token per kWh produced"
echo "✅ Renewable Bonus:"
echo "   - Solar: +0.5 tokens/kWh"
echo "   - Wind: +0.6 tokens/kWh" 
echo "   - Hydro: +0.4 tokens/kWh"
echo "   - Biomass: +0.3 tokens/kWh"
echo "   - Geothermal: +0.7 tokens/kWh"
echo "✅ Authority Allocation: Genesis and governance-approved minting"
echo "✅ Validator Rewards: Block validation and consensus participation"
echo ""
echo "All token creation is backed by real energy production and"
echo "verified by Thai energy authorities (EGAT, MEA, PEA, ERC)."
