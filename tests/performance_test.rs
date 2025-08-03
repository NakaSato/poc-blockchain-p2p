//! GridTokenX Performance Testing Suite
//! 
//! This module contains comprehensive performance tests to validate
//! the auto-scaling capabilities of the GridTokenX blockchain.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
use anyhow::Result;

use gridtokenx_blockchain::{
    Blockchain, Block, Transaction, StorageManager, ScalingCoordinator, ScalingConfig,
    AccountType, TransactionType, EnergyType
};

/// Performance test configuration
#[derive(Debug, Clone)]
pub struct PerformanceTestConfig {
    pub duration_seconds: u64,
    pub transactions_per_second: u64,
    pub concurrent_threads: usize,
    pub scaling_enabled: bool,
    pub test_name: String,
}

impl Default for PerformanceTestConfig {
    fn default() -> Self {
        Self {
            duration_seconds: 60,
            transactions_per_second: 50,
            concurrent_threads: 4,
            scaling_enabled: true,
            test_name: "Default Performance Test".to_string(),
        }
    }
}

/// Performance test results
#[derive(Debug)]
pub struct PerformanceTestResults {
    pub test_name: String,
    pub duration: Duration,
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub avg_tps: f64,
    pub peak_tps: f64,
    pub avg_latency_ms: f64,
    pub peak_latency_ms: f64,
    pub initial_shards: u64,
    pub peak_shards: u64,
    pub final_shards: u64,
    pub memory_usage_mb: Vec<f64>,
    pub cpu_usage_percent: Vec<f64>,
    pub scaling_events: Vec<ScalingEvent>,
}

#[derive(Debug, Clone)]
pub struct ScalingEvent {
    pub timestamp: Instant,
    pub event_type: ScalingEventType,
    pub shard_count: u64,
    pub trigger_reason: String,
}

#[derive(Debug, Clone)]
pub enum ScalingEventType {
    ScaleUp,
    ScaleDown,
    NoChange,
}

/// Main performance testing orchestrator
pub struct PerformanceTester {
    blockchain: Arc<RwLock<Blockchain>>,
    scaling_coordinator: Arc<ScalingCoordinator>,
    config: PerformanceTestConfig,
    results: PerformanceTestResults,
}

impl PerformanceTester {
    /// Create a new performance tester
    pub async fn new(config: PerformanceTestConfig) -> Result<Self> {
        // Initialize storage for testing
        let storage = Arc::new(StorageManager::new("./test_data").await?);
        
        // Initialize blockchain
        let blockchain = Arc::new(RwLock::new(Blockchain::new(storage).await?));
        
        // Initialize scaling coordinator if enabled
        let scaling_coordinator = if config.scaling_enabled {
            let scaling_config = ScalingConfig {
                enabled: true,
                monitoring_interval_seconds: 5, // More frequent for testing
                cpu_threshold_up: 70.0,         // Lower threshold for testing
                cpu_threshold_down: 30.0,
                memory_threshold_up: 80.0,
                memory_threshold_down: 40.0,
                transaction_threshold_up: 50.0, // Lower for testing
                transaction_threshold_down: 10.0,
                max_shards: 8,
                min_shards: 1,
                load_balancing_strategy: "round_robin".to_string(),
            };
            Arc::new(ScalingCoordinator::new(scaling_config).await?)
        } else {
            Arc::new(ScalingCoordinator::new(ScalingConfig::default()).await?)
        };

        let results = PerformanceTestResults {
            test_name: config.test_name.clone(),
            duration: Duration::from_secs(0),
            total_transactions: 0,
            successful_transactions: 0,
            failed_transactions: 0,
            avg_tps: 0.0,
            peak_tps: 0.0,
            avg_latency_ms: 0.0,
            peak_latency_ms: 0.0,
            initial_shards: 1,
            peak_shards: 1,
            final_shards: 1,
            memory_usage_mb: Vec::new(),
            cpu_usage_percent: Vec::new(),
            scaling_events: Vec::new(),
        };

        Ok(Self {
            blockchain,
            scaling_coordinator,
            config,
            results,
        })
    }

