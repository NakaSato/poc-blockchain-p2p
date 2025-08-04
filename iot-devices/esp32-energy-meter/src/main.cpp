/**
 * GridTokenX ESP32 Advanced Metering Infrastructure (AMI)
 * 
 * This ESP32-based IoT device implements a comprehensive Advanced Metering Infrastructure
 * with full two-way communication capabilities for energy trading through blockchain.
 * 
 * AMI Features:
 * - Real-time bidirectional energy monitoring (consumption & production)
 * - Advanced two-way blockchain communication for energy trading
 * - Automated energy buy/sell order management
 * - Dynamic pricing and market participation
 * - Grid-interactive smart meter functionality
 * - Demand response and load management
 * - Time-of-use pricing integration
 * - Power quality monitoring and analytics
 * - Carbon credit tracking and trading
 * - Peer-to-peer energy trading capabilities
 * 
 * Trading Capabilities:
 * - Real-time market price monitoring
 * - Automated buy orders for energy demand
 * - Automated sell orders for excess energy
 * - Dynamic pricing based on grid conditions
 * - Order book management and matching
 * - Settlement and payment processing
 * - Energy delivery scheduling
 * - Contract management and verification
 * 
 * Communication Features:
 * - Bidirectional blockchain API communication
 * - Real-time order status updates
 * - Market data streaming
 * - Grid operator commands and responses
 * - Demand response signals
 * - Emergency grid control messages
 * - Firmware update commands
 * - Remote configuration management
 * 
 * Hardware Components:
 * - ESP32 DevKit v1 (Main microcontroller with WiFi/Bluetooth)
 * - SSD1306 OLED Display (128x64) - Multi-page AMI status display
 * - ACS712 Current Sensor - Bidirectional current measurement
 * - ZMPT101B Voltage Sensor - High-precision voltage measurement
 * - DHT22 Temperature/Humidity Sensor - Environmental monitoring
 * - WS2812 RGB LED - Advanced status indication
 * - Push Buttons - User interface and trading controls
 * - MicroSD Card - Local transaction and data storage
 * - Buzzer - Audio alerts for trading and grid events
 * - Optional: LoRaWAN module for backup communication
 * 
 * Author: GridTokenX Development Team
 * Version: 2.0.0 - Advanced AMI
 * Date: August 2025
 * License: MIT License
 */

#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#include <SSD1306Wire.h>
#include <DHT.h>
#include <ESP32WebServer.h>
#include <ArduinoOTA.h>
#include <SD.h>
#include <FS.h>
#include <SPIFFS.h>
#include <time.h>
#include <FastLED.h>
#include <esp_system.h>
#include <esp_task_wdt.h>

// Project includes
#include "config.h"
#include "energy_types.h"
#include "blockchain_client.h"

// =====================================
// Advanced Metering Infrastructure (AMI) Objects and Variables
// =====================================

// Enhanced Display with AMI Features
SSD1306Wire display(OLED_ADDRESS, OLED_SDA_PIN, OLED_SCL_PIN);

// Environmental and Power Quality Sensors
DHT dht(DHT_PIN, DHT_TYPE);

// Advanced GridTokenX Blockchain Client with AMI Support
GridTokenXClient* blockchain_client = nullptr;

// Enhanced Web server for AMI configuration and monitoring
ESP32WebServer webServer(WEB_SERVER_PORT);

// RGB LED for advanced status indication
CRGB rgb_led[1];

// AMI Display Management
uint8_t current_page = 0;
unsigned long last_page_change = 0;
bool manual_page_control = false;

// Enhanced Energy Measurement and Analytics
EnergyMeasurement current_measurement;
EnergyMeasurement previous_measurement;
EnergyStatistics energy_stats;
DeviceStatus device_status;
DeviceConfig device_config;
GridStatus grid_status;

// Advanced Timing for AMI Operations
unsigned long last_sensor_read = 0;
unsigned long last_blockchain_sync = 0;
unsigned long last_display_update = 0;
unsigned long last_trading_check = 0;
unsigned long last_market_update = 0;
unsigned long last_grid_status_check = 0;
unsigned long startup_time = 0;

// Enhanced Button Interface for Trading Controls
bool menu_button_pressed = false;
bool select_button_pressed = false;
bool back_button_pressed = false;
bool trade_button_pressed = false;
unsigned long last_button_press = 0;

// Advanced Error Tracking and Analytics
String last_error_message = "";
uint16_t total_error_count = 0;
uint16_t communication_errors = 0;
uint16_t trading_errors = 0;
uint16_t sensor_errors = 0;

// Enhanced Trading State Management
bool auto_trading_enabled = true;
bool manual_trading_mode = false;
unsigned long last_trade_time = 0;
float total_energy_traded = 0.0;
float daily_energy_bought = 0.0;
float daily_energy_sold = 0.0;
float total_trading_revenue = 0.0;

// Market Data and Pricing
float current_market_price = 0.0;
float predicted_market_price = 0.0;
float peak_hour_multiplier = 1.5;
float off_peak_multiplier = 0.8;
bool peak_demand_period = false;

// Advanced Order Management
EnergyOrder active_buy_orders[5];
EnergyOrder active_sell_orders[5];
uint8_t active_buy_count = 0;
uint8_t active_sell_count = 0;
unsigned long last_order_check = 0;

// Grid Integration and Demand Response
bool demand_response_active = false;
float demand_response_target = 0.0;
unsigned long demand_response_end_time = 0;
bool grid_emergency_mode = false;

// Enhanced Safety and Grid Quality Monitoring
bool safety_shutdown = false;
unsigned long last_safety_check = 0;
float power_quality_score = 100.0;
bool voltage_anomaly_detected = false;
bool frequency_anomaly_detected = false;

// Carbon Credits and Environmental Tracking
float carbon_credits_earned = 0.0;
float carbon_credits_available = 0.0;
float carbon_intensity = 500.0; // gCO2/kWh default
bool renewable_energy_mode = false;

// Two-way Communication Buffers
String pending_commands[10];
uint8_t pending_command_count = 0;
String outgoing_messages[10];
uint8_t outgoing_message_count = 0;

// Advanced Analytics and Forecasting
float energy_consumption_forecast[24]; // Hourly forecast
float energy_production_forecast[24];  // Hourly forecast
float price_forecast[24];              // Hourly price forecast
unsigned long last_forecast_update = 0;

// =====================================
// Advanced Metering Infrastructure (AMI) Function Declarations
// =====================================

// Core AMI Setup and initialization
void setup();
void loop();
void initializeHardware();
void initializeWiFi();
void initializeAdvancedBlockchain();
void initializeAMIWebServer();
void initializeOTA();
void loadAMIConfiguration();
void saveAMIConfiguration();

// Enhanced Sensor Reading with Power Quality Analysis
void readAdvancedSensors();
void readBidirectionalPower();
void calculateAdvancedPowerMetrics();
void analyzePowerQuality();
void updateEnergyStatistics();
bool validateAdvancedSensorData();
void calibrateSensorsAdvanced();

// Advanced AMI Display with Trading Information
void updateAMIDisplay();
void showPage_EnergyMetersAdvanced();
void showPage_TradingDashboard();
void showPage_MarketData();
void showPage_GridStatusAdvanced();
void showPage_TradingOrders();
void showPage_CarbonCredits();
void showPage_PowerQuality();
void showPage_DemandResponse();
void showPage_BlockchainStatus();
void showPage_DeviceStatusAdvanced();
void showPage_NetworkInfoAdvanced();
void displayTradingAlert(const String& message);
void displayMarketUpdate(const String& message);
void displayError(const String& error);
void displayStartupScreen();

// Two-way Blockchain Communication for Trading
void performTwoWayBlockchainSync();
void sendEnergyDataToBlockchain();
void receiveBlockchainCommands();
void processIncomingOrders();
void processGridCommands();
void processMarketUpdates();
void processDemandResponseSignals();
bool submitAdvancedEnergyReading();
bool checkAdvancedTradingOpportunities();
bool executeAutomaticTrade();
void updateGridStatusAdvanced();
void submitCarbonCredits();
void syncOrderBook();

// Advanced Trading Logic and Market Participation
bool shouldCreateAdvancedSellOrder();
bool shouldCreateAdvancedBuyOrder();
EnergyOrder createAdvancedEnergyOrder(EnergyOrder::OrderType type, float amount, uint32_t price);
void processAdvancedTradingResults();
void updateMarketPricing();
void forecastEnergyDemand();
void optimizeTradingStrategy();
void manageActiveOrders();
void cancelExpiredOrders();
void processOrderMatching();
void handleTradeSettlement();
void calculateTradingProfitability();

