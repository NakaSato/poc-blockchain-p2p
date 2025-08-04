#ifndef GRIDTOKENX_CONFIG_H
#define GRIDTOKENX_CONFIG_H

// =====================================
// GridTokenX ESP32 Smart Energy Meter
// Comprehensive Configuration File
// Version: 1.0.0
// =====================================

// Project Information
#define FIRMWARE_VERSION "1.0.0"
#define DEVICE_MODEL "GridTokenX-SM-ESP32"
#define MANUFACTURER "GridTokenX Labs"
#define BUILD_DATE __DATE__
#define BUILD_TIME __TIME__

// =====================================
// WiFi Configuration
// =====================================
#define WIFI_SSID "YourWiFiNetwork"
#define WIFI_PASSWORD "YourWiFiPassword"
#define WIFI_TIMEOUT_MS 10000
#define WIFI_RETRY_ATTEMPTS 3
#define WIFI_RECONNECT_INTERVAL_MS 30000

// =====================================
// GridTokenX Blockchain Configuration
// =====================================
#define BLOCKCHAIN_API_HOST "192.168.1.100"  // Your GridTokenX node IP
#define BLOCKCHAIN_API_PORT 8080
#define BLOCKCHAIN_API_BASE_URL "http://192.168.1.100:8080/api/v1"
#define BLOCKCHAIN_API_TIMEOUT 15000

// API Endpoints
#define API_ENERGY_READINGS "/energy/readings"
#define API_ENERGY_ORDERS "/energy/orders"
#define API_GRID_STATUS "/grid/status"
#define API_IOT_REGISTER "/iot/register"
#define API_ACCOUNTS "/accounts"
#define API_STATUS "/status"

// Device Authentication
#define DEVICE_ID "ESP32_METER_001"
#define DEVICE_TYPE "smart_energy_meter"
#define DEVICE_ADDRESS "0x1234567890abcdef1234567890abcdef12345678"
#define DEVICE_LOCATION "13.7563,100.5018"   // Bangkok coordinates
#define DEVICE_ZONE "MEA-BANGKOK-ZONE-1"     // MEA jurisdiction
#define API_KEY "your-gridtokenx-api-key-here"

// =====================================
// Hardware Pin Configuration
// =====================================

// I2C Display (SSD1306)
#define OLED_SDA_PIN 21
#define OLED_SCL_PIN 22
#define OLED_RST_PIN 16
#define OLED_WIDTH 128
#define OLED_HEIGHT 64
#define OLED_ADDRESS 0x3C

// Sensors
#define DHT_PIN 4
#define DHT_TYPE DHT22
#define CURRENT_SENSOR_PIN 36  // A0 - ACS712
#define VOLTAGE_SENSOR_PIN 39  // A3 - ZMPT101B
#define LIGHT_SENSOR_PIN 34    // A6 - LDR
#define TEMPERATURE_SENSOR_PIN 35  // A7 - DS18B20

// LEDs and Indicators
#define BUILTIN_LED_PIN 2
#define RGB_LED_PIN 12         // WS2812
#define STATUS_LED_PIN 13      // Green LED
#define ERROR_LED_PIN 14       // Red LED
#define NETWORK_LED_PIN 15     // Blue LED
#define BUZZER_PIN 27

// User Interface
#define BOOT_BUTTON_PIN 0
#define MENU_BUTTON_PIN 25
#define SELECT_BUTTON_PIN 32
#define BACK_BUTTON_PIN 33
#define RESET_BUTTON_PIN 26

// SD Card (SPI)
#define SD_SCK_PIN 18
#define SD_MISO_PIN 19
#define SD_MOSI_PIN 23
#define SD_CS_PIN 5

// Relay Control (Smart Switch)
#define RELAY_1_PIN 17         // Load control
#define RELAY_2_PIN 16         // Backup control

// External Communication
#define RX2_PIN 9
#define TX2_PIN 10

// =====================================
// Energy Measurement Configuration
// =====================================

// Current Sensor (ACS712) Calibration
#define ACS712_SENSITIVITY 100.0      // mV/A for 20A sensor
#define ACS712_VCC 5.0
#define ACS712_ZERO_CURRENT_VOLTAGE 2.5
#define ACS712_NOISE_THRESHOLD 0.05

// Voltage Sensor (ZMPT101B) Calibration
#define VOLTAGE_SENSOR_RATIO 0.00322  // Conversion factor
#define MAINS_VOLTAGE_NOMINAL 220.0   // Thailand standard voltage
#define VOLTAGE_CALIBRATION_FACTOR 1.0
#define VOLTAGE_DIVIDER_RATIO 11.0

// Power Calculation Parameters
#define POWER_FACTOR_DEFAULT 0.95
#define GRID_FREQUENCY_NOMINAL 50.0   // Hz (Thailand standard)
#define POWER_CALCULATION_SAMPLES 100
#define ADC_RESOLUTION 4096           // 12-bit ADC
#define ADC_REFERENCE_VOLTAGE 3.3

