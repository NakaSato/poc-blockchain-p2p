//! GridTokenX Blockchain Performance Benchmarks
//! 
//! Comprehensive benchmark suite using Criterion to measure
//! blockchain performance and auto-scaling effectiveness.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::runtime::Runtime;

use gridtokenx_blockchain::{
    Blockchain, Block, Transaction, StorageManager, ScalingCoordinator, ScalingConfig,
    AccountType, TransactionType, EnergyType
};

/// Benchmark transaction processing performance
fn bench_transaction_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("transaction_processing");
    group.throughput(Throughput::Elements(1));
    
    for tx_count in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("energy_transactions", tx_count),
            tx_count,
            |b, &tx_count| {
                b.to_async(&rt).iter(|| async {
                    let storage = Arc::new(StorageManager::new("./bench_data").await.unwrap());
                    let blockchain = Arc::new(RwLock::new(Blockchain::new(storage).await.unwrap()));
                    
                    for i in 0..tx_count {
                        let tx = create_energy_transaction(i).await;
                        let mut bc = blockchain.write().await;
                        // Process transaction (simplified)
                        black_box(tx);
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark auto-scaling performance
fn bench_scaling_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("scaling_performance");
    group.measurement_time(Duration::from_secs(30));
    
    for load_level in ["light", "medium", "heavy"].iter() {
        group.bench_with_input(
            BenchmarkId::new("scaling_response", load_level),
            load_level,
            |b, &load_level| {
                b.to_async(&rt).iter(|| async {
                    let (tps, threads) = match load_level {
                        "light" => (20, 2),
                        "medium" => (60, 4),
                        "heavy" => (120, 8),
                        _ => (20, 2),
                    };
                    
                    let scaling_config = ScalingConfig {
                        enabled: true,
                        monitoring_interval_seconds: 1,
                        cpu_threshold_up: 70.0,
                        cpu_threshold_down: 30.0,
                        memory_threshold_up: 80.0,
                        memory_threshold_down: 40.0,
                        transaction_threshold_up: 50.0,
                        transaction_threshold_down: 15.0,
                        max_shards: 8,
                        min_shards: 1,
                        load_balancing_strategy: "round_robin".to_string(),
                    };
                    
                    let coordinator = ScalingCoordinator::new(scaling_config).await.unwrap();
                    coordinator.start().await.unwrap();
                    
                    // Simulate load
                    for i in 0..tps {
                        let tx = create_energy_transaction(i as u64).await;
                        black_box(coordinator.process_transaction(tx).await);
                    }
                    
                    // Check scaling response
                    let metrics = coordinator.get_scaling_metrics().await.unwrap();
                    black_box(metrics);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark block creation and validation
fn bench_block_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("block_operations");
    group.throughput(Throughput::Elements(1));
    
    for tx_count in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("block_creation", tx_count),
            tx_count,
            |b, &tx_count| {
                b.to_async(&rt).iter(|| async {
                    let mut transactions = Vec::new();
                    for i in 0..tx_count {
                        transactions.push(create_energy_transaction(i as u64).await);
                    }
                    
                    let block = Block::new(
                        1,
                        "previous_hash".to_string(),
                        transactions,
                        0,
                        "validator".to_string(),
                    ).await.unwrap();
                    
                    black_box(block);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark storage performance
fn bench_storage_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("storage_operations");
    group.throughput(Throughput::Elements(1));
    
    for operation in ["write", "read"].iter() {
        group.bench_with_input(
            BenchmarkId::new("rocksdb_operations", operation),
            operation,
            |b, &operation| {
                b.to_async(&rt).iter(|| async {
                    let storage = StorageManager::new("./bench_storage").await.unwrap();
                    
                    match operation {
                        "write" => {
                            for i in 0..100 {
                                let key = format!("bench_key_{}", i);
                                let value = format!("bench_value_{}", i);
                                storage.put(&key, value.as_bytes()).await.unwrap();
                            }
                        }
                        "read" => {
                            for i in 0..100 {
                                let key = format!("bench_key_{}", i);
                                let result = storage.get(&key).await;
                                black_box(result);
                            }
                        }
                        _ => {}
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark scaling efficiency
fn bench_scaling_efficiency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("scaling_efficiency");
    group.measurement_time(Duration::from_secs(60));
    
    // Compare performance with and without scaling
    for scaling_enabled in [false, true].iter() {
        group.bench_with_input(
            BenchmarkId::new("throughput_comparison", scaling_enabled),
            scaling_enabled,
            |b, &scaling_enabled| {
                b.to_async(&rt).iter(|| async {
                    let scaling_config = ScalingConfig {
                        enabled: scaling_enabled,
                        monitoring_interval_seconds: 2,
                        cpu_threshold_up: 70.0,
                        cpu_threshold_down: 30.0,
                        memory_threshold_up: 80.0,
                        memory_threshold_down: 40.0,
                        transaction_threshold_up: 40.0,
                        transaction_threshold_down: 10.0,
                        max_shards: 6,
                        min_shards: 1,
                        load_balancing_strategy: "round_robin".to_string(),
                    };
                    
                    let coordinator = ScalingCoordinator::new(scaling_config).await.unwrap();
                    if scaling_enabled {
                        coordinator.start().await.unwrap();
                    }
                    
                    // Process consistent workload
                    let mut successful_transactions = 0;
                    for i in 0..100 {
                        let tx = create_energy_transaction(i).await;
                        match coordinator.process_transaction(tx).await {
                            Ok(_) => successful_transactions += 1,
                            Err(_) => {}
                        }
                    }
                    
                    let metrics = coordinator.get_scaling_metrics().await.unwrap();
                    
                    // Return performance metrics for comparison
                    black_box((successful_transactions, metrics.total_tps, metrics.average_latency_ms));
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark Thai energy market specific operations
fn bench_thai_energy_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("thai_energy_operations");
    group.throughput(Throughput::Elements(1));
    
    let operations = [
        "solar_trade",
        "wind_trade", 
        "hydro_trade",
        "peak_hour_trade",
        "carbon_credit_calculation"
    ];
    
    for operation in operations.iter() {
        group.bench_with_input(
            BenchmarkId::new("energy_market", operation),
            operation,
            |b, &operation| {
                b.to_async(&rt).iter(|| async {
                    match operation {
                        "solar_trade" => {
                            let tx = Transaction::new_energy_trade(
                                "solar_farm_001".to_string(),
                                "consumer_001".to_string(),
                                50.0, // kWh
                                4500, // THB
                                EnergyType::Solar,
                                AccountType::Producer,
                                AccountType::Consumer,
                            );
                            black_box(tx);
                        }
                        "wind_trade" => {
                            let tx = Transaction::new_energy_trade(
                                "wind_farm_001".to_string(),
                                "industrial_001".to_string(),
                                100.0, // kWh
                                5500, // THB
                                EnergyType::Wind,
                                AccountType::Producer,
                                AccountType::Industrial,
                            );
                            black_box(tx);
                        }
                        "hydro_trade" => {
                            let tx = Transaction::new_energy_trade(
                                "hydro_plant_001".to_string(),
                                "grid_operator_mea".to_string(),
                                200.0, // kWh
                                9000, // THB
                                EnergyType::Hydro,
                                AccountType::Producer,
                                AccountType::GridOperator,
                            );
                            black_box(tx);
                        }
                        "peak_hour_trade" => {
                            // Simulate peak hour pricing (6 PM - 10 PM, 1.5x multiplier)
                            let tx = Transaction::new_energy_trade(
                                "battery_storage_001".to_string(),
                                "residential_001".to_string(),
                                25.0, // kWh
                                6750, // THB (peak hour pricing)
                                EnergyType::Battery,
                                AccountType::Storage,
                                AccountType::Consumer,
                            );
                            black_box(tx);
                        }
                        "carbon_credit_calculation" => {
                            // Simulate carbon credit calculation for renewable energy
                            let energy_amount = 100.0; // kWh
                            let solar_credit_rate = 0.5; // credits per kWh
                            let carbon_credits = energy_amount * solar_credit_rate;
                            black_box(carbon_credits);
                        }
                        _ => {}
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Helper function to create test energy transaction
async fn create_energy_transaction(id: u64) -> Transaction {
    Transaction::new_energy_trade(
        format!("producer_{}", id % 100),
        format!("consumer_{}", (id + 50) % 100),
        10.0 + (id as f64 * 0.1), // Amount in kWh
        4000 + (id % 2000),       // Price in THB
        EnergyType::Solar,
        AccountType::Producer,
        AccountType::Consumer,
    )
}

criterion_group!(
    benches,
    bench_transaction_processing,
    bench_scaling_performance,
    bench_block_operations,
    bench_storage_operations,
    bench_scaling_efficiency,
    bench_thai_energy_operations
);

criterion_main!(benches);