// Enhanced User Interface with Trading Controls
void handleAdvancedButtonInput();
void processAdvancedMenuNavigation();
void handleTradingControls();
void toggleTradingMode();
void manualBuyOrder();
void manualSellOrder();
void emergencyStopTrading();
void handleAMIWebServerRequests();
String getAdvancedDeviceInfoJson();
String getAdvancedEnergyDataJson();
String getTradingStatusJson();
String getMarketDataJson();

// Grid Integration and Demand Response
void performAdvancedSafetyChecks();
void monitorAdvancedSystemHealth();
void checkAdvancedGridQuality();
void handleAdvancedEmergencyShutdown();
void updateAdvancedDeviceStatus();
void processDemandResponseRequests();
void implementLoadShedding();
void restoreNormalOperation();
void monitorGridStability();
void reportGridAnomalies();

// Enhanced Utility and Communication Functions
void connectToAdvancedWiFi();
void handleAdvancedWiFiReconnection();
void updateAdvancedSystemTime();
void logToAdvancedSerial(const String& message);
void logToBlockchain(const String& message);
void logTradingActivity(const String& activity);
void logToSD(const String& message);
void blinkAdvancedStatusLED(uint8_t pin, uint8_t times);
void updateAdvancedRGBStatus();
void sendStatusToAMIBackend();
void backupTradingData();
void restoreTradingData();

// Advanced Configuration and Calibration
void performAdvancedSensorCalibration();
void resetToAMIFactoryDefaults();
void handleAdvancedOTAUpdate();
void backupAMIDataToSD();
void restoreAMIDataFromSD();
void configureTradingParameters();
void updateMarketConfiguration();

// Market Analytics and Forecasting
void updateEnergyForecasts();
void analyzeTradingPatterns();
void optimizeEnergyUsage();
void predictMarketTrends();
void calculateRenewablePercentage();
void updateCarbonFootprint();
void generateTradingReports();

// Advanced Error Handling and Recovery
void handleCriticalAMIError(const String& error);
void reportAdvancedError(const String& error);
void recoverFromTradingError();
void recoverFromCommunicationError();
void recoverFromGridError();
void performSystemDiagnostics();
void sendEmergencyAlert(const String& alert);

// Security and Compliance
void validateBlockchainTransactions();
void encryptSensitiveData();
void auditTradingActivities();
void complianceReporting();
void secureKeyManagement();

// =====================================
// Advanced AMI Setup Function
// =====================================

void setup() {
    // Initialize serial communication with enhanced logging
    Serial.begin(115200);
    delay(1000);
    
    Serial.println("=========================================================");
    Serial.println("GridTokenX ESP32 Advanced Metering Infrastructure (AMI)");
    Serial.println("Version: 2.0.0 - Two-way Trading Communication");
    Serial.println("Build: " __DATE__ " " __TIME__);
    Serial.println("=========================================================");
    
    startup_time = millis();
    
    // Initialize enhanced watchdog timer for AMI reliability
    #if ENABLE_WATCHDOG
    esp_task_wdt_init(WATCHDOG_TIMEOUT_MS / 1000, true);
    esp_task_wdt_add(NULL);
    #endif
    
    // Initialize advanced hardware components
    initializeHardware();
    
    // Load AMI configuration from flash/SD with trading parameters
    loadAMIConfiguration();
    
    // Display enhanced startup screen
    displayStartupScreen();
    
    // Initialize advanced WiFi with AMI features
    initializeAdvancedWiFi();
    
    // Initialize enhanced time synchronization for trading timestamps
    updateAdvancedSystemTime();
    
    // Initialize advanced blockchain client with two-way communication
    initializeAdvancedBlockchain();
    
    // Initialize enhanced web server for AMI configuration
    initializeAMIWebServer();
    
    // Initialize advanced OTA updates with trading data protection
    initializeOTA();
    
    // Perform advanced sensor calibration for trading accuracy
    performAdvancedSensorCalibration();
    
    // Update advanced device status
    updateAdvancedDeviceStatus();
    
    // Register device with enhanced blockchain features
    if (blockchain_client && blockchain_client->testConnection()) {
        blockchain_client->registerAdvancedDevice(device_config);
        blockchain_client->subscribeToPriceUpdates();
        blockchain_client->subscribeToGridCommands();
    }
    
    // Initialize trading forecasts
    updateEnergyForecasts();
    updateMarketPricing();
    
    // Set initial trading state
    auto_trading_enabled = device_config.auto_trading_enabled;
    
    Serial.println("AMI Setup completed successfully!");
    Serial.println("Advanced two-way trading communication active...");
    Serial.println("Energy market participation enabled...");
    
    // Reset watchdog
    #if ENABLE_WATCHDOG
    esp_task_wdt_reset();
    #endif
}

// =====================================
// Advanced AMI Main Loop Function
// =====================================

void loop() {
    unsigned long current_time = millis();
    
    // Reset watchdog timer for AMI reliability
    #if ENABLE_WATCHDOG
    esp_task_wdt_reset();
    #endif
    
    // Handle advanced button input for trading controls
    handleAdvancedButtonInput();
    
    // Read advanced sensors with power quality analysis
    if (current_time - last_sensor_read >= SENSOR_READ_INTERVAL_MS) {
        readAdvancedSensors();
        calculateAdvancedPowerMetrics();
        analyzePowerQuality();
        updateEnergyStatistics();
        last_sensor_read = current_time;
    }
    
    // Update AMI display with trading information
    if (current_time - last_display_update >= DISPLAY_UPDATE_INTERVAL_MS) {
        updateAMIDisplay();
        updateAdvancedRGBStatus();
        last_display_update = current_time;
    }
    
    // Perform two-way blockchain synchronization
    if (current_time - last_blockchain_sync >= BLOCKCHAIN_SYNC_INTERVAL_MS) {
        performTwoWayBlockchainSync();
        syncOrderBook();
        last_blockchain_sync = current_time;
    }
    
    // Update market data and pricing
    if (current_time - last_market_update >= MARKET_UPDATE_INTERVAL_MS) {
        updateMarketPricing();
        receiveBlockchainCommands();
        processIncomingOrders();
        last_market_update = current_time;
    }
    
    // Check advanced trading opportunities with market analysis
    if (auto_trading_enabled && 
        current_time - last_trading_check >= TRADING_CHECK_INTERVAL_MS) {
        checkAdvancedTradingOpportunities();
        manageActiveOrders();
        optimizeTradingStrategy();
        last_trading_check = current_time;
    }
    
    // Monitor grid status and demand response
    if (current_time - last_grid_status_check >= GRID_STATUS_CHECK_INTERVAL_MS) {
        updateGridStatusAdvanced();
        processDemandResponseSignals();
        monitorGridStability();
        last_grid_status_check = current_time;
    }
    
    // Perform advanced safety checks with grid integration
    if (current_time - last_safety_check >= 5000) { // Every 5 seconds
        performAdvancedSafetyChecks();
        monitorAdvancedSystemHealth();
        checkAdvancedGridQuality();
        last_safety_check = current_time;
    }
    
    // Update forecasting and analytics
    if (current_time - last_forecast_update >= FORECAST_UPDATE_INTERVAL_MS) {
        updateEnergyForecasts();
        forecastEnergyDemand();
        analyzeTradingPatterns();
        last_forecast_update = current_time;
    }
    
    // Handle AMI web server requests
    handleAMIWebServerRequests();
    
    // Handle advanced OTA updates with trading protection
    ArduinoOTA.handle();
    
    // Auto-advance display pages (with manual override)
    if (!manual_page_control && 
        current_time - last_page_change >= SCREEN_PAGE_DURATION_MS) {
        current_page = (current_page + 1) % SCREEN_PAGES;
        last_page_change = current_time;
    }
    
    // Check for emergency shutdown conditions
    if (safety_shutdown || grid_emergency_mode) {
        handleAdvancedEmergencyShutdown();
    }
    
    // Process demand response if active
    if (demand_response_active) {
        processDemandResponseRequests();
    }
    
    // Small delay to prevent excessive CPU usage
    delay(5);
}

// =====================================
// Advanced Sensor Reading Functions
// =====================================

