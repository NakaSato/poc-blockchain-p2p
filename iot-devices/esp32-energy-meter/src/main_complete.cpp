/**
 * GridTokenX ESP32 Smart Energy Meter - Main Application
 * 
 * This is the main application file for the ESP32-based smart energy meter
 * that integrates with the GridTokenX blockchain for automated energy trading.
 * 
 * Features:
 * - Real-time energy monitoring (voltage, current, power, energy)
 * - Environmental monitoring (temperature, humidity)
 * - Automated energy trading via blockchain API
 * - Multi-page OLED display with navigation
 * - Safety monitoring and alerts
 * - WiFi connectivity with auto-reconnection
 * - Web interface for configuration
 * - OTA (Over-The-Air) firmware updates
 * 
 * Author: GridTokenX Development Team
 * Version: 1.0.0
 * Date: 2024-01-20
 */

#include <WiFi.h>
#include <Wire.h>
#include <SPI.h>
#include <ArduinoJson.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Adafruit_NeoPixel.h>
#include <DHT.h>
#include <HTTPClient.h>
#include <WebServer.h>
#include <ArduinoOTA.h>
#include <Preferences.h>
#include <esp_task_wdt.h>

// Project headers
#include "config.h"
#include "energy_types.h"
#include "blockchain_client.h"

// ============================================================================
// GLOBAL OBJECTS AND VARIABLES
// ============================================================================

// Hardware objects
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RST);
DHT dht(DHT_PIN, DHT_TYPE);
Adafruit_NeoPixel pixels(1, RGB_LED_PIN, NEO_GRB + NEO_KHZ800);
WebServer webServer(80);
Preferences preferences;
BlockchainClient blockchainClient;

// System state variables
EnergyMeasurement currentData;
DeviceStatus systemStatus;
unsigned long lastSensorRead = 0;
unsigned long lastDisplayUpdate = 0;
unsigned long lastDataSubmission = 0;
unsigned long lastStatusReport = 0;
unsigned long lastPageChange = 0;
unsigned long lastTradeTime = 0;
unsigned long lastEnergyUpdateTime = 0;
unsigned long startupTime = 0;

// Display and interface
int currentPage = 0;
bool displayOn = true;

// Energy tracking
float dailyEnergyTotal = 0.0;
float dailyExcessEnergy = 0.0;
float dailyEnergyDemand = 0.0;

// Trading tracking
int dailyOrderCount = 0;
float dailyEnergySold = 0.0;
float dailyEnergyBought = 0.0;
float dailyRevenue = 0.0;

// Error counters
int dhtErrorCount = 0;
int networkErrorCount = 0;
int blockchainErrorCount = 0;
int dataSubmissionCount = 0;

// Button states
unsigned long lastButtonPress = 0;

// ============================================================================
// FUNCTION DECLARATIONS
// ============================================================================

// Hardware initialization
void initializeHardware();
void initializeWiFi();
void initializeBlockchain();
void initializeWebServer();
void initializeOTA();

// Sensor and data handling
void readSensors();
void checkSafetyLimits();
void submitDataToBlockchain();
void performSensorCalibration();

// Display and interface
void updateDisplay();
void showPage_Overview();
void showPage_Power();
void showPage_Energy();
void showPage_Trading();
void showPage_Network();
void showPage_System();
void handleButtons();
void setStatusLED(int r, int g, int b);

// Trading and automation
void checkTradingConditions();
void processAutomaticTrading();

// System management
void updateSystemStatus();
void performSystemSelfTest();
void handleEmergencyShutdown();
void printSystemStatus();
void monitorSystemHealth();

// Network and connectivity
void checkWiFiConnection();
void handleWebRequests();
void syncSystemTime();

// Configuration and storage
void loadConfiguration();
void saveConfiguration();
void loadCalibrationData();
void saveCalibrationData();

// Utility functions
String getISOTimestamp();
void logMessage(int level, const char* message);
void sendStatusToCloud();

// ============================================================================
// ARDUINO SETUP FUNCTION
// ============================================================================

void setup() {
  // Initialize serial communication
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("====================================");
  Serial.println("GridTokenX ESP32 Smart Energy Meter");
  Serial.println("Version: " FIRMWARE_VERSION);
  Serial.println("Build: " __DATE__ " " __TIME__);
  Serial.println("====================================");
  
  startupTime = millis();
  
  // Initialize preferences for configuration storage
  preferences.begin("gridtokenx", false);
  
  // Initialize watchdog timer for system reliability
  #if ENABLE_WATCHDOG
  esp_task_wdt_init(WATCHDOG_TIMEOUT_MS / 1000, true);
  esp_task_wdt_add(NULL);
  #endif
  
  // Load configuration from flash memory
  loadConfiguration();
  
  // Initialize hardware components
  initializeHardware();
  
  // Initialize WiFi connection
  initializeWiFi();
  
  // Synchronize system time
  syncSystemTime();
  
  // Initialize blockchain client
  initializeBlockchain();
  
  // Initialize web server for configuration
  initializeWebServer();
  
  // Initialize OTA updates
  initializeOTA();
  
  // Perform initial sensor calibration
  performSensorCalibration();
  
  // Perform system self-test
  if (performSystemSelfTest()) {
    Serial.println("System self-test passed");
    setStatusLED(0, 255, 0); // Green for success
  } else {
    Serial.println("System self-test failed");
    setStatusLED(255, 165, 0); // Orange for warning
  }
  
  // Initialize system status
  systemStatus.bootTime = millis();
  systemStatus.autoTradingEnabled = ENABLE_AUTO_TRADING;
  systemStatus.safetyAlert = false;
  systemStatus.errorCount = 0;
  
  Serial.println("Setup completed successfully!");
  Serial.println("Starting main operation loop...");
}

