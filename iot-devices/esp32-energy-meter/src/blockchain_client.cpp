/**
 * GridTokenX Blockchain Client Implementation
 * 
 * Implementation of the blockchain communication client for ESP32
 * Provides secure communication with GridTokenX blockchain API
 * 
 * Features:
 * - HTTP/HTTPS communication with blockchain nodes
 * - Energy data submission and retrieval
 * - Automated trading capabilities
 * - Device registration and management
 * - Error handling and retry logic
 * - Security and authentication
 * 
 * Author: GridTokenX Development Team
 * Version: 1.0.0
 */

#include "blockchain_client.h"
#include <WiFiClientSecure.h>
#include <base64.h>

// =====================================
// GridTokenXClient Implementation
// =====================================

GridTokenXClient::GridTokenXClient() {
    initialized = false;
    connected = false;
    last_sync_time = 0;
    consecutive_errors = 0;
    api_base_url = BLOCKCHAIN_API_BASE_URL;
    device_id = DEVICE_ID;
    api_key = API_KEY;
    device_address = DEVICE_ADDRESS;
}

GridTokenXClient::~GridTokenXClient() {
    disconnect();
}

bool GridTokenXClient::begin() {
    return begin(BLOCKCHAIN_API_BASE_URL, DEVICE_ID, API_KEY);
}

bool GridTokenXClient::begin(const String& api_url, const String& device_id, const String& api_key) {
    this->api_base_url = api_url;
    this->device_id = device_id;
    this->api_key = api_key;
    
    Serial.println("Initializing GridTokenX Blockchain Client...");
    Serial.println("API URL: " + api_url);
    Serial.println("Device ID: " + device_id);
    
    // Test connection to blockchain API
    if (!testConnection()) {
        Serial.println("Failed to connect to blockchain API");
        return false;
    }
    
    initialized = true;
    connected = true;
    consecutive_errors = 0;
    
    Serial.println("Blockchain client initialized successfully!");
    return true;
}

bool GridTokenXClient::testConnection() {
    ApiResponse response;
    String status_url = buildApiUrl(API_STATUS);
    
    Serial.println("Testing blockchain connection...");
    Serial.println("URL: " + status_url);
    
    if (!makeHttpRequest("GET", status_url, "", response)) {
        Serial.println("Connection test failed - HTTP request failed");
        return false;
    }
    
    if (response.status_code == 200) {
        Serial.println("Blockchain connection test successful!");
        return true;
    } else {
        Serial.println("Connection test failed - Status code: " + String(response.status_code));
        Serial.println("Response: " + String(response.message));
        return false;
    }
}

bool GridTokenXClient::isConnected() {
    return connected && initialized;
}

void GridTokenXClient::disconnect() {
    http.end();
    connected = false;
    Serial.println("Blockchain client disconnected");
}

bool GridTokenXClient::registerDevice(const DeviceConfig& config) {
    Serial.println("Registering device with blockchain...");
    
    String request_body = ApiRequestBuilder::buildDeviceRegistrationRequest(config);
    String register_url = buildApiUrl(API_IOT_REGISTER);
    
    ApiResponse response;
    if (!makeHttpRequest("POST", register_url, request_body, response)) {
        Serial.println("Device registration failed - HTTP request failed");
        return false;
    }
    
    if (response.status_code == 200 || response.status_code == 201) {
        Serial.println("Device registered successfully!");
        return true;
    } else {
        Serial.println("Device registration failed - Status: " + String(response.status_code));
        Serial.println("Message: " + String(response.message));
        return false;
    }
}

bool GridTokenXClient::submitEnergyReading(const EnergyMeasurement& measurement) {
    if (!isConnected()) {
        Serial.println("Cannot submit energy reading - not connected to blockchain");
        return false;
    }
    
    String request_body = ApiRequestBuilder::buildEnergyReadingRequest(measurement);
    String readings_url = buildApiUrl(API_ENERGY_READINGS);
    
    ApiResponse response;
    if (!makeHttpRequest("POST", readings_url, request_body, response)) {
        handleApiError(response);
        return false;
    }
    
    if (response.status_code == 200 || response.status_code == 201) {
        Serial.println("Energy reading submitted successfully");
        consecutive_errors = 0;
        last_sync_time = millis();
        return true;
    } else {
        Serial.println("Failed to submit energy reading - Status: " + String(response.status_code));
        handleApiError(response);
        return false;
    }
}