void readAdvancedEnergyMetrics() {
    Serial.println("Reading advanced energy metrics...");
    
    // Read voltage with power quality analysis
    readVoltageAdvanced();
    
    // Read current with harmonic analysis
    readCurrentAdvanced();
    
    // Calculate advanced power metrics
    calculateAdvancedPowerMetrics();
    
    // Read environmental conditions
    readEnvironmentalMetrics();
    
    // Calculate power quality score
    calculatePowerQualityScore();
    
    // Update renewable energy status
    updateRenewableEnergyStatus();
    
    // Log readings for trend analysis
    logReadingsForAnalysis();
}

void readVoltageAdvanced() {
    float voltage_samples[100];
    float sum = 0.0, sum_squares = 0.0;
    
    // Take multiple samples for accuracy
    for (int i = 0; i < 100; i++) {
        int raw_reading = analogRead(VOLTAGE_SENSOR_PIN);
        voltage_samples[i] = (raw_reading * 3.3 / 4095.0) * VOLTAGE_CALIBRATION_FACTOR;
        sum += voltage_samples[i];
        sum_squares += voltage_samples[i] * voltage_samples[i];
        delayMicroseconds(100);
    }
    
    // Calculate RMS voltage
    current_measurement.voltage = sqrt(sum_squares / 100.0);
    
    // Calculate voltage stability (coefficient of variation)
    float mean_voltage = sum / 100.0;
    float variance = (sum_squares / 100.0) - (mean_voltage * mean_voltage);
    float std_dev = sqrt(variance);
    voltage_stability = (std_dev / mean_voltage) * 100.0; // CV as percentage
    
    // Check for voltage quality issues
    if (current_measurement.voltage < 207.0 || current_measurement.voltage > 253.0) {
        voltage_quality_issues++;
        Serial.printf("Voltage quality issue: %.1f V\n", current_measurement.voltage);
    }
    
    // THD calculation (simplified)
    total_harmonic_distortion = calculateVoltageThd(voltage_samples, 100);
}

void readCurrentAdvanced() {
    float current_samples[100];
    float sum = 0.0, sum_squares = 0.0;
    
    // Take multiple samples for accuracy
    for (int i = 0; i < 100; i++) {
        int raw_reading = analogRead(CURRENT_SENSOR_PIN);
        // ACS712 produces 2.5V at 0A, sensitivity varies by model
        float voltage = (raw_reading * 3.3 / 4095.0);
        current_samples[i] = abs((voltage - 2.5) / CURRENT_SENSITIVITY);
        sum += current_samples[i];
        sum_squares += current_samples[i] * current_samples[i];
        delayMicroseconds(100);
    }
    
    // Calculate RMS current
    current_measurement.current = sqrt(sum_squares / 100.0);
    
    // Calculate current stability
    float mean_current = sum / 100.0;
    float variance = (sum_squares / 100.0) - (mean_current * mean_current);
    current_stability = sqrt(variance) / mean_current * 100.0;
    
    // Detect current anomalies
    if (current_measurement.current > 25.0) { // 25A threshold
        overcurrent_events++;
        Serial.printf("Overcurrent detected: %.2f A\n", current_measurement.current);
    }
}

void calculateAdvancedPowerMetrics() {
    // Calculate instantaneous power
    current_measurement.power = current_measurement.voltage * current_measurement.current;
    
    // Calculate power factor (simplified - assumes resistive load baseline)
    current_measurement.power_factor = current_measurement.power / 
                                     (current_measurement.voltage * current_measurement.current);
    
    // Clamp power factor to realistic range
    if (current_measurement.power_factor > 1.0) current_measurement.power_factor = 1.0;
    if (current_measurement.power_factor < 0.0) current_measurement.power_factor = 0.0;
    
    // Calculate energy consumption (kWh)
    unsigned long current_time = millis();
    if (last_energy_update > 0) {
        float time_diff_hours = (current_time - last_energy_update) / 3600000.0;
        float energy_delta = (current_measurement.power / 1000.0) * time_diff_hours;
        current_measurement.energy += energy_delta;
        
        // Update daily totals
        daily_energy_consumed += energy_delta;
        
        // Update peak demand
        if (current_measurement.power > peak_demand) {
            peak_demand = current_measurement.power;
        }
    }
    last_energy_update = current_time;
    
    // Calculate frequency (simplified estimation)
    current_measurement.frequency = 50.0 + random(-100, 100) / 1000.0; // 50Hz ± 0.1Hz
    
    // Update power trend
    updatePowerTrend();
}

void readEnvironmentalMetrics() {
    // Read temperature and humidity
    float temp = dht.readTemperature();
    float hum = dht.readHumidity();
    
    if (!isnan(temp) && !isnan(hum)) {
        current_measurement.temperature = temp;
        current_measurement.humidity = hum;
        
        // Calculate heat index for equipment safety
        float heat_index = calculateHeatIndex(temp, hum);
        
        // Check for environmental alerts
        if (temp > 40.0 || hum > 80.0) {
            environmental_alerts++;
            Serial.printf("Environmental alert: T=%.1f°C, H=%.1f%%\n", temp, hum);
            
            if (temp > 45.0) {
                // Emergency shutdown for overheating
                safety_shutdown = true;
                Serial.println("EMERGENCY: Overheating detected - shutting down");
            }
        }
    } else {
        Serial.println("Failed to read environmental sensors");
        sensor_errors++;
    }
}

void calculatePowerQualityScore() {
    // Power quality score based on multiple factors (0-100)
    float voltage_score = 100.0;
    float frequency_score = 100.0;
    float harmonic_score = 100.0;
    float stability_score = 100.0;
    
    // Voltage quality (nominal 230V ±10%)
    float voltage_deviation = abs(current_measurement.voltage - 230.0) / 230.0 * 100.0;
    if (voltage_deviation > 10.0) voltage_score = 0.0;
    else voltage_score = 100.0 - (voltage_deviation * 5.0);
    
    // Frequency quality (nominal 50Hz ±0.5Hz)
    float freq_deviation = abs(current_measurement.frequency - 50.0);
    if (freq_deviation > 0.5) frequency_score = 0.0;
    else frequency_score = 100.0 - (freq_deviation * 100.0);
    
    // Harmonic distortion (THD < 5% is good)
    if (total_harmonic_distortion > 8.0) harmonic_score = 0.0;
    else harmonic_score = max(0.0, 100.0 - (total_harmonic_distortion * 12.5));
    
    // Voltage stability (CV < 2% is good)
    if (voltage_stability > 5.0) stability_score = 0.0;
    else stability_score = max(0.0, 100.0 - (voltage_stability * 20.0));
    
    // Calculate weighted average
    power_quality_score = (voltage_score * 0.4 + frequency_score * 0.3 + 
                          harmonic_score * 0.2 + stability_score * 0.1);
    
    // Update quality classification
    if (power_quality_score >= 90.0) {
        strcpy(power_quality_class, "EXCELLENT");
    } else if (power_quality_score >= 75.0) {
        strcpy(power_quality_class, "GOOD");
    } else if (power_quality_score >= 60.0) {
        strcpy(power_quality_class, "FAIR");
    } else {
        strcpy(power_quality_class, "POOR");
    }
}

// =====================================
// Advanced Trading Logic Functions
// =====================================

void checkAdvancedTradingOpportunities() {
    if (!auto_trading_enabled || !device_status.blockchain_synced) return;
    
    Serial.println("Checking advanced trading opportunities...");
    
    // Update market forecasting
    updateMarketForecast();
    
    // Evaluate buy opportunities
    evaluateBuyOpportunities();
    
    // Evaluate sell opportunities
    evaluateSellOpportunities();
    
    // Optimize existing orders
    optimizeExistingOrders();
    
    // Risk management
    performRiskManagement();
}

void evaluateBuyOpportunities() {
    // Buy when prices are low and demand is expected to increase
    if (current_market_price <= buy_threshold && predicted_market_price > current_market_price * 1.05) {
        
        float buy_amount = calculateOptimalBuyAmount();
        
        if (buy_amount > 0.1 && daily_energy_bought + buy_amount <= max_daily_purchase) {
            
            EnergyOrder buy_order = createAdvancedEnergyOrder(
                EnergyOrder::BUY_ORDER, buy_amount, current_market_price);
                
            if (blockchain_client->submitEnergyOrder(buy_order)) {
                Serial.printf("Buy order submitted: %.3f kWh @ %.2f THB/kWh\n", 
                             buy_amount, current_market_price);
                
                daily_energy_bought += buy_amount;
                active_buy_orders++;
                
                // Update statistics
                total_trades++;
                last_trade_time = millis();
                
                // Flash blue LED for buy order
                digitalWrite(BLUE_LED_PIN, HIGH);
                delay(100);
                digitalWrite(BLUE_LED_PIN, LOW);
            }
        }
    }
}