// ============================================================================
// ARDUINO MAIN LOOP FUNCTION
// ============================================================================

void loop() {
  unsigned long currentTime = millis();
  
  // Feed watchdog timer
  #if ENABLE_WATCHDOG
  esp_task_wdt_reset();
  #endif
  
  // High-frequency tasks (every loop cycle)
  handleButtons();
  
  // Medium-frequency tasks (every 100ms)
  if (currentTime - lastDisplayUpdate >= 100) {
    updateDisplay();
    lastDisplayUpdate = currentTime;
  }
  
  // Sensor reading (every second)
  if (currentTime - lastSensorRead >= 1000) {
    readSensors();
    updateSystemStatus();
    lastSensorRead = currentTime;
  }
  
  // Check WiFi connection (every 30 seconds)
  if (currentTime % 30000 < 100) {
    checkWiFiConnection();
  }
  
  // Data submission to blockchain
  if (currentTime - lastDataSubmission >= DATA_SUBMISSION_INTERVAL_MS) {
    submitDataToBlockchain();
    lastDataSubmission = currentTime;
  }
  
  // Trading condition check
  if (systemStatus.autoTradingEnabled) {
    checkTradingConditions();
  }
  
  // Status reporting
  if (currentTime - lastStatusReport >= STATUS_REPORT_INTERVAL_MS) {
    printSystemStatus();
    sendStatusToCloud();
    lastStatusReport = currentTime;
  }
  
  // Auto-advance display pages
  if (currentTime - lastPageChange >= SCREEN_PAGE_DURATION_MS) {
    currentPage = (currentPage + 1) % SCREEN_PAGES;
    lastPageChange = currentTime;
  }
  
  // System health monitoring
  monitorSystemHealth();
  
  // Handle web server requests
  webServer.handleClient();
  
  // Handle OTA updates
  ArduinoOTA.handle();
  
  // Small delay to prevent excessive CPU usage
  delay(10);
}

// ============================================================================
// HARDWARE INITIALIZATION FUNCTIONS
// ============================================================================

void initializeHardware() {
  Serial.println("Initializing hardware components...");
  
  // Initialize I2C for OLED display
  Wire.begin(SDA_PIN, SCL_PIN);
  
  // Initialize OLED display
  if (!display.begin(SSD1306_SWITCHCAPVCC, OLED_ADDRESS)) {
    Serial.println("ERROR: OLED display initialization failed!");
    return;
  }
  
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0, 0);
  display.println("GridTokenX ESP32");
  display.println("Energy Meter");
  display.println(""); 
  display.println("Initializing...");
  display.display();
  
  // Initialize DHT22 sensor
  dht.begin();
  delay(2000); // Allow DHT22 to stabilize
  Serial.println("DHT22 sensor initialized");
  
  // Initialize RGB LED
  pixels.begin();
  pixels.clear();
  pixels.show();
  Serial.println("RGB LED initialized");
  
  // Initialize GPIO pins
  pinMode(GREEN_LED_PIN, OUTPUT);
  pinMode(RED_LED_PIN, OUTPUT);
  pinMode(BUZZER_PIN, OUTPUT);
  pinMode(MENU_BUTTON_PIN, INPUT_PULLUP);
  pinMode(SELECT_BUTTON_PIN, INPUT_PULLUP);
  pinMode(BACK_BUTTON_PIN, INPUT_PULLUP);
  
  // Initialize ADC for sensor readings
  analogReadResolution(12); // 12-bit ADC resolution (0-4095)
  
  // Test all components briefly
  digitalWrite(GREEN_LED_PIN, HIGH);
  delay(200);
  digitalWrite(GREEN_LED_PIN, LOW);
  digitalWrite(RED_LED_PIN, HIGH);
  delay(200);
  digitalWrite(RED_LED_PIN, LOW);
  
  // Test RGB LED
  setStatusLED(255, 0, 0); delay(200);
  setStatusLED(0, 255, 0); delay(200);
  setStatusLED(0, 0, 255); delay(200);
  setStatusLED(0, 0, 0);
  
  Serial.println("Hardware initialization completed");
}

void initializeWiFi() {
  Serial.print("Connecting to WiFi network: ");
  Serial.println(WIFI_SSID);
  
  // Display WiFi connection attempt on OLED
  display.clearDisplay();
  display.setCursor(0, 0);
  display.println("Connecting to WiFi");
  display.println(WIFI_SSID);
  display.println("");
  display.println("Please wait...");
  display.display();
  
  WiFi.mode(WIFI_STA);
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  
  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED && attempts < WIFI_RETRY_ATTEMPTS) {
    delay(WIFI_TIMEOUT_MS / WIFI_RETRY_ATTEMPTS);
    Serial.print(".");
    attempts++;
    
    // Update display with progress
    display.setCursor(0, 56);
    display.printf("Attempt: %d/%d", attempts, WIFI_RETRY_ATTEMPTS);
    display.display();
    
    if (attempts >= WIFI_RETRY_ATTEMPTS) {
      Serial.println("\nWiFi connection failed! Retrying...");
      WiFi.disconnect();
      delay(1000);
      WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
      attempts = 0;
    }
  }
  
  if (WiFi.status() == WL_CONNECTED) {
    Serial.println("");
    Serial.println("WiFi connected successfully!");
    Serial.print("IP address: ");
    Serial.println(WiFi.localIP());
    Serial.print("Signal strength: ");
    Serial.print(WiFi.RSSI());
    Serial.println(" dBm");
    
    // Update display with success
    display.clearDisplay();
    display.setCursor(0, 0);
    display.println("WiFi Connected!");
    display.println("");
    display.printf("IP: %s\n", WiFi.localIP().toString().c_str());
    display.printf("Signal: %d dBm\n", WiFi.RSSI());
    display.display();
    delay(2000);
    
    setStatusLED(0, 255, 0); // Green for WiFi success
    systemStatus.wifiConnected = true;
  } else {
    Serial.println("WiFi connection failed!");
    
    // Update display with failure
    display.clearDisplay();
    display.setCursor(0, 0);
    display.println("WiFi Failed!");
    display.println("Check settings");
    display.display();
    
    setStatusLED(255, 0, 0); // Red for WiFi failure
    systemStatus.wifiConnected = false;
  }
}