// Measurement Intervals
#define SENSOR_READ_INTERVAL_MS 1000      // 1 second raw readings
#define POWER_CALC_INTERVAL_MS 5000       // 5 seconds power calculation
#define ENERGY_CALC_INTERVAL_MS 60000     // 1 minute energy calculation
#define BLOCKCHAIN_SYNC_INTERVAL_MS 300000 // 5 minutes blockchain sync
#define DISPLAY_UPDATE_INTERVAL_MS 2000    // 2 seconds display update

// =====================================
// Energy Trading Configuration
// =====================================

// Automated Trading Parameters
#define ENABLE_AUTO_TRADING true
#define EXCESS_ENERGY_THRESHOLD 1.0    // kWh surplus to trigger sell order
#define ENERGY_DEMAND_THRESHOLD 0.5    // kWh deficit to trigger buy order
#define MAX_TRADING_PRICE 6000         // THB per kWh maximum
#define MIN_TRADING_PRICE 3000         // THB per kWh minimum
#define DEFAULT_ENERGY_PRICE 4500      // THB per kWh default

// Trading Logic
#define TRADING_CHECK_INTERVAL_MS 300000  // 5 minutes between checks
#define ORDER_EXPIRATION_HOURS 24         // Order validity period
#define MIN_TRADE_AMOUNT 0.1              // kWh minimum trade size
#define MAX_TRADE_AMOUNT 10.0             // kWh maximum trade size

// Market Integration
#define ENERGY_SOURCE_TYPE "solar"         // Default energy source
#define GRID_LOCATION "BKK-01-SUB001"     // Grid connection point
#define ACCOUNT_TYPE "consumer"           // Device account type

// =====================================
// Grid Quality Monitoring
// =====================================

// Voltage Limits (Thailand Grid Code)
#define VOLTAGE_MIN_LIMIT 207.0        // V (220V -6%)
#define VOLTAGE_MAX_LIMIT 233.0        // V (220V +6%)
#define VOLTAGE_WARNING_MIN 210.0      // V (early warning)
#define VOLTAGE_WARNING_MAX 230.0      // V (early warning)

// Frequency Limits
#define FREQUENCY_MIN_LIMIT 49.5       // Hz
#define FREQUENCY_MAX_LIMIT 50.5       // Hz
#define FREQUENCY_WARNING_MIN 49.8     // Hz
#define FREQUENCY_WARNING_MAX 50.2     // Hz

// Power Quality Thresholds
#define POWER_FACTOR_MIN 0.85           // Minimum acceptable power factor
#define POWER_FACTOR_WARNING 0.90       // Warning threshold
#define THD_MAX_THRESHOLD 5.0           // % Total Harmonic Distortion
#define FLICKER_THRESHOLD 1.0           // Voltage flicker limit

// =====================================
// Data Storage and Logging
// =====================================

// Local Storage Settings
#define ENABLE_SD_CARD true
#define LOG_FILE_PREFIX "/energy_log_"
#define CONFIG_FILE "/device_config.json"
#define CALIBRATION_FILE "/calibration.json"
#define BACKUP_INTERVAL_MS 3600000     // 1 hour backup interval

// Data Retention
#define MAX_LOG_FILES 100
#define LOG_ROTATION_SIZE_KB 1024      // 1MB per log file
#define DATA_RETENTION_DAYS 30         // Keep 30 days of data
#define COMPRESS_OLD_LOGS true

// Logging Levels
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_WARNING 2
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_DEBUG 4
#define CURRENT_LOG_LEVEL LOG_LEVEL_INFO

// =====================================
// Security Configuration
// =====================================

// TLS/SSL Settings
#define ENABLE_SSL true
#define VERIFY_SSL_CERT true
#define SSL_TIMEOUT_MS 10000

// Device Authentication
#define ENABLE_DEVICE_AUTH true
#define AUTH_TOKEN_REFRESH_INTERVAL_MS 86400000  // 24 hours
#define DEVICE_CERT_FILE "/device_cert.pem"
#define DEVICE_KEY_FILE "/device_key.pem"

// Data Encryption
#define ENCRYPT_LOCAL_DATA true
#define ENCRYPTION_KEY_SIZE 256         // AES-256

// =====================================
// Display Configuration
// =====================================

// Screen Management
#define SCREEN_PAGES 6
#define SCREEN_PAGE_DURATION_MS 5000   // Auto-advance interval
#define SCREEN_BRIGHTNESS 128          // 0-255
#define SCREEN_TIMEOUT_MS 60000        // Screen saver timeout

