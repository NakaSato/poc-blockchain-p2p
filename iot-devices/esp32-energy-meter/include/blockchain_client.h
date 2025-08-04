#ifndef BLOCKCHAIN_CLIENT_H
#define BLOCKCHAIN_CLIENT_H

#include <Arduino.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#include <WiFiClientSecure.h>
#include "config.h"
#include "energy_types.h"

// =====================================
// GridTokenX Blockchain Client
// =====================================

class GridTokenXClient {
private:
    // HTTP client for blockchain communication
    HTTPClient http;
    WiFiClient client;
    WiFiClientSecure secureClient;
    
    // Configuration
    String api_base_url;
    String device_id;
    String api_key;
    String device_address;
    
    // Connection state
    bool initialized;
    bool connected;
    unsigned long last_sync_time;
    uint16_t consecutive_errors;
    
    // Internal methods
    String buildApiUrl(const String& endpoint);
    bool makeHttpRequest(const String& method, const String& url, const String& payload, ApiResponse& response);
    String createAuthHeader();
    bool validateResponse(const ApiResponse& response);
    void handleApiError(const ApiResponse& response);
    
public:
    // Constructor and initialization
    GridTokenXClient();
    ~GridTokenXClient();
    
    // Connection management
    bool begin();
    bool begin(const String& api_url, const String& device_id, const String& api_key);
    bool testConnection();
    bool isConnected();
    void disconnect();
    
    // Device registration and management
    bool registerDevice(const DeviceConfig& config);
    bool updateDeviceInfo(const DeviceStatus& status);
    bool heartbeat();
    
    // Energy data submission
    bool submitEnergyReading(const EnergyMeasurement& measurement);
    bool submitBatchReadings(const EnergyMeasurement readings[], size_t count);
    bool submitGridData(const GridStatus& grid_status);
    
    // Energy trading
    bool submitEnergyOrder(const EnergyOrder& order);
    bool cancelEnergyOrder(const String& order_id);
    bool getActiveOrders(String& orders_json);
    bool getOrderStatus(const String& order_id, String& status_json);
    
    // Market data retrieval
    bool getCurrentEnergyPrice(float& price);
    bool getMarketData(String& market_json);
    bool getGridStatus(GridStatus& status);
    bool getEnergyStats(String& stats_json);
    
    // Account management
    bool getAccountInfo(const String& address, String& account_json);
    bool getAccountBalance(const String& address, uint32_t& balance);
    bool getTransactionHistory(const String& address, String& history_json);
    
    // Blockchain operations
    bool getBlockchainStatus(String& status_json);
    bool getLatestBlock(String& block_json);
    bool getTransaction(const String& tx_hash, BlockchainTransaction& transaction);
    bool submitTransaction(const String& transaction_data);
    
    // Carbon credits and sustainability
    bool getCarbonCredits(const String& address, float& credits);
    bool submitCarbonData(float energy_amount, EnergyOrder::EnergySource source);
    
    // Configuration and settings
    bool getDeviceConfig(DeviceConfig& config);
    bool updateDeviceConfig(const DeviceConfig& config);
    bool syncTime();
    
    // Error handling and diagnostics
    uint16_t getErrorCount();
    String getLastError();
    void resetErrorCount();
    bool runDiagnostics();
    
    // Utility methods
    bool isApiEndpointAvailable(const String& endpoint);
    String getApiVersion();
    unsigned long getLastSyncTime();
    void setApiTimeout(uint32_t timeout_ms);
    
    // Automated trading support
    bool enableAutoTrading(const DeviceConfig& config);
    bool disableAutoTrading();
    bool checkTradingOpportunities(String& opportunities_json);
    bool executeTrade(const EnergyOrder& order);
    
    // Real-time data streaming
    bool startDataStream();
    bool stopDataStream();
    bool sendRealtimeData(const EnergyMeasurement& measurement);
    
    // Security and authentication
    bool rotateApiKey();
    bool verifyDeviceSignature(const String& data, const String& signature);
    String signData(const String& data);
    
    // Batch operations
    bool batchSubmitOrders(const EnergyOrder orders[], size_t count);
    bool batchCancelOrders(const String order_ids[], size_t count);
    bool batchGetOrderStatus(const String order_ids[], size_t count, String& status_json);
    