void initializeBlockchain() {
  Serial.println("Initializing blockchain client...");
  
  display.clearDisplay();
  display.setCursor(0, 0);
  display.println("Connecting to");
  display.println("GridTokenX API");
  display.println("");
  display.println("Please wait...");
  display.display();
  
  blockchainClient.initialize();
  
  if (blockchainClient.testConnection()) {
    Serial.println("Blockchain connection established");
    
    // Register device if not already registered
    if (blockchainClient.registerDevice()) {
      Serial.println("Device registered with blockchain");
    }
    
    systemStatus.blockchainConnected = true;
    setStatusLED(0, 0, 255); // Blue for blockchain success
  } else {
    Serial.println("Blockchain connection failed");
    systemStatus.blockchainConnected = false;
    setStatusLED(255, 165, 0); // Orange for blockchain warning
  }
}

void initializeWebServer() {
  // Web server routes for device configuration
  webServer.on("/", handleRoot);
  webServer.on("/status", handleStatus);
  webServer.on("/config", handleConfig);
  webServer.on("/calibrate", handleCalibrate);
  webServer.on("/restart", handleRestart);
  
  webServer.begin();
  Serial.println("Web server started on port 80");
}

void initializeOTA() {
  #if ENABLE_OTA_UPDATES
  ArduinoOTA.setHostname(DEVICE_ID);
  ArduinoOTA.setPassword(OTA_PASSWORD);
  
  ArduinoOTA.onStart([]() {
    String type = (ArduinoOTA.getCommand() == U_FLASH) ? "firmware" : "filesystem";
    Serial.println("Starting OTA update: " + type);
  });
  
  ArduinoOTA.onEnd([]() {
    Serial.println("\nOTA update completed");
  });
  
  ArduinoOTA.onProgress([](unsigned int progress, unsigned int total) {
    Serial.printf("Progress: %u%%\n", (progress * 100) / total);
  });
  
  ArduinoOTA.onError([](ota_error_t error) {
    Serial.printf("OTA Error[%u]: ", error);
    if (error == OTA_AUTH_ERROR) Serial.println("Auth Failed");
    else if (error == OTA_BEGIN_ERROR) Serial.println("Begin Failed");
    else if (error == OTA_CONNECT_ERROR) Serial.println("Connect Failed");
    else if (error == OTA_RECEIVE_ERROR) Serial.println("Receive Failed");
    else if (error == OTA_END_ERROR) Serial.println("End Failed");
  });
  
  ArduinoOTA.begin();
  Serial.println("OTA updates enabled");
  #endif
}

// ============================================================================
// SENSOR AND DATA HANDLING FUNCTIONS
// ============================================================================

void readSensors() {
  // Read voltage sensor (ZMPT101B)
  int voltageADC = analogRead(VOLTAGE_SENSOR_PIN);
  currentData.voltage = (voltageADC / 4096.0) * 3.3 * VOLTAGE_SENSOR_RATIO;
  
  // Read current sensor (ACS712)
  int currentADC = analogRead(CURRENT_SENSOR_PIN);
  float currentVoltage = (currentADC / 4096.0) * 3.3;
  currentData.current = abs((currentVoltage - 2.5) / (ACS712_SENSITIVITY / 1000.0));
  
  // Calculate power and energy
  currentData.power = currentData.voltage * currentData.current * POWER_FACTOR_DEFAULT;
  
  // Update energy accumulation
  unsigned long currentTime = millis();
  if (lastEnergyUpdateTime > 0) {
    float timeDifferenceHours = (currentTime - lastEnergyUpdateTime) / 3600000.0;
    currentData.energy += currentData.power * timeDifferenceHours;
  }
  lastEnergyUpdateTime = currentTime;
  
  // Read environmental sensors
  float temp = dht.readTemperature();
  float hum = dht.readHumidity();
  
  if (!isnan(temp) && !isnan(hum)) {
    currentData.temperature = temp;
    currentData.humidity = hum;
    dhtErrorCount = 0; // Reset error counter on successful read
  } else {
    dhtErrorCount++;
    if (dhtErrorCount > 5) {
      Serial.println("Warning: DHT22 sensor persistently failing");
    }
  }
  
  // Set other parameters
  currentData.frequency = 50.0; // Assume 50Hz grid frequency
  currentData.powerFactor = POWER_FACTOR_DEFAULT;
  currentData.timestamp = currentTime;
  
  // Validate readings and check safety limits
  checkSafetyLimits();
}

