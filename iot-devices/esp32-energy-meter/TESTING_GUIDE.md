# GridTokenX ESP32 Testing and Validation Suite

## 🧪 Test Overview

This document provides comprehensive testing procedures for the GridTokenX ESP32 Smart Energy Meter POC prototype to ensure reliable operation and integration with the blockchain network.

## 📋 Test Categories

### 1. Hardware Tests
### 2. Software Tests  
### 3. Network Tests
### 4. Blockchain Integration Tests
### 5. Performance Tests
### 6. Security Tests
### 7. Environmental Tests

---

## 🔌 1. Hardware Tests

### 1.1 Power Supply Test
**Objective**: Verify stable power delivery to all components

**Procedure**:
1. Connect ESP32 to 5V power supply
2. Measure voltage at test points:
   - ESP32 3.3V rail: 3.3V ±3%
   - 5V rail: 5.0V ±5%
   - Current consumption: < 500mA peak

**Expected Results**:
```
✅ 3.3V Rail: 3.20V - 3.40V
✅ 5V Rail: 4.75V - 5.25V  
✅ Current: < 500mA
✅ No voltage fluctuations > 100mV
```

**Validation Commands**:
```cpp
// Add to main.cpp for voltage monitoring
float vcc = ESP.getVcc() / 1024.0;
Serial.printf("VCC: %.2fV\n", vcc);
```

### 1.2 Sensor Connectivity Test  
**Objective**: Verify all sensors are properly connected and responding

**Test Matrix**:
```
┌─────────────────┬─────────┬──────────────┬─────────────────┐
│ Component       │ Pin     │ Test Method  │ Expected Result │
├─────────────────┼─────────┼──────────────┼─────────────────┤
│ DHT22 Sensor    │ GPIO 4  │ Read T/H     │ Valid readings  │
│ ACS712 Current  │ GPIO 36 │ ADC Reading  │ 2.5V ± 0.1V     │
│ ZMPT101B Volt   │ GPIO 39 │ ADC Reading  │ Variable        │
│ OLED Display    │ I2C     │ Screen Test  │ Display active  │
│ RGB LED         │ GPIO 12 │ Color Test   │ All colors      │
│ Status LEDs     │ 13,14   │ Blink Test   │ LED response    │
│ Buttons         │ 0,25,32 │ Input Test   │ Button response │
└─────────────────┴─────────┴──────────────┴─────────────────┘
```

**Test Code**:
```cpp
void testHardwareComponents() {
  Serial.println("=== Hardware Component Test ===");
  
  // Test DHT22
  float temp = dht.readTemperature();
  float humidity = dht.readHumidity();
  Serial.printf("DHT22 - Temp: %.1f°C, Humidity: %.1f%%\n", temp, humidity);
  
  // Test Current Sensor
  int currentADC = analogRead(CURRENT_SENSOR_PIN);
  float currentVoltage = (currentADC / 4096.0) * 3.3;
  Serial.printf("ACS712 - ADC: %d, Voltage: %.2fV\n", currentADC, currentVoltage);
  
  // Test Voltage Sensor  
  int voltageADC = analogRead(VOLTAGE_SENSOR_PIN);
  float voltageReading = (voltageADC / 4096.0) * 3.3;
  Serial.printf("ZMPT101B - ADC: %d, Voltage: %.2fV\n", voltageADC, voltageReading);
  
  // Test Display
  display.clearDisplay();
  display.setCursor(0, 0);
  display.println("HARDWARE TEST");
  display.display();
  Serial.println("OLED - Display updated");
  
  // Test LEDs
  digitalWrite(GREEN_LED_PIN, HIGH);
  delay(200);
  digitalWrite(GREEN_LED_PIN, LOW);
  digitalWrite(RED_LED_PIN, HIGH);
  delay(200);
  digitalWrite(RED_LED_PIN, LOW);
  Serial.println("LEDs - Blink test completed");
  
  // Test RGB LED
  pixels.setPixelColor(0, pixels.Color(255, 0, 0)); // Red
  pixels.show();
  delay(200);
  pixels.setPixelColor(0, pixels.Color(0, 255, 0)); // Green  
  pixels.show();
  delay(200);
  pixels.setPixelColor(0, pixels.Color(0, 0, 255)); // Blue
  pixels.show();
  delay(200);
  pixels.clear();
  pixels.show();
  Serial.println("RGB LED - Color test completed");
}
```

