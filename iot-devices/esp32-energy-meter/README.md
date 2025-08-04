# GridTokenX ESP32 Smart Energy Meter - POC Prototype

## 🌟 Overview

This ESP32-based IoT device serves as a **Proof-of-Concept (POC) prototype** for integrating smart energy meters with the GridTokenX blockchain platform. The device demonstrates real-world IoT integration with blockchain technology for Thailand's energy market.

### 🎯 Key Features

- **📊 Energy Consumption Monitoring**: Real-time power usage and energy consumption tracking
- **⛓️ Blockchain Integration**: Direct submission of energy data to GridTokenX blockchain via REST API
- **📱 Real-time Display**: OLED screen showing energy metrics, blockchain status, and trading information
- **🤖 Automated Trading**: Automated energy trading based on consumption patterns and grid conditions
- **🔌 Grid Quality Monitoring**: Monitor frequency, voltage, and power quality parameters
- **🌿 Carbon Credit Tracking**: Automatic calculation of renewable energy carbon credits
- **🔐 Secure Communication**: Encrypted communication with blockchain nodes
- **📡 IoT Device Management**: Remote monitoring and configuration capabilities

## 🛠️ Hardware Requirements

### Essential Components
- **ESP32 DevKit v1** - Main microcontroller with WiFi/Bluetooth
- **SSD1306 OLED Display (128x64)** - Status and metrics display
- **DHT22 Temperature/Humidity Sensor** - Environmental monitoring for grid quality
- **Breadboard and Jumper Wires** - Prototyping connections
- **Power Supply (5V/3.3V)** - Stable power source

### Advanced Components (Production Ready)
- **ACS712 Current Sensor (20A)** - Accurate current measurement
- **ZMPT101B Voltage Sensor** - AC voltage measurement (220V → 3.3V safely)
- **MCP3008 ADC** - High-resolution analog-to-digital conversion
- **DS3231 RTC Module** - Precise timekeeping for energy billing
- **MicroSD Card Module** - Local data storage and backup
- **ESP32-CAM** - Visual monitoring and QR code scanning
- **Relay Module** - Smart switch functionality

### Status Indicators
- **RGB LED (WS2812)** - Multi-color status indication
- **Buzzer** - Audio alerts for grid anomalies
- **Push Buttons (x3)** - Manual control and menu navigation
- **Rotary Encoder** - Settings adjustment

## 🔌 Wiring Diagram