bool GridTokenXClient::submitEnergyOrder(const EnergyOrder& order) {
    if (!isConnected()) {
        Serial.println("Cannot submit energy order - not connected to blockchain");
        return false;
    }
    
    String request_body = ApiRequestBuilder::buildEnergyOrderRequest(order);
    String orders_url = buildApiUrl(API_ENERGY_ORDERS);
    
    Serial.println("Submitting energy order...");
    Serial.println("Order type: " + String(order.order_type == EnergyOrder::BUY_ORDER ? "BUY" : "SELL"));
    Serial.println("Amount: " + String(order.energy_amount) + " kWh");
    Serial.println("Price: " + String(order.price_per_kwh) + " tokens/kWh");
    
    ApiResponse response;
    if (!makeHttpRequest("POST", orders_url, request_body, response)) {
        handleApiError(response);
        return false;
    }
    
    if (response.status_code == 200 || response.status_code == 201) {
        Serial.println("Energy order submitted successfully");
        
        // Parse response to get order ID
        DynamicJsonDocument doc(1024);
        deserializeJson(doc, response.data);
        if (doc.containsKey("order_id")) {
            Serial.println("Order ID: " + doc["order_id"].as<String>());
        }
        
        return true;
    } else {
        Serial.println("Failed to submit energy order - Status: " + String(response.status_code));
        handleApiError(response);
        return false;
    }
}

bool GridTokenXClient::getCurrentEnergyPrice(float& price) {
    String market_url = buildApiUrl("/energy/market-price");
    
    ApiResponse response;
    if (!makeHttpRequest("GET", market_url, "", response)) {
        return false;
    }
    
    if (response.status_code == 200) {
        return ApiResponseParser::parseEnergyPrice(response.data, price);
    }
    
    return false;
}

bool GridTokenXClient::getGridStatus(GridStatus& status) {
    String grid_url = buildApiUrl(API_GRID_STATUS);
    
    ApiResponse response;
    if (!makeHttpRequest("GET", grid_url, "", response)) {
        return false;
    }
    
    if (response.status_code == 200) {
        return ApiResponseParser::parseGridStatus(response.data, status);
    }
    
    return false;
}

bool GridTokenXClient::getAccountBalance(const String& address, uint32_t& balance) {
    String account_url = buildApiUrl(API_ACCOUNTS) + "/" + address;
    
    ApiResponse response;
    if (!makeHttpRequest("GET", account_url, "", response)) {
        return false;
    }
    
    if (response.status_code == 200) {
        String dummy_address;
        return ApiResponseParser::parseAccountInfo(response.data, dummy_address, balance);
    }
    
    return false;
}

bool GridTokenXClient::heartbeat() {
    String heartbeat_url = buildApiUrl("/iot/heartbeat");
    String payload = "{"device_id":"" + device_id + "","timestamp":" + String(millis()) + "}";
    
    ApiResponse response;
    return makeHttpRequest("POST", heartbeat_url, payload, response) && 
           (response.status_code == 200);
}

// =====================================
// Private Helper Methods
// =====================================

String GridTokenXClient::buildApiUrl(const String& endpoint) {
    String url = api_base_url;
    if (!endpoint.startsWith("/")) {
        url += "/";
    }
    url += endpoint;
    return url;
}