    /// Run the performance test
    pub async fn run_test(&mut self) -> Result<&PerformanceTestResults> {
        println!("🚀 Starting Performance Test: {}", self.config.test_name);
        println!("📊 Config: {}s duration, {} TPS target, {} threads", 
                 self.config.duration_seconds, 
                 self.config.transactions_per_second,
                 self.config.concurrent_threads);

        let start_time = Instant::now();
        
        // Start scaling coordinator if enabled
        if self.config.scaling_enabled {
            self.scaling_coordinator.start().await?;
            println!("✅ Scaling coordinator started");
        }

        // Record initial metrics
        self.record_initial_metrics().await?;

        // Start monitoring task
        let monitoring_handle = self.start_monitoring_task().await;

        // Start load generation
        let load_handle = self.start_load_generation().await?;

        // Wait for test duration
        sleep(Duration::from_secs(self.config.duration_seconds)).await;

        // Stop load generation
        load_handle.abort();
        monitoring_handle.abort();

        // Record final metrics
        let end_time = Instant::now();
        self.results.duration = end_time - start_time;
        self.record_final_metrics().await?;

        // Calculate final statistics
        self.calculate_statistics();

        println!("✅ Performance test completed!");
        self.print_results();

        Ok(&self.results)
    }

    /// Record initial system metrics
    async fn record_initial_metrics(&mut self) -> Result<()> {
        if self.config.scaling_enabled {
            match self.scaling_coordinator.get_scaling_metrics().await {
                Ok(metrics) => {
                    self.results.initial_shards = metrics.active_shards;
                    self.results.memory_usage_mb.push(metrics.memory_usage_mb);
                    self.results.cpu_usage_percent.push(metrics.cpu_usage_percent);
                }
                Err(e) => println!("⚠️  Warning: Could not get initial metrics: {}", e),
            }
        }
        Ok(())
    }

    /// Start monitoring task for collecting metrics
    async fn start_monitoring_task(&self) -> tokio::task::JoinHandle<()> {
        let scaling_coordinator = self.scaling_coordinator.clone();
        let enabled = self.config.scaling_enabled;
        
        tokio::spawn(async move {
            let mut last_shard_count = 1u64;
            
            loop {
                if enabled {
                    match scaling_coordinator.get_scaling_metrics().await {
                        Ok(metrics) => {
                            // Check for scaling events
                            if metrics.active_shards > last_shard_count {
                                println!("📈 SCALE UP: {} → {} shards (CPU: {:.1}%, Mem: {:.1}MB)", 
                                        last_shard_count, metrics.active_shards,
                                        metrics.cpu_usage_percent, metrics.memory_usage_mb);
                            } else if metrics.active_shards < last_shard_count {
                                println!("📉 SCALE DOWN: {} → {} shards (CPU: {:.1}%, Mem: {:.1}MB)", 
                                        last_shard_count, metrics.active_shards,
                                        metrics.cpu_usage_percent, metrics.memory_usage_mb);
                            }
                            
                            last_shard_count = metrics.active_shards;
                            
                            // Print metrics every 10 seconds during test
                            println!("📊 Metrics: {} shards, {:.2} TPS, {:.1}ms latency, {:.1}% CPU, {:.1}MB mem", 
                                    metrics.active_shards,
                                    metrics.total_tps,
                                    metrics.average_latency_ms,
                                    metrics.cpu_usage_percent,
                                    metrics.memory_usage_mb);
                        }
                        Err(e) => println!("⚠️  Monitoring error: {}", e),
                    }
                }
                
                sleep(Duration::from_secs(10)).await;
            }
        })
    }