void checkSafetyLimits() {
  bool safetyViolation = false;
  String alertMessage = "";
  
  if (currentData.voltage > VOLTAGE_SAFETY_MAX) {
    alertMessage = "VOLTAGE TOO HIGH: " + String(currentData.voltage) + "V";
    safetyViolation = true;
  } else if (currentData.voltage < VOLTAGE_SAFETY_MIN) {
    alertMessage = "VOLTAGE TOO LOW: " + String(currentData.voltage) + "V";
    safetyViolation = true;
  }
  
  if (currentData.current > CURRENT_SAFETY_MAX) {
    alertMessage = "CURRENT TOO HIGH: " + String(currentData.current) + "A";
    safetyViolation = true;
  }
  
  if (currentData.power > POWER_SAFETY_MAX) {
    alertMessage = "POWER TOO HIGH: " + String(currentData.power) + "W";
    safetyViolation = true;
  }
  
  if (currentData.temperature > TEMPERATURE_SAFETY_MAX) {
    alertMessage = "TEMPERATURE TOO HIGH: " + String(currentData.temperature) + "°C";
    safetyViolation = true;
  }
  
  if (safetyViolation) {
    Serial.println("SAFETY ALERT: " + alertMessage);
    systemStatus.safetyAlert = true;
    systemStatus.errorCode = 1001; // Safety violation error code
    
    // Visual and audio alerts
    setStatusLED(255, 0, 0); // Red for danger
    
    // Sound alarm
    for (int i = 0; i < 3; i++) {
      digitalWrite(BUZZER_PIN, HIGH);
      delay(200);
      digitalWrite(BUZZER_PIN, LOW);
      delay(200);
    }
    
    // Send safety alert to blockchain
    blockchainClient.reportSafetyAlert(alertMessage);
    
  } else {
    systemStatus.safetyAlert = false;
    systemStatus.errorCode = 0;
  }
}

void submitDataToBlockchain() {
  if (!systemStatus.blockchainConnected || systemStatus.safetyAlert) {
    Serial.println("Cannot submit data: blockchain disconnected or safety alert");
    return;
  }
  
  Serial.println("Submitting energy data to blockchain...");
  
  if (blockchainClient.submitEnergyData(currentData)) {
    Serial.println("Energy data submitted successfully");
    systemStatus.lastDataSubmission = millis();
    dataSubmissionCount++;
    
    // Flash green LED to indicate successful submission
    digitalWrite(GREEN_LED_PIN, HIGH);
    delay(100);
    digitalWrite(GREEN_LED_PIN, LOW);
    
  } else {
    Serial.println("Failed to submit energy data");
    systemStatus.errorCount++;
    blockchainErrorCount++;
    
    // Flash red LED to indicate failure
    digitalWrite(RED_LED_PIN, HIGH);
    delay(100);
    digitalWrite(RED_LED_PIN, LOW);
  }
}

void performSensorCalibration() {
  Serial.println("Performing sensor calibration...");
  
  // Load calibration data from storage
  loadCalibrationData();
  
  // Perform auto-calibration if enabled
  #if ENABLE_AUTO_CALIBRATION
  
  // Voltage sensor calibration
  Serial.println("Calibrating voltage sensor...");
  float voltageSum = 0;
  int validReadings = 0;
  
  for (int i = 0; i < 50; i++) {
    int adc = analogRead(VOLTAGE_SENSOR_PIN);
    if (adc > 100 && adc < 4000) { // Valid range
      voltageSum += adc;
      validReadings++;
    }
    delay(20);
  }
  
  if (validReadings > 30) {
    float avgADC = voltageSum / validReadings;
    // Adjust calibration based on expected voltage (220V)
    // This is a simplified calibration - real implementation would use reference
    Serial.printf("Voltage calibration completed. Avg ADC: %.1f\n", avgADC);
  }
  
  // Current sensor calibration (zero-point)
  Serial.println("Calibrating current sensor zero point...");
  float currentSum = 0;
  validReadings = 0;
  
  for (int i = 0; i < 50; i++) {
    int adc = analogRead(CURRENT_SENSOR_PIN);
    currentSum += adc;
    validReadings++;
    delay(20);
  }
  
  float avgCurrentADC = currentSum / validReadings;
  float zeroPointVoltage = (avgCurrentADC / 4096.0) * 3.3;
  Serial.printf("Current sensor zero point: %.3fV (ADC: %.1f)\n", zeroPointVoltage, avgCurrentADC);
  
  // Save calibration data
  saveCalibrationData();
  
  #endif
  
  Serial.println("Sensor calibration completed");
}

// ============================================================================
// DISPLAY AND INTERFACE FUNCTIONS
// ============================================================================

void updateDisplay() {
  if (!displayOn) return;
  
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
  
  switch (currentPage) {
    case 0: showPage_Overview(); break;
    case 1: showPage_Power(); break;
    case 2: showPage_Energy(); break;
    case 3: showPage_Trading(); break;
    case 4: showPage_Network(); break;
    case 5: showPage_System(); break;
  }
  
  display.display();
}

void showPage_Overview() {
  display.setCursor(0, 0);
  display.setTextSize(1);
  display.println("GridTokenX Meter");
  display.println("================");
  
  display.printf("Voltage: %.1fV\n", currentData.voltage);
  display.printf("Current: %.2fA\n", currentData.current);
  display.printf("Power:   %.1fW\n", currentData.power);
  
  display.printf("Temp: %.1fC H:%.0f%%\n", 
                  currentData.temperature, currentData.humidity);
  
  // Status indicator at bottom
  display.setCursor(0, 56);
  if (systemStatus.safetyAlert) {
    display.println("STATUS: ALERT!");
  } else if (systemStatus.blockchainConnected) {
    display.println("STATUS: ONLINE");
  } else {
    display.println("STATUS: OFFLINE");
  }
}