### 1.3 I2C Bus Test
**Objective**: Verify I2C communication integrity

**Test Procedure**:
```cpp
void scanI2CDevices() {
  Serial.println("Scanning I2C devices...");
  int deviceCount = 0;
  
  for (byte address = 1; address < 127; address++) {
    Wire.beginTransmission(address);
    byte error = Wire.endTransmission();
    
    if (error == 0) {
      Serial.printf("I2C device found at address 0x%02X\n", address);
      deviceCount++;
    }
  }
  
  Serial.printf("Found %d I2C devices\n", deviceCount);
  // Expected: At least 1 device (OLED at 0x3C or 0x3D)
}
```

---

## 💻 2. Software Tests

### 2.1 Boot Sequence Test
**Objective**: Verify proper system initialization

**Test Sequence**:
1. Power on device
2. Monitor serial output for initialization stages
3. Verify all systems start correctly

**Expected Boot Log**:
```
=== GridTokenX ESP32 Energy Meter ===
Version: 1.0.0
Build: 2024-01-20T10:30:00Z

[INIT] Starting system initialization...
[WIFI] Connecting to WiFi...
[WIFI] Connected! IP: 192.168.1.150
[I2C]  Initializing I2C bus...
[OLED] Display initialized
[DHT]  DHT22 sensor ready
[ADC]  Current sensor initialized
[ADC]  Voltage sensor initialized  
[LED]  Status LEDs initialized
[BTC]  Blockchain client starting...
[BTC]  Connected to GridTokenX API
[REG]  Device registration successful
[SYS]  System ready - entering main loop
```

### 2.2 Configuration Validation Test
**Objective**: Ensure all configuration parameters are valid

**Test Code**:
```cpp
void validateConfiguration() {
  Serial.println("=== Configuration Validation ===");
  
  // Validate WiFi configuration
  if (strlen(WIFI_SSID) == 0) {
    Serial.println("❌ ERROR: WIFI_SSID not configured");
  } else {
    Serial.printf("✅ WiFi SSID: %s\n", WIFI_SSID);
  }
  
  // Validate blockchain configuration
  if (strlen(BLOCKCHAIN_API_HOST) == 0) {
    Serial.println("❌ ERROR: BLOCKCHAIN_API_HOST not configured");
  } else {
    Serial.printf("✅ Blockchain Host: %s:%d\n", BLOCKCHAIN_API_HOST, BLOCKCHAIN_API_PORT);
  }
  
  // Validate device configuration
  if (strlen(DEVICE_ID) == 0) {
    Serial.println("❌ ERROR: DEVICE_ID not configured");
  } else {
    Serial.printf("✅ Device ID: %s\n", DEVICE_ID);
  }
  
  // Validate thresholds
  Serial.printf("✅ Excess Energy Threshold: %.2f kWh\n", EXCESS_ENERGY_THRESHOLD);
  Serial.printf("✅ Energy Demand Threshold: %.2f kWh\n", ENERGY_DEMAND_THRESHOLD);
  Serial.printf("✅ Max Trading Price: %d THB/kWh\n", MAX_TRADING_PRICE);
}
```

### 2.3 Memory Management Test
**Objective**: Monitor memory usage and detect leaks

**Test Implementation**:
```cpp
void monitorMemoryUsage() {
  static uint32_t lastHeapSize = 0;
  uint32_t currentHeap = ESP.getFreeHeap();
  uint32_t maxAllocHeap = ESP.getMaxAllocHeap();
  uint32_t minFreeHeap = ESP.getMinFreeHeap();
  
  Serial.printf("Memory - Free: %d, Max Alloc: %d, Min Free: %d\n", 
                currentHeap, maxAllocHeap, minFreeHeap);
  
  if (lastHeapSize > 0) {
    int32_t heapDelta = currentHeap - lastHeapSize;
    if (heapDelta < -1000) {  // More than 1KB decrease
      Serial.printf("⚠️  MEMORY WARNING: Heap decreased by %d bytes\n", -heapDelta);
    }
  }
  
  lastHeapSize = currentHeap;
  
  // Memory thresholds
  if (currentHeap < 50000) {  // Less than 50KB free
    Serial.println("❌ CRITICAL: Low memory warning");
  }
}
```

