# GridTokenX ESP32 Deployment Guide

## 🚀 Production Deployment Guide

This guide covers deploying the GridTokenX ESP32 Smart Energy Meter POC prototype from development to production environments.

## 📋 Pre-Deployment Checklist

### ✅ Development Validation
- [ ] All tests passed (hardware, software, network, blockchain)
- [ ] Code review completed
- [ ] Performance benchmarks met
- [ ] Security audit passed
- [ ] Documentation updated
- [ ] Configuration validated for production environment

### ✅ Hardware Preparation
- [ ] ESP32 DevKit v1 with stable power supply
- [ ] All sensors calibrated and tested
- [ ] OLED display functional
- [ ] Enclosure weatherproofing (if outdoor installation)
- [ ] Backup hardware units available

### ✅ Network Infrastructure
- [ ] Stable WiFi network with adequate signal strength (-70 dBm or better)
- [ ] Internet connectivity verified
- [ ] Firewall rules configured for blockchain API access
- [ ] Network security measures in place

### ✅ Blockchain Infrastructure
- [ ] GridTokenX blockchain node operational
- [ ] API endpoints accessible and responsive
- [ ] Device API keys generated and secure
- [ ] Account balances and permissions configured

---

## 🔧 Production Configuration

### 1. Environment-Specific Settings

#### Production Config Template (`config_production.h`)
```cpp
#ifndef CONFIG_PRODUCTION_H
#define CONFIG_PRODUCTION_H

// Production Environment Settings
#define ENVIRONMENT "PRODUCTION"
#define DEBUG_LEVEL 2  // Reduced logging for production
#define ENABLE_SERIAL_OUTPUT false  // Disable for deployment

// Production WiFi Configuration
#define WIFI_SSID "GridTokenX_Production_WiFi"
#define WIFI_PASSWORD "your_secure_production_password"
#define WIFI_TIMEOUT_MS 15000
#define WIFI_RETRY_ATTEMPTS 5

// Production Blockchain Configuration  
#define BLOCKCHAIN_API_HOST "blockchain.gridtokenx.co.th"
#define BLOCKCHAIN_API_PORT 443  // HTTPS for production
#define BLOCKCHAIN_USE_HTTPS true
#define API_KEY "prod_api_key_32_characters_long"

// Production Device Configuration
#define DEVICE_ID "ESP32_PROD_METER_[UNIQUE_ID]"
#define DEVICE_LOCATION "13.7563,100.5018"  // Actual installation coordinates
#define DEVICE_ZONE "MEA-BANGKOK-ZONE-1"   // Production grid zone
#define DEVICE_OWNER_ID "CUSTOMER_12345"

// Production Trading Configuration
#define ENABLE_AUTO_TRADING true
#define EXCESS_ENERGY_THRESHOLD 0.5    // Conservative for production
#define ENERGY_DEMAND_THRESHOLD 0.3
#define MAX_TRADING_PRICE 5500         // Production market rates
#define MIN_TRADING_PRICE 3500
#define TRADING_COOLDOWN_MS 300000     // 5 minutes between trades

// Production Safety Configuration
#define VOLTAGE_SAFETY_MAX 250.0       // 250V maximum
#define VOLTAGE_SAFETY_MIN 200.0       // 200V minimum  
#define CURRENT_SAFETY_MAX 25.0        // 25A maximum
#define POWER_SAFETY_MAX 5000.0        // 5kW maximum
#define TEMPERATURE_SAFETY_MAX 60.0    // 60°C maximum

// Production Data Submission
#define DATA_SUBMISSION_INTERVAL_MS 60000   // 1 minute intervals
#define DATA_BATCH_SIZE 10                  // Batch submissions
#define DATA_RETRY_ATTEMPTS 3
#define DATA_RETRY_DELAY_MS 5000

// Production Monitoring
#define HEARTBEAT_INTERVAL_MS 300000     // 5 minutes
#define STATUS_REPORT_INTERVAL_MS 3600000 // 1 hour
#define ERROR_REPORTING_ENABLED true

// Production Security
#define ENABLE_OTA_UPDATES false         // Disable OTA in production
#define ENABLE_WEB_SERVER false          // Disable web interface
#define ENABLE_TELNET_DEBUG false        // Disable remote debug

#endif // CONFIG_PRODUCTION_H
```

