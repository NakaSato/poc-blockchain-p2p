/**
 * GridTokenX ESP32 Configuration
 * 
 * Configuration file for the ESP32 Smart Energy Meter
 * Modify these settings according to your deployment environment
 */

#ifndef GRIDTOKENX_CONFIG_H
#define GRIDTOKENX_CONFIG_H

// Network Configuration
#define WIFI_SSID "YOUR_WIFI_SSID"
#define WIFI_PASSWORD "YOUR_WIFI_PASSWORD"
#define WIFI_TIMEOUT_MS 20000

// GridTokenX Blockchain Configuration
#define BLOCKCHAIN_API_HOST "192.168.1.100"  // Your GridTokenX node IP
#define BLOCKCHAIN_API_PORT 8080
#define BLOCKCHAIN_API_PATH "/api/v1"
#define BLOCKCHAIN_SYNC_INTERVAL_MS 300000    // 5 minutes

// Device Configuration
#define DEVICE_ID "ESP32-ENERGY-METER-001"
#define DEVICE_TYPE "smart_meter"
#define DEVICE_LOCATION "13.7563,100.5018"   // Bangkok coordinates
#define DEVICE_ZONE "MEA-BANGKOK-ZONE-1"     // MEA jurisdiction

// Hardware Pin Configuration
#define PIN_OLED_SDA 21
#define PIN_OLED_SCL 22
#define PIN_OLED_RST 16
#define PIN_DHT 4
#define PIN_CURRENT_SENSOR A0
#define PIN_VOLTAGE_SENSOR A3
#define PIN_LED_STATUS 2
#define PIN_LED_ERROR 15
#define PIN_LED_NETWORK 12
#define PIN_BUTTON_SYNC 0
#define PIN_BUTTON_RESET 25

// Sensor Configuration
#define DHT_TYPE DHT22
#define CURRENT_SENSOR_SENSITIVITY 100.0     // mV/A for ACS712-20A
#define VOLTAGE_DIVIDER_RATIO 11.0           // 220V to 3.3V
#define POWER_CALCULATION_SAMPLES 100
#define SENSOR_READ_INTERVAL_MS 30000        // 30 seconds

// Energy Meter Configuration
#define ENERGY_PRICE_DEFAULT 3500.0          // Default tokens per kWh
#define ENERGY_THRESHOLD_HIGH 5000.0         // High consumption warning (Watts)
#define ENERGY_THRESHOLD_LOW 100.0           // Low consumption threshold (Watts)
#define CARBON_CREDIT_RATE_GRID 0.2          // Grid energy carbon credits
#define CARBON_CREDIT_RATE_SOLAR 0.8         // Solar energy carbon credits

// Display Configuration
#define DISPLAY_UPDATE_INTERVAL_MS 5000      // 5 seconds
#define DISPLAY_BRIGHTNESS 255               // 0-255
#define DISPLAY_CONTRAST 128                 // 0-255

// Security Configuration
#define DEVICE_PRIVATE_KEY "your-device-private-key-here"
#define API_TIMEOUT_MS 10000                 // 10 seconds
#define MAX_RETRY_ATTEMPTS 3
#define HEARTBEAT_INTERVAL_MS 60000          // 1 minute

// NTP Configuration
#define NTP_SERVER1 "pool.ntp.org"
#define NTP_SERVER2 "time.nist.gov"
#define GMT_OFFSET_SEC (7 * 3600)            // GMT+7 for Thailand
#define DAYLIGHT_OFFSET_SEC 0

// Debugging
#define DEBUG_ENABLED true
#define DEBUG_SERIAL_SPEED 115200
#define DEBUG_LOG_LEVEL 3                    // 0=Error, 1=Warn, 2=Info, 3=Debug

// Energy Trading Configuration
#define ENABLE_AUTO_TRADING false            // Automatic energy trading
#define MIN_TRADE_AMOUNT_KWH 0.1            // Minimum trade amount
#define MAX_TRADE_AMOUNT_KWH 100.0          // Maximum trade amount
#define TRADING_MARGIN_PERCENT 5.0          // Trading profit margin

// Grid Operator Configuration (Thailand specific)
#define GRID_OPERATOR "MEA"                  // MEA, PEA, or EGAT
#define GRID_FREQUENCY_HZ 50.0               // Thailand grid frequency
#define GRID_VOLTAGE_NOMINAL 220.0           // Nominal voltage (Volts)
#define GRID_VOLTAGE_TOLERANCE 10.0          // Voltage tolerance (%)

// OTA Update Configuration
#define ENABLE_OTA_UPDATES true
#define OTA_UPDATE_URL "http://192.168.1.100:8080/firmware"
#define FIRMWARE_VERSION "1.0.0"
#define CHECK_UPDATE_INTERVAL_MS 86400000    // 24 hours

// Data Storage Configuration
#define ENABLE_LOCAL_STORAGE true
#define MAX_STORED_READINGS 1000
#define STORAGE_CLEANUP_INTERVAL_MS 3600000  // 1 hour

// Error Handling
#define MAX_WIFI_RETRIES 3
#define MAX_API_RETRIES 3
#define ERROR_LED_BLINK_PATTERN 3            // Number of blinks for errors
#define SUCCESS_LED_BLINK_PATTERN 1          // Number of blinks for success

#endif // GRIDTOKENX_CONFIG_H