---

## 🌐 3. Network Tests

### 3.1 WiFi Connectivity Test
**Objective**: Verify stable WiFi connection and auto-reconnection

**Test Scenarios**:
1. **Initial Connection Test**
2. **Signal Strength Test**  
3. **Reconnection Test**
4. **Multiple Network Test**

**Test Implementation**:
```cpp
void testWiFiConnectivity() {
  Serial.println("=== WiFi Connectivity Test ===");
  
  // Test 1: Connection speed
  unsigned long connectStart = millis();
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  
  while (WiFi.status() != WL_CONNECTED && millis() - connectStart < 10000) {
    delay(500);
    Serial.print(".");
  }
  
  if (WiFi.status() == WL_CONNECTED) {
    unsigned long connectTime = millis() - connectStart;
    Serial.printf("\n✅ Connected in %lu ms\n", connectTime);
    Serial.printf("✅ IP Address: %s\n", WiFi.localIP().toString().c_str());
    Serial.printf("✅ Signal Strength: %d dBm\n", WiFi.RSSI());
    Serial.printf("✅ MAC Address: %s\n", WiFi.macAddress().c_str());
  } else {
    Serial.println("\n❌ Connection failed");
    return;
  }
  
  // Test 2: Network performance
  Serial.println("Testing network performance...");
  
  // Ping test
  if (Ping.ping("8.8.8.8", 3)) {
    Serial.printf("✅ Ping successful: %d ms\n", Ping.averageTime());
  } else {
    Serial.println("❌ Ping failed");
  }
  
  // DNS resolution test
  IPAddress dnsResult;
  if (WiFi.hostByName("www.google.com", dnsResult)) {
    Serial.printf("✅ DNS resolution: %s\n", dnsResult.toString().c_str());
  } else {
    Serial.println("❌ DNS resolution failed");
  }
}

void testWiFiReconnection() {
  Serial.println("=== WiFi Reconnection Test ===");
  
  Serial.println("Disconnecting WiFi...");
  WiFi.disconnect();
  delay(2000);
  
  Serial.println("Attempting reconnection...");
  unsigned long reconnectStart = millis();
  
  // Trigger reconnection logic
  initializeWiFi();
  
  if (WiFi.status() == WL_CONNECTED) {
    unsigned long reconnectTime = millis() - reconnectStart;
    Serial.printf("✅ Reconnected in %lu ms\n", reconnectTime);
  } else {
    Serial.println("❌ Reconnection failed");
  }
}
```

### 3.2 HTTP Client Test
**Objective**: Verify HTTP/HTTPS communication capabilities

**Test Cases**:
```cpp
void testHTTPClient() {
  Serial.println("=== HTTP Client Test ===");
  
  HTTPClient http;
  
  // Test 1: Basic HTTP GET
  http.begin("http://httpbin.org/get");
  int httpResponseCode = http.GET();
  
  if (httpResponseCode == 200) {
    Serial.println("✅ HTTP GET successful");
    String payload = http.getString();
    Serial.printf("Response length: %d bytes\n", payload.length());
  } else {
    Serial.printf("❌ HTTP GET failed: %d\n", httpResponseCode);
  }
  http.end();
  
  // Test 2: HTTPS GET  
  http.begin("https://httpbin.org/get");
  httpResponseCode = http.GET();
  
  if (httpResponseCode == 200) {
    Serial.println("✅ HTTPS GET successful");
  } else {
    Serial.printf("❌ HTTPS GET failed: %d\n", httpResponseCode);
  }
  http.end();
  
  // Test 3: POST with JSON
  http.begin("http://httpbin.org/post");
  http.addHeader("Content-Type", "application/json");
  
  StaticJsonDocument<200> testDoc;
  testDoc["test"] = "ESP32";
  testDoc["timestamp"] = millis();
  
  String jsonString;
  serializeJson(testDoc, jsonString);
  
  httpResponseCode = http.POST(jsonString);
  
  if (httpResponseCode == 200) {
    Serial.println("✅ HTTP POST successful");
  } else {
    Serial.printf("❌ HTTP POST failed: %d\n", httpResponseCode);
  }
  http.end();
}
```