### 2. Secure Credential Management

#### API Key Management
```cpp
// Store API keys securely in EEPROM or SPIFFS
void storeSecureCredentials() {
  preferences.begin("gridtokenx", false);
  
  // Encrypt API key before storage
  String encryptedKey = encryptApiKey(API_KEY);
  preferences.putString("api_key", encryptedKey);
  
  // Store device certificate
  preferences.putString("device_cert", DEVICE_CERTIFICATE);
  
  preferences.end();
}

String getSecureApiKey() {
  preferences.begin("gridtokenx", true);
  String encryptedKey = preferences.getString("api_key", "");
  preferences.end();
  
  return decryptApiKey(encryptedKey);
}
```

### 3. Production Logging Configuration

#### Logging System
```cpp
// Production logging levels
typedef enum {
  LOG_LEVEL_ERROR = 1,
  LOG_LEVEL_WARN = 2,
  LOG_LEVEL_INFO = 3,
  LOG_LEVEL_DEBUG = 4
} LogLevel;

void logMessage(LogLevel level, const char* message) {
  if (level <= PRODUCTION_LOG_LEVEL) {
    String timestamp = getISOTimestamp();
    String deviceId = String(DEVICE_ID);
    
    // Format: [TIMESTAMP] [DEVICE_ID] [LEVEL] MESSAGE
    String logEntry = String("[") + timestamp + "] [" + deviceId + "] ";
    
    switch (level) {
      case LOG_LEVEL_ERROR: logEntry += "[ERROR] "; break;
      case LOG_LEVEL_WARN:  logEntry += "[WARN]  "; break;
      case LOG_LEVEL_INFO:  logEntry += "[INFO]  "; break;
      case LOG_LEVEL_DEBUG: logEntry += "[DEBUG] "; break;
    }
    
    logEntry += message;
    
    // Production logging options:
    // 1. Send to blockchain logging service
    // 2. Store locally and batch upload
    // 3. Send to syslog server
    sendLogToBlockchain(logEntry);
  }
}

// Usage examples
logMessage(LOG_LEVEL_INFO, "Device started successfully");
logMessage(LOG_LEVEL_ERROR, "Sensor reading failed");
logMessage(LOG_LEVEL_WARN, "High temperature detected");
```

---

## 🏭 Deployment Procedures

### Phase 1: Pre-Production Testing

#### 1. Staging Environment Setup
```bash
# Create staging configuration
cp include/config.h include/config_staging.h

# Edit staging config with test environment settings
# - Use staging blockchain node
# - Enable debug logging
# - Use test API keys
# - Set conservative trading limits
```

#### 2. Staging Deployment Test
```cpp
// Staging test procedure
void runStagingTests() {
  logMessage(LOG_LEVEL_INFO, "Starting staging deployment test");
  
  // Test 1: Basic functionality
  if (!testBasicFunctionality()) {
    logMessage(LOG_LEVEL_ERROR, "Basic functionality test failed");
    return;
  }
  
  // Test 2: Blockchain integration
  if (!testBlockchainIntegration()) {
    logMessage(LOG_LEVEL_ERROR, "Blockchain integration test failed");
    return;
  }
  
  // Test 3: Trading operations
  if (!testTradingOperations()) {
    logMessage(LOG_LEVEL_ERROR, "Trading operations test failed");
    return;
  }
  
  // Test 4: Long-running stability (24 hours)
  if (!testLongRunningStability()) {
    logMessage(LOG_LEVEL_ERROR, "Stability test failed");
    return;
  }
  
  logMessage(LOG_LEVEL_INFO, "All staging tests passed - ready for production");
}
```

### Phase 2: Production Deployment

#### 1. Firmware Preparation
```bash
# Build production firmware
pio run -e production

# Generate firmware checksum
sha256sum .pio/build/production/firmware.bin > firmware.sha256

# Sign firmware (if using secure boot)
espsecure.py sign_data --keyfile private_key.pem --output firmware_signed.bin firmware.bin
```