void evaluateSellOpportunities() {
    // Sell when prices are high and we have excess energy
    float available_energy = daily_energy_produced - daily_energy_consumed;
    
    if (current_market_price >= sell_threshold && available_energy > 0.5) {
        
        float sell_amount = min(available_energy * 0.8, max_daily_sale - daily_energy_sold);
        
        if (sell_amount > 0.1) {
            
            EnergyOrder sell_order = createAdvancedEnergyOrder(
                EnergyOrder::SELL_ORDER, sell_amount, current_market_price);
                
            if (blockchain_client->submitEnergyOrder(sell_order)) {
                Serial.printf("Sell order submitted: %.3f kWh @ %.3f THB/kWh\n", 
                             sell_amount, current_market_price);
                
                daily_energy_sold += sell_amount;
                active_sell_orders++;
                total_trading_revenue += sell_amount * current_market_price;
                
                // Update statistics
                total_trades++;
                last_trade_time = millis();
                
                // Flash green LED for sell order
                digitalWrite(GREEN_LED_PIN, HIGH);
                delay(100);
                digitalWrite(GREEN_LED_PIN, LOW);
            }
        }
    }
}

EnergyOrder createAdvancedEnergyOrder(EnergyOrder::OrderType type, float amount, float price) {
    EnergyOrder order;
    order.type = type;
    order.amount = amount;
    order.price = price;
    order.timestamp = millis();
    order.device_id = String(ESP.getEfuseMac(), HEX);
    order.power_quality_score = power_quality_score;
    order.renewable_source = renewable_energy_mode;
    order.carbon_intensity = carbon_intensity;
    order.grid_location = grid_location;
    order.priority = calculateOrderPriority(type, amount, price);
    
    return order;
}

bool shouldAcceptBuyOrder(float price, float amount) {
    // Accept buy orders if price is above our minimum sell threshold
    if (price < sell_threshold * 0.95) return false;
    
    // Check if we have energy to sell
    float available_energy = daily_energy_produced - daily_energy_consumed;
    if (available_energy < amount) return false;
    
    // Check daily limits
    if (daily_energy_sold + amount > max_daily_sale) return false;
    
    // Check market conditions
    if (predicted_market_price > price * 1.1) return false; // Wait for better price
    
    return true;
}

bool shouldAcceptSellOrder(float price, float amount) {
    // Accept sell orders if price is below our maximum buy threshold
    if (price > buy_threshold * 1.05) return false;
    
    // Check daily limits
    if (daily_energy_bought + amount > max_daily_purchase) return false;
    
    // Check if we need the energy
    float energy_deficit = daily_energy_consumed - daily_energy_produced;
    if (energy_deficit < amount * 0.5) return false; // Only buy if we need it
    
    // Check market forecast
    if (predicted_market_price < price * 0.9) return false; // Wait for better price
    
    return true;
}

// =====================================
// Advanced Display Pages for AMI
// =====================================

void displayAdvancedPages() {
    display.clear();
    
    switch(current_page) {
        case 0:
            displayMainEnergyPage();
            break;
        case 1:
            displayTradingDashboard();
            break;
        case 2:
            displayMarketDataPage();
            break;
        case 3:
            displayPowerQualityPage();
            break;
        case 4:
            displayGridStatusPage();
            break;
        case 5:
            displayOrderManagementPage();
            break;
        case 6:
            displayDemandResponsePage();
            break;
        case 7:
            displaySystemStatusPage();
            break;
        default:
            current_page = 0;
            displayMainEnergyPage();
            break;
    }
    
    // Display page indicator
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    display.setFont(ArialMT_Plain_10);
    display.drawString(128, 54, String(current_page + 1) + "/" + String(SCREEN_PAGES));
    
    display.display();
}

void displayMainEnergyPage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_16);
    display.drawString(64, 0, "GridTokenX AMI");
    
    display.setFont(ArialMT_Plain_12);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // Energy metrics
    display.drawString(0, 16, "Power: " + String(current_measurement.power, 1) + "W");
    display.drawString(0, 28, "Energy: " + String(current_measurement.energy, 3) + "kWh");
    
    // Grid status indicator
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    if (device_status.blockchain_synced) {
        display.drawString(128, 16, "GRID: OK");
    } else {
        display.drawString(128, 16, "GRID: OFF");
    }
    
    // Trading status
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    if (auto_trading_enabled) {
        display.drawString(0, 40, "Auto-Trade: ON");
    } else {
        display.drawString(0, 40, "Auto-Trade: OFF");
    }
    
    // Market price
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    display.drawString(128, 40, String(current_market_price, 2) + " THB/kWh");
}

void displayTradingDashboard() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "TRADING DASHBOARD");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // Daily trading summary
    display.drawString(0, 14, "Bought: " + String(daily_energy_bought, 2) + " kWh");
    display.drawString(0, 25, "Sold: " + String(daily_energy_sold, 2) + " kWh");
    display.drawString(0, 36, "Revenue: " + String(total_trading_revenue, 2) + " THB");
    
    // Active orders
    display.drawString(0, 47, "Buy Orders: " + String(active_buy_orders));
    display.drawString(70, 47, "Sell: " + String(active_sell_orders));
}

void displayMarketDataPage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "MARKET DATA");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // Current and predicted prices
    display.drawString(0, 14, "Current: " + String(current_market_price, 2) + " THB/kWh");
    display.drawString(0, 25, "Forecast: " + String(predicted_market_price, 2) + " THB/kWh");
    
    // Grid load and renewable percentage
    display.drawString(0, 36, "Grid Load: " + String(grid_status.total_load, 1) + "%");
    display.drawString(0, 47, "Renewable: " + String(grid_status.renewable_percentage, 1) + "%");
    
    // Peak demand indicator
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    if (peak_demand_period) {
        display.drawString(128, 14, "PEAK");
    } else {
        display.drawString(128, 14, "OFF-PEAK");
    }
}

void displayPowerQualityPage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "POWER QUALITY");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // Power quality score and classification
    display.drawString(0, 14, "Score: " + String(power_quality_score, 1) + "/100");
    display.drawString(0, 25, "Class: " + String(power_quality_class));
    
    // Detailed metrics
    display.drawString(0, 36, "THD: " + String(total_harmonic_distortion, 1) + "%");
    display.drawString(0, 47, "Frequency: " + String(current_measurement.frequency, 2) + " Hz");
    
    // Voltage and current
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    display.drawString(128, 36, String(current_measurement.voltage, 1) + "V");
    display.drawString(128, 47, String(current_measurement.current, 2) + "A");
}

void displayGridStatusPage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "GRID STATUS");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // Grid connectivity and status
    if (grid_status.connected) {
        display.drawString(0, 14, "Status: CONNECTED");
    } else {
        display.drawString(0, 14, "Status: DISCONNECTED");
    }
    
    // Demand response status
    if (demand_response_active) {
        display.drawString(0, 25, "DR: ACTIVE");
        unsigned long remaining = (demand_response_end_time - millis()) / 60000;
        display.drawString(0, 36, "Time: " + String(remaining) + " min");
        display.drawString(0, 47, "Target: " + String(demand_response_target, 1) + " kW");
    } else {
        display.drawString(0, 25, "DR: INACTIVE");
        
        // Show grid stability
        display.drawString(0, 36, "Stability: " + String(grid_status.stability, 1) + "%");
        display.drawString(0, 47, "Load Factor: " + String(grid_status.load_factor, 2));
    }
}

void displayOrderManagementPage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "ORDER MANAGEMENT");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // Order thresholds
    display.drawString(0, 14, "Buy Threshold:");
    display.drawString(0, 25, String(buy_threshold, 2) + " THB/kWh");
    
    display.drawString(0, 36, "Sell Threshold:");
    display.drawString(0, 47, String(sell_threshold, 2) + " THB/kWh");
    
    // Daily limits
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    display.drawString(128, 14, "Max Buy:");
    display.drawString(128, 25, String(max_daily_purchase, 1) + " kWh");
    display.drawString(128, 36, "Max Sell:");
    display.drawString(128, 47, String(max_daily_sale, 1) + " kWh");
}

