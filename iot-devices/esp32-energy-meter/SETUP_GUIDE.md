# GridTokenX ESP32 Setup and Installation Guide

## 📋 Prerequisites

### Software Requirements
- **PlatformIO IDE** (VS Code extension) or **Arduino IDE 2.0+**
- **ESP32 Board Package** version 2.0.0 or later
- **Git** for version control
- **GridTokenX Blockchain Node** running locally or accessible remotely

### Hardware Requirements
- **ESP32 DevKit v1** - Main microcontroller
- **SSD1306 OLED Display (128x64)** - I2C interface
- **DHT22 Temperature/Humidity Sensor**
- **ACS712 Current Sensor (20A)** - Optional for real measurements
- **ZMPT101B Voltage Sensor** - Optional for real measurements
- **Breadboard and Jumper Wires**
- **Power Supply (5V/3.3V)**
- **MicroSD Card Module** - Optional for data logging
- **RGB LED (WS2812)** - Optional for status indication

## 🛠️ Installation Steps

### Step 1: Install Development Environment

#### Option A: PlatformIO (Recommended)
```bash
# Install PlatformIO Core
pip install platformio

# Or install VS Code with PlatformIO extension
code --install-extension platformio.platformio-ide
```

#### Option B: Arduino IDE
1. Download and install Arduino IDE 2.0+ from [arduino.cc](https://www.arduino.cc/en/software)
2. Install ESP32 board package via Board Manager
3. Install required libraries manually

### Step 2: Clone the Project
```bash
git clone https://github.com/your-org/gridtokenx-blockchain.git
cd gridtokenx-blockchain/iot-devices/esp32-energy-meter
```

### Step 3: Hardware Assembly

#### Basic Wiring (Essential Components)
```
ESP32 DevKit v1 → Component Connections:

Power Connections:
├── 3.3V → OLED VCC, DHT22 VCC
├── 5V   → ACS712 VCC (if using)
├── GND  → All component grounds

I2C (OLED Display):
├── GPIO 21 (SDA) → OLED SDA
├── GPIO 22 (SCL) → OLED SCL

Sensors:
├── GPIO 4  → DHT22 Data Pin
├── GPIO 36 → ACS712 Output (A0)
├── GPIO 39 → ZMPT101B Output (A3)

Status LEDs:
├── GPIO 2  → Built-in LED
├── GPIO 12 → RGB LED Data (WS2812)
├── GPIO 13 → Status LED (Green)
├── GPIO 14 → Error LED (Red)

User Interface:
├── GPIO 0  → Boot/Menu Button
├── GPIO 25 → Menu Button
├── GPIO 32 → Select Button
├── GPIO 33 → Back Button
```

### Step 4: Configuration

#### Configure WiFi and Blockchain Settings
```cpp
// Edit include/config.h
#define WIFI_SSID "YourWiFiNetwork"
#define WIFI_PASSWORD "YourWiFiPassword"
#define BLOCKCHAIN_API_HOST "192.168.1.100"  // Your GridTokenX node IP
#define BLOCKCHAIN_API_PORT 8080
#define DEVICE_ID "ESP32_METER_001"
#define API_KEY "your-gridtokenx-api-key-here"
```

#### Device Location and Zone
```cpp
#define DEVICE_LOCATION "13.7563,100.5018"   // Bangkok coordinates
#define DEVICE_ZONE "MEA-BANGKOK-ZONE-1"     // Grid operator zone
```

### Step 5: Build and Upload

#### Using PlatformIO
```bash
# Build the project
pio run

# Upload to ESP32
pio run --target upload

# Open serial monitor
pio device monitor
```

#### Using Arduino IDE
1. Open `src/main.cpp` in Arduino IDE
2. Select **ESP32 Dev Module** from Tools → Board
3. Select correct COM port from Tools → Port
4. Click **Upload** button
5. Open Serial Monitor (115200 baud)

### Step 6: GridTokenX Blockchain Setup

#### Start GridTokenX Node
```bash
# Navigate to blockchain directory
cd ../../

# Start the blockchain node
cargo run --release --bin gridtokenx-node

# Verify API is accessible
curl http://localhost:8080/api/v1/status
```

#### Register Device with Blockchain
The ESP32 will automatically attempt to register with the blockchain on first startup. Monitor the serial output for registration status.

## 🔧 Configuration Options

### Network Settings
```cpp
// Advanced WiFi configuration
#define WIFI_TIMEOUT_MS 10000
#define WIFI_RETRY_ATTEMPTS 3
#define WIFI_RECONNECT_INTERVAL_MS 30000
```

### Energy Monitoring
```cpp
// Sensor calibration
#define ACS712_SENSITIVITY 100.0      // mV/A for 20A sensor
#define VOLTAGE_SENSOR_RATIO 0.00322  // Conversion factor
#define POWER_FACTOR_DEFAULT 0.95
```

### Automated Trading
```cpp
// Trading thresholds
#define EXCESS_ENERGY_THRESHOLD 1.0    // kWh
#define ENERGY_DEMAND_THRESHOLD 0.5    // kWh
#define MAX_TRADING_PRICE 6000         // THB per kWh
#define MIN_TRADING_PRICE 3000         // THB per kWh
```

### Display Settings
```cpp
// Display configuration
#define SCREEN_PAGES 6
#define SCREEN_PAGE_DURATION_MS 5000   // Auto-advance
#define SCREEN_BRIGHTNESS 128          // 0-255
```

## 🧪 Testing and Validation

### 1. Basic Connectivity Test
```bash
# Check serial output for:
# - WiFi connection status
# - Blockchain API connection
# - Device registration confirmation
```

### 2. Sensor Reading Test
Monitor serial output for:
- Voltage and current readings
- Power calculations
- Temperature and humidity
- Data validation status

### 3. Blockchain Integration Test
```bash
# Use curl to verify data submission
curl -X GET http://localhost:8080/api/v1/energy/readings \
  -H "Authorization: Bearer your-api-key"
```

### 4. Display Function Test
- Navigate through display pages using buttons
- Verify all information is displayed correctly
- Check status LED indicators

## 🐛 Troubleshooting

### Common Issues and Solutions

#### WiFi Connection Problems
```
Issue: ESP32 fails to connect to WiFi
Solutions:
1. Verify SSID and password in config.h
2. Check WiFi signal strength (minimum -80 dBm)
3. Ensure 2.4GHz network (ESP32 doesn't support 5GHz)
4. Check for special characters in WiFi credentials
```

#### Blockchain API Connection Issues
```
Issue: Cannot connect to GridTokenX API
Solutions:
1. Verify blockchain node is running: curl http://localhost:8080/api/v1/status
2. Check firewall settings on blockchain host
3. Verify IP address and port in config.h
4. Check API key configuration
```

#### Sensor Reading Issues
```
Issue: Invalid or erratic sensor readings
Solutions:
1. Check wiring connections
2. Verify power supply stability (5V ±5%)
3. Calibrate sensors using known reference values
4. Check for electromagnetic interference
```

#### Display Problems
```
Issue: OLED display not working
Solutions:
1. Verify I2C wiring (SDA=21, SCL=22)
2. Check display address (usually 0x3C or 0x3D)
3. Ensure adequate power supply
4. Test with I2C scanner sketch
```

### Debug Mode
Enable debug output by modifying config.h:
```cpp
#define DEBUG_LEVEL 4  // 0=None, 1=Error, 2=Warn, 3=Info, 4=Debug
#define ENABLE_SERIAL_OUTPUT true
```

## 📊 Monitoring and Maintenance

### Serial Monitor Output
Monitor the following key information:
- **Energy Readings**: Voltage, current, power, energy
- **Blockchain Status**: Connection, sync time, transaction status
- **Network Health**: WiFi signal, API response times
- **System Health**: Memory usage, uptime, error counts

### Web Interface
Access device configuration via web browser:
```
http://[ESP32_IP_ADDRESS]/
Default credentials: admin / gridtokenx
```

### Over-the-Air Updates
Enable OTA updates in config.h:
```cpp
#define ENABLE_OTA_UPDATES true
#define OTA_PASSWORD "gridtokenx-ota"
```

## 🔐 Security Considerations

### Network Security
- Use WPA2/WPA3 encrypted WiFi networks
- Configure firewall rules for blockchain API access
- Use HTTPS/TLS for API communication where possible

### Device Security
- Change default OTA password
- Regularly update firmware
- Monitor for unusual network activity
- Use strong API keys

### Data Protection
- Enable local data encryption if storing sensitive information
- Implement secure key management
- Regular security audits and updates

## 📈 Performance Optimization

### Memory Management
- Monitor heap memory usage
- Optimize JSON document sizes
- Use PROGMEM for constant strings
- Implement proper task stack sizes

### Power Consumption
- Implement sleep modes for battery operation
- Optimize sensor reading intervals
- Use efficient display update strategies
- Monitor current consumption

### Network Optimization
- Implement exponential backoff for retries
- Batch API requests where possible
- Use compression for large data transfers
- Monitor and optimize payload sizes

## 🔄 Maintenance Schedule

### Daily Checks
- Monitor system health via serial output
- Check blockchain synchronization status
- Verify energy readings accuracy

### Weekly Maintenance
- Review error logs and statistics
- Check for firmware updates
- Validate trading performance
- Monitor network connectivity

### Monthly Tasks
- Sensor calibration verification
- Security audit and updates
- Performance optimization review
- Backup configuration and data

## 📞 Support and Resources

### Documentation
- [GridTokenX Blockchain Documentation](../../../docs/README.md)
- [ESP32 Technical Reference](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/)
- [PlatformIO Documentation](https://docs.platformio.org/)

### Community Support
- GitHub Issues: Report bugs and request features
- Discord Community: Real-time support and discussions
- Technical Blog: Latest updates and tutorials

### Professional Support
For enterprise deployments and custom integrations, contact the GridTokenX development team for professional support services.

---

**GridTokenX ESP32 Smart Energy Meter** - Powering the Future of Energy Trading 🇹🇭⚡