#### 2. Device Provisioning
```cpp
void provisionProductionDevice() {
  logMessage(LOG_LEVEL_INFO, "Starting device provisioning");
  
  // 1. Generate unique device ID
  String macAddress = WiFi.macAddress();
  String deviceId = "ESP32_PROD_" + macAddress.substring(9);
  deviceId.replace(":", "");
  
  // 2. Store device configuration
  preferences.begin("device", false);
  preferences.putString("device_id", deviceId);
  preferences.putString("provision_date", getISOTimestamp());
  preferences.putString("firmware_version", FIRMWARE_VERSION);
  preferences.end();
  
  // 3. Register with blockchain
  if (registerDeviceWithBlockchain(deviceId)) {
    logMessage(LOG_LEVEL_INFO, "Device registered successfully");
  } else {
    logMessage(LOG_LEVEL_ERROR, "Device registration failed");
  }
  
  // 4. Perform initial calibration
  if (performSensorCalibration()) {
    logMessage(LOG_LEVEL_INFO, "Sensor calibration completed");
  } else {
    logMessage(LOG_LEVEL_WARN, "Sensor calibration needs manual adjustment");
  }
  
  logMessage(LOG_LEVEL_INFO, "Device provisioning completed");
}
```

#### 3. Installation Procedure

##### Step 1: Site Preparation
```
1. Verify electrical installation meets safety standards
2. Ensure stable internet connectivity at installation site
3. Prepare weatherproof enclosure for outdoor installations
4. Document installation location coordinates
5. Verify access to electrical panel for sensor installation
```

##### Step 2: Hardware Installation
```cpp
// Installation validation checklist
void validateInstallation() {
  logMessage(LOG_LEVEL_INFO, "Starting installation validation");
  
  // Check power supply stability
  float vcc = readSupplyVoltage();
  if (vcc < 4.8 || vcc > 5.2) {
    logMessage(LOG_LEVEL_ERROR, "Power supply voltage out of range: " + String(vcc) + "V");
  }
  
  // Check sensor connections
  if (!validateSensorConnections()) {
    logMessage(LOG_LEVEL_ERROR, "Sensor connection validation failed");
  }
  
  // Check WiFi signal strength
  int rssi = WiFi.RSSI();
  if (rssi < -80) {
    logMessage(LOG_LEVEL_WARN, "WiFi signal weak: " + String(rssi) + " dBm");
  }
  
  // Check ambient conditions
  float temperature = readAmbientTemperature();
  if (temperature < 0 || temperature > 50) {
    logMessage(LOG_LEVEL_WARN, "Ambient temperature out of optimal range: " + String(temperature) + "°C");
  }
  
  logMessage(LOG_LEVEL_INFO, "Installation validation completed");
}
```

##### Step 3: Configuration and Testing
```cpp
void configureProductionDevice() {
  // 1. Set production configuration
  loadProductionConfig();
  
  // 2. Connect to production WiFi
  if (!connectToProductionWiFi()) {
    logMessage(LOG_LEVEL_ERROR, "Failed to connect to production WiFi");
    return;
  }
  
  // 3. Connect to production blockchain
  if (!connectToProductionBlockchain()) {
    logMessage(LOG_LEVEL_ERROR, "Failed to connect to production blockchain");
    return;
  }
  
  // 4. Perform system self-test
  if (!performSystemSelfTest()) {
    logMessage(LOG_LEVEL_ERROR, "System self-test failed");
    return;
  }
  
  // 5. Start data collection
  startDataCollection();
  
  logMessage(LOG_LEVEL_INFO, "Production device configured and operational");
}
```

---

## 📊 Monitoring and Maintenance

### 1. Production Monitoring Dashboard