---

## 🔗 4. Blockchain Integration Tests

### 4.1 API Connectivity Test
**Objective**: Verify connection to GridTokenX blockchain API

**Test Procedure**:
```cpp
void testBlockchainConnectivity() {
  Serial.println("=== Blockchain Connectivity Test ===");
  
  BlockchainClient client;
  
  // Test 1: Check API status
  if (client.checkAPIStatus()) {
    Serial.println("✅ Blockchain API is accessible");
  } else {
    Serial.println("❌ Cannot connect to blockchain API");
    return;
  }
  
  // Test 2: Device registration
  if (client.registerDevice()) {
    Serial.println("✅ Device registration successful");
  } else {
    Serial.println("❌ Device registration failed");
  }
  
  // Test 3: Authentication
  if (client.authenticate()) {
    Serial.println("✅ Authentication successful");
  } else {
    Serial.println("❌ Authentication failed");
  }
  
  // Test 4: Get account balance
  float balance = client.getAccountBalance();
  if (balance >= 0) {
    Serial.printf("✅ Account balance: %.2f GTX\n", balance);
  } else {
    Serial.println("❌ Failed to retrieve account balance");
  }
}
```

### 4.2 Data Submission Test
**Objective**: Test energy data submission to blockchain

**Test Implementation**:
```cpp
void testDataSubmission() {
  Serial.println("=== Data Submission Test ===");
  
  BlockchainClient client;
  
  // Create test energy measurement
  EnergyMeasurement testData;
  testData.timestamp = millis();
  testData.voltage = 220.5;
  testData.current = 5.2;
  testData.power = testData.voltage * testData.current;
  testData.energy = testData.power * 0.001; // 1 second = 0.001 hour
  testData.frequency = 50.0;
  testData.powerFactor = 0.95;
  testData.temperature = 28.5;
  testData.humidity = 65.2;
  
  Serial.println("Submitting test data...");
  if (client.submitEnergyData(testData)) {
    Serial.println("✅ Energy data submitted successfully");
  } else {
    Serial.println("❌ Energy data submission failed");
  }
  
  // Verify data was received
  delay(2000);
  
  // Query recent readings
  Serial.println("Verifying data submission...");
  // Implementation depends on blockchain API query capabilities
}
```

### 4.3 Trading Function Test  
**Objective**: Test automated energy trading capabilities

**Test Scenarios**:
```cpp
void testTradingFunctions() {
  Serial.println("=== Trading Functions Test ===");
  
  BlockchainClient client;
  
  // Test 1: Create sell order (excess energy)
  EnergyOrder sellOrder;
  sellOrder.type = "SELL";
  sellOrder.quantity = 1.5;  // 1.5 kWh
  sellOrder.price = 4500;    // 4500 THB per kWh
  sellOrder.timestamp = millis();
  strcpy(sellOrder.deviceId, DEVICE_ID);
  
  Serial.println("Creating sell order...");
  if (client.createEnergyOrder(sellOrder)) {
    Serial.println("✅ Sell order created successfully");
  } else {
    Serial.println("❌ Sell order creation failed");
  }
  
  // Test 2: Create buy order (energy demand)
  EnergyOrder buyOrder;
  buyOrder.type = "BUY";
  buyOrder.quantity = 0.8;   // 0.8 kWh
  buyOrder.price = 3800;     // 3800 THB per kWh
  buyOrder.timestamp = millis();
  strcpy(buyOrder.deviceId, DEVICE_ID);
  
  Serial.println("Creating buy order...");
  if (client.createEnergyOrder(buyOrder)) {
    Serial.println("✅ Buy order created successfully");
  } else {
    Serial.println("❌ Buy order creation failed");
  }
  
  // Test 3: Query market prices
  float currentPrice = client.getCurrentMarketPrice();
  if (currentPrice > 0) {
    Serial.printf("✅ Current market price: %.2f THB/kWh\n", currentPrice);
  } else {
    Serial.println("❌ Failed to get market price");
  }
  
  // Test 4: Check trading history
  Serial.println("Checking trading history...");
  // Implementation depends on API capabilities
}
```