// Page Definitions
#define PAGE_ENERGY_METERS 0
#define PAGE_GRID_STATUS 1
#define PAGE_BLOCKCHAIN_STATUS 2
#define PAGE_TRADING_INFO 3
#define PAGE_DEVICE_STATUS 4
#define PAGE_NETWORK_INFO 5

// Display Text
#define DISPLAY_TITLE "GridTokenX Meter"
#define DISPLAY_FOOTER "v1.0.0"

// =====================================
// Communication Protocols
// =====================================

// HTTP Client Settings
#define HTTP_TIMEOUT_MS 15000
#define HTTP_RETRY_ATTEMPTS 3
#define HTTP_USER_AGENT "GridTokenX-ESP32/1.0"

// MQTT Configuration (Optional)
#define ENABLE_MQTT false
#define MQTT_BROKER "mqtt.gridtokenx.com"
#define MQTT_PORT 1883
#define MQTT_CLIENT_ID DEVICE_ID
#define MQTT_USERNAME ""
#define MQTT_PASSWORD ""
#define MQTT_KEEP_ALIVE 60

// NTP Time Synchronization
#define NTP_SERVER "pool.ntp.org"
#define NTP_BACKUP_SERVER "time.google.com"
#define TIMEZONE_OFFSET_SECONDS 25200  // GMT+7 (Thailand)
#define TIME_SYNC_INTERVAL_MS 3600000  // 1 hour

// =====================================
// Performance and Monitoring
// =====================================

// Watchdog Configuration
#define ENABLE_WATCHDOG true
#define WATCHDOG_TIMEOUT_MS 30000
#define WATCHDOG_RESET_THRESHOLD 3

// Memory Management
#define MEMORY_CHECK_INTERVAL_MS 60000
#define MIN_FREE_HEAP_BYTES 10000
#define HEAP_WARNING_THRESHOLD 5000

// Task Priorities
#define TASK_PRIORITY_HIGH 3
#define TASK_PRIORITY_NORMAL 2
#define TASK_PRIORITY_LOW 1

// Stack Sizes
#define STACK_SIZE_LARGE 8192
#define STACK_SIZE_MEDIUM 4096
#define STACK_SIZE_SMALL 2048

// =====================================
// Advanced Features
// =====================================

// Over-The-Air Updates
#define ENABLE_OTA_UPDATES true
#define OTA_PASSWORD "gridtokenx-ota"
#define OTA_PORT 3232
#define OTA_CHECK_INTERVAL_MS 86400000  // Daily check

// Web Server
#define ENABLE_WEB_SERVER true
#define WEB_SERVER_PORT 80
#define WEB_AUTH_USERNAME "admin"
#define WEB_AUTH_PASSWORD "gridtokenx"

// Telnet Debug Server
#define ENABLE_TELNET_DEBUG false
#define TELNET_PORT 23

// =====================================
// Safety and Error Handling
// =====================================

// Error Recovery
#define MAX_CONSECUTIVE_ERRORS 5
#define REBOOT_ON_CRITICAL_ERROR true
#define ERROR_RECOVERY_DELAY_MS 5000
#define SAFE_MODE_THRESHOLD 10

// Safety Limits
#define MAX_CURRENT_THRESHOLD 25.0     // A (overcurrent protection)
#define MAX_POWER_THRESHOLD 5500.0     // W (overpower protection)
#define TEMPERATURE_SHUTDOWN_LIMIT 80.0 // °C (thermal protection)
#define HUMIDITY_WARNING_LIMIT 95.0    // % RH

// Grid Anomaly Detection
#define VOLTAGE_SPIKE_THRESHOLD 250.0  // V
#define FREQUENCY_DEVIATION_LIMIT 1.0  // Hz
#define POWER_FLUCTUATION_LIMIT 20.0   // % variation

// =====================================
// Calibration and Maintenance
// =====================================

// Calibration Parameters
#define ENABLE_AUTO_CALIBRATION true
#define CALIBRATION_INTERVAL_DAYS 30
#define CALIBRATION_SAMPLES 1000

// Maintenance Schedules
#define MAINTENANCE_CHECK_INTERVAL_MS 604800000  // Weekly
#define SENSOR_CLEANING_REMINDER_DAYS 90
#define FIRMWARE_UPDATE_CHECK_DAYS 7

// =====================================
// Regional Settings (Thailand)
// =====================================

// Utility Integration
#define UTILITY_PROVIDER "MEA"         // MEA, PEA, or EGAT
#define TARIFF_STRUCTURE "TOU"         // Time-of-Use
#define BILLING_CURRENCY "THB"
#define TIMEZONE "Asia/Bangkok"

// Thai Energy Regulations
#define METER_ACCURACY_CLASS 1.0       // IEC 62053-21
#define GRID_CODE_COMPLIANCE "TGC-2019"
#define SAFETY_STANDARD "TIS_2510"

#endif // GRIDTOKENX_CONFIG_H