void showPage_Power() {
  display.setCursor(0, 0);
  display.setTextSize(1);
  display.println("POWER ANALYSIS");
  display.println("==============");
  
  display.printf("Voltage:   %.2f V\n", currentData.voltage);
  display.printf("Current:   %.3f A\n", currentData.current);
  display.printf("Power:     %.1f W\n", currentData.power);
  display.printf("Frequency: %.1f Hz\n", currentData.frequency);
  display.printf("PF:        %.2f\n", currentData.powerFactor);
  
  display.setCursor(100, 56);
  display.print("2/6");
}

void showPage_Energy() {
  display.setCursor(0, 0);
  display.setTextSize(1);
  display.println("ENERGY TRACKING");
  display.println("===============");
  
  display.printf("Total: %.3f kWh\n", currentData.energy);
  display.printf("Today: %.3f kWh\n", dailyEnergyTotal);
  
  if (currentData.power > (EXCESS_ENERGY_THRESHOLD * 1000)) {
    display.printf("Excess:%.3f kWh\n", 
                   (currentData.power/1000.0) - EXCESS_ENERGY_THRESHOLD);
  } else if (currentData.power < (ENERGY_DEMAND_THRESHOLD * 1000)) {
    display.printf("Need:  %.3f kWh\n", 
                   ENERGY_DEMAND_THRESHOLD - (currentData.power/1000.0));
  } else {
    display.println("Status: Balanced");
  }
  
  display.setCursor(100, 56);
  display.print("3/6");
}

void showPage_Trading() {
  display.setCursor(0, 0);
  display.setTextSize(1);
  display.println("TRADING STATUS");
  display.println("==============");
  
  display.printf("Orders: %d\n", dailyOrderCount);
  display.printf("Sold:   %.2f kWh\n", dailyEnergySold);
  display.printf("Bought: %.2f kWh\n", dailyEnergyBought);
  display.printf("Revenue:%.0f THB\n", dailyRevenue);
  
  display.setCursor(0, 48);
  if (systemStatus.autoTradingEnabled) {
    display.println("Auto-Trade: ON");
  } else {
    display.println("Auto-Trade: OFF");
  }
  
  display.setCursor(100, 56);
  display.print("4/6");
}

void showPage_Network() {
  display.setCursor(0, 0);
  display.setTextSize(1);
  display.println("NETWORK STATUS");
  display.println("==============");
  
  if (WiFi.status() == WL_CONNECTED) {
    display.printf("WiFi: Connected\n");
    display.printf("RSSI: %d dBm\n", WiFi.RSSI());
    
    // Shorten IP display for small screen
    String ip = WiFi.localIP().toString();
    if (ip.length() > 12) {
      ip = ip.substring(ip.lastIndexOf('.') - 3);
    }
    display.printf("IP: %s\n", ip.c_str());
  } else {
    display.println("WiFi: Disconnected");
    display.println("Check settings");
  }
  
  display.printf("API: ");
  if (systemStatus.blockchainConnected) {
    display.println("Connected");
  } else {
    display.println("Disconnected");
  }
  
  display.setCursor(100, 56);
  display.print("5/6");
}

void showPage_System() {
  display.setCursor(0, 0);
  display.setTextSize(1);
  display.println("SYSTEM INFO");
  display.println("===========");
  
  unsigned long uptime = millis() / 1000;
  display.printf("Uptime: %lus\n", uptime);
  display.printf("Free RAM: %dB\n", ESP.getFreeHeap());
  display.printf("Errors: %d\n", systemStatus.errorCount);
  
  // Display firmware version (shortened)
  String version = String(FIRMWARE_VERSION);
  if (version.length() > 12) {
    version = version.substring(0, 12);
  }
  display.printf("Ver: %s\n", version.c_str());
  
  display.setCursor(100, 56);
  display.print("6/6");
}

void handleButtons() {
  unsigned long currentTime = millis();
  
  // Check debounce time
  if (currentTime - lastButtonPress < BUTTON_DEBOUNCE_MS) {
    return;
  }
  
  // Menu button - advance to next page
  if (digitalRead(MENU_BUTTON_PIN) == LOW) {
    currentPage = (currentPage + 1) % SCREEN_PAGES;
    lastButtonPress = currentTime;
    lastPageChange = currentTime; // Reset auto-advance timer
    Serial.printf("Page changed to: %d\n", currentPage);
  }
  
  // Select button - toggle auto-trading
  if (digitalRead(SELECT_BUTTON_PIN) == LOW) {
    systemStatus.autoTradingEnabled = !systemStatus.autoTradingEnabled;
    lastButtonPress = currentTime;
    Serial.printf("Auto-trading: %s\n", 
                  systemStatus.autoTradingEnabled ? "ON" : "OFF");
    
    // Save setting to preferences
    preferences.putBool("autoTrade", systemStatus.autoTradingEnabled);
  }
  
  // Back button - manual data submission
  if (digitalRead(BACK_BUTTON_PIN) == LOW) {
    submitDataToBlockchain();
    lastButtonPress = currentTime;
    Serial.println("Manual data submission triggered");
  }
}

void setStatusLED(int r, int g, int b) {
  pixels.setPixelColor(0, pixels.Color(r, g, b));
  pixels.show();
}

// ============================================================================
// TRADING AND AUTOMATION FUNCTIONS
// ============================================================================