---

## ⚡ 5. Performance Tests

### 5.1 Sensor Reading Performance
**Objective**: Measure sensor reading speed and accuracy

**Test Implementation**:
```cpp
void testSensorPerformance() {
  Serial.println("=== Sensor Performance Test ===");
  
  const int numReadings = 100;
  float voltageReadings[numReadings];
  float currentReadings[numReadings];
  float tempReadings[numReadings];
  
  unsigned long startTime = millis();
  
  // Collect multiple readings
  for (int i = 0; i < numReadings; i++) {
    voltageReadings[i] = readVoltage();
    currentReadings[i] = readCurrent();
    tempReadings[i] = dht.readTemperature();
    delay(10);  // 10ms between readings
  }
  
  unsigned long endTime = millis();
  unsigned long totalTime = endTime - startTime;
  
  Serial.printf("✅ %d readings completed in %lu ms\n", numReadings, totalTime);
  Serial.printf("✅ Average reading time: %.2f ms\n", (float)totalTime / numReadings);
  
  // Calculate statistics
  float voltageSum = 0, currentSum = 0, tempSum = 0;
  float voltageMin = 999, voltageMax = 0;
  float currentMin = 999, currentMax = 0;
  
  for (int i = 0; i < numReadings; i++) {
    voltageSum += voltageReadings[i];
    currentSum += currentReadings[i];
    tempSum += tempReadings[i];
    
    if (voltageReadings[i] < voltageMin) voltageMin = voltageReadings[i];
    if (voltageReadings[i] > voltageMax) voltageMax = voltageReadings[i];
    if (currentReadings[i] < currentMin) currentMin = currentReadings[i];
    if (currentReadings[i] > currentMax) currentMax = currentReadings[i];
  }
  
  Serial.printf("Voltage - Avg: %.2fV, Min: %.2fV, Max: %.2fV, Range: %.2fV\n",
                voltageSum/numReadings, voltageMin, voltageMax, voltageMax-voltageMin);
  Serial.printf("Current - Avg: %.2fA, Min: %.2fA, Max: %.2fA, Range: %.2fA\n", 
                currentSum/numReadings, currentMin, currentMax, currentMax-currentMin);
  Serial.printf("Temperature - Avg: %.1f°C\n", tempSum/numReadings);
}
```

### 5.2 Display Update Performance
**Objective**: Measure display refresh rates and response times

**Test Code**:
```cpp
void testDisplayPerformance() {
  Serial.println("=== Display Performance Test ===");
  
  const int numUpdates = 50;
  unsigned long startTime = millis();
  
  for (int i = 0; i < numUpdates; i++) {
    display.clearDisplay();
    display.setCursor(0, 0);
    display.printf("Update: %d", i);
    display.setCursor(0, 20);
    display.printf("Time: %lu", millis());
    display.display();
  }
  
  unsigned long endTime = millis();
  unsigned long totalTime = endTime - startTime;
  
  Serial.printf("✅ %d display updates in %lu ms\n", numUpdates, totalTime);
  Serial.printf("✅ Average update time: %.2f ms\n", (float)totalTime / numUpdates);
  Serial.printf("✅ Effective FPS: %.1f\n", 1000.0 / ((float)totalTime / numUpdates));
}
```

### 5.3 Network Performance Test
**Objective**: Measure API call latency and throughput

