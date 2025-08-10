#!/bin/bash

# GridTokenX Bonus Token Configuration Test Script
# This script demonstrates how to configure and test bonus tokens

echo "🎁 GridTokenX Bonus Token Configuration Test"
echo "============================================"

NODE_URL="http://localhost:8080/api/v1"
CONFIG_FILE="config.toml"

echo ""
echo "📋 Current Bonus Token Configuration:"
echo "-----------------------------------"

echo "🌱 Renewable Energy Bonuses:"
echo "  Solar:      +0.5 tokens/kWh (Total: 1.5x)"
echo "  Wind:       +0.6 tokens/kWh (Total: 1.6x)"
echo "  Hydro:      +0.4 tokens/kWh (Total: 1.4x)"
echo "  Biomass:    +0.3 tokens/kWh (Total: 1.3x)"
echo "  Geothermal: +0.7 tokens/kWh (Total: 1.7x)"

echo ""
echo "⏰ Time-Based Bonuses:"
echo "  Peak Hours:  +0.2 tokens/kWh (6PM-10PM)"
echo "  Off-Peak:    +0.1 tokens/kWh"
echo "  Weekend:     +0.05 tokens/kWh"
echo "  Holiday:     +0.1 tokens/kWh"

echo ""
echo "⚡ Grid Stability Bonuses:"
echo "  Frequency:   +0.1 tokens/kWh"
echo "  Voltage:     +0.1 tokens/kWh"
echo "  Load Balance:+0.15 tokens/kWh"
echo "  Emergency:   +0.5 tokens/kWh"

echo ""
echo "🏆 Quality Bonuses:"
echo "  High Quality:+0.1 tokens/kWh (PF>0.95, THD<3%)"
echo "  Med Quality: +0.05 tokens/kWh (PF>0.90, THD<5%)"

echo ""
echo "📍 Regional Multipliers:"
echo "  Bangkok:     1.0x (base)"
echo "  Central:     1.1x (+10%)"
echo "  Northern:    1.2x (+20%)"
echo "  Northeast:   1.3x (+30%)"
echo "  Southern:    1.15x (+15%)"

echo ""
echo "💰 Volume Tier Bonuses:"
echo "  Tier 1 (0-100 kWh):     +2%"
echo "  Tier 2 (100-1000 kWh):  +5%"
echo "  Tier 3 (1000+ kWh):     +10%"

echo ""
echo "🏛️ Validator Bonuses:"
echo "  Block Reward:        50,000 tokens"
echo "  Consensus Bonus:     10,000 tokens"
echo "  Uptime Bonus:        5,000 tokens"
echo "  Grid Stability:      15,000 tokens"
echo "  Energy Verification: 100 tokens/tx"
echo "  Governance:          2,000 tokens"

echo ""
echo "🧮 Bonus Token Calculation Examples:"
echo "===================================="

echo ""
echo "Example 1: Solar Farm (Northern Thailand, Peak Hours, High Quality)"
echo "----------------------------------------------------------------"
echo "Base Production: 1,000 kWh"
echo ""
echo "Calculations:"
echo "  Base tokens:          1,000 × 1.0    = 1,000 tokens"
echo "  Solar bonus:          1,000 × 0.5    = 500 tokens"
echo "  Peak hours bonus:     1,000 × 0.2    = 200 tokens"
echo "  High quality bonus:   1,000 × 0.1    = 100 tokens"
echo "  Subtotal:                              1,800 tokens"
echo "  Northern multiplier:  1,800 × 1.2    = 2,160 tokens"
echo ""
echo "  🎯 TOTAL TOKENS: 2,160 tokens (2.16x multiplier)"

echo ""
echo "Example 2: Wind Farm (Grid Stability Services)"
echo "--------------------------------------------"
echo "Base Production: 500 kWh"
echo ""
echo "Calculations:"
echo "  Base tokens:          500 × 1.0      = 500 tokens"
echo "  Wind bonus:           500 × 0.6      = 300 tokens"
echo "  Frequency stability:  500 × 0.1      = 50 tokens"
echo "  Voltage regulation:   500 × 0.1      = 50 tokens"
echo ""
echo "  🎯 TOTAL TOKENS: 900 tokens (1.8x multiplier)"

echo ""
echo "Example 3: Large Geothermal Plant (Weekend, High Volume)"
echo "-------------------------------------------------------"
echo "Base Production: 5,000 kWh"
echo ""
echo "Calculations:"
echo "  Base tokens:          5,000 × 1.0    = 5,000 tokens"
echo "  Geothermal bonus:     5,000 × 0.7    = 3,500 tokens"
echo "  Weekend bonus:        5,000 × 0.05   = 250 tokens"
echo "  Volume tier bonus:    8,750 × 0.05   = 437.5 tokens (5%)"
echo ""
echo "  🎯 TOTAL TOKENS: 9,187.5 tokens (1.84x multiplier)"

echo ""
echo "🔧 Configuration File Validation:"
echo "================================"

if [[ -f "$CONFIG_FILE" ]]; then
    echo "✅ Configuration file found: $CONFIG_FILE"
    
    # Check if bonus tokens are enabled
    if grep -q "enabled = true" "$CONFIG_FILE"; then
        echo "✅ Bonus token system is enabled"
    else
        echo "❌ Bonus token system may be disabled"
    fi
    
    # Check renewable multipliers
    echo ""
    echo "📊 Configured Renewable Multipliers:"
    grep -A 10 "\[energy.bonus_tokens.renewable_multipliers\]" "$CONFIG_FILE" 2>/dev/null || echo "⚠️  Renewable multipliers section not found"
    
    # Check carbon credit rates
    echo ""
    echo "🌿 Configured Carbon Credit Rates:"
    grep -A 10 "\[energy.carbon_credits.credit_rates\]" "$CONFIG_FILE" 2>/dev/null || echo "⚠️  Carbon credit rates section not found"
    
else
    echo "❌ Configuration file not found: $CONFIG_FILE"
fi

echo ""
echo "🧪 Testing Bonus Token API (if node is running):"
echo "==============================================="

# Test if the node is running
if curl -s "$NODE_URL/status" > /dev/null 2>&1; then
    echo "✅ Node is running"
    
    echo ""
    echo "📈 Current Token Supply:"
    curl -s "$NODE_URL/tokens/supply" | jq '.' 2>/dev/null || echo "API response not available"
    
    echo ""
    echo "⚙️  Current Bonus Configuration:"
    curl -s "$NODE_URL/config/bonus-rates" | jq '.' 2>/dev/null || echo "Bonus rates API not available"
    
else
    echo "⚠️  Node is not running. Start with: cargo run -- start"
fi

echo ""
echo "🚀 Quick Start Commands:"
echo "======================="
echo ""
echo "1. Update bonus configuration:"
echo "   nano config.toml"
echo ""
echo "2. Start node with new config:"
echo "   cargo run -- start --config config.toml"
echo ""
echo "3. Test energy production with bonuses:"
echo "   curl -X POST $NODE_URL/energy/production \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{"
echo "       \"device_id\": \"solar_panel_001\","
echo "       \"energy_produced\": 100.0,"
echo "       \"energy_source\": \"solar\","
echo "       \"location\": \"Northern_Thailand\","
echo "       \"grid_services\": [\"frequency_regulation\"]"
echo "     }'"
echo ""
echo "4. Check account balance:"
echo "   curl $NODE_URL/accounts/{address}/balance"
echo ""
echo "📚 For detailed configuration guide, see:"
echo "   docs/BONUS_TOKEN_CONFIGURATION.md"