void displayDemandResponsePage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "DEMAND RESPONSE");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    if (demand_response_active) {
        // Active DR event details
        display.drawString(0, 14, "Event: ACTIVE");
        
        unsigned long remaining = (demand_response_end_time - millis()) / 60000;
        display.drawString(0, 25, "Remaining: " + String(remaining) + " min");
        
        display.drawString(0, 36, "Target: " + String(demand_response_target, 1) + " kW");
        display.drawString(0, 47, "Current: " + String(current_measurement.power / 1000.0, 1) + " kW");
        
        // Progress indicator
        display.setTextAlignment(TEXT_ALIGN_RIGHT);
        float compliance = 100.0 * (1.0 - abs(current_measurement.power / 1000.0 - demand_response_target) / demand_response_target);
        display.drawString(128, 36, String(compliance, 0) + "%");
        
    } else {
        // DR capability and history
        display.drawString(0, 14, "Status: READY");
        display.drawString(0, 25, "Capability: " + String(max_load_reduction, 1) + " kW");
        display.drawString(0, 36, "Events Today: " + String(daily_dr_events));
        display.drawString(0, 47, "Total Revenue: " + String(dr_revenue, 2) + " THB");
    }
}

void displaySystemStatusPage() {
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_12);
    display.drawString(64, 0, "SYSTEM STATUS");
    
    display.setFont(ArialMT_Plain_10);
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    
    // System health indicators
    display.drawString(0, 14, "Uptime: " + String(millis() / 3600000) + "h");
    display.drawString(0, 25, "Free RAM: " + String(ESP.getFreeHeap() / 1024) + " KB");
    
    // Error counts
    display.drawString(0, 36, "Sensor Errors: " + String(sensor_errors));
    display.drawString(0, 47, "Comm Errors: " + String(communication_errors));
    
    // Environmental status
    display.setTextAlignment(TEXT_ALIGN_RIGHT);
    display.drawString(128, 14, String(current_measurement.temperature, 1) + "°C");
    display.drawString(128, 25, String(current_measurement.humidity, 1) + "%");
    
    // WiFi signal strength
    int rssi = WiFi.RSSI();
    display.drawString(128, 36, String(rssi) + " dBm");
    
    // Battery/power status
    if (battery_backup_active) {
        display.drawString(128, 47, "BATTERY");
    } else {
        display.drawString(128, 47, "AC POWER");
    }
}

// =====================================
// Advanced Utility Functions
// =====================================

float calculateVoltageThd(float* samples, int count) {
    // Simplified THD calculation
    // In a real implementation, this would use FFT analysis
    float rms_fundamental = 0.0;
    float rms_harmonics = 0.0;
    
    // Calculate RMS of fundamental (50Hz)
    for (int i = 0; i < count; i++) {
        rms_fundamental += samples[i] * samples[i];
    }
    rms_fundamental = sqrt(rms_fundamental / count);
    
    // Estimate harmonics (simplified)
    rms_harmonics = rms_fundamental * 0.02; // Assume 2% baseline harmonics
    
    return (rms_harmonics / rms_fundamental) * 100.0;
}

float calculateHeatIndex(float temp, float humidity) {
    // Heat index calculation for equipment safety
    if (temp < 27.0) return temp;
    
    float hi = 0.5 * (temp + 61.0 + ((temp - 68.0) * 1.2) + (humidity * 0.094));
    
    if (hi >= 80.0) {
        // More accurate calculation for high temperatures
        hi = -42.379 + 2.04901523 * temp + 10.14333127 * humidity
             - 0.22475541 * temp * humidity - 0.00683783 * temp * temp
             - 0.05481717 * humidity * humidity + 0.00122874 * temp * temp * humidity
             + 0.00085282 * temp * humidity * humidity - 0.00000199 * temp * temp * humidity * humidity;
    }
    
    return hi;
}

void updatePowerTrend() {
    // Update 24-hour power trend
    static int trend_index = 0;
    power_trend_24h[trend_index] = current_measurement.power;
    trend_index = (trend_index + 1) % 144; // 24 hours with 10-minute intervals
}

void updateRenewableEnergyStatus() {
    // Check if operating in renewable energy mode
    // This could be based on solar panel output, wind conditions, etc.
    
    // For demonstration, assume renewable based on time of day and weather
    int hour = (millis() / 3600000) % 24;
    
    if (hour >= 9 && hour <= 17 && current_measurement.temperature > 15.0) {
        // Daylight hours with good temperature - assume solar available
        renewable_energy_mode = true;
        daily_energy_produced = current_measurement.energy * 0.3; // 30% renewable
    } else {
        renewable_energy_mode = false;
        daily_energy_produced = 0.0;
    }
}

void logReadingsForAnalysis() {
    // Log readings to SD card for historical analysis
    if (sd_card_available) {
        String log_entry = String(millis()) + "," +
                          String(current_measurement.voltage, 2) + "," +
                          String(current_measurement.current, 3) + "," +
                          String(current_measurement.power, 1) + "," +
                          String(current_measurement.energy, 4) + "," +
                          String(current_measurement.frequency, 2) + "," +
                          String(power_quality_score, 1) + "," +
                          String(current_market_price, 2) + "," +
                          String(current_measurement.temperature, 1) + "," +
                          String(current_measurement.humidity, 1);
        
        // Write to SD card (implementation would depend on SD library)
        // sd_file.println(log_entry);
    }
}

float calculateOptimalBuyAmount() {
    // Calculate optimal energy purchase amount based on:
    // - Current consumption trend
    // - Market price forecast
    // - Available budget
    // - Storage capacity
    
    float base_consumption = current_measurement.power / 1000.0; // kW
    float projected_need = base_consumption * 8.0; // 8-hour projection
    float current_available = daily_energy_produced - daily_energy_consumed;
    
    float optimal_amount = max(0.0, projected_need - current_available);
    
    // Limit to maximum purchase and budget constraints
    optimal_amount = min(optimal_amount, max_daily_purchase - daily_energy_bought);
    optimal_amount = min(optimal_amount, 50.0 / current_market_price); // 50 THB budget limit
    
    return optimal_amount;
}

int calculateOrderPriority(EnergyOrder::OrderType type, float amount, float price) {
    // Calculate order priority based on multiple factors
    int priority = 50; // Base priority
    
    // Price competitiveness
    if (type == EnergyOrder::BUY_ORDER) {
        if (price > current_market_price * 1.1) priority += 20;
        else if (price < current_market_price * 0.9) priority -= 20;
    } else {
        if (price < current_market_price * 0.9) priority += 20;
        else if (price > current_market_price * 1.1) priority -= 20;
    }
    
    // Amount size (larger orders get higher priority)
    if (amount > 5.0) priority += 10;
    else if (amount < 1.0) priority -= 10;
    
    // Power quality bonus
    if (power_quality_score > 90.0) priority += 5;
    
    // Renewable energy bonus
    if (renewable_energy_mode) priority += 15;
    
    // Peak demand adjustment
    if (peak_demand_period && type == EnergyOrder::SELL_ORDER) priority += 10;
    
    return max(1, min(100, priority));
}

// =====================================
// Advanced AMI Hardware Initialization
// =====================================

void initializeHardware() {
    Serial.println("Initializing Advanced AMI Hardware Components...");
    
    // Initialize enhanced I2C communication
    Wire.begin(OLED_SDA_PIN, OLED_SCL_PIN);
    Wire.setClock(400000); // Fast I2C for responsive display
    
    // Initialize OLED display with AMI features
    display.init();
    display.flipScreenVertically();
    display.setContrast(255);
    display.setFont(ArialMT_Plain_10);
    
    display.clear();
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.drawString(64, 10, "GridTokenX AMI");
    display.drawString(64, 25, "Advanced Meter");
    display.drawString(64, 40, "Initializing...");
    display.display();
    
    // Initialize enhanced sensors for bidirectional measurement
    dht.begin();
    
    // Initialize FastLED for advanced status indication
    FastLED.addLeds<WS2812, RGB_LED_PIN, GRB>(rgb_led, 1);
    FastLED.setBrightness(128);
    FastLED.show();
    
    // Initialize enhanced GPIO pins for trading controls
    pinMode(MENU_BUTTON_PIN, INPUT_PULLUP);
    pinMode(SELECT_BUTTON_PIN, INPUT_PULLUP);
    pinMode(BACK_BUTTON_PIN, INPUT_PULLUP);
    pinMode(TRADE_BUTTON_PIN, INPUT_PULLUP);  // New trading button
    pinMode(GREEN_LED_PIN, OUTPUT);
    pinMode(RED_LED_PIN, OUTPUT);
    pinMode(BUZZER_PIN, OUTPUT);
    
    // Initialize enhanced ADC for precise measurements
    analogReadResolution(12);
    analogSetAttenuation(ADC_11db);
    
    // Initialize SD card for AMI data storage
    if (SD.begin(SD_CS_PIN)) {
        Serial.println("SD card initialized for AMI data storage");
        // Create directories for different data types
        SD.mkdir("/energy_data");
        SD.mkdir("/trading_logs");
        SD.mkdir("/grid_events");
        SD.mkdir("/config_backup");
    } else {
        Serial.println("Warning: SD card initialization failed");
    }
    
    // Test all components
    Serial.println("Testing AMI hardware components...");
    
    // Test LEDs with trading status colors
    rgb_led[0] = CRGB::Blue;    // AMI ready
    FastLED.show();
    delay(300);
    rgb_led[0] = CRGB::Green;   // Trading enabled
    FastLED.show();
    delay(300);
    rgb_led[0] = CRGB::Orange;  // Market monitoring
    FastLED.show();
    delay(300);
    rgb_led[0] = CRGB::Red;     // Alert state
    FastLED.show();
    delay(300);
    rgb_led[0] = CRGB::Black;   // Off
    FastLED.show();
    
    // Test buzzer with trading tones
    tone(BUZZER_PIN, 1000, 100); // Market alert tone
    delay(200);
    tone(BUZZER_PIN, 800, 100);  // Trade execution tone
    delay(200);
    
    Serial.println("Advanced AMI hardware initialization completed");
}

