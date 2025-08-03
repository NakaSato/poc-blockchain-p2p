use std::time::Duration;
use tokio::time::sleep;
use anyhow::Result;
use gridtokenx_blockchain::{
    ScalingCoordinator, ScalingConfig,
    blockchain::{
        Transaction, 
        transaction::{
            EnergyTransaction, EnergySource, DeliveryWindow, GridLocation, 
            EnergyQualityMetrics, ComplianceData, EnergyOrderType
        }
    }
};
use chrono::{Utc, Duration as ChronoDuration};

// Helper function to create test transactions
fn create_test_transaction(id: usize) -> Result<Transaction> {
    let energy_tx = EnergyTransaction {
        energy_amount: 10.5 + (id as f64 * 0.1),
        price_per_kwh: 5000 + (id % 1000) as u64,
        total_value: ((10.5 + (id as f64 * 0.1)) * (5000.0 + (id % 1000) as f64)) as u64,
        energy_source: EnergySource::Solar,
        delivery_window: DeliveryWindow {
            start_time: Utc::now(),
            end_time: Utc::now() + ChronoDuration::hours(1),
            flexibility_minutes: 30,
        },
        grid_location: GridLocation {
            province_code: "BKK".to_string(),
            distribution_area: "Central".to_string(),
            substation_id: format!("SUB_{}", id),
            voltage_level: 22.0, // 22kV distribution level
            coordinates: Some((13.7563, 100.5018)), // Bangkok coordinates
        },
        carbon_credits: 0.5,
        quality_metrics: EnergyQualityMetrics {
            frequency: 50.0,
            voltage: 220.0,
            power_factor: 0.95,
            thd: 1.2,
            reliability_score: 99,
        },
        compliance_data: ComplianceData {
            erc_approved: true,
            utility_registration: Some(format!("REG_{}", id)),
            environmental_compliance: true,
            safety_certifications: vec!["ISO50001".to_string()],
            rec_certificate: Some(format!("REC_{}", id)),
        },
        order_type: EnergyOrderType::Sell,
    };
    
    Transaction::new_energy_trade(
        format!("seller_{}", id),
        format!("buyer_{}", id),
        energy_tx,
        100,  // fee
        id as u64,  // nonce
    )
}

#[tokio::test]
async fn test_scaling_coordinator_integration() -> Result<()> {
    println!("🚀 Testing GridTokenX Auto-Scaling Integration...");
    
    // Create scaling coordinator with default configuration
    let config = ScalingConfig::default();
    let coordinator = ScalingCoordinator::new(config).await?;
    
    // Start the coordinator
    coordinator.start().await?;
    println!("✅ ScalingCoordinator started successfully");
    
    // Let it run for a short time to collect initial metrics
    sleep(Duration::from_secs(2)).await;
    
    // Check initial metrics
    let initial_metrics = coordinator.get_scaling_metrics().await?;
    println!("📊 Initial Metrics:");
    println!("   Shards: {}", initial_metrics.active_shards);
    println!("   TPS: {:.1}", initial_metrics.total_tps);
    println!("   CPU: {:.1}%", initial_metrics.cpu_usage_percent);
    println!("   Memory: {:.1}MB", initial_metrics.memory_usage_mb);
    
    // Create test transactions
    let test_transactions: Vec<Transaction> = (0..5)
        .map(|i| create_test_transaction(i))
        .collect::<Result<Vec<_>>>()?;

    println!("⚡ Processing test transactions...");
    let processed = coordinator.process_transactions_scaled(test_transactions).await?;
    println!("✅ Processed {} transactions", processed.len());
    
    // Wait a bit for metrics to update
    sleep(Duration::from_secs(2)).await;
    
    // Check final metrics
    let final_metrics = coordinator.get_scaling_metrics().await?;
    println!("📊 Final Metrics:");
    println!("   Shards: {}", final_metrics.active_shards);
    println!("   TPS: {:.1}", final_metrics.total_tps);
    println!("   CPU: {:.1}%", final_metrics.cpu_usage_percent);
    println!("   Memory: {:.1}MB", final_metrics.memory_usage_mb);
    
    // Verify scaling is working
    assert!(final_metrics.active_shards >= 1);
    assert!(final_metrics.active_shards <= 8);
    assert!(final_metrics.total_tps >= 0.0);
    
    println!("🎉 Auto-scaling integration test completed successfully!");
    
    Ok(())
}

#[tokio::test]
async fn test_scaling_metrics_collection() -> Result<()> {
    println!("📊 Testing Scaling Metrics Collection...");
    
    let coordinator = ScalingCoordinator::new(ScalingConfig::default()).await?;
    coordinator.start().await?;
    
    // Collect metrics multiple times to verify consistency
    for i in 1..=3 {
        let metrics = coordinator.get_scaling_metrics().await?;
        println!("   Sample {}: Shards={}, TPS={:.1}, CPU={:.1}%", 
            i, metrics.active_shards, metrics.total_tps, metrics.cpu_usage_percent);
        
        // Verify metrics are reasonable
        assert!(metrics.active_shards >= 1);
        assert!(metrics.total_tps >= 0.0);
        assert!(metrics.cpu_usage_percent >= 0.0 && metrics.cpu_usage_percent <= 100.0);
        assert!(metrics.memory_usage_mb > 0.0);
        
        sleep(Duration::from_millis(500)).await;
    }
    
    println!("✅ Metrics collection test passed!");
    Ok(())
}

#[tokio::test]
async fn test_transaction_processing_scaling() -> Result<()> {
    println!("⚡ Testing Transaction Processing with Scaling...");
    
    let coordinator = ScalingCoordinator::new(ScalingConfig::default()).await?;
    coordinator.start().await?;
    
    // Create different sizes of transaction batches
    let small_batch: Vec<Transaction> = (0..2)
        .map(|i| create_test_transaction(i))
        .collect::<Result<Vec<_>>>()?;
    let medium_batch: Vec<Transaction> = (0..10)
        .map(|i| create_test_transaction(i))
        .collect::<Result<Vec<_>>>()?;
    let large_batch: Vec<Transaction> = (0..50)
        .map(|i| create_test_transaction(i))
        .collect::<Result<Vec<_>>>()?;
    
    // Process each batch and measure performance
    println!("   Processing small batch (2 txs)...");
    let small_result = coordinator.process_transactions_scaled(small_batch).await?;
    let small_metrics = coordinator.get_scaling_metrics().await?;
    
    println!("   Processing medium batch (10 txs)...");
    let medium_result = coordinator.process_transactions_scaled(medium_batch).await?;
    let medium_metrics = coordinator.get_scaling_metrics().await?;
    
    println!("   Processing large batch (50 txs)...");
    let large_result = coordinator.process_transactions_scaled(large_batch).await?;
    let large_metrics = coordinator.get_scaling_metrics().await?;
    
    // Verify results
    assert_eq!(small_result.len(), 2);
    assert_eq!(medium_result.len(), 10);
    assert_eq!(large_result.len(), 50);
    
    println!("📊 Processing Results:");
    println!("   Small batch: {} shards, {:.1} TPS", small_metrics.active_shards, small_metrics.total_tps);
    println!("   Medium batch: {} shards, {:.1} TPS", medium_metrics.active_shards, medium_metrics.total_tps);
    println!("   Large batch: {} shards, {:.1} TPS", large_metrics.active_shards, large_metrics.total_tps);
    
    println!("✅ Transaction processing scaling test passed!");
    Ok(())
}
