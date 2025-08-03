//! GridTokenX Performance Testing CLI
//! 
//! Command-line tool for running comprehensive performance tests
//! and monitoring auto-scaling behavior of the GridTokenX blockchain.

use clap::{Parser, Subcommand};
use anyhow::Result;
use tokio::time::{sleep, Duration};
use std::time::Instant;
use gridtokenx_blockchain::{
    Blockchain, BlockchainConfig, ScalingCoordinator, ScalingConfig
};

mod performance_test;
use performance_test::{PerformanceTester, PerformanceTestConfig};

#[derive(Parser)]
#[command(name = "gridtokenx-perf")]
#[command(about = "GridTokenX Performance Testing Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a quick performance test
    Quick {
        /// Duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,
        /// Target transactions per second
        #[arg(short, long, default_value = "25")]
        tps: u64,
    },
    /// Run a stress test to trigger scaling
    Stress {
        /// Duration in seconds
        #[arg(short, long, default_value = "120")]
        duration: u64,
        /// Target transactions per second
        #[arg(short, long, default_value = "100")]
        tps: u64,
        /// Number of concurrent threads
        #[arg(short, long, default_value = "8")]
        threads: usize,
    },
    /// Run a scaling demonstration
    Scaling {
        /// Phase duration in seconds
        #[arg(short, long, default_value = "45")]
        phase_duration: u64,
    },
    /// Run benchmarks comparing with/without scaling
    Benchmark {
        /// Test duration in seconds
        #[arg(short, long, default_value = "60")]
        duration: u64,
    },
    /// Monitor a running node's scaling behavior
    Monitor {
        /// Monitoring duration in seconds (0 = infinite)
        #[arg(short, long, default_value = "300")]
        duration: u64,
        /// Monitoring interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Quick { duration, tps } => {
            run_quick_test(duration, tps).await?;
        }
        Commands::Stress { duration, tps, threads } => {
            run_stress_test(duration, tps, threads).await?;
        }
        Commands::Scaling { phase_duration } => {
            run_scaling_demo(phase_duration).await?;
        }
        Commands::Benchmark { duration } => {
            run_benchmark_comparison(duration).await?;
        }
        Commands::Monitor { duration, interval } => {
            run_monitoring(duration, interval).await?;
        }
    }

    Ok(())
}

/// Run a quick performance test
async fn run_quick_test(duration: u64, tps: u64) -> Result<()> {
    println!("🚀 Running Quick Performance Test");
    
    let config = PerformanceTestConfig {
        test_name: "Quick Performance Test".to_string(),
        duration_seconds: duration,
        transactions_per_second: tps,
        concurrent_threads: 4,
        scaling_enabled: true,
    };

    let mut tester = PerformanceTester::new(config).await?;
    tester.run_test().await?;

    Ok(())
}

/// Run a stress test designed to trigger auto-scaling
async fn run_stress_test(duration: u64, tps: u64, threads: usize) -> Result<()> {
    println!("🔥 Running Stress Test - Designed to Trigger Auto-Scaling");
    
    let config = PerformanceTestConfig {
        test_name: "Stress Test".to_string(),
        duration_seconds: duration,
        transactions_per_second: tps,
        concurrent_threads: threads,
        scaling_enabled: true,
    };

    let mut tester = PerformanceTester::new(config).await?;
    let results = tester.run_test().await?;

    // Analyze scaling behavior
    analyze_scaling_performance(results);

    Ok(())
}

/// Run a scaling demonstration with varying load phases
async fn run_scaling_demo(phase_duration: u64) -> Result<()> {
    println!("📊 Running Auto-Scaling Demonstration");
    println!("This test will run multiple phases with increasing load to demonstrate scaling behavior");

    let phases = vec![
        ("Low Load Phase", 10, 2),
        ("Medium Load Phase", 40, 4),
        ("High Load Phase", 80, 6),
        ("Peak Load Phase", 120, 8),
        ("Cool Down Phase", 20, 2),
    ];

    for (phase_name, tps, threads) in phases {
        println!("\n🔄 Starting: {}", phase_name);
        
        let config = PerformanceTestConfig {
            test_name: format!("Scaling Demo - {}", phase_name),
            duration_seconds: phase_duration,
            transactions_per_second: tps,
            concurrent_threads: threads,
            scaling_enabled: true,
        };

        let mut tester = PerformanceTester::new(config).await?;
        tester.run_test().await?;

        // Brief pause between phases
        println!("⏸️  Pausing 10 seconds before next phase...");
        sleep(Duration::from_secs(10)).await;
    }

    println!("\n✅ Auto-scaling demonstration completed!");
    Ok(())
}

/// Run benchmark comparing performance with and without scaling
async fn run_benchmark_comparison(duration: u64) -> Result<()> {
    println!("⚡ Running Benchmark: Scaling vs No Scaling");

    // Test WITHOUT scaling
    println!("\n📊 Phase 1: Testing WITHOUT auto-scaling");
    let config_no_scaling = PerformanceTestConfig {
        test_name: "Benchmark - No Scaling".to_string(),
        duration_seconds: duration,
        transactions_per_second: 60,
        concurrent_threads: 6,
        scaling_enabled: false,
    };

    let mut tester_no_scaling = PerformanceTester::new(config_no_scaling).await?;
    let results_no_scaling = tester_no_scaling.run_test().await?;

    println!("\n⏸️  Cooling down for 15 seconds...");
    sleep(Duration::from_secs(15)).await;

    // Test WITH scaling
    println!("\n📊 Phase 2: Testing WITH auto-scaling");
    let config_with_scaling = PerformanceTestConfig {
        test_name: "Benchmark - With Scaling".to_string(),
        duration_seconds: duration,
        transactions_per_second: 60,
        concurrent_threads: 6,
        scaling_enabled: true,
    };

    let mut tester_with_scaling = PerformanceTester::new(config_with_scaling).await?;
    let results_with_scaling = tester_with_scaling.run_test().await?;

    // Compare results
    print_benchmark_comparison(results_no_scaling, results_with_scaling);

    Ok(())
}