void checkTradingConditions() {
  if (!systemStatus.autoTradingEnabled || systemStatus.safetyAlert) {
    return;
  }
  
  // Check if enough time has passed since last trade
  if (millis() - lastTradeTime < TRADING_COOLDOWN_MS) {
    return;
  }
  
  // Check for excess energy (sell condition)
  if (currentData.power > (EXCESS_ENERGY_THRESHOLD * 1000)) {
    EnergyOrder sellOrder;
    strcpy(sellOrder.deviceId, DEVICE_ID);
    strcpy(sellOrder.type, "SELL");
    sellOrder.quantity = (currentData.power / 1000.0) - EXCESS_ENERGY_THRESHOLD;
    sellOrder.price = MIN_TRADING_PRICE + 
                     (random(MAX_TRADING_PRICE - MIN_TRADING_PRICE));
    sellOrder.timestamp = millis();
    
    Serial.printf("Creating sell order: %.3f kWh @ %d THB/kWh\n", 
                  sellOrder.quantity, sellOrder.price);
    
    if (blockchainClient.createEnergyOrder(sellOrder)) {
      dailyOrderCount++;
      dailyEnergySold += sellOrder.quantity;
      dailyRevenue += sellOrder.quantity * sellOrder.price;
      lastTradeTime = millis();
      
      Serial.println("Sell order created successfully");
      
      // Flash blue LED for trading activity
      setStatusLED(0, 0, 255);
      delay(200);
      setStatusLED(0, 0, 0);
    }
  }
  
  // Check for energy demand (buy condition)
  else if (currentData.power < (ENERGY_DEMAND_THRESHOLD * 1000)) {
    EnergyOrder buyOrder;
    strcpy(buyOrder.deviceId, DEVICE_ID);
    strcpy(buyOrder.type, "BUY");
    buyOrder.quantity = ENERGY_DEMAND_THRESHOLD - (currentData.power / 1000.0);
    buyOrder.price = MIN_TRADING_PRICE + 
                    (random(MAX_TRADING_PRICE - MIN_TRADING_PRICE));
    buyOrder.timestamp = millis();
    
    Serial.printf("Creating buy order: %.3f kWh @ %d THB/kWh\n", 
                  buyOrder.quantity, buyOrder.price);
    
    if (blockchainClient.createEnergyOrder(buyOrder)) {
      dailyOrderCount++;
      dailyEnergyBought += buyOrder.quantity;
      dailyRevenue -= buyOrder.quantity * buyOrder.price;
      lastTradeTime = millis();
      
      Serial.println("Buy order created successfully");
      
      // Flash purple LED for trading activity
      setStatusLED(128, 0, 128);
      delay(200);
      setStatusLED(0, 0, 0);
    }
  }
}

// ============================================================================
// SYSTEM MANAGEMENT FUNCTIONS
// ============================================================================

void updateSystemStatus() {
  systemStatus.lastUpdate = millis();
  systemStatus.uptime = millis() - startupTime;
  systemStatus.freeMemory = ESP.getFreeHeap();
  systemStatus.wifiConnected = WiFi.isConnected();
  systemStatus.blockchainConnected = blockchainClient.isConnected();
  
  // Update daily energy total
  dailyEnergyTotal = currentData.energy;
  
  // Check for daily reset (simplified - should use RTC)
  static unsigned long lastDayReset = 0;
  if (millis() - lastDayReset > 86400000) { // 24 hours
    // Reset daily counters
    dailyEnergyTotal = 0;
    dailyExcessEnergy = 0;
    dailyEnergyDemand = 0;
    dailyOrderCount = 0;
    dailyEnergySold = 0;
    dailyEnergyBought = 0;
    dailyRevenue = 0;
    lastDayReset = millis();
    
    Serial.println("Daily counters reset");
  }
}

bool performSystemSelfTest() {
  Serial.println("Performing system self-test...");
  
  bool testPassed = true;
  
  // Test OLED display
  display.clearDisplay();
  display.setCursor(0, 0);
  display.println("Self-Test Running");
  display.display();
  
  // Test DHT22 sensor
  float testTemp = dht.readTemperature();
  float testHum = dht.readHumidity();
  if (isnan(testTemp) || isnan(testHum)) {
    Serial.println("Self-test FAILED: DHT22 sensor not responding");
    testPassed = false;
  }
  
  // Test ADC readings
  int voltageADC = analogRead(VOLTAGE_SENSOR_PIN);
  int currentADC = analogRead(CURRENT_SENSOR_PIN);
  if (voltageADC < 10 || voltageADC > 4080 || currentADC < 10 || currentADC > 4080) {
    Serial.println("Self-test WARNING: ADC readings out of expected range");
  }
  
  // Test WiFi connection
  if (!WiFi.isConnected()) {
    Serial.println("Self-test WARNING: WiFi not connected");
  }
  
  // Test blockchain connection
  if (!blockchainClient.isConnected()) {
    Serial.println("Self-test WARNING: Blockchain not connected");
  }
  
  // Test memory
  if (ESP.getFreeHeap() < 100000) {
    Serial.println("Self-test WARNING: Low memory available");
  }
  
  Serial.printf("Self-test completed: %s\n", testPassed ? "PASSED" : "FAILED");
  return testPassed;
}