#### Device Status Monitoring
```cpp
void sendStatusReport() {
  StaticJsonDocument<1024> statusDoc;
  
  // Device information
  statusDoc["device_id"] = DEVICE_ID;
  statusDoc["timestamp"] = getISOTimestamp();
  statusDoc["firmware_version"] = FIRMWARE_VERSION;
  statusDoc["uptime_ms"] = millis();
  
  // System health
  statusDoc["free_heap"] = ESP.getFreeHeap();
  statusDoc["wifi_rssi"] = WiFi.RSSI();
  statusDoc["wifi_connected"] = WiFi.isConnected();
  
  // Sensor status
  statusDoc["sensors"]["voltage_ok"] = isVoltageSensorWorking();
  statusDoc["sensors"]["current_ok"] = isCurrentSensorWorking();
  statusDoc["sensors"]["temperature_ok"] = isTemperatureSensorWorking();
  
  // Energy data summary
  statusDoc["energy"]["voltage"] = lastVoltageReading;
  statusDoc["energy"]["current"] = lastCurrentReading;
  statusDoc["energy"]["power"] = lastPowerReading;
  statusDoc["energy"]["daily_energy"] = dailyEnergyTotal;
  
  // Trading summary
  statusDoc["trading"]["orders_today"] = dailyOrderCount;
  statusDoc["trading"]["energy_sold"] = dailyEnergySold;
  statusDoc["trading"]["energy_bought"] = dailyEnergyBought;
  statusDoc["trading"]["revenue"] = dailyRevenue;
  
  // Error counts
  statusDoc["errors"]["sensor_errors"] = sensorErrorCount;
  statusDoc["errors"]["network_errors"] = networkErrorCount;
  statusDoc["errors"]["blockchain_errors"] = blockchainErrorCount;
  
  String statusJson;
  serializeJson(statusDoc, statusJson);
  
  // Send to monitoring service
  sendStatusToMonitoring(statusJson);
}
```

### 2. Automated Health Checks

#### Continuous Health Monitoring
```cpp
void performHealthCheck() {
  bool systemHealthy = true;
  
  // Check memory usage
  uint32_t freeHeap = ESP.getFreeHeap();
  if (freeHeap < 50000) {  // Less than 50KB
    logMessage(LOG_LEVEL_WARN, "Low memory warning: " + String(freeHeap) + " bytes");
    systemHealthy = false;
  }
  
  // Check WiFi connectivity
  if (!WiFi.isConnected()) {
    logMessage(LOG_LEVEL_ERROR, "WiFi disconnected");
    systemHealthy = false;
    
    // Attempt reconnection
    reconnectWiFi();
  }
  
  // Check blockchain connectivity
  if (!blockchainClient.checkAPIStatus()) {
    logMessage(LOG_LEVEL_ERROR, "Blockchain API unreachable");
    systemHealthy = false;
  }
  
  // Check sensor readings
  if (!validateSensorReadings()) {
    logMessage(LOG_LEVEL_ERROR, "Sensor readings invalid");
    systemHealthy = false;
  }
  
  // Update system status
  updateSystemStatusLED(systemHealthy);
  
  // Send alert if unhealthy for extended period
  if (!systemHealthy) {
    consecutiveHealthFailures++;
    if (consecutiveHealthFailures >= 5) {
      sendCriticalAlert("System health check failing consistently");
    }
  } else {
    consecutiveHealthFailures = 0;
  }
}
```

### 3. Remote Diagnostics

#### Diagnostic Data Collection
```cpp
void collectDiagnosticData() {
  StaticJsonDocument<2048> diagnostics;
  
  // System information
  diagnostics["system"]["chip_revision"] = ESP.getChipRevision();
  diagnostics["system"]["cpu_freq_mhz"] = ESP.getCpuFreqMHz();
  diagnostics["system"]["flash_size"] = ESP.getFlashChipSize();
  diagnostics["system"]["free_heap"] = ESP.getFreeHeap();
  diagnostics["system"]["largest_free_block"] = ESP.getMaxAllocHeap();
  
  // Network diagnostics
  diagnostics["network"]["mac_address"] = WiFi.macAddress();
  diagnostics["network"]["ip_address"] = WiFi.localIP().toString();
  diagnostics["network"]["subnet"] = WiFi.subnetMask().toString();
  diagnostics["network"]["gateway"] = WiFi.gatewayIP().toString();
  diagnostics["network"]["dns"] = WiFi.dnsIP().toString();
  diagnostics["network"]["rssi"] = WiFi.RSSI();
  
  // Sensor diagnostics
  diagnostics["sensors"]["voltage_adc"] = analogRead(VOLTAGE_SENSOR_PIN);
  diagnostics["sensors"]["current_adc"] = analogRead(CURRENT_SENSOR_PIN);
  diagnostics["sensors"]["temperature"] = dht.readTemperature();
  diagnostics["sensors"]["humidity"] = dht.readHumidity();
  
  // Performance metrics
  diagnostics["performance"]["loop_time_avg"] = averageLoopTime;
  diagnostics["performance"]["loop_time_max"] = maxLoopTime;
  diagnostics["performance"]["api_calls_success"] = apiCallsSuccess;
  diagnostics["performance"]["api_calls_failed"] = apiCallsFailed;
  
  // Error statistics
  diagnostics["errors"]["total_errors"] = totalErrorCount;
  diagnostics["errors"]["network_timeouts"] = networkTimeoutCount;
  diagnostics["errors"]["sensor_failures"] = sensorFailureCount;
  diagnostics["errors"]["memory_errors"] = memoryErrorCount;
  
  String diagnosticsJson;
  serializeJson(diagnostics, diagnosticsJson);
  
  // Send to diagnostic service
  sendDiagnosticData(diagnosticsJson);
}
```