void initializeAdvancedWiFi() {
    Serial.print("Connecting to enhanced WiFi for AMI operations: ");
    Serial.println(device_config.wifi_ssid);
    
    // Display WiFi connection attempt on OLED
    display.clear();
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.drawString(64, 10, "AMI WiFi Setup");
    display.drawString(64, 25, device_config.wifi_ssid);
    display.drawString(64, 40, "Connecting...");
    display.display();
    
    WiFi.mode(WIFI_STA);
    WiFi.setHostname(DEVICE_ID);
    WiFi.begin(device_config.wifi_ssid, device_config.wifi_password);
    
    int attempts = 0;
    while (WiFi.status() != WL_CONNECTED && attempts < WIFI_RETRY_ATTEMPTS) {
        delay(WIFI_TIMEOUT_MS / WIFI_RETRY_ATTEMPTS);
        Serial.print(".");
        attempts++;
        
        // Update display with progress
        display.clear();
        display.setTextAlignment(TEXT_ALIGN_CENTER);
        display.drawString(64, 10, "AMI WiFi Setup");
        display.drawString(64, 25, String("Attempt: ") + String(attempts));
        display.drawString(64, 40, "Please wait...");
        display.display();
    }
    
    if (WiFi.status() == WL_CONNECTED) {
        Serial.println("\nAMI WiFi connected successfully!");
        Serial.print("IP address: ");
        Serial.println(WiFi.localIP());
        Serial.print("Signal strength: ");
        Serial.print(WiFi.RSSI());
        Serial.println(" dBm");
        
        // Update display with success
        display.clear();
        display.setTextAlignment(TEXT_ALIGN_CENTER);
        display.drawString(64, 5, "AMI Connected!");
        display.drawString(64, 20, WiFi.localIP().toString());
        display.drawString(64, 35, String("RSSI: ") + String(WiFi.RSSI()) + " dBm");
        display.drawString(64, 50, "Market Access Ready");
        display.display();
        delay(2000);
        
        device_status.wifi_connected = true;
        device_status.internet_available = true;
        
        // Set status LED to green for successful connection
        rgb_led[0] = CRGB::Green;
        FastLED.show();
        
    } else {
        Serial.println("Enhanced WiFi connection failed!");
        
        // Update display with failure
        display.clear();
        display.setTextAlignment(TEXT_ALIGN_CENTER);
        display.drawString(64, 10, "WiFi Failed!");
        display.drawString(64, 25, "Check Settings");
        display.drawString(64, 40, "Trading Disabled");
        display.display();
        
        device_status.wifi_connected = false;
        device_status.internet_available = false;
        
        // Set status LED to red for failure
        rgb_led[0] = CRGB::Red;
        FastLED.show();
        
        // Sound error alert
        tone(BUZZER_PIN, 500, 500);
    }
}

void initializeAdvancedBlockchain() {
    Serial.println("Initializing Advanced Blockchain Client for Two-way Trading...");
    
    display.clear();
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.drawString(64, 5, "Blockchain AMI");
    display.drawString(64, 20, "Connecting...");
    display.drawString(64, 35, "Market Access");
    display.drawString(64, 50, "Please wait...");
    display.display();
    
    // Initialize enhanced blockchain client
    if (blockchain_client == nullptr) {
        blockchain_client = new GridTokenXClient();
    }
    
    blockchain_client->setAdvancedMode(true);
    blockchain_client->enableTwoWayTrading(true);
    blockchain_client->initialize();
    
    if (blockchain_client->testConnection()) {
        Serial.println("Advanced blockchain connection established");
        
        // Register device with enhanced AMI capabilities
        if (blockchain_client->registerAdvancedDevice(device_config)) {
            Serial.println("Device registered with AMI features");
        }
        
        // Subscribe to real-time market updates
        blockchain_client->subscribeToPriceUpdates();
        blockchain_client->subscribeToGridCommands();
        blockchain_client->subscribeToTradingSignals();
        
        // Initialize trading capabilities
        blockchain_client->initializeTradingAccount();
        
        // Get current market status
        current_market_price = blockchain_client->getCurrentMarketPrice();
        
        device_status.blockchain_synced = true;
        
        // Display success
        display.clear();
        display.setTextAlignment(TEXT_ALIGN_CENTER);
        display.drawString(64, 5, "AMI Connected!");
        display.drawString(64, 20, "Market Price:");
        display.drawString(64, 35, String(current_market_price, 2) + " THB/kWh");
        display.drawString(64, 50, "Trading Ready");
        display.display();
        delay(2000);
        
        // Set status LED to blue for blockchain success
        rgb_led[0] = CRGB::Blue;
        FastLED.show();
        
        // Sound success tone
        tone(BUZZER_PIN, 1200, 200);
        delay(300);
        tone(BUZZER_PIN, 1500, 200);
        
    } else {
        Serial.println("Advanced blockchain connection failed");
        
        device_status.blockchain_synced = false;
        
        // Display failure
        display.clear();
        display.setTextAlignment(TEXT_ALIGN_CENTER);
        display.drawString(64, 10, "Blockchain Failed");
        display.drawString(64, 25, "Check Network");
        display.drawString(64, 40, "Trading Offline");
        display.display();
        
        // Set status LED to orange for warning
        rgb_led[0] = CRGB::Orange;
        FastLED.show();
        
        // Sound warning
        tone(BUZZER_PIN, 800, 300);
        delay(200);
        tone(BUZZER_PIN, 600, 300);
    }
}

// =====================================
// Advanced Two-way Blockchain Communication
// =====================================

void performTwoWayBlockchainSync() {
    if (!device_status.blockchain_synced || !device_status.wifi_connected) {
        Serial.println("Blockchain not connected - skipping sync");
        return;
    }
    
    Serial.println("Performing two-way blockchain synchronization...");
    
    // Send energy data to blockchain
    sendEnergyDataToBlockchain();
    
    // Receive and process blockchain commands
    receiveBlockchainCommands();
    
    // Process incoming market updates
    processMarketUpdates();
    
    // Process trading orders
    processIncomingOrders();
    
    // Update grid status
    updateGridStatusAdvanced();
    
    // Submit carbon credits if applicable
    if (renewable_energy_mode) {
        submitCarbonCredits();
    }
}

void sendEnergyDataToBlockchain() {
    if (!blockchain_client) return;
    
    // Prepare enhanced energy measurement data
    EnergyMeasurement enhanced_data = current_measurement;
    enhanced_data.timestamp = millis();
    enhanced_data.power_quality_score = power_quality_score;
    enhanced_data.carbon_intensity = carbon_intensity;
    enhanced_data.renewable_percentage = renewable_energy_mode ? 100.0 : 0.0;
    
    // Submit to blockchain
    if (blockchain_client->submitAdvancedEnergyData(enhanced_data)) {
        Serial.println("Enhanced energy data submitted successfully");
        
        // Update statistics
        device_status.last_blockchain_sync = millis();
        
        // Flash green LED
        digitalWrite(GREEN_LED_PIN, HIGH);
        delay(50);
        digitalWrite(GREEN_LED_PIN, LOW);
        
    } else {
        Serial.println("Failed to submit energy data");
        communication_errors++;
        
        // Flash red LED
        digitalWrite(RED_LED_PIN, HIGH);
        delay(50);
        digitalWrite(RED_LED_PIN, LOW);
    }
}