    // Event handling
    typedef void (*EventCallback)(const String& event_type, const String& event_data);
    void setEventCallback(EventCallback callback);
    void processEvents();
    
    // Constants
    static const uint16_t MAX_RETRIES = 3;
    static const uint32_t DEFAULT_TIMEOUT = 15000;
    static const uint32_t HEARTBEAT_INTERVAL = 60000;
    static const uint16_t MAX_CONSECUTIVE_ERRORS = 5;
};

// =====================================
// Utility Functions
// =====================================

// HTTP response parser
class ApiResponseParser {
public:
    static bool parseEnergyPrice(const String& json, float& price);
    static bool parseOrderStatus(const String& json, EnergyOrder& order);
    static bool parseAccountInfo(const String& json, String& address, uint32_t& balance);
    static bool parseGridStatus(const String& json, GridStatus& status);
    static bool parseMarketData(const String& json, float& price, float& volume);
    static bool parseTransactionStatus(const String& json, BlockchainTransaction& tx);
    static bool parseDeviceConfig(const String& json, DeviceConfig& config);
    static bool parseErrorResponse(const String& json, String& error_message);
};

// Request builder
class ApiRequestBuilder {
public:
    static String buildEnergyReadingRequest(const EnergyMeasurement& measurement);
    static String buildEnergyOrderRequest(const EnergyOrder& order);
    static String buildDeviceRegistrationRequest(const DeviceConfig& config);
    static String buildDeviceStatusRequest(const DeviceStatus& status);
    static String buildGridDataRequest(const GridStatus& status);
    static String buildCarbonCreditRequest(float energy, EnergyOrder::EnergySource source);
    static String buildConfigUpdateRequest(const DeviceConfig& config);
    static String buildTransactionRequest(const String& transaction_data);
};

// Connection manager
class ConnectionManager {
private:
    static bool wifi_connected;
    static unsigned long last_connection_check;
    static uint8_t connection_retries;
    
public:
    static bool ensureWiFiConnection();
    static bool checkInternetConnectivity();
    static void handleConnectionFailure();
    static int8_t getSignalStrength();
    static String getNetworkInfo();
    static bool isStableConnection();
    
    static const uint32_t CONNECTION_CHECK_INTERVAL = 30000;
    static const uint8_t MAX_CONNECTION_RETRIES = 5;
    static const int8_t MIN_SIGNAL_STRENGTH = -80; // dBm
};

// Error codes for blockchain operations
enum class BlockchainError {
    SUCCESS = 0,
    NETWORK_ERROR = 1,
    API_ERROR = 2,
    AUTHENTICATION_ERROR = 3,
    INVALID_DATA = 4,
    TIMEOUT_ERROR = 5,
    SERVER_ERROR = 6,
    RATE_LIMIT_ERROR = 7,
    INSUFFICIENT_BALANCE = 8,
    INVALID_ORDER = 9,
    ORDER_NOT_FOUND = 10,
    DEVICE_NOT_REGISTERED = 11,
    CONFIGURATION_ERROR = 12,
    SECURITY_ERROR = 13,
    UNKNOWN_ERROR = 99
};

// Helper function to convert error code to string
String blockchainErrorToString(BlockchainError error);

// Global client instance (optional singleton pattern)
extern GridTokenXClient* g_blockchain_client;

#endif // BLOCKCHAIN_CLIENT_H
    
    // Status and statistics
    bool isConnected() { return is_connected; }
    uint32_t getTotalRequests() { return total_requests; }
    uint32_t getSuccessfulRequests() { return successful_requests; }
    uint32_t getFailedRequests() { return failed_requests; }
    uint32_t getLastSyncTimestamp() { return last_sync_timestamp; }
    
private:
    String api_host;
    int api_port;
    String api_path;
    String device_id;
    bool is_connected;
    uint32_t last_sync_timestamp;
    uint32_t total_requests;
    uint32_t successful_requests;
    uint32_t failed_requests;
    
    // Helper methods
    BlockchainResponse parseBlockchainResponse(const String& response_body);
    String generateSignature(const String& data);
    void log_message(const String& message);
    uint32_t getCurrentTimestamp();
};

#endif // BLOCKCHAIN_CLIENT_H