**Test Implementation**:
```cpp
void testNetworkPerformance() {
  Serial.println("=== Network Performance Test ===");
  
  BlockchainClient client;
  const int numRequests = 10;
  unsigned long latencies[numRequests];
  int successCount = 0;
  
  for (int i = 0; i < numRequests; i++) {
    unsigned long requestStart = millis();
    
    if (client.checkAPIStatus()) {
      latencies[successCount] = millis() - requestStart;
      successCount++;
    }
    
    delay(100);  // Brief pause between requests
  }
  
  if (successCount > 0) {
    unsigned long totalLatency = 0;
    unsigned long minLatency = 999999, maxLatency = 0;
    
    for (int i = 0; i < successCount; i++) {
      totalLatency += latencies[i];
      if (latencies[i] < minLatency) minLatency = latencies[i];
      if (latencies[i] > maxLatency) maxLatency = latencies[i];
    }
    
    Serial.printf("✅ Successful requests: %d/%d\n", successCount, numRequests);
    Serial.printf("✅ Average latency: %lu ms\n", totalLatency / successCount);
    Serial.printf("✅ Min latency: %lu ms\n", minLatency);
    Serial.printf("✅ Max latency: %lu ms\n", maxLatency);
  } else {
    Serial.println("❌ No successful requests");
  }
}
```

---

## 🔒 6. Security Tests

### 6.1 Authentication Test
**Objective**: Verify secure authentication mechanisms

**Test Procedures**:
```cpp
void testAuthentication() {
  Serial.println("=== Authentication Security Test ===");
  
  BlockchainClient client;
  
  // Test 1: Valid API key
  Serial.println("Testing valid API key...");
  if (client.authenticate()) {
    Serial.println("✅ Valid authentication successful");
  } else {
    Serial.println("❌ Valid authentication failed");
  }
  
  // Test 2: Invalid API key (temporarily change)
  String originalKey = String(API_KEY);
  strcpy((char*)API_KEY, "invalid_key_test");
  
  Serial.println("Testing invalid API key...");
  if (!client.authenticate()) {
    Serial.println("✅ Invalid authentication properly rejected");
  } else {
    Serial.println("❌ Security issue: Invalid key accepted");
  }
  
  // Restore original key
  strcpy((char*)API_KEY, originalKey.c_str());
  
  // Test 3: TLS/SSL certificate validation
  Serial.println("Testing TLS certificate validation...");
  // Implementation depends on specific TLS library
}
```

### 6.2 Data Integrity Test
**Objective**: Verify data integrity during transmission

**Test Implementation**:
```cpp
void testDataIntegrity() {
  Serial.println("=== Data Integrity Test ===");
  
  // Create test data with known values
  EnergyMeasurement originalData;
  originalData.voltage = 220.123;
  originalData.current = 5.456;
  originalData.power = originalData.voltage * originalData.current;
  originalData.timestamp = 1640995200000; // Known timestamp
  
  // Serialize to JSON
  StaticJsonDocument<512> doc;
  doc["voltage"] = originalData.voltage;
  doc["current"] = originalData.current;
  doc["power"] = originalData.power;
  doc["timestamp"] = originalData.timestamp;
  
  String jsonString;
  serializeJson(doc, jsonString);
  
  Serial.printf("Original JSON: %s\n", jsonString.c_str());
  
  // Parse back from JSON
  StaticJsonDocument<512> parsedDoc;
  deserializeJson(parsedDoc, jsonString);
  
  EnergyMeasurement parsedData;
  parsedData.voltage = parsedDoc["voltage"];
  parsedData.current = parsedDoc["current"];
  parsedData.power = parsedDoc["power"];
  parsedData.timestamp = parsedDoc["timestamp"];
  
  // Verify data integrity
  bool integrityOK = true;
  if (abs(parsedData.voltage - originalData.voltage) > 0.001) integrityOK = false;
  if (abs(parsedData.current - originalData.current) > 0.001) integrityOK = false;
  if (abs(parsedData.power - originalData.power) > 0.001) integrityOK = false;
  if (parsedData.timestamp != originalData.timestamp) integrityOK = false;
  
  if (integrityOK) {
    Serial.println("✅ Data integrity maintained");
  } else {
    Serial.println("❌ Data integrity compromised");
  }
}
```

---

## 🌡️ 7. Environmental Tests

### 7.1 Temperature Stress Test
**Objective**: Verify operation under temperature variations

**Test Procedure**:
1. Operate device in normal conditions (20-25°C)
2. Monitor performance in elevated temperature (35-40°C)
3. Test cold operation (5-10°C)
4. Record any performance degradation