---

## 🔄 Update and Maintenance Procedures

### 1. Firmware Updates

#### Secure Update Process
```cpp
bool performSecureUpdate(String firmwareUrl, String expectedHash) {
  logMessage(LOG_LEVEL_INFO, "Starting secure firmware update");
  
  // 1. Download firmware
  HTTPClient http;
  http.begin(firmwareUrl);
  int httpCode = http.GET();
  
  if (httpCode != 200) {
    logMessage(LOG_LEVEL_ERROR, "Failed to download firmware: " + String(httpCode));
    return false;
  }
  
  // 2. Verify checksum
  String downloadedHash = calculateSHA256(http.getStream());
  if (downloadedHash != expectedHash) {
    logMessage(LOG_LEVEL_ERROR, "Firmware checksum mismatch");
    return false;
  }
  
  // 3. Perform update
  if (!Update.begin(UPDATE_SIZE_UNKNOWN)) {
    logMessage(LOG_LEVEL_ERROR, "Update initialization failed");
    return false;
  }
  
  size_t written = Update.writeStream(http.getStream());
  
  if (!Update.end(true)) {
    logMessage(LOG_LEVEL_ERROR, "Update installation failed");
    return false;
  }
  
  logMessage(LOG_LEVEL_INFO, "Firmware update successful - rebooting");
  ESP.restart();
  
  return true;
}
```

### 2. Configuration Updates

#### Dynamic Configuration Management
```cpp
void updateConfiguration(String configJson) {
  StaticJsonDocument<1024> config;
  deserializeJson(config, configJson);
  
  // Update trading parameters
  if (config.containsKey("trading")) {
    EXCESS_ENERGY_THRESHOLD = config["trading"]["excess_threshold"];
    ENERGY_DEMAND_THRESHOLD = config["trading"]["demand_threshold"];
    MAX_TRADING_PRICE = config["trading"]["max_price"];
    MIN_TRADING_PRICE = config["trading"]["min_price"];
    
    logMessage(LOG_LEVEL_INFO, "Trading configuration updated");
  }
  
  // Update monitoring intervals
  if (config.containsKey("monitoring")) {
    DATA_SUBMISSION_INTERVAL_MS = config["monitoring"]["data_interval"];
    STATUS_REPORT_INTERVAL_MS = config["monitoring"]["status_interval"];
    
    logMessage(LOG_LEVEL_INFO, "Monitoring configuration updated");
  }
  
  // Save updated configuration
  saveConfigurationToEEPROM();
}
```

### 3. Calibration Updates

#### Remote Sensor Calibration
```cpp
void performRemoteCalibration(String calibrationData) {
  StaticJsonDocument<512> calibration;
  deserializeJson(calibration, calibrationData);
  
  // Update voltage sensor calibration
  if (calibration.containsKey("voltage")) {
    VOLTAGE_SENSOR_RATIO = calibration["voltage"]["ratio"];
    VOLTAGE_SENSOR_OFFSET = calibration["voltage"]["offset"];
  }
  
  // Update current sensor calibration
  if (calibration.containsKey("current")) {
    ACS712_SENSITIVITY = calibration["current"]["sensitivity"];
    CURRENT_SENSOR_OFFSET = calibration["current"]["offset"];
  }
  
  // Validate calibration
  if (validateCalibration()) {
    saveCalibrationToEEPROM();
    logMessage(LOG_LEVEL_INFO, "Remote calibration completed successfully");
  } else {
    logMessage(LOG_LEVEL_ERROR, "Remote calibration validation failed");
  }
}
```