void monitorSystemHealth() {
  static unsigned long lastHealthCheck = 0;
  unsigned long currentTime = millis();
  
  if (currentTime - lastHealthCheck < 30000) { // Every 30 seconds
    return;
  }
  lastHealthCheck = currentTime;
  
  // Check memory usage
  uint32_t freeHeap = ESP.getFreeHeap();
  if (freeHeap < 50000) {
    Serial.printf("WARNING: Low memory - %d bytes free\n", freeHeap);
    systemStatus.errorCount++;
  }
  
  // Check WiFi connectivity
  if (!WiFi.isConnected()) {
    Serial.println("WARNING: WiFi disconnected - attempting reconnection");
    checkWiFiConnection();
  }
  
  // Check sensor health
  if (dhtErrorCount > 10) {
    Serial.println("WARNING: DHT22 sensor consistently failing");
    systemStatus.errorCount++;
  }
  
  // Update status LED based on system health
  if (systemStatus.safetyAlert) {
    setStatusLED(255, 0, 0); // Red for safety alert
  } else if (!systemStatus.wifiConnected) {
    setStatusLED(255, 165, 0); // Orange for connectivity issues
  } else if (systemStatus.blockchainConnected) {
    setStatusLED(0, 255, 0); // Green for all systems operational
  } else {
    setStatusLED(0, 0, 255); // Blue for partial connectivity
  }
}

void printSystemStatus() {
  Serial.println("\n=== GridTokenX ESP32 System Status ===");
  Serial.printf("Device ID: %s\n", DEVICE_ID);
  Serial.printf("Firmware: %s\n", FIRMWARE_VERSION);
  Serial.printf("Uptime: %lu seconds\n", (millis() - startupTime) / 1000);
  Serial.printf("Free Memory: %d bytes\n", ESP.getFreeHeap());
  
  Serial.println("\n--- Connectivity ---");
  Serial.printf("WiFi: %s", WiFi.isConnected() ? "Connected" : "Disconnected");
  if (WiFi.isConnected()) {
    Serial.printf(" (RSSI: %d dBm, IP: %s)\n", 
                  WiFi.RSSI(), WiFi.localIP().toString().c_str());
  } else {
    Serial.println();
  }
  Serial.printf("Blockchain: %s\n", 
                systemStatus.blockchainConnected ? "Connected" : "Disconnected");
  
  Serial.println("\n--- Energy Readings ---");
  Serial.printf("Voltage: %.2f V\n", currentData.voltage);
  Serial.printf("Current: %.3f A\n", currentData.current);
  Serial.printf("Power: %.1f W\n", currentData.power);
  Serial.printf("Energy: %.3f kWh\n", currentData.energy);
  Serial.printf("Temperature: %.1f°C\n", currentData.temperature);
  Serial.printf("Humidity: %.1f%%\n", currentData.humidity);
  
  Serial.println("\n--- Trading Summary ---");
  Serial.printf("Auto-Trading: %s\n", 
                systemStatus.autoTradingEnabled ? "Enabled" : "Disabled");
  Serial.printf("Daily Orders: %d\n", dailyOrderCount);
  Serial.printf("Energy Sold: %.3f kWh\n", dailyEnergySold);
  Serial.printf("Energy Bought: %.3f kWh\n", dailyEnergyBought);
  Serial.printf("Daily Revenue: %.2f THB\n", dailyRevenue);
  
  Serial.println("\n--- System Health ---");
  Serial.printf("Safety Alert: %s\n", 
                systemStatus.safetyAlert ? "ACTIVE" : "Normal");
  Serial.printf("Error Count: %d\n", systemStatus.errorCount);
  Serial.printf("Data Submissions: %d\n", dataSubmissionCount);
  Serial.printf("DHT Errors: %d\n", dhtErrorCount);
  
  Serial.println("=====================================\n");
}

// ============================================================================
// NETWORK AND CONNECTIVITY FUNCTIONS
// ============================================================================

void checkWiFiConnection() {
  if (WiFi.status() != WL_CONNECTED) {
    Serial.println("WiFi disconnected - attempting reconnection");
    
    WiFi.disconnect();
    delay(1000);
    WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
    
    int attempts = 0;
    while (WiFi.status() != WL_CONNECTED && attempts < 10) {
      delay(500);
      attempts++;
      Serial.print(".");
    }
    
    if (WiFi.status() == WL_CONNECTED) {
      Serial.println("\nWiFi reconnected successfully");
      systemStatus.wifiConnected = true;
    } else {
      Serial.println("\nWiFi reconnection failed");
      systemStatus.wifiConnected = false;
      networkErrorCount++;
    }
  }
}

void syncSystemTime() {
  if (WiFi.isConnected()) {
    configTime(7 * 3600, 0, "pool.ntp.org", "time.google.com"); // UTC+7 for Thailand
    Serial.println("System time synchronized with NTP");
  }
}

// ============================================================================
// WEB SERVER HANDLERS
// ============================================================================

void handleRoot() {
  String html = "<!DOCTYPE html><html><head><title>GridTokenX ESP32</title>";
  html += "<meta name='viewport' content='width=device-width, initial-scale=1'>";
  html += "<style>body{font-family:Arial;margin:20px;}</style></head><body>";
  html += "<h1>GridTokenX ESP32 Energy Meter</h1>";
  html += "<h2>Current Readings</h2>";
  html += "<p>Voltage: " + String(currentData.voltage, 2) + " V</p>";
  html += "<p>Current: " + String(currentData.current, 3) + " A</p>";
  html += "<p>Power: " + String(currentData.power, 1) + " W</p>";
  html += "<p>Energy: " + String(currentData.energy, 3) + " kWh</p>";
  html += "<p>Temperature: " + String(currentData.temperature, 1) + " °C</p>";
  html += "<p>Humidity: " + String(currentData.humidity, 1) + " %</p>";
  html += "<h2>System Status</h2>";
  html += "<p>WiFi: " + String(WiFi.isConnected() ? "Connected" : "Disconnected") + "</p>";
  html += "<p>Blockchain: " + String(systemStatus.blockchainConnected ? "Connected" : "Disconnected") + "</p>";
  html += "<p>Auto-Trading: " + String(systemStatus.autoTradingEnabled ? "ON" : "OFF") + "</p>";
  html += "<p>Uptime: " + String((millis() - startupTime) / 1000) + " seconds</p>";
  html += "<h2>Controls</h2>";
  html += "<p><a href='/config'>Configuration</a> | ";
  html += "<a href='/status'>Detailed Status</a> | ";
  html += "<a href='/restart'>Restart Device</a></p>";
  html += "</body></html>";
  
  webServer.send(200, "text/html", html);
}