```
ESP32 DevKit v1 Pin Configuration:
┌─────────────────────────────────────────────────────────────┐
│                    ESP32 DevKit v1                         │
├─────────────────────────────────────────────────────────────┤
│ Power & Ground:                                             │
│ ├── 3.3V → OLED VCC, DHT22 VCC, Sensors VCC               │
│ ├── 5V   → ACS712 VCC, Relay VCC                          │
│ ├── GND  → All component grounds                           │
│                                                             │
│ I2C Communication (Display):                                │
│ ├── GPIO 21 (SDA) → OLED SDA                              │
│ ├── GPIO 22 (SCL) → OLED SCL                              │
│                                                             │
│ Analog Sensors:                                             │
│ ├── GPIO 36 (A0) → ACS712 Current Sensor Output           │
│ ├── GPIO 39 (A3) → ZMPT101B Voltage Sensor Output         │
│ ├── GPIO 34 (A6) → Light Sensor (for environmental data)  │
│                                                             │
│ Digital Sensors & Communication:                            │
│ ├── GPIO 4  → DHT22 Data Pin                              │
│ ├── GPIO 16 → RX2 (Serial communication)                  │
│ ├── GPIO 17 → TX2 (Serial communication)                  │
│                                                             │
│ Status LEDs & Indicators:                                   │
│ ├── GPIO 2  → Built-in LED (WiFi status)                  │
│ ├── GPIO 12 → RGB LED Data (WS2812)                       │
│ ├── GPIO 13 → Status LED (Blockchain sync)                │
│ ├── GPIO 14 → Error LED (Red)                             │
│ ├── GPIO 27 → Buzzer (Grid alerts)                        │
│                                                             │
│ User Interface:                                             │
│ ├── GPIO 0  → BOOT Button (Reset/Menu)                    │
│ ├── GPIO 15 → Menu Button                                 │
│ ├── GPIO 32 → Select Button                               │
│ ├── GPIO 33 → Back Button                                 │
│                                                             │
│ Storage & Expansion:                                        │
│ ├── GPIO 18 → SD Card SCK                                 │
│ ├── GPIO 19 → SD Card MISO                                │
│ ├── GPIO 23 → SD Card MOSI                                │
│ ├── GPIO 5  → SD Card CS                                  │
│                                                             │
│ Smart Switch Control (Optional):                            │
│ ├── GPIO 25 → Relay Control 1 (Load switching)           │
│ ├── GPIO 26 → Relay Control 2 (Backup switching)         │
└─────────────────────────────────────────────────────────────┘

GridTokenX Blockchain Integration:
┌─────────────────────────────────────────────────────────────┐
│                    Network Architecture                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ ESP32 Smart Meter ←→ WiFi Router ←→ Internet               │
│         │                                     │             │
│         ↓                                     ↓             │
│   Local Storage                     GridTokenX Blockchain   │
│   (SD Card)                              API Endpoints      │
│         │                                     │             │
│         ↓                                     ↓             │
│   Backup Data               ┌─────────────────────────────┐ │
│   Configuration             │ Blockchain Operations:       │ │
│   Energy Logs               │ • Submit energy readings     │ │
│                             │ • Query energy orders       │ │
│                             │ • Monitor grid status       │ │
│                             │ • Execute automated trades  │ │
│                             │ • Update carbon credits     │ │
│                             └─────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start Guide

### Step 1: Hardware Assembly
1. Connect ESP32 to breadboard
2. Wire OLED display (I2C: SDA=21, SCL=22)
3. Connect DHT22 sensor (Data=GPIO4)
4. Add current sensor (ACS712 → GPIO36)
5. Connect status LEDs and buttons
6. Power up with 5V supply

### Step 2: Software Setup
```bash
# Install PlatformIO
pip install platformio

# Clone and setup project
git clone [your-repo-url]
cd blockchain/iot-devices/esp32-energy-meter

# Configure WiFi and blockchain settings
cp include/config.h.example include/config.h
# Edit config.h with your settings

# Build and upload
pio run --target upload
pio device monitor
```

### Step 3: GridTokenX Integration
1. Configure blockchain node endpoint in `config.h`
2. Set up device credentials and certificates
3. Register device with GridTokenX platform
4. Start energy monitoring and trading

## 📊 Energy Monitoring Features

### Real-time Measurements
- **Current (A)**: Instantaneous current draw
- **Voltage (V)**: Line voltage monitoring  
- **Power (W)**: Real power consumption
- **Energy (kWh)**: Cumulative energy consumption
- **Power Factor**: Power quality measurement
- **Frequency (Hz)**: Grid frequency monitoring

### Environmental Monitoring
- **Temperature (°C)**: Ambient temperature
- **Humidity (%)**: Relative humidity
- **Light Level**: Environmental light sensor

### Grid Quality Metrics
- **Voltage Stability**: Monitoring voltage fluctuations
- **Frequency Stability**: Grid frequency variations
- **Power Factor**: Reactive power monitoring
- **Harmonic Distortion**: Power quality analysis

## ⛓️ Blockchain Integration

### API Endpoints Integration
```cpp
// GridTokenX API Integration Examples
POST /api/v1/energy/readings     // Submit energy measurements
GET  /api/v1/energy/orders       // Query available energy orders
POST /api/v1/energy/orders       // Place energy trade orders
GET  /api/v1/grid/status         // Get grid status information
POST /api/v1/iot/register        // Register IoT device
GET  /api/v1/accounts/{address}  // Get account information
```

### Automated Trading Logic
```cpp
// Pseudo-code for automated trading
if (energyProduced > energyConsumed) {
    submitSellOrder(excessEnergy, currentMarketPrice);
} else if (energyDemand > threshold) {
    submitBuyOrder(requiredEnergy, maxPrice);
}
```

### Security Features
- **TLS/SSL Communication**: Secure API communication
- **Device Authentication**: X.509 certificates
- **Data Integrity**: Cryptographic signatures
- **Local Backup**: Offline operation capability
    └── Reset Button → GPIO 25
```