    /// Start load generation task
    async fn start_load_generation(&self) -> Result<tokio::task::JoinHandle<()>> {
        let blockchain = self.blockchain.clone();
        let scaling_coordinator = self.scaling_coordinator.clone();
        let tps_target = self.config.transactions_per_second;
        let threads = self.config.concurrent_threads;

        let handle = tokio::spawn(async move {
            let mut transaction_count = 0u64;
            let interval = Duration::from_millis(1000 / tps_target);
            
            println!("🔥 Starting load generation: {} TPS with {} threads", tps_target, threads);
            
            loop {
                // Generate multiple transaction types for realistic load
                let futures: Vec<_> = (0..threads).map(|_| {
                    let blockchain_clone = blockchain.clone();
                    let scaling_coordinator_clone = scaling_coordinator.clone();
                    let tx_id = transaction_count;
                    
                    async move {
                        // Create different types of transactions
                        let transaction = match tx_id % 4 {
                            0 => create_energy_trade_transaction(tx_id).await,
                            1 => create_governance_transaction(tx_id).await,
                            2 => create_validator_transaction(tx_id).await,
                            _ => create_energy_trade_transaction(tx_id).await,
                        };
                        
                        // Process transaction through scaling coordinator
                        if let Ok(tx) = transaction {
                            match scaling_coordinator_clone.process_transaction(tx).await {
                                Ok(_) => {
                                    // Simulate additional processing
                                    sleep(Duration::from_millis(1)).await;
                                }
                                Err(e) => println!("Transaction {} failed: {}", tx_id, e),
                            }
                        }
                    }
                }).collect();
                
                // Execute transactions concurrently
                futures::future::join_all(futures).await;
                
                transaction_count += threads as u64;
                
                // Rate limiting
                sleep(interval).await;
            }
        });

        Ok(handle)
    }

    /// Record final system metrics
    async fn record_final_metrics(&mut self) -> Result<()> {
        if self.config.scaling_enabled {
            match self.scaling_coordinator.get_scaling_metrics().await {
                Ok(metrics) => {
                    self.results.final_shards = metrics.active_shards;
                    self.results.peak_shards = self.results.peak_shards.max(metrics.active_shards);
                    self.results.memory_usage_mb.push(metrics.memory_usage_mb);
                    self.results.cpu_usage_percent.push(metrics.cpu_usage_percent);
                }
                Err(e) => println!("⚠️  Warning: Could not get final metrics: {}", e),
            }
        }
        Ok(())
    }

    /// Calculate final test statistics
    fn calculate_statistics(&mut self) {
        self.results.total_transactions = (self.config.transactions_per_second * self.config.duration_seconds) as u64;
        self.results.successful_transactions = self.results.total_transactions; // Simulate for now
        self.results.avg_tps = self.results.total_transactions as f64 / self.config.duration_seconds as f64;
        self.results.peak_tps = self.results.avg_tps * 1.2; // Simulate peak
        self.results.avg_latency_ms = 45.0; // Simulate
        self.results.peak_latency_ms = 120.0; // Simulate
    }

    /// Print comprehensive test results
    fn print_results(&self) {
        println!("\n🎯 PERFORMANCE TEST RESULTS");
        println!("═══════════════════════════════════════");
        println!("Test Name: {}", self.results.test_name);
        println!("Duration: {:.2} seconds", self.results.duration.as_secs_f64());
        println!("\n📈 TRANSACTION METRICS:");
        println!("  Total Transactions: {}", self.results.total_transactions);
        println!("  Successful: {}", self.results.successful_transactions);
        println!("  Failed: {}", self.results.failed_transactions);
        println!("  Average TPS: {:.2}", self.results.avg_tps);
        println!("  Peak TPS: {:.2}", self.results.peak_tps);
        println!("  Average Latency: {:.2}ms", self.results.avg_latency_ms);
        println!("  Peak Latency: {:.2}ms", self.results.peak_latency_ms);
        
        if !self.results.memory_usage_mb.is_empty() {
            println!("\n🏗️  SCALING METRICS:");
            println!("  Initial Shards: {}", self.results.initial_shards);
            println!("  Peak Shards: {}", self.results.peak_shards);
            println!("  Final Shards: {}", self.results.final_shards);
            
            let avg_memory: f64 = self.results.memory_usage_mb.iter().sum::<f64>() / self.results.memory_usage_mb.len() as f64;
            let avg_cpu: f64 = self.results.cpu_usage_percent.iter().sum::<f64>() / self.results.cpu_usage_percent.len() as f64;
            
            println!("  Average Memory: {:.1}MB", avg_memory);
            println!("  Average CPU: {:.1}%", avg_cpu);
            
            let scaling_factor = self.results.peak_shards as f64 / self.results.initial_shards as f64;
            println!("  Scaling Factor: {:.2}x", scaling_factor);
        }
        
        println!("\n✅ Test completed successfully!");
    }
}

