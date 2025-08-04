/**
 * GridTokenX ESP32 Energy Data Types
 * 
 * Data structures and types for energy meter data exchange
 * with the GridTokenX blockchain
 */

#ifndef ENERGY_TYPES_H
#define ENERGY_TYPES_H

#include <Arduino.h>
#include <ArduinoJson.h>

// Energy measurement data structure
struct EnergyReading {
    String device_id;
    String timestamp;
    float energy_consumed_kwh;    // Total energy consumed (kWh)
    float energy_produced_kwh;    // Total energy produced (kWh)
    float instantaneous_power_w;  // Current power consumption/production (Watts)
    float voltage_v;              // RMS voltage (Volts)
    float current_a;              // RMS current (Amperes)
    float frequency_hz;           // Grid frequency (Hz)
    float power_factor;           // Power factor (0.0 - 1.0)
    float temperature_c;          // Ambient temperature (Celsius)
    float humidity_percent;       // Relative humidity (%)
    String location;              // GPS coordinates or zone identifier
    String energy_source;         // "grid", "solar", "wind", "battery"
    String grid_operator;         // "EGAT", "MEA", "PEA"
    float carbon_credits;         // Calculated carbon credits
    uint32_t sequence_number;     // Reading sequence number
    bool is_valid;                // Data validation flag
};

// Energy trade order structure
struct EnergyTradeOrder {
    String device_id;
    String order_id;
    String order_type;            // "buy", "sell"
    float amount_kwh;             // Amount of energy (kWh)
    float price_per_kwh;          // Price in tokens per kWh
    String energy_type;           // "renewable", "conventional"
    String time_slot;             // Time slot for energy delivery
    String location_preference;   // Preferred trading location
    uint32_t expiration_timestamp; // Order expiration time
    bool auto_execute;            // Automatic execution flag
};

// Device status structure
struct DeviceStatus {
    String device_id;
    String firmware_version;
    String last_sync_timestamp;
    bool wifi_connected;
    bool blockchain_connected;
    int wifi_signal_strength;     // RSSI in dBm
    float uptime_hours;           // Device uptime in hours
    uint32_t total_readings_sent; // Total readings transmitted
    uint32_t successful_syncs;    // Successful blockchain syncs
    uint32_t failed_syncs;        // Failed blockchain syncs
    float battery_voltage;        // Battery voltage (for battery-powered devices)
    bool maintenance_mode;        // Maintenance mode flag
    String error_messages[5];     // Recent error messages
    int error_count;              // Number of active errors
};

// Grid quality metrics
struct GridQualityMetrics {
    float voltage_thd_percent;    // Total Harmonic Distortion for voltage
    float current_thd_percent;    // Total Harmonic Distortion for current
    float voltage_unbalance_percent; // Voltage unbalance
    float frequency_deviation_hz; // Frequency deviation from nominal
    bool voltage_sag_detected;    // Voltage sag event
    bool voltage_swell_detected;  // Voltage swell event
    bool power_outage_detected;   // Power outage event
    uint32_t power_interruptions; // Number of power interruptions
    float power_quality_index;    // Overall power quality index (0.0-1.0)
};

// Blockchain transaction response
struct BlockchainResponse {
    bool success;
    String transaction_hash;
    String block_hash;
    uint32_t block_number;
    float transaction_fee;
    String error_message;
    uint32_t confirmation_time_ms;
    float account_balance;        // Updated account balance
    float energy_price_current;   // Current energy price
};

// Energy pricing information
struct EnergyPricing {
    float base_price_per_kwh;     // Base energy price
    float peak_multiplier;        // Peak hour multiplier
    float off_peak_multiplier;    // Off-peak hour multiplier
    float renewable_bonus;        // Renewable energy bonus
    float carbon_credit_value;    // Carbon credit value per unit
    String tariff_structure;      // "time_of_use", "flat_rate", "tiered"
    uint32_t valid_until_timestamp; // Pricing validity timestamp
};

// Trading session information
struct TradingSession {
    String session_id;
    uint32_t start_timestamp;
    uint32_t end_timestamp;
    float total_volume_kwh;       // Total traded volume
    float average_price;          // Average trading price
    uint32_t total_trades;        // Number of completed trades
    float your_trades_volume;     // Your device's trading volume
    float profit_loss;            // Profit/loss from trading
    bool session_active;          // Session status
};

// Function prototypes for data handling
String serializeEnergyReading(const EnergyReading& reading);
EnergyReading deserializeEnergyReading(const String& json);
String serializeTradeOrder(const EnergyTradeOrder& order);
EnergyTradeOrder deserializeTradeOrder(const String& json);
String serializeDeviceStatus(const DeviceStatus& status);
BlockchainResponse deserializeBlockchainResponse(const String& json);
bool validateEnergyReading(const EnergyReading& reading);
float calculateCarbonCredits(float energy_kwh, const String& energy_source);
String generateOrderId();
uint32_t getCurrentTimestamp();

// Utility functions
class EnergyDataUtils {
public:
    static String formatTimestamp(uint32_t timestamp);
    static bool isValidDeviceId(const String& device_id);
    static float calculatePowerFactor(float active_power, float apparent_power);
    static float calculateTHD(float* harmonics, int harmonic_count);
    static String getGridOperatorFromLocation(const String& location);
    static bool isRenewableEnergySource(const String& energy_source);
    static float applyTariffStructure(float base_price, const String& tariff, uint32_t timestamp);
    static String generateDeviceSignature(const String& data, const String& private_key);
    static bool verifyDataIntegrity(const EnergyReading& reading);
};

// Constants for energy calculations
const float GRID_VOLTAGE_NOMINAL = 220.0;    // Volts (Thailand standard)
const float GRID_FREQUENCY_NOMINAL = 50.0;   // Hz (Thailand standard)
const float POWER_FACTOR_MINIMUM = 0.85;     // Minimum acceptable power factor
const float VOLTAGE_TOLERANCE = 0.1;         // ±10% voltage tolerance
const float FREQUENCY_TOLERANCE = 0.5;       // ±0.5 Hz frequency tolerance

// Carbon credit rates (credits per kWh)
const float CARBON_CREDIT_SOLAR = 0.8;
const float CARBON_CREDIT_WIND = 0.7;
const float CARBON_CREDIT_HYDRO = 0.6;
const float CARBON_CREDIT_BIOMASS = 0.4;
const float CARBON_CREDIT_GRID = 0.2;
const float CARBON_CREDIT_COAL = 0.0;

// Energy source classifications
const char* RENEWABLE_SOURCES[] = {"solar", "wind", "hydro", "biomass", "geothermal"};
const char* CONVENTIONAL_SOURCES[] = {"grid", "coal", "gas", "nuclear"};

#endif // ENERGY_TYPES_H
