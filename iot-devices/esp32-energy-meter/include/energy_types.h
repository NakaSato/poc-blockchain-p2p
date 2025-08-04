#ifndef ENERGY_TYPES_H#define ENERGY_TYPES_H#include <Arduino.h>#include <ArduinoJson.h>// =====================================// GridTokenX Energy Data Types// =====================================// Energy measurement structurestruct EnergyMeasurement {    // Basic electrical parameters    float voltage;              // Volts (V)    float current;              // Amperes (A)    float power;                // Watts (W)    float energy;               // Kilowatt-hours (kWh)    float power_factor;         // Power factor (0.0-1.0)    float frequency;            // Frequency (Hz)        // Advanced power quality metrics    float thd_voltage;          // Total Harmonic Distortion - Voltage (%)    float thd_current;          // Total Harmonic Distortion - Current (%)    float reactive_power;       // Reactive power (VAR)    float apparent_power;       // Apparent power (VA)        // Environmental factors    float temperature;          // Celsius (°C)    float humidity;            // Relative humidity (%)    float light_level;         // Light sensor reading (0-100%)        // Timing and location    unsigned long timestamp;    // Unix timestamp    float latitude;            // GPS latitude    float longitude;           // GPS longitude    char device_id[32];        // Device identifier    char zone[16];             // Grid zone (e.g., "MEA-BKK-Z1")        // Data quality indicators    bool voltage_stable;       // Voltage within limits    bool frequency_stable;     // Frequency within limits    bool data_valid;           // Overall data validity    uint8_t signal_quality;    // Signal quality (0-100)};// Grid status informationstruct GridStatus {    // Grid health indicators    bool grid_connected;       // Grid connection status    bool grid_stable;          // Grid stability indicator    float grid_voltage;        // Grid voltage (V)    float grid_frequency;      // Grid frequency (Hz)        // Load information    float total_load;          // Total grid load (MW)    float renewable_percentage; // Renewable energy percentage    float carbon_intensity;    // Carbon intensity (gCO2/kWh)        // Demand response    bool peak_demand_period;   // Peak demand indicator    float demand_charge_rate;  // Demand charge rate    uint8_t load_priority;     // Load priority level (1-5)        // Grid events    bool power_outage;         // Power outage alert    bool voltage_anomaly;      // Voltage anomaly detected    bool frequency_anomaly;    // Frequency anomaly detected        unsigned long last_update; // Last grid status update};// Energy trading order structurestruct EnergyOrder {    char order_id[64];         // Unique order identifier    char device_address[42];   // Blockchain address        // Order details    enum OrderType {        BUY_ORDER,        SELL_ORDER,        CANCEL_ORDER    } order_type;        float energy_amount;       // Energy amount (kWh)    uint32_t price_per_kwh;    // Price per kWh (in tokens)    uint32_t total_value;      // Total order value (tokens)        // Energy characteristics    enum EnergySource {        SOLAR,        WIND,        HYDRO,        BIOMASS,        GEOTHERMAL,        GRID_MIXED,        UNKNOWN    } energy_source;        float carbon_credits;      // Carbon credits associated    uint8_t energy_quality;    // Energy quality score (0-100)        // Order constraints    unsigned long creation_time;    // Order creation timestamp    unsigned long expiration_time;  // Order expiration timestamp    char grid_location[32];         // Grid connection point    bool immediate_settlement;      // Immediate settlement flag        // Market data    float market_price;        // Current market price    bool price_flexible;       // Accept market price fluctuation    uint8_t priority_level;    // Order priority (1-5)};// Blockchain transaction datastruct BlockchainTransaction {    char transaction_hash[67]; // Transaction hash (0x + 64 chars)    char from_address[42];     // Sender address    char to_address[42];       // Recipient address        // Transaction details    enum TransactionType {        ENERGY_TRADE,        METER_READING,        GRID_DATA,        DEVICE_REGISTRATION,        ENERGY_GENERATION,        CARBON_CREDIT    } transaction_type;        float energy_amount;       // Energy amount (kWh)    uint32_t token_amount;     // Token amount    uint32_t gas_price;        // Transaction fee        // Status tracking    enum TransactionStatus {        PENDING,        CONFIRMED,        FAILED,        CANCELLED    } status;        uint32_t block_number;     // Block number (if confirmed)    unsigned long timestamp;   // Transaction timestamp    uint8_t confirmations;     // Number of confirmations};// Device status and healthstruct DeviceStatus {    // Device information    char device_id[32];        // Device identifier    char firmware_version[16]; // Firmware version    char hardware_version[16]; // Hardware version        // System health    uint32_t uptime_seconds;   // Device uptime    uint8_t cpu_usage;         // CPU usage percentage    uint32_t free_heap;        // Free heap memory (bytes)    uint32_t total_heap;       // Total heap memory (bytes)        // Network status    bool wifi_connected;       // WiFi connection status    int8_t wifi_rssi;          // WiFi signal strength (dBm)    bool internet_available;   // Internet connectivity
    bool blockchain_synced;    // Blockchain sync status
    
    // Sensor status
    bool voltage_sensor_ok;    // Voltage sensor status
    bool current_sensor_ok;    // Current sensor status
    bool temperature_sensor_ok; // Temperature sensor status
    bool humidity_sensor_ok;   // Humidity sensor status
    bool display_ok;           // Display status
    bool sd_card_ok;           // SD card status
    
    // Error tracking
    uint16_t error_count;      // Total error count
    char last_error[64];       // Last error message
    unsigned long last_error_time; // Last error timestamp
    
    // Maintenance
    unsigned long last_calibration; // Last calibration time
    unsigned long next_maintenance; // Next maintenance due
};