---

## 🚨 Emergency Procedures

### 1. Emergency Shutdown

#### Safe System Shutdown
```cpp
void emergencyShutdown(String reason) {
  logMessage(LOG_LEVEL_ERROR, "EMERGENCY SHUTDOWN: " + reason);
  
  // 1. Stop all trading operations
  stopAutomaticTrading();
  
  // 2. Send emergency alert
  sendEmergencyAlert(reason);
  
  // 3. Save critical data
  saveCriticalDataToEEPROM();
  
  // 4. Shut down safely
  display.clearDisplay();
  display.setCursor(0, 0);
  display.println("EMERGENCY STOP");
  display.setCursor(0, 20);
  display.println(reason);
  display.display();
  
  // Flash error LED
  while (true) {
    digitalWrite(RED_LED_PIN, HIGH);
    delay(500);
    digitalWrite(RED_LED_PIN, LOW);
    delay(500);
  }
}
```

### 2. Recovery Procedures

#### System Recovery
```cpp
void performSystemRecovery() {
  logMessage(LOG_LEVEL_INFO, "Starting system recovery");
  
  // 1. Reset configuration to defaults
  loadDefaultConfiguration();
  
  // 2. Clear error states
  clearAllErrorStates();
  
  // 3. Restart WiFi connection
  WiFi.disconnect();
  delay(1000);
  initializeWiFi();
  
  // 4. Restart blockchain connection
  blockchainClient.disconnect();
  delay(1000);
  blockchainClient.initialize();
  
  // 5. Restart data collection
  startDataCollection();
  
  logMessage(LOG_LEVEL_INFO, "System recovery completed");
}
```

---

## 📈 Production Optimization

### 1. Performance Tuning

#### Optimized Task Scheduling
```cpp
// Optimized main loop for production
void optimizedMainLoop() {
  static unsigned long lastSensorRead = 0;
  static unsigned long lastDataSubmission = 0;
  static unsigned long lastStatusReport = 0;
  static unsigned long lastHealthCheck = 0;
  
  unsigned long currentTime = millis();
  
  // High-frequency tasks (every 100ms)
  readSensors();
  updateDisplay();
  
  // Medium-frequency tasks (every 1 second)
  if (currentTime - lastSensorRead >= 1000) {
    processSensorData();
    checkTradingConditions();
    lastSensorRead = currentTime;
  }
  
  // Low-frequency tasks (every minute)
  if (currentTime - lastDataSubmission >= DATA_SUBMISSION_INTERVAL_MS) {
    submitEnergyData();
    lastDataSubmission = currentTime;
  }
  
  // Very low-frequency tasks
  if (currentTime - lastStatusReport >= STATUS_REPORT_INTERVAL_MS) {
    sendStatusReport();
    lastStatusReport = currentTime;
  }
  
  if (currentTime - lastHealthCheck >= 30000) {  // 30 seconds
    performHealthCheck();
    lastHealthCheck = currentTime;
  }
  
  // Yield to system tasks
  yield();
}
```

### 2. Resource Optimization

#### Memory Management
```cpp
void optimizeMemoryUsage() {
  // Use PROGMEM for constant strings
  const char statusMessages[] PROGMEM = {
    "System Normal\0"
    "Low Battery\0"
    "Sensor Error\0"
    "Network Error\0"
  };
  
  // Minimize JSON document sizes
  StaticJsonDocument<256> smallDoc;  // Instead of 1024
  
  // Reuse buffers
  static char buffer[512];
  memset(buffer, 0, sizeof(buffer));
  
  // Clear temporary objects
  sensors.clear();
  temporaryData.clear();
}
```

This comprehensive deployment guide ensures successful transition from development to production for the GridTokenX ESP32 Smart Energy Meter, with robust monitoring, maintenance, and emergency procedures for reliable operation.

---

**GridTokenX Production Deployment** - Scaling Smart Energy Infrastructure 🏭⚡