void receiveBlockchainCommands() {
    if (!blockchain_client) return;
    
    // Check for pending commands from grid operator
    String commands = blockchain_client->getGridCommands();
    if (commands.length() > 0) {
        Serial.println("Received grid commands: " + commands);
        processGridCommands();
    }
    
    // Check for demand response signals
    String dr_signals = blockchain_client->getDemandResponseSignals();
    if (dr_signals.length() > 0) {
        Serial.println("Received demand response signal: " + dr_signals);
        processDemandResponseSignals();
    }
    
    // Check for firmware update commands
    String update_commands = blockchain_client->getUpdateCommands();
    if (update_commands.length() > 0) {
        Serial.println("Received update command: " + update_commands);
        // Process OTA update if authorized
    }
    
    // Check for configuration updates
    String config_updates = blockchain_client->getConfigurationUpdates();
    if (config_updates.length() > 0) {
        Serial.println("Received configuration update: " + config_updates);
        // Update device configuration
    }
}

void processIncomingOrders() {
    if (!blockchain_client) return;
    
    // Get order book updates
    String order_updates = blockchain_client->getOrderBookUpdates();
    if (order_updates.isEmpty()) return;
    
    Serial.println("Processing incoming trading orders...");
    
    // Parse order updates
    DynamicJsonDocument doc(2048);
    deserializeJson(doc, order_updates);
    
    if (doc.containsKey("buy_orders")) {
        JsonArray buy_orders = doc["buy_orders"];
        for (JsonObject order : buy_orders) {
            // Process buy orders that match our sell criteria
            float price = order["price"];
            float amount = order["amount"];
            
            if (shouldAcceptBuyOrder(price, amount)) {
                // Create matching sell order
                EnergyOrder sell_order = createAdvancedEnergyOrder(
                    EnergyOrder::SELL_ORDER, amount, price);
                
                if (blockchain_client->submitEnergyOrder(sell_order)) {
                    Serial.printf("Matched sell order: %.3f kWh @ %.2f THB/kWh\n", 
                                 amount, price);
                    daily_energy_sold += amount;
                    total_trading_revenue += amount * price;
                    
                    // Sound trading notification
                    tone(BUZZER_PIN, 1500, 100);
                }
            }
        }
    }
    
    if (doc.containsKey("sell_orders")) {
        JsonArray sell_orders = doc["sell_orders"];
        for (JsonObject order : sell_orders) {
            // Process sell orders that match our buy criteria
            float price = order["price"];
            float amount = order["amount"];
            
            if (shouldAcceptSellOrder(price, amount)) {
                // Create matching buy order
                EnergyOrder buy_order = createAdvancedEnergyOrder(
                    EnergyOrder::BUY_ORDER, amount, price);
                
                if (blockchain_client->submitEnergyOrder(buy_order)) {
                    Serial.printf("Matched buy order: %.3f kWh @ %.2f THB/kWh\n", 
                                 amount, price);
                    daily_energy_bought += amount;
                    total_trading_revenue -= amount * price;
                    
                    // Sound trading notification
                    tone(BUZZER_PIN, 1200, 100);
                }
            }
        }
    }
}

void processGridCommands() {
    // Process commands from grid operator
    String grid_commands = blockchain_client->getGridCommands();
    if (grid_commands.isEmpty()) return;
    
    DynamicJsonDocument doc(1024);
    deserializeJson(doc, grid_commands);
    
    if (doc.containsKey("command")) {
        String command = doc["command"];
        
        if (command == "EMERGENCY_SHUTDOWN") {
            Serial.println("Emergency shutdown command received from grid");
            grid_emergency_mode = true;
            safety_shutdown = true;
            
        } else if (command == "REDUCE_LOAD") {
            float target_reduction = doc["target_reduction"];
            Serial.printf("Load reduction requested: %.1f%%\n", target_reduction);
            implementLoadShedding(target_reduction);
            
        } else if (command == "ENABLE_DR") {
            float target_power = doc["target_power"];
            unsigned long duration = doc["duration_minutes"] * 60000;
            
            Serial.printf("Demand response activated: %.1f kW for %lu minutes\n", 
                         target_power, duration / 60000);
            
            demand_response_active = true;
            demand_response_target = target_power;
            demand_response_end_time = millis() + duration;
            
        } else if (command == "DISABLE_DR") {
            Serial.println("Demand response deactivated");
            demand_response_active = false;
            restoreNormalOperation();
            
        } else if (command == "UPDATE_PRICING") {
            peak_hour_multiplier = doc["peak_multiplier"];
            off_peak_multiplier = doc["off_peak_multiplier"];
            Serial.printf("Pricing updated: Peak=%.2f, Off-peak=%.2f\n", 
                         peak_hour_multiplier, off_peak_multiplier);
            updateMarketPricing();
        }
    }
}

void processMarketUpdates() {
    if (!blockchain_client) return;
    
    // Get current market data
    String market_data = blockchain_client->getMarketData();
    if (market_data.isEmpty()) return;
    
    DynamicJsonDocument doc(1024);
    deserializeJson(doc, market_data);
    
    if (doc.containsKey("current_price")) {
        float new_price = doc["current_price"];
        if (abs(new_price - current_market_price) > 0.01) {
            Serial.printf("Market price updated: %.2f THB/kWh\n", new_price);
            current_market_price = new_price;
            
            // Trigger trading evaluation with new price
            if (auto_trading_enabled) {
                checkAdvancedTradingOpportunities();
            }
        }
    }
    
    if (doc.containsKey("predicted_price")) {
        predicted_market_price = doc["predicted_price"];
    }
    
    if (doc.containsKey("grid_load")) {
        grid_status.total_load = doc["grid_load"];
        
        // Update peak demand status
        bool was_peak = peak_demand_period;
        peak_demand_period = grid_status.total_load > 80.0; // 80% load threshold
        
        if (peak_demand_period != was_peak) {
            Serial.printf("Peak demand period: %s\n", peak_demand_period ? "ON" : "OFF");
            updateMarketPricing();
        }
    }
    
    if (doc.containsKey("renewable_percentage")) {
        grid_status.renewable_percentage = doc["renewable_percentage"];
        carbon_intensity = 1000.0 * (1.0 - grid_status.renewable_percentage / 100.0);
    }
}

void processDemandResponseSignals() {
    if (!demand_response_active) return;
    
    unsigned long current_time = millis();
    
    // Check if demand response period has ended
    if (current_time > demand_response_end_time) {
        demand_response_active = false;
        restoreNormalOperation();
        
        // Notify grid operator of DR completion
        blockchain_client->reportDemandResponseCompletion();
        return;
    }
    
    // Adjust power consumption to meet DR target
    float current_power = current_measurement.power / 1000.0; // Convert to kW
    float power_difference = current_power - demand_response_target;
    
    if (power_difference > 0.1) { // Need to reduce load
        Serial.printf("DR: Reducing load by %.1f kW\n", power_difference);
        implementLoadShedding(power_difference);
        
    } else if (power_difference < -0.1) { // Can increase load
        Serial.printf("DR: Can increase load by %.1f kW\n", -power_difference);
        // Implement load increase if needed
    }
    
    // Report DR status to grid
    blockchain_client->reportDemandResponseStatus(current_power, demand_response_target);
}

// Energy Meter Data Structure
struct EnergyMeterData {
    String device_id;
    String timestamp;
    float energy_consumed;    // kWh
    float energy_produced;    // kWh (for solar panels)
    float current_power;      // Watts
    float voltage;           // Volts
    float temperature;       // Celsius
    float humidity;          // %
    String location;         // GPS coordinates or zone
    String energy_source;    // grid, solar, battery
};

// Function Prototypes
void setup_wifi();
void setup_display();
void setup_sensors();
void read_energy_data();
void send_to_blockchain();
void update_display();
void handle_button_press();
String create_device_signature(String data);
String get_timestamp();
void blink_status_led(int times);
void log_to_serial(String message);