bool GridTokenXClient::makeHttpRequest(const String& method, const String& url, 
                                     const String& payload, ApiResponse& response) {
    
    // Ensure WiFi is connected
    if (WiFi.status() != WL_CONNECTED) {
        response.success = false;
        response.status_code = 0;
        strcpy(response.message, "WiFi not connected");
        return false;
    }
    
    // Initialize HTTP client
    http.begin(url);
    http.setTimeout(BLOCKCHAIN_API_TIMEOUT);
    
    // Set headers
    http.addHeader("Content-Type", "application/json");
    http.addHeader("User-Agent", "GridTokenX-ESP32/1.0");
    
    // Add authentication header if API key is available
    if (api_key.length() > 0) {
        http.addHeader("Authorization", "Bearer " + api_key);
    }
    
    // Add device identification headers
    http.addHeader("X-Device-ID", device_id);
    http.addHeader("X-Device-Address", device_address);
    http.addHeader("X-Device-Type", DEVICE_TYPE);
    
    int http_code = -1;
    
    // Make the HTTP request
    if (method == "GET") {
        http_code = http.GET();
    } else if (method == "POST") {
        http_code = http.POST(payload);
    } else if (method == "PUT") {
        http_code = http.PUT(payload);
    } else if (method == "DELETE") {
        http_code = http.sendRequest("DELETE", payload);
    }
    
    // Process response
    response.status_code = http_code;
    response.timestamp = millis();
    
    if (http_code > 0) {
        String response_payload = http.getString();
        response.success = (http_code >= 200 && http_code < 300);
        
        // Copy response data (truncate if too long)
        int copy_length = min(response_payload.length(), (unsigned int)(sizeof(response.data) - 1));
        response_payload.substring(0, copy_length).toCharArray(response.data, sizeof(response.data));
        
        // Extract error message if available
        if (!response.success) {
            String error_msg;
            if (ApiResponseParser::parseErrorResponse(response_payload, error_msg)) {
                error_msg.toCharArray(response.message, sizeof(response.message));
            } else {
                String("HTTP " + String(http_code)).toCharArray(response.message, sizeof(response.message));
            }
        } else {
            strcpy(response.message, "Success");
        }
    } else {
        response.success = false;
        String("HTTP request failed: " + String(http_code)).toCharArray(response.message, sizeof(response.message));
        strcpy(response.data, "");
    }
    
    http.end();
    return response.success;
}

void GridTokenXClient::handleApiError(const ApiResponse& response) {
    consecutive_errors++;
    
    Serial.println("API Error occurred:");
    Serial.println("Status Code: " + String(response.status_code));
    Serial.println("Message: " + String(response.message));
    Serial.println("Consecutive Errors: " + String(consecutive_errors));
    
    if (consecutive_errors >= MAX_CONSECUTIVE_ERRORS) {
        Serial.println("Maximum consecutive errors reached - entering recovery mode");
        connected = false;
        
        // Implement exponential backoff or other recovery strategies
        delay(5000 * consecutive_errors); // Progressive delay
    }
}

// =====================================
// API Response Parser Implementation
// =====================================

bool ApiResponseParser::parseEnergyPrice(const String& json, float& price) {
    DynamicJsonDocument doc(512);
    if (deserializeJson(doc, json) != DeserializationError::Ok) {
        return false;
    }
    
    if (doc.containsKey("price")) {
        price = doc["price"].as<float>();
        return true;
    }
    
    return false;
}

bool ApiResponseParser::parseGridStatus(const String& json, GridStatus& status) {
    DynamicJsonDocument doc(1024);
    if (deserializeJson(doc, json) != DeserializationError::Ok) {
        return false;
    }
    
    status.grid_connected = doc["connected"].as<bool>();
    status.grid_stable = doc["stable"].as<bool>();
    status.grid_voltage = doc["voltage"].as<float>();
    status.grid_frequency = doc["frequency"].as<float>();
    status.total_load = doc["total_load"].as<float>();
    status.renewable_percentage = doc["renewable_percentage"].as<float>();
    status.carbon_intensity = doc["carbon_intensity"].as<float>();
    status.peak_demand_period = doc["peak_demand"].as<bool>();
    status.last_update = millis();
    
    return true;
}

bool ApiResponseParser::parseAccountInfo(const String& json, String& address, uint32_t& balance) {
    DynamicJsonDocument doc(512);
    if (deserializeJson(doc, json) != DeserializationError::Ok) {
        return false;
    }
    
    if (doc.containsKey("address") && doc.containsKey("balance")) {
        address = doc["address"].as<String>();
        balance = doc["balance"].as<uint32_t>();
        return true;
    }
    
    return false;
}

bool ApiResponseParser::parseErrorResponse(const String& json, String& error_message) {
    DynamicJsonDocument doc(512);
    if (deserializeJson(doc, json) != DeserializationError::Ok) {
        return false;
    }
    
    if (doc.containsKey("error")) {
        error_message = doc["error"].as<String>();
        return true;
    } else if (doc.containsKey("message")) {
        error_message = doc["message"].as<String>();
        return true;
    }
    
    return false;
}

// =====================================
// API Request Builder Implementation
// =====================================