**Monitoring Code**:
```cpp
void monitorEnvironmentalConditions() {
  float ambientTemp = dht.readTemperature();
  float humidity = dht.readHumidity();
  
  Serial.printf("Environmental - Temp: %.1f°C, Humidity: %.1f%%\n", ambientTemp, humidity);
  
  // Check operating limits
  if (ambientTemp > OPERATING_TEMP_MAX) {
    Serial.println("⚠️ Temperature above operating limit");
    // Implement thermal protection
  }
  
  if (ambientTemp < OPERATING_TEMP_MIN) {
    Serial.println("⚠️ Temperature below operating limit");
  }
  
  if (humidity > OPERATING_HUMIDITY_MAX) {
    Serial.println("⚠️ Humidity above operating limit");
  }
}
```

### 7.2 Power Fluctuation Test
**Objective**: Test operation under varying power conditions

**Test Scenarios**:
- Normal operation (5.0V)
- Low voltage operation (4.5V)
- High voltage operation (5.5V)
- Brownout recovery
- Power interruption recovery

---

## 📊 Test Results Documentation

### Test Report Template
```
=== GridTokenX ESP32 Test Report ===
Date: [Test Date]
Firmware Version: [Version]
Tester: [Name]

Hardware Tests:
□ Power Supply: PASS/FAIL
□ Sensor Connectivity: PASS/FAIL  
□ I2C Communication: PASS/FAIL
□ Display Function: PASS/FAIL

Software Tests:
□ Boot Sequence: PASS/FAIL
□ Configuration: PASS/FAIL
□ Memory Management: PASS/FAIL

Network Tests:
□ WiFi Connectivity: PASS/FAIL
□ HTTP/HTTPS: PASS/FAIL
□ Reconnection: PASS/FAIL

Blockchain Tests:
□ API Connectivity: PASS/FAIL
□ Data Submission: PASS/FAIL
□ Trading Functions: PASS/FAIL

Performance Tests:
□ Sensor Performance: PASS/FAIL
□ Display Performance: PASS/FAIL
□ Network Performance: PASS/FAIL

Security Tests:
□ Authentication: PASS/FAIL
□ Data Integrity: PASS/FAIL

Environmental Tests:
□ Temperature Range: PASS/FAIL
□ Power Variations: PASS/FAIL

Overall Result: PASS/FAIL
Notes: [Additional observations]
```

---

## 🔧 Automated Test Execution

### Test Runner Implementation
```cpp
void runAllTests() {
  Serial.println("=== Starting Comprehensive Test Suite ===");
  
  bool allTestsPassed = true;
  
  // Hardware Tests
  Serial.println("\n--- HARDWARE TESTS ---");
  if (!runHardwareTests()) allTestsPassed = false;
  
  // Software Tests
  Serial.println("\n--- SOFTWARE TESTS ---");
  if (!runSoftwareTests()) allTestsPassed = false;
  
  // Network Tests
  Serial.println("\n--- NETWORK TESTS ---");
  if (!runNetworkTests()) allTestsPassed = false;
  
  // Blockchain Tests
  Serial.println("\n--- BLOCKCHAIN TESTS ---");
  if (!runBlockchainTests()) allTestsPassed = false;
  
  // Performance Tests
  Serial.println("\n--- PERFORMANCE TESTS ---");
  if (!runPerformanceTests()) allTestsPassed = false;
  
  // Security Tests
  Serial.println("\n--- SECURITY TESTS ---");
  if (!runSecurityTests()) allTestsPassed = false;
  
  // Final Results
  Serial.println("\n=== TEST SUITE COMPLETE ===");
  if (allTestsPassed) {
    Serial.println("🎉 ALL TESTS PASSED!");
    setStatusLED(0, 255, 0); // Green
  } else {
    Serial.println("❌ SOME TESTS FAILED");
    setStatusLED(255, 0, 0); // Red
  }
}
```

This comprehensive testing suite ensures the GridTokenX ESP32 Smart Energy Meter POC prototype meets all functional, performance, and security requirements for reliable blockchain integration and energy monitoring operations.

---

**GridTokenX Testing Suite** - Ensuring Reliable Smart Energy Operations 🧪⚡