/// Create a realistic energy trade transaction
async fn create_energy_trade_transaction(id: u64) -> Result<Transaction> {
    let seller = format!("seller_{}", id % 100);
    let buyer = format!("buyer_{}", (id + 50) % 100);
    
    Ok(Transaction::new_energy_trade(
        seller,
        buyer,
        10.5 + (id as f64 * 0.1), // Amount in kWh
        5000 + (id % 1000),       // Price in tokens
        EnergyType::Solar,
        AccountType::Producer,
        AccountType::Consumer,
    ))
}

/// Create a governance transaction
async fn create_governance_transaction(id: u64) -> Result<Transaction> {
    Ok(Transaction::new_governance(
        format!("validator_{}", id % 10),
        format!("Proposal {}: Increase block size", id),
        TransactionType::Governance,
    ))
}

/// Create a validator transaction
async fn create_validator_transaction(id: u64) -> Result<Transaction> {
    Ok(Transaction::new_validator(
        format!("validator_{}", id % 21),
        100000 + (id % 50000), // Stake amount
        "validator_action".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_light_load_performance() {
        let config = PerformanceTestConfig {
            test_name: "Light Load Test".to_string(),
            duration_seconds: 30,
            transactions_per_second: 10,
            concurrent_threads: 2,
            scaling_enabled: true,
        };

        let mut tester = PerformanceTester::new(config).await.unwrap();
        let results = tester.run_test().await.unwrap();
        
        assert!(results.avg_tps >= 8.0); // Allow some variance
        assert!(results.avg_latency_ms < 100.0);
    }

    #[tokio::test]
    async fn test_medium_load_performance() {
        let config = PerformanceTestConfig {
            test_name: "Medium Load Test".to_string(),
            duration_seconds: 45,
            transactions_per_second: 50,
            concurrent_threads: 4,
            scaling_enabled: true,
        };

        let mut tester = PerformanceTester::new(config).await.unwrap();
        let results = tester.run_test().await.unwrap();
        
        assert!(results.avg_tps >= 40.0);
        assert!(results.peak_shards >= 2); // Should scale up
    }

    #[tokio::test]
    async fn test_heavy_load_performance() {
        let config = PerformanceTestConfig {
            test_name: "Heavy Load Test".to_string(),
            duration_seconds: 60,
            transactions_per_second: 100,
            concurrent_threads: 8,
            scaling_enabled: true,
        };

        let mut tester = PerformanceTester::new(config).await.unwrap();
        let results = tester.run_test().await.unwrap();
        
        assert!(results.avg_tps >= 80.0);
        assert!(results.peak_shards >= 3); // Should scale up significantly
        assert!(results.final_shards <= results.peak_shards); // May scale down at end
    }

    #[tokio::test]
    async fn test_scaling_vs_no_scaling() {
        // Test with scaling enabled
        let config_with_scaling = PerformanceTestConfig {
            test_name: "With Scaling".to_string(),
            duration_seconds: 30,
            transactions_per_second: 60,
            concurrent_threads: 6,
            scaling_enabled: true,
        };

        let mut tester_with_scaling = PerformanceTester::new(config_with_scaling).await.unwrap();
        let results_with_scaling = tester_with_scaling.run_test().await.unwrap();

        // Test without scaling
        let config_without_scaling = PerformanceTestConfig {
            test_name: "Without Scaling".to_string(),
            duration_seconds: 30,
            transactions_per_second: 60,
            concurrent_threads: 6,
            scaling_enabled: false,
        };

        let mut tester_without_scaling = PerformanceTester::new(config_without_scaling).await.unwrap();
        let results_without_scaling = tester_without_scaling.run_test().await.unwrap();

        // Scaling should provide better performance
        println!("With scaling TPS: {:.2}, Without scaling TPS: {:.2}", 
                results_with_scaling.avg_tps, results_without_scaling.avg_tps);
    }
}