/// Monitor a running node's scaling behavior
async fn run_monitoring(duration: u64, interval: u64) -> Result<()> {
    use gridtokenx_blockchain::{ScalingCoordinator, ScalingConfig};

    println!("👀 Monitoring blockchain scaling behavior");
    println!("Duration: {}s, Update interval: {}s", duration, interval);

    let scaling_config = ScalingConfig::default();
    let scaling_coordinator = ScalingCoordinator::new(scaling_config).await?;
    
    let start_time = Instant::now();
    let mut last_shard_count = 1usize;

    loop {
        if duration > 0 && start_time.elapsed().as_secs() >= duration {
            break;
        }

        match scaling_coordinator.get_scaling_metrics().await {
            Ok(metrics) => {
                let timestamp = chrono::Utc::now().format("%H:%M:%S");
                
                // Detect scaling events
                if metrics.active_shards > last_shard_count {
                    println!("📈 [{}] SCALE UP: {} → {} shards", 
                            timestamp, last_shard_count, metrics.active_shards);
                } else if metrics.active_shards < last_shard_count {
                    println!("📉 [{}] SCALE DOWN: {} → {} shards", 
                            timestamp, last_shard_count, metrics.active_shards);
                }

                // Regular metrics display
                println!("[{}] Shards: {}, TPS: {:.1}, Latency: {:.1}ms, CPU: {:.1}%, Mem: {:.1}MB",
                        timestamp,
                        metrics.active_shards,
                        metrics.total_tps,
                        metrics.average_latency_ms,
                        metrics.cpu_usage_percent,
                        metrics.memory_usage_mb);

                last_shard_count = metrics.active_shards;
            }
            Err(e) => {
                println!("⚠️  Error getting metrics: {}", e);
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }

    println!("✅ Monitoring completed");
    Ok(())
}

/// Analyze scaling performance from test results
fn analyze_scaling_performance(results: &performance_test::PerformanceTestResults) {
    println!("\n🔍 SCALING ANALYSIS");
    println!("═══════════════════════════════════════");
    
    let scaling_efficiency = if results.initial_shards > 0 {
        results.avg_tps / results.initial_shards as f64
    } else {
        results.avg_tps
    };

    let scaling_factor = if results.initial_shards > 0 {
        results.peak_shards as f64 / results.initial_shards as f64
    } else {
        1.0
    };

    println!("Scaling Factor: {:.2}x", scaling_factor);
    println!("Scaling Efficiency: {:.2} TPS per shard", scaling_efficiency);
    
    if scaling_factor > 1.5 {
        println!("✅ Excellent scaling response!");
    } else if scaling_factor > 1.2 {
        println!("✅ Good scaling response");
    } else {
        println!("⚠️  Limited scaling response");
    }

    if results.peak_shards > results.final_shards {
        println!("✅ Smart scale-down detected after peak load");
    }
}

/// Print comparison between scaling and non-scaling results
fn print_benchmark_comparison(
    no_scaling: &performance_test::PerformanceTestResults,
    with_scaling: &performance_test::PerformanceTestResults,
) {
    println!("\n📊 BENCHMARK COMPARISON");
    println!("═══════════════════════════════════════");
    
    let tps_improvement = (with_scaling.avg_tps - no_scaling.avg_tps) / no_scaling.avg_tps * 100.0;
    let latency_improvement = (no_scaling.avg_latency_ms - with_scaling.avg_latency_ms) / no_scaling.avg_latency_ms * 100.0;
    
    println!("                     │ No Scaling │ With Scaling │ Improvement");
    println!("─────────────────────┼────────────┼──────────────┼────────────");
    println!("Average TPS          │ {:10.2} │ {:12.2} │ {:7.1}%", 
             no_scaling.avg_tps, with_scaling.avg_tps, tps_improvement);
    println!("Peak TPS             │ {:10.2} │ {:12.2} │ {:7.1}%", 
             no_scaling.peak_tps, with_scaling.peak_tps, 
             (with_scaling.peak_tps - no_scaling.peak_tps) / no_scaling.peak_tps * 100.0);
    println!("Average Latency (ms) │ {:10.2} │ {:12.2} │ {:7.1}%", 
             no_scaling.avg_latency_ms, with_scaling.avg_latency_ms, latency_improvement);
    println!("Active Shards        │ {:10} │ {:12} │ {:7.1}x", 
             no_scaling.final_shards, with_scaling.peak_shards,
             with_scaling.peak_shards as f64 / no_scaling.final_shards as f64);
    
    println!("\n🎯 VERDICT:");
    if tps_improvement > 20.0 {
        println!("🔥 Auto-scaling provides SIGNIFICANT performance improvement!");
    } else if tps_improvement > 10.0 {
        println!("✅ Auto-scaling provides good performance improvement");
    } else if tps_improvement > 0.0 {
        println!("✅ Auto-scaling provides modest performance improvement");
    } else {
        println!("⚠️  Auto-scaling overhead detected - may need tuning");
    }
}