String ApiRequestBuilder::buildEnergyReadingRequest(const EnergyMeasurement& measurement) {
    DynamicJsonDocument doc(1024);
    
    doc["device_id"] = measurement.device_id;
    doc["timestamp"] = measurement.timestamp;
    doc["voltage"] = measurement.voltage;
    doc["current"] = measurement.current;
    doc["power"] = measurement.power;
    doc["energy"] = measurement.energy;
    doc["power_factor"] = measurement.power_factor;
    doc["frequency"] = measurement.frequency;
    doc["temperature"] = measurement.temperature;
    doc["humidity"] = measurement.humidity;
    doc["zone"] = measurement.zone;
    doc["data_valid"] = measurement.data_valid;
    doc["signal_quality"] = measurement.signal_quality;
    
    String json_string;
    serializeJson(doc, json_string);
    return json_string;
}

String ApiRequestBuilder::buildEnergyOrderRequest(const EnergyOrder& order) {
    DynamicJsonDocument doc(1024);
    
    doc["device_address"] = order.device_address;
    doc["order_type"] = (order.order_type == EnergyOrder::BUY_ORDER) ? "buy" : "sell";
    doc["energy_amount"] = order.energy_amount;
    doc["price_per_kwh"] = order.price_per_kwh;
    doc["total_value"] = order.total_value;
    
    // Convert energy source enum to string
    const char* source_names[] = {"solar", "wind", "hydro", "biomass", "geothermal", "grid_mixed", "unknown"};
    doc["energy_source"] = source_names[order.energy_source];
    
    doc["carbon_credits"] = order.carbon_credits;
    doc["energy_quality"] = order.energy_quality;
    doc["grid_location"] = order.grid_location;
    doc["expiration_hours"] = ORDER_EXPIRATION_HOURS;
    
    String json_string;
    serializeJson(doc, json_string);
    return json_string;
}

String ApiRequestBuilder::buildDeviceRegistrationRequest(const DeviceConfig& config) {
    DynamicJsonDocument doc(1024);
    
    doc["device_id"] = DEVICE_ID;
    doc["device_type"] = DEVICE_TYPE;
    doc["device_address"] = DEVICE_ADDRESS;
    doc["location"] = DEVICE_LOCATION;
    doc["zone"] = DEVICE_ZONE;
    doc["firmware_version"] = FIRMWARE_VERSION;
    doc["manufacturer"] = MANUFACTURER;
    doc["model"] = DEVICE_MODEL;
    
    // Add capabilities
    JsonArray capabilities = doc.createNestedArray("capabilities");
    capabilities.add("energy_monitoring");
    capabilities.add("automated_trading");
    capabilities.add("grid_monitoring");
    capabilities.add("carbon_tracking");
    
    String json_string;
    serializeJson(doc, json_string);
    return json_string;
}

// =====================================
// Connection Manager Implementation
// =====================================

bool ConnectionManager::wifi_connected = false;
unsigned long ConnectionManager::last_connection_check = 0;
uint8_t ConnectionManager::connection_retries = 0;