## Software Setup

### 1. Install PlatformIO

```bash
# Install PlatformIO using pip
pip install platformio

# Or install PlatformIO IDE in VS Code
# Search for "PlatformIO IDE" in VS Code extensions
```

### 2. Clone and Build

```bash
# Navigate to the ESP32 project directory
cd /Users/chanthawat/Development/blockchain/iot-devices/esp32-energy-meter

# Build the project
pio run

# Upload to ESP32 (connect ESP32 via USB)
pio run --target upload

# Monitor serial output
pio device monitor
```

### 3. Configuration

Edit the configuration in `include/config.h`:

```cpp
// WiFi Configuration
#define WIFI_SSID "Your_WiFi_Network"
#define WIFI_PASSWORD "Your_WiFi_Password"

// GridTokenX Node Configuration
#define BLOCKCHAIN_API_HOST "192.168.1.100"  // Your node IP
#define BLOCKCHAIN_API_PORT 8080

// Device Configuration
#define DEVICE_ID "ESP32-ENERGY-METER-001"
#define DEVICE_LOCATION "13.7563,100.5018"   // Bangkok coordinates
#define DEVICE_ZONE "MEA-BANGKOK-ZONE-1"
```

## GridTokenX Blockchain Integration

### 1. Start GridTokenX Node

Ensure your GridTokenX blockchain node is running:

```bash
# Navigate to main blockchain directory
cd /Users/chanthawat/Development/blockchain

# Run the blockchain node
cargo run --release --bin gridtokenx-node
```

### 2. ESP32 API Endpoints

The ESP32 communicates with these GridTokenX API endpoints:

```http
POST /api/v1/energy/meter-reading     # Submit energy readings
POST /api/v1/energy/orders            # Submit trade orders
GET  /api/v1/energy/pricing           # Get current energy prices
GET  /api/v1/accounts/{id}/balance    # Get account balance
POST /api/v1/devices/register         # Register device
GET  /api/v1/status                   # Check node status
```

### 3. Data Flow

```
ESP32 Energy Meter
       ↓
   WiFi Network
       ↓
GridTokenX REST API
       ↓
   Blockchain Node
       ↓
  GridTokenX Network
```

## Features

### ✅ Current Features

1. **Energy Monitoring**
   - Real-time power consumption measurement
   - Energy accumulation (kWh tracking)
   - Temperature and humidity monitoring
   - Grid voltage and frequency simulation

2. **Blockchain Integration**
   - Automatic data submission every 5 minutes
   - Device registration with blockchain
   - Digital signature for data integrity
   - Real-time blockchain connectivity status

3. **User Interface**
   - OLED display with real-time metrics
   - WiFi and blockchain connection status
   - Manual sync button
   - Status LED indicators

4. **Security Features**
   - SHA256-based device signatures
   - Encrypted communication with HTTPS support
   - Device ID-based authentication
   - Data validation and integrity checks

### 🔄 Planned Features

1. **Enhanced Energy Trading**
   - Automatic buy/sell order placement
   - Smart contract integration
   - Price optimization algorithms
   - Carbon credit calculations

2. **Advanced Grid Monitoring**
   - Power quality analysis (THD, harmonics)
   - Voltage sag/swell detection
   - Frequency monitoring
   - Grid stability reporting

3. **IoT Network Integration**
   - MQTT communication protocol
   - Multiple device mesh networking
   - Over-the-air (OTA) firmware updates
   - Remote configuration management

## Usage Instructions

### Initial Setup

1. **Power On**: Connect ESP32 to power source
2. **WiFi Connection**: Device will automatically connect to configured WiFi
3. **Blockchain Registration**: Device registers itself with GridTokenX network
4. **Status Check**: Verify connectivity on OLED display

### Normal Operation

- **Automatic Mode**: Device continuously monitors energy and syncs with blockchain
- **Manual Sync**: Press sync button for immediate data transmission
- **Status Monitoring**: Check OLED display for real-time metrics

### Display Information