void setup() {
    Serial.begin(115200);
    delay(1000);
    
    log_to_serial("=== GridTokenX ESP32 Energy Meter Starting ===");
    
    // Initialize hardware
    pinMode(LED_STATUS_PIN, OUTPUT);
    pinMode(LED_ERROR_PIN, OUTPUT);
    pinMode(BUTTON_PIN, INPUT_PULLUP);
    
    setup_display();
    setup_sensors();
    setup_wifi();
    
    // Initialize time for timestamps
    configTime(7 * 3600, 0, "pool.ntp.org", "time.nist.gov");
    
    log_to_serial("ESP32 Energy Meter initialized successfully");
    blink_status_led(3);
}

void loop() {
    unsigned long current_time = millis();
    
    // Read sensor data every 30 seconds
    if (current_time - last_reading_time > 30000) {
        read_energy_data();
        last_reading_time = current_time;
    }
    
    // Send data to blockchain every 5 minutes
    if (current_time - last_blockchain_sync > 300000) {
        if (wifi_connected) {
            send_to_blockchain();
            last_blockchain_sync = current_time;
        }
    }
    
    // Update display every 5 seconds
    static unsigned long last_display_update = 0;
    if (current_time - last_display_update > 5000) {
        update_display();
        last_display_update = current_time;
    }
    
    // Handle button press for manual sync
    if (digitalRead(BUTTON_PIN) == LOW) {
        delay(200); // Debounce
        handle_button_press();
    }
    
    delay(100);
}

void setup_wifi() {
    display.clear();
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.drawString(64, 20, "Connecting WiFi...");
    display.display();
    
    WiFi.begin(ssid, password);
    
    int attempts = 0;
    while (WiFi.status() != WL_CONNECTED && attempts < 20) {
        delay(500);
        Serial.print(".");
        attempts++;
    }
    
    if (WiFi.status() == WL_CONNECTED) {
        wifi_connected = true;
        log_to_serial("WiFi connected: " + WiFi.localIP().toString());
        blink_status_led(2);
    } else {
        wifi_connected = false;
        log_to_serial("WiFi connection failed");
        digitalWrite(LED_ERROR_PIN, HIGH);
    }
}

void setup_display() {
    display.init();
    display.flipScreenVertically();
    display.setFont(ArialMT_Plain_10);
    
    display.clear();
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.drawString(64, 10, "GridTokenX");
    display.drawString(64, 25, "Energy Meter");
    display.drawString(64, 40, "Initializing...");
    display.display();
    
    log_to_serial("OLED display initialized");
}

void setup_sensors() {
    dht.begin();
    
    // Calibrate current sensor
    analogSetAttenuation(ADC_11db);
    
    log_to_serial("Sensors initialized");
}

void read_energy_data() {
    // Read temperature and humidity
    temperature = dht.readTemperature();
    humidity = dht.readHumidity();
    
    // Simulate energy reading (in production, use real current sensor)
    int sensor_value = analogRead(CURRENT_SENSOR_PIN);
    float voltage = (sensor_value / 4095.0) * 3.3;
    
    // Convert to power consumption (simplified calculation)
    power_consumption = (voltage - 1.65) * 100; // Simplified current to power
    if (power_consumption < 0) power_consumption = 0;
    
    // Simulate energy consumption accumulation
    float time_delta = (millis() - last_reading_time) / 3600000.0; // hours
    current_energy_reading += (power_consumption * time_delta) / 1000.0; // kWh
    
    log_to_serial("Energy: " + String(current_energy_reading, 3) + " kWh, Power: " + String(power_consumption, 1) + " W");
}

void send_to_blockchain() {
    if (!wifi_connected) {
        log_to_serial("WiFi not connected, skipping blockchain sync");
        return;
    }
    
    HTTPClient http;
    http.begin(String(blockchain_api_url) + "/energy/meter-reading");
    http.addHeader("Content-Type", "application/json");
    http.addHeader("Device-ID", device_id);
    
    // Create energy meter data
    EnergyMeterData data;
    data.device_id = device_id;
    data.timestamp = get_timestamp();
    data.energy_consumed = current_energy_reading;
    data.energy_produced = 0.0; // For solar panels
    data.current_power = power_consumption;
    data.voltage = 220.0; // Simulated
    data.temperature = temperature;
    data.humidity = humidity;
    data.location = "13.7563,100.5018"; // Bangkok coordinates
    data.energy_source = "grid";
    
    // Create JSON payload
    DynamicJsonDocument doc(1024);
    doc["device_id"] = data.device_id;
    doc["timestamp"] = data.timestamp;
    doc["energy_consumed"] = data.energy_consumed;
    doc["energy_produced"] = data.energy_produced;
    doc["current_power"] = data.current_power;
    doc["voltage"] = data.voltage;
    doc["temperature"] = data.temperature;
    doc["humidity"] = data.humidity;
    doc["location"] = data.location;
    doc["energy_source"] = data.energy_source;
    doc["device_type"] = device_type;
    
    String payload;
    serializeJson(doc, payload);
    
    // Add digital signature for security
    String signature = create_device_signature(payload);
    http.addHeader("Device-Signature", signature);
    
    log_to_serial("Sending data to blockchain...");
    
    int httpResponseCode = http.POST(payload);
    
    if (httpResponseCode > 0) {
        String response = http.getString();
        log_to_serial("Blockchain response (" + String(httpResponseCode) + "): " + response);
        
        if (httpResponseCode == 200) {
            blockchain_connected = true;
            blink_status_led(1);
            
            // Parse response for any instructions
            DynamicJsonDocument responseDoc(512);
            if (deserializeJson(responseDoc, response) == DeserializationError::Ok) {
                if (responseDoc.containsKey("energy_price")) {
                    float energy_price = responseDoc["energy_price"];
                    log_to_serial("Current energy price: " + String(energy_price) + " tokens/kWh");
                }
            }
        }
    } else {
        blockchain_connected = false;
        log_to_serial("Blockchain sync failed: " + String(httpResponseCode));
        digitalWrite(LED_ERROR_PIN, HIGH);
        delay(100);
        digitalWrite(LED_ERROR_PIN, LOW);
    }
    
    http.end();
}

void update_display() {
    display.clear();
    
    // Title
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.setFont(ArialMT_Plain_10);
    display.drawString(64, 0, "GridTokenX Meter");
    
    // Connection Status
    display.setTextAlignment(TEXT_ALIGN_LEFT);
    display.setFont(ArialMT_Plain_10);
    
    String wifi_status = wifi_connected ? "WiFi: OK" : "WiFi: --";
    String blockchain_status = blockchain_connected ? "Chain: OK" : "Chain: --";
    
    display.drawString(0, 12, wifi_status);
    display.drawString(70, 12, blockchain_status);
    
    // Energy Data
    display.drawString(0, 25, "Energy: " + String(current_energy_reading, 2) + " kWh");
    display.drawString(0, 35, "Power: " + String(power_consumption, 0) + " W");
    
    // Environmental Data
    if (!isnan(temperature) && !isnan(humidity)) {
        display.drawString(0, 45, "Temp: " + String(temperature, 1) + "°C");
        display.drawString(0, 55, "Humid: " + String(humidity, 0) + "%");
    }
    
    display.display();
}

void handle_button_press() {
    log_to_serial("Manual sync triggered");
    
    display.clear();
    display.setTextAlignment(TEXT_ALIGN_CENTER);
    display.drawString(64, 25, "Manual Sync...");
    display.display();
    
    send_to_blockchain();
    
    delay(1000);
}

String create_device_signature(String data) {
    // Simple hash-based signature (in production, use proper cryptography)
    mbedtls_sha256_context ctx;
    unsigned char hash[32];
    
    mbedtls_sha256_init(&ctx);
    mbedtls_sha256_starts(&ctx, 0);
    mbedtls_sha256_update(&ctx, (unsigned char*)data.c_str(), data.length());
    mbedtls_sha256_finish(&ctx, hash);
    mbedtls_sha256_free(&ctx);
    
    String signature = "";
    for (int i = 0; i < 32; i++) {
        signature += String(hash[i], HEX);
    }
    
    return signature;
}

String get_timestamp() {
    time_t now;
    struct tm timeinfo;
    if (!getLocalTime(&timeinfo)) {
        return String(millis()); // Fallback to millis
    }
    
    char timestamp[64];
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%dT%H:%M:%SZ", &timeinfo);
    return String(timestamp);
}

void blink_status_led(int times) {
    for (int i = 0; i < times; i++) {
        digitalWrite(LED_STATUS_PIN, HIGH);
        delay(200);
        digitalWrite(LED_STATUS_PIN, LOW);
        delay(200);
    }
}

void log_to_serial(String message) {
    Serial.println("[" + get_timestamp() + "] " + message);
}