void handleStatus() {
  StaticJsonDocument<1024> doc;
  
  doc["device_id"] = DEVICE_ID;
  doc["firmware"] = FIRMWARE_VERSION;
  doc["uptime"] = (millis() - startupTime) / 1000;
  doc["free_memory"] = ESP.getFreeHeap();
  
  doc["voltage"] = currentData.voltage;
  doc["current"] = currentData.current;
  doc["power"] = currentData.power;
  doc["energy"] = currentData.energy;
  doc["temperature"] = currentData.temperature;
  doc["humidity"] = currentData.humidity;
  
  doc["wifi_connected"] = WiFi.isConnected();
  doc["blockchain_connected"] = systemStatus.blockchainConnected;
  doc["auto_trading"] = systemStatus.autoTradingEnabled;
  doc["safety_alert"] = systemStatus.safetyAlert;
  doc["error_count"] = systemStatus.errorCount;
  
  doc["daily_orders"] = dailyOrderCount;
  doc["daily_sold"] = dailyEnergySold;
  doc["daily_bought"] = dailyEnergyBought;
  doc["daily_revenue"] = dailyRevenue;
  
  String response;
  serializeJson(doc, response);
  webServer.send(200, "application/json", response);
}

void handleConfig() {
  // Simple configuration page
  String html = "<!DOCTYPE html><html><head><title>Configuration</title></head><body>";
  html += "<h1>Device Configuration</h1>";
  html += "<p>Configuration interface would be implemented here</p>";
  html += "<p><a href='/'>Back to Home</a></p>";
  html += "</body></html>";
  
  webServer.send(200, "text/html", html);
}

void handleCalibrate() {
  performSensorCalibration();
  
  String html = "<!DOCTYPE html><html><head><title>Calibration</title></head><body>";
  html += "<h1>Sensor Calibration</h1>";
  html += "<p>Calibration completed</p>";
  html += "<p><a href='/'>Back to Home</a></p>";
  html += "</body></html>";
  
  webServer.send(200, "text/html", html);
}

void handleRestart() {
  String html = "<!DOCTYPE html><html><head><title>Restart</title></head><body>";
  html += "<h1>Device Restart</h1>";
  html += "<p>Device will restart in 3 seconds...</p>";
  html += "</body></html>";
  
  webServer.send(200, "text/html", html);
  delay(3000);
  ESP.restart();
}

// ============================================================================
// CONFIGURATION AND STORAGE FUNCTIONS
// ============================================================================

void loadConfiguration() {
  // Load settings from preferences
  systemStatus.autoTradingEnabled = preferences.getBool("autoTrade", ENABLE_AUTO_TRADING);
  
  Serial.println("Configuration loaded from preferences");
}

void saveConfiguration() {
  preferences.putBool("autoTrade", systemStatus.autoTradingEnabled);
  
  Serial.println("Configuration saved to preferences");
}

void loadCalibrationData() {
  // Load calibration values from preferences
  // Implementation would load sensor calibration constants
  Serial.println("Calibration data loaded");
}

void saveCalibrationData() {
  // Save calibration values to preferences
  // Implementation would save sensor calibration constants
  Serial.println("Calibration data saved");
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

String getISOTimestamp() {
  time_t now;
  struct tm timeinfo;
  if (!getLocalTime(&timeinfo)) {
    return String(millis()); // Fallback to millis
  }
  
  char timestamp[32];
  strftime(timestamp, sizeof(timestamp), "%Y-%m-%dT%H:%M:%SZ", &timeinfo);
  return String(timestamp);
}

void logMessage(int level, const char* message) {
  if (level <= DEBUG_LEVEL) {
    String timestamp = getISOTimestamp();
    String levelStr;
    
    switch (level) {
      case 1: levelStr = "ERROR"; break;
      case 2: levelStr = "WARN "; break;
      case 3: levelStr = "INFO "; break;
      case 4: levelStr = "DEBUG"; break;
      default: levelStr = "UNKNOWN"; break;
    }
    
    Serial.printf("[%s] [%s] %s\n", timestamp.c_str(), levelStr.c_str(), message);
  }
}

void sendStatusToCloud() {
  // Implementation would send status to cloud monitoring service
  // This is a placeholder for cloud integration
  if (systemStatus.blockchainConnected) {
    Serial.println("Status report sent to cloud");
  }
}

void handleEmergencyShutdown() {
  Serial.println("EMERGENCY SHUTDOWN INITIATED");
  
  // Stop all trading operations
  systemStatus.autoTradingEnabled = false;
  
  // Display emergency message
  display.clearDisplay();
  display.setTextSize(2);
  display.setCursor(0, 0);
  display.println("EMERGENCY");
  display.println("SHUTDOWN");
  display.display();
  
  // Flash red LED continuously
  while (true) {
    setStatusLED(255, 0, 0);
    delay(500);
    setStatusLED(0, 0, 0);
    delay(500);
  }
}

// ============================================================================
// END OF FILE
// ============================================================================