bool ConnectionManager::ensureWiFiConnection() {
    if (WiFi.status() == WL_CONNECTED) {
        wifi_connected = true;
        connection_retries = 0;
        return true;
    }
    
    unsigned long current_time = millis();
    if (current_time - last_connection_check < CONNECTION_CHECK_INTERVAL) {
        return false; // Too soon to check again
    }
    
    last_connection_check = current_time;
    connection_retries++;
    
    if (connection_retries > MAX_CONNECTION_RETRIES) {
        Serial.println("Max WiFi connection retries reached");
        return false;
    }
    
    Serial.println("Attempting WiFi reconnection... (Attempt " + String(connection_retries) + ")");
    
    WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
    
    int timeout = 0;
    while (WiFi.status() != WL_CONNECTED && timeout < 20) { // 10 second timeout
        delay(500);
        Serial.print(".");
        timeout++;
    }
    
    if (WiFi.status() == WL_CONNECTED) {
        Serial.println("
WiFi reconnected successfully!");
        Serial.println("IP address: " + WiFi.localIP().toString());
        wifi_connected = true;
        connection_retries = 0;
        return true;
    } else {
        Serial.println("
WiFi reconnection failed");
        wifi_connected = false;
        return false;
    }
}

bool ConnectionManager::checkInternetConnectivity() {
    // Simple ping test to Google DNS
    WiFiClient client;
    if (client.connect("8.8.8.8", 53)) {
        client.stop();
        return true;
    }
    return false;
}

int8_t ConnectionManager::getSignalStrength() {
    return WiFi.RSSI();
}

String ConnectionManager::getNetworkInfo() {
    String info = "SSID: " + WiFi.SSID() + "
";
    info += "IP: " + WiFi.localIP().toString() + "
";
    info += "RSSI: " + String(WiFi.RSSI()) + " dBm
";
    info += "Channel: " + String(WiFi.channel()) + "
";
    return info;
}

// =====================================
// Utility Functions
// =====================================

String blockchainErrorToString(BlockchainError error) {
    switch (error) {
        case BlockchainError::SUCCESS: return "Success";
        case BlockchainError::NETWORK_ERROR: return "Network Error";
        case BlockchainError::API_ERROR: return "API Error";
        case BlockchainError::AUTHENTICATION_ERROR: return "Authentication Error";
        case BlockchainError::INVALID_DATA: return "Invalid Data";
        case BlockchainError::TIMEOUT_ERROR: return "Timeout Error";
        case BlockchainError::SERVER_ERROR: return "Server Error";
        case BlockchainError::RATE_LIMIT_ERROR: return "Rate Limit Error";
        case BlockchainError::INSUFFICIENT_BALANCE: return "Insufficient Balance";
        case BlockchainError::INVALID_ORDER: return "Invalid Order";
        case BlockchainError::ORDER_NOT_FOUND: return "Order Not Found";
        case BlockchainError::DEVICE_NOT_REGISTERED: return "Device Not Registered";
        case BlockchainError::CONFIGURATION_ERROR: return "Configuration Error";
        case BlockchainError::SECURITY_ERROR: return "Security Error";
        default: return "Unknown Error";
    }
}

// Global client instance
GridTokenXClient* g_blockchain_client = nullptr;

#include "blockchain_client.h"
#include "config.h"
#include "energy_types.h"
#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#include <mbedtls/sha256.h>

BlockchainClient::BlockchainClient() {
    api_host = BLOCKCHAIN_API_HOST;
    api_port = BLOCKCHAIN_API_PORT;
    api_path = BLOCKCHAIN_API_PATH;
    device_id = DEVICE_ID;
    last_sync_timestamp = 0;
    total_requests = 0;
    successful_requests = 0;
    failed_requests = 0;
    is_connected = false;
}

bool BlockchainClient::begin() {
    log_message("Initializing Blockchain Client...");
    
    // Test connectivity to blockchain node
    return testConnection();
}

bool BlockchainClient::testConnection() {
    HTTPClient http;
    String url = "http://" + api_host + ":" + String(api_port) + api_path + "/status";
    
    http.begin(url);
    http.setTimeout(API_TIMEOUT_MS);
    http.addHeader("Content-Type", "application/json");
    http.addHeader("User-Agent", "GridTokenX-ESP32-Client/1.0");
    
    int response_code = http.GET();
    
    if (response_code == 200) {
        String response = http.getString();
        log_message("Blockchain connection successful: " + response);
        is_connected = true;
        http.end();
        return true;
    } else {
        log_message("Blockchain connection failed: " + String(response_code));
        is_connected = false;
        http.end();
        return false;
    }
}

BlockchainResponse BlockchainClient::submitEnergyReading(const EnergyReading& reading) {
    BlockchainResponse response;
    response.success = false;
    
    if (!is_connected && !testConnection()) {
        response.error_message = "Blockchain not connected";
        return response;
    }
    
    HTTPClient http;
    String url = "http://" + api_host + ":" + String(api_port) + api_path + "/energy/meter-reading";
    
    http.begin(url);
    http.setTimeout(API_TIMEOUT_MS);
    http.addHeader("Content-Type", "application/json");
    http.addHeader("Device-ID", device_id);
    http.addHeader("Device-Type", DEVICE_TYPE);
    
    // Create JSON payload
    DynamicJsonDocument doc(2048);
    doc["device_id"] = reading.device_id;
    doc["timestamp"] = reading.timestamp;
    doc["energy_consumed"] = reading.energy_consumed_kwh;
    doc["energy_produced"] = reading.energy_produced_kwh;
    doc["instantaneous_power"] = reading.instantaneous_power_w;
    doc["voltage"] = reading.voltage_v;
    doc["current"] = reading.current_a;
    doc["frequency"] = reading.frequency_hz;
    doc["power_factor"] = reading.power_factor;
    doc["temperature"] = reading.temperature_c;
    doc["humidity"] = reading.humidity_percent;
    doc["location"] = reading.location;
    doc["energy_source"] = reading.energy_source;
    doc["grid_operator"] = reading.grid_operator;
    doc["carbon_credits"] = reading.carbon_credits;
    doc["sequence_number"] = reading.sequence_number;
    
    String payload;
    serializeJson(doc, payload);
    
    // Add digital signature for data integrity
    String signature = generateSignature(payload);
    http.addHeader("Device-Signature", signature);
    
    log_message("Submitting energy reading to blockchain...");
    
    uint32_t start_time = millis();
    int response_code = http.POST(payload);
    uint32_t response_time = millis() - start_time;
    
    total_requests++;
    
    if (response_code == 200 || response_code == 201) {
        String response_body = http.getString();
        response = parseBlockchainResponse(response_body);
        response.confirmation_time_ms = response_time;
        response.success = true;
        
        successful_requests++;
        last_sync_timestamp = getCurrentTimestamp();
        
        log_message("Energy reading submitted successfully");
        log_message("Transaction hash: " + response.transaction_hash);
        
    } else {
        failed_requests++;
        response.error_message = "HTTP Error: " + String(response_code);
        log_message("Energy reading submission failed: " + response.error_message);
    }
    
    http.end();
    return response;
}

BlockchainResponse BlockchainClient::submitTradeOrder(const EnergyTradeOrder& order) {
    BlockchainResponse response;
    response.success = false;
    
    HTTPClient http;
    String url = "http://" + api_host + ":" + String(api_port) + api_path + "/energy/orders";
    
    http.begin(url);
    http.setTimeout(API_TIMEOUT_MS);
    http.addHeader("Content-Type", "application/json");
    http.addHeader("Device-ID", device_id);
    
    // Create JSON payload for trade order
    DynamicJsonDocument doc(1024);
    doc["device_id"] = order.device_id;
    doc["order_id"] = order.order_id;
    doc["order_type"] = order.order_type;
    doc["amount_kwh"] = order.amount_kwh;
    doc["price_per_kwh"] = order.price_per_kwh;
    doc["energy_type"] = order.energy_type;
    doc["time_slot"] = order.time_slot;
    doc["location_preference"] = order.location_preference;
    doc["expiration_timestamp"] = order.expiration_timestamp;
    doc["auto_execute"] = order.auto_execute;
    
    String payload;
    serializeJson(doc, payload);
    
    String signature = generateSignature(payload);
    http.addHeader("Device-Signature", signature);
    
    log_message("Submitting trade order to blockchain...");
    
    int response_code = http.POST(payload);
    total_requests++;
    
    if (response_code == 200 || response_code == 201) {
        String response_body = http.getString();
        response = parseBlockchainResponse(response_body);
        response.success = true;
        successful_requests++;
        
        log_message("Trade order submitted successfully");
        log_message("Order ID: " + order.order_id);
        
    } else {
        failed_requests++;
        response.error_message = "HTTP Error: " + String(response_code);
        log_message("Trade order submission failed: " + response.error_message);
    }
    
    http.end();
    return response;
}

EnergyPricing BlockchainClient::getCurrentEnergyPricing() {
    EnergyPricing pricing;
    
    HTTPClient http;
    String url = "http://" + api_host + ":" + String(api_port) + api_path + "/energy/pricing";
    
    http.begin(url);
    http.setTimeout(API_TIMEOUT_MS);
    http.addHeader("Content-Type", "application/json");
    
    int response_code = http.GET();
    
    if (response_code == 200) {
        String response_body = http.getString();
        
        DynamicJsonDocument doc(1024);
        if (deserializeJson(doc, response_body) == DeserializationError::Ok) {
            pricing.base_price_per_kwh = doc["base_price_per_kwh"] | 3500.0;
            pricing.peak_multiplier = doc["peak_multiplier"] | 1.5;
            pricing.off_peak_multiplier = doc["off_peak_multiplier"] | 0.8;
            pricing.renewable_bonus = doc["renewable_bonus"] | 500.0;
            pricing.carbon_credit_value = doc["carbon_credit_value"] | 100.0;
            pricing.tariff_structure = doc["tariff_structure"] | "time_of_use";
            pricing.valid_until_timestamp = doc["valid_until_timestamp"] | 0;
            
            log_message("Energy pricing updated: " + String(pricing.base_price_per_kwh) + " tokens/kWh");
        }
    } else {
        log_message("Failed to get energy pricing: " + String(response_code));
        // Use default values
        pricing.base_price_per_kwh = ENERGY_PRICE_DEFAULT;
        pricing.peak_multiplier = 1.5;
        pricing.off_peak_multiplier = 0.8;
        pricing.renewable_bonus = 500.0;
        pricing.carbon_credit_value = 100.0;
        pricing.tariff_structure = "time_of_use";
        pricing.valid_until_timestamp = 0;
    }
    
    http.end();
    return pricing;
}

String BlockchainClient::getAccountBalance() {
    HTTPClient http;
    String url = "http://" + api_host + ":" + String(api_port) + api_path + "/accounts/" + device_id + "/balance";
    
    http.begin(url);
    http.setTimeout(API_TIMEOUT_MS);
    http.addHeader("Device-ID", device_id);
    
    int response_code = http.GET();
    String balance = "0.0";
    
    if (response_code == 200) {
        String response_body = http.getString();
        
        DynamicJsonDocument doc(512);
        if (deserializeJson(doc, response_body) == DeserializationError::Ok) {
            balance = doc["balance"] | "0.0";
            log_message("Account balance: " + balance + " tokens");
        }
    } else {
        log_message("Failed to get account balance: " + String(response_code));
    }
    
    http.end();
    return balance;
}

bool BlockchainClient::registerDevice() {
    HTTPClient http;
    String url = "http://" + api_host + ":" + String(api_port) + api_path + "/devices/register";
    
    http.begin(url);
    http.setTimeout(API_TIMEOUT_MS);
    http.addHeader("Content-Type", "application/json");
    
    // Create device registration payload
    DynamicJsonDocument doc(1024);
    doc["device_id"] = device_id;
    doc["device_type"] = DEVICE_TYPE;
    doc["firmware_version"] = FIRMWARE_VERSION;
    doc["location"] = DEVICE_LOCATION;
    doc["zone"] = DEVICE_ZONE;
    doc["grid_operator"] = GRID_OPERATOR;
    doc["capabilities"] = JsonArray();
    doc["capabilities"].add("energy_measurement");
    doc["capabilities"].add("energy_trading");
    doc["capabilities"].add("grid_monitoring");
    
    String payload;
    serializeJson(doc, payload);
    
    String signature = generateSignature(payload);
    http.addHeader("Device-Signature", signature);
    
    log_message("Registering device with blockchain...");
    
    int response_code = http.POST(payload);
    
    if (response_code == 200 || response_code == 201) {
        log_message("Device registered successfully");
        http.end();
        return true;
    } else {
        log_message("Device registration failed: " + String(response_code));
        http.end();
        return false;
    }
}

BlockchainResponse BlockchainClient::parseBlockchainResponse(const String& response_body) {
    BlockchainResponse response;
    
    DynamicJsonDocument doc(1024);
    if (deserializeJson(doc, response_body) == DeserializationError::Ok) {
        response.success = doc["success"] | false;
        response.transaction_hash = doc["transaction_hash"] | "";
        response.block_hash = doc["block_hash"] | "";
        response.block_number = doc["block_number"] | 0;
        response.transaction_fee = doc["transaction_fee"] | 0.0;
        response.error_message = doc["error_message"] | "";
        response.account_balance = doc["account_balance"] | 0.0;
        response.energy_price_current = doc["energy_price_current"] | 0.0;
    }
    
    return response;
}

String BlockchainClient::generateSignature(const String& data) {
    // Simple hash-based signature (in production, use proper ECDSA)
    mbedtls_sha256_context ctx;
    unsigned char hash[32];
    
    mbedtls_sha256_init(&ctx);
    mbedtls_sha256_starts(&ctx, 0);
    mbedtls_sha256_update(&ctx, (unsigned char*)data.c_str(), data.length());
    mbedtls_sha256_update(&ctx, (unsigned char*)DEVICE_PRIVATE_KEY, strlen(DEVICE_PRIVATE_KEY));
    mbedtls_sha256_finish(&ctx, hash);
    mbedtls_sha256_free(&ctx);
    
    String signature = "";
    for (int i = 0; i < 32; i++) {
        signature += String(hash[i], HEX);
    }
    
    return signature;
}

void BlockchainClient::log_message(const String& message) {
    if (DEBUG_ENABLED) {
        Serial.println("[BlockchainClient] " + message);
    }
}

uint32_t BlockchainClient::getCurrentTimestamp() {
    time_t now;
    time(&now);
    return (uint32_t)now;
}