// Configuration structure
struct DeviceConfig {
    // Network settings
    char wifi_ssid[32];
    char wifi_password[64];
    char blockchain_api_url[128];
    char api_key[64];
    
    // Measurement settings
    float current_calibration;
    float voltage_calibration;
    uint16_t reading_interval_ms;
    uint16_t sync_interval_ms;
    
    // Trading settings
    bool auto_trading_enabled;
    float excess_energy_threshold;
    float energy_demand_threshold;
    uint32_t max_trade_price;
    uint32_t min_trade_price;
    
    // Display settings
    uint8_t brightness;
    uint16_t page_duration_ms;
    bool auto_advance;
    
    // Safety settings
    float max_current_limit;
    float max_voltage_limit;
    float temperature_limit;
    
    // Checksum for validation
    uint32_t config_checksum;
};

// API response structure
struct ApiResponse {
    bool success;              // Operation success flag
    uint16_t status_code;      // HTTP status code
    char message[128];         // Response message
    char data[512];           // Response data (JSON string)
    unsigned long timestamp;   // Response timestamp
};

// Energy statistics
struct EnergyStatistics {
    // Daily statistics
    float daily_consumption;   // Daily energy consumption (kWh)
    float daily_production;    // Daily energy production (kWh)
    float daily_cost;          // Daily energy cost (THB)
    float daily_savings;       // Daily savings from trading (THB)
    
    // Monthly statistics
    float monthly_consumption; // Monthly energy consumption (kWh)
    float monthly_production;  // Monthly energy production (kWh)
    float monthly_cost;        // Monthly energy cost (THB)
    float monthly_savings;     // Monthly savings (THB)
    
    // Trading statistics
    uint32_t total_trades;     // Total number of trades
    float total_energy_traded; // Total energy traded (kWh)
    float total_trade_value;   // Total trading value (THB)
    float average_trade_price; // Average trading price (THB/kWh)
    
    // Carbon footprint
    float carbon_footprint;    // Carbon footprint (kgCO2)
    float carbon_credits_earned; // Carbon credits earned
    float renewable_percentage; // Renewable energy percentage
    
    // Grid contribution
    float grid_feed_in;        // Energy fed to grid (kWh)
    float grid_consumption;    // Energy consumed from grid (kWh)
    float peak_demand;         // Peak demand (kW)
    float load_factor;         // Load factor
};

// Function declarations for energy data handling
bool isValidEnergyMeasurement(const EnergyMeasurement& measurement);
String energyMeasurementToJson(const EnergyMeasurement& measurement);
bool jsonToEnergyMeasurement(const String& json, EnergyMeasurement& measurement);

bool isValidEnergyOrder(const EnergyOrder& order);
String energyOrderToJson(const EnergyOrder& order);
bool jsonToEnergyOrder(const String& json, EnergyOrder& order);

String deviceStatusToJson(const DeviceStatus& status);
bool jsonToDeviceStatus(const String& json, DeviceStatus& status);

String configToJson(const DeviceConfig& config);
bool jsonToConfig(const String& json, DeviceConfig& config);

// Utility functions
float calculatePowerFactor(float real_power, float apparent_power);
float calculateEnergy(float power, float time_hours);
float calculateCarbonCredits(float energy_kwh, EnergyOrder::EnergySource source);
bool isGridQualityGood(const EnergyMeasurement& measurement);
uint8_t calculateEnergyQuality(const EnergyMeasurement& measurement);

// Constants for energy calculations
const float VOLTAGE_NOMINAL = 220.0;      // Thailand standard voltage
const float FREQUENCY_NOMINAL = 50.0;     // Thailand standard frequency
const float POWER_FACTOR_GOOD = 0.95;     // Good power factor threshold
const float THD_LIMIT = 5.0;              // THD limit percentage

// Carbon credit rates (kg CO2 / kWh)
const float SOLAR_CARBON_RATE = 0.5;
const float WIND_CARBON_RATE = 0.6;
const float HYDRO_CARBON_RATE = 0.4;
const float BIOMASS_CARBON_RATE = 0.3;
const float GEOTHERMAL_CARBON_RATE = 0.7;
const float GRID_CARBON_RATE = 0.5;       // Thailand grid average

#endif // ENERGY_TYPES_H