```
GridTokenX Meter
WiFi: OK    Chain: OK
Energy: 12.45 kWh
Power: 1250 W
Temp: 28.5°C
Humid: 65%
```

### LED Indicators

- **Status LED (Green)**: 
  - Solid: Normal operation
  - Blinking: Data transmission
- **Error LED (Red)**:
  - Solid: Critical error
  - Blinking: Network issues
- **Network LED (Blue)**:
  - Solid: Connected to blockchain
  - Off: Disconnected

## Troubleshooting

### Common Issues

1. **WiFi Connection Failed**
   ```
   Solution: Check SSID/password in config.h
   Reset: Hold reset button for 5 seconds
   ```

2. **Blockchain Connection Failed**
   ```
   Solution: Verify GridTokenX node is running
   Check: Node IP address and port configuration
   ```

3. **Sensor Reading Errors**
   ```
   Solution: Check sensor wiring
   Verify: DHT22 connections and power supply
   ```

4. **Display Not Working**
   ```
   Solution: Check I2C connections (SDA/SCL)
   Verify: OLED display address (0x3C)
   ```

### Debug Mode

Enable detailed logging by setting in `config.h`:
```cpp
#define DEBUG_ENABLED true
#define DEBUG_LOG_LEVEL 3
```

Monitor serial output:
```bash
pio device monitor --baud 115200
```

## Testing Scenarios

### 1. Basic Connectivity Test
```bash
# Check device registration
curl http://localhost:8080/api/v1/devices/ESP32-ENERGY-METER-001

# Verify energy readings
curl http://localhost:8080/api/v1/energy/meter-readings?device_id=ESP32-ENERGY-METER-001
```

### 2. Energy Trading Simulation
```bash
# Submit test energy reading
curl -X POST http://localhost:8080/api/v1/energy/meter-reading \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "ESP32-ENERGY-METER-001",
    "energy_consumed": 10.5,
    "current_power": 1200,
    "timestamp": "2025-08-03T10:30:00Z"
  }'
```

### 3. Load Testing
- Simulate multiple ESP32 devices
- Test concurrent data submissions
- Monitor blockchain performance

## Production Deployment

### Security Hardening
1. **Change Default Keys**: Update device private keys
2. **Enable HTTPS**: Use SSL/TLS for API communication
3. **Firewall Rules**: Restrict network access
4. **Regular Updates**: Keep firmware updated

### Scalability Considerations
1. **Device Management**: Implement device fleet management
2. **Data Storage**: Plan for large-scale data storage
3. **Network Bandwidth**: Monitor and optimize data transmission
4. **Blockchain Scaling**: Ensure blockchain can handle device load

## Performance Metrics

### Expected Performance
- **Data Transmission**: 5-minute intervals
- **Response Time**: < 2 seconds per API call
- **Power Consumption**: ~150mA average
- **Memory Usage**: ~200KB flash, ~50KB RAM
- **Uptime**: 99%+ with stable WiFi

### Monitoring
- Device uptime tracking
- Data transmission success rate
- Blockchain sync performance
- Network connectivity statistics

## Future Enhancements

### Phase 1: Enhanced Measurements
- Real current/voltage sensors
- Power quality analysis
- Grid stability monitoring

### Phase 2: Smart Trading
- Automated energy trading algorithms
- Machine learning price optimization
- Carbon credit marketplace integration

### Phase 3: Grid Integration
- SCADA system integration
- Smart grid communication protocols
- Demand response capabilities

### Phase 4: Scale Deployment
- Mass production considerations
- Enterprise device management
- Regulatory compliance (Thai standards)

## Support and Documentation

### Additional Resources
- [GridTokenX Main Documentation](../docs/README.md)
- [API Reference](../docs/api.md)
- [Energy Trading Guide](../docs/energy-trading.md)
- [Thai Grid Integration](../docs/thai-grid-integration.md)

### Community Support
- **GitHub Issues**: Report bugs and feature requests
- **Discord**: Join GridTokenX community
- **Email**: support@gridtokenx.com

---

**GridTokenX ESP32 Energy Meter** - IoT + Blockchain for Thailand's Energy Future 🇹🇭⚡🔗
