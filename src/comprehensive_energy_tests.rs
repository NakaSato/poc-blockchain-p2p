//! Comprehensive tests for energy trading and market functionality

use crate::energy::*;
use crate::blockchain::{Transaction, Block};
use crate::consensus_poa::{Authority, ThaiAuthorityType};
use chrono::{Utc, Duration};
use std::collections::HashMap;
use anyhow::Result;

#[cfg(test)]
mod energy_trading_tests {
    use super::*;

    fn create_test_energy_producer() -> EnergyProducer {
        EnergyProducer {
            id: "producer_001".to_string(),
            name: "Solar Farm Bangkok".to_string(),
            address: "0x1234567890abcdef".to_string(),
            energy_type: EnergyType::Solar,
            capacity_mw: 100.0,
            location: EnergyLocation {
                region: "Bangkok".to_string(),
                province: "Bangkok".to_string(),
                grid_connection: "BKK_Grid_01".to_string(),
                coordinates: (13.7563, 100.5018), // Bangkok coordinates
            },
            certification: EnergyCertification {
                license_number: "SOLAR-BKK-2024-001".to_string(),
                issued_by: "ERC".to_string(),
                valid_until: Utc::now() + Duration::days(365),
                renewable_certificate: true,
                carbon_credits_eligible: true,
            },
            operational_status: OperationalStatus::Active,
            created_at: Utc::now(),
            total_energy_produced: 0.0,
            total_energy_sold: 0.0,
            reputation_score: 95.0,
            metadata: HashMap::new(),
        }
    }

    fn create_test_energy_consumer() -> EnergyConsumer {
        EnergyConsumer {
            id: "consumer_001".to_string(),
            name: "Industrial Complex".to_string(),
            address: "0xabcdef1234567890".to_string(),
            consumer_type: ConsumerType::Industrial,
            demand_profile: DemandProfile {
                base_load_mw: 50.0,
                peak_load_mw: 150.0,
                average_daily_consumption: 2400.0, // MWh
                seasonal_variation: 0.2,
                time_of_use_preferences: vec![
                    TimeOfUsePreference {
                        start_hour: 9,
                        end_hour: 17,
                        preference_weight: 1.0,
                        max_price_per_mwh: 120.0,
                    }
                ],
            },
            location: EnergyLocation {
                region: "Central".to_string(),
                province: "Pathum Thani".to_string(),
                grid_connection: "PTH_Grid_01".to_string(),
                coordinates: (14.0208, 100.5250),
            },
            contract_preferences: ContractPreferences {
                preferred_energy_types: vec![EnergyType::Solar, EnergyType::Hydro],
                max_contract_duration_hours: 24,
                renewable_energy_percentage: 80.0,
                carbon_neutral_requirement: true,
            },
            created_at: Utc::now(),
            total_energy_consumed: 0.0,
            total_payments: 0.0,
            reputation_score: 88.0,
            metadata: HashMap::new(),
        }
    }

    fn create_test_energy_trade() -> EnergyTrade {
        EnergyTrade {
            id: "trade_001".to_string(),
            producer_id: "producer_001".to_string(),
            consumer_id: "consumer_001".to_string(),
            energy_amount_mwh: 100.0,
            price_per_mwh: 100.0,
            total_price: 10_000.0,
            energy_type: EnergyType::Solar,
            delivery_start: Utc::now() + Duration::hours(1),
            delivery_end: Utc::now() + Duration::hours(25),
            contract_terms: ContractTerms {
                payment_terms: "Net 30".to_string(),
                delivery_requirements: "Real-time delivery with 95% uptime".to_string(),
                penalty_clauses: vec![
                    "5% penalty for delivery shortfall > 10%".to_string(),
                    "1% bonus for renewable energy compliance".to_string(),
                ],
                quality_requirements: "Grid-standard frequency and voltage".to_string(),
            },
            trade_status: TradeStatus::Pending,
            created_at: Utc::now(),
            matched_at: None,
            completed_at: None,
            settlement_transaction: None,
            renewable_certificates: 80.0,
            carbon_credits: 15.0,
            grid_stability_contribution: 5.0,
            metadata: HashMap::new(),
        }
    }

    fn create_test_market_config() -> MarketConfig {
        MarketConfig {
            base_price_per_mwh: 80.0,
            renewable_energy_premium: 20.0,
            peak_hour_multiplier: 1.5,
            off_peak_discount: 0.8,
            carbon_credit_price: 25.0,
            grid_stability_bonus: 10.0,
            max_price_volatility: 0.3,
            trading_fee_percentage: 0.5,
            settlement_period_hours: 24,
            max_contract_duration_hours: 168, // 1 week
            regional_pricing: HashMap::from([
                ("Bangkok".to_string(), 1.1),
                ("Central".to_string(), 1.0),
                ("Northern".to_string(), 0.9),
                ("Northeastern".to_string(), 0.85),
                ("Southern".to_string(), 0.95),
            ]),
            peak_hours: vec![9, 10, 11, 18, 19, 20],
            renewable_targets: HashMap::from([
                (2024, 30.0),
                (2025, 35.0),
                (2030, 50.0),
            ]),
        }
    }

    #[tokio::test]
    async fn test_energy_market_creation() -> Result<()> {
        let config = create_test_market_config();
        let market = EnergyMarket::new(config).await?;
        
        assert_eq!(market.get_active_trades_count().await, 0);
        assert_eq!(market.get_producers_count().await, 0);
        assert_eq!(market.get_consumers_count().await, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_producer_registration() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        let producer = create_test_energy_producer();
        let producer_id = producer.id.clone();

        market.register_producer(producer).await?;
        assert_eq!(market.get_producers_count().await, 1);

        let retrieved = market.get_producer(&producer_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Solar Farm Bangkok");

        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_registration() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        let consumer = create_test_energy_consumer();
        let consumer_id = consumer.id.clone();

        market.register_consumer(consumer).await?;
        assert_eq!(market.get_consumers_count().await, 1);

        let retrieved = market.get_consumer(&consumer_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Industrial Complex");

        Ok(())
    }

    #[tokio::test]
    async fn test_energy_trade_creation() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Register producer and consumer first
        let producer = create_test_energy_producer();
        let consumer = create_test_energy_consumer();
        
        market.register_producer(producer).await?;
        market.register_consumer(consumer).await?;

        // Create trade
        let trade = create_test_energy_trade();
        let trade_id = trade.id.clone();

        market.create_trade(trade).await?;
        assert_eq!(market.get_active_trades_count().await, 1);

        let retrieved = market.get_trade(&trade_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().energy_amount_mwh, 100.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_dynamic_pricing() -> Result<()> {
        let config = create_test_market_config();
        let market = EnergyMarket::new(config).await?;

        // Test peak hour pricing
        let peak_price = market.calculate_dynamic_price(
            EnergyType::Solar,
            "Bangkok",
            10, // 10 AM (peak hour)
            100.0, // 100 MWh
            0.8,   // 80% renewable
        ).await?;

        // Test off-peak pricing
        let off_peak_price = market.calculate_dynamic_price(
            EnergyType::Solar,
            "Bangkok",
            14, // 2 PM (off-peak)
            100.0,
            0.8,
        ).await?;

        // Peak price should be higher than off-peak
        assert!(peak_price > off_peak_price);

        // Test renewable energy premium
        let renewable_price = market.calculate_dynamic_price(
            EnergyType::Solar,
            "Central",
            14,
            100.0,
            1.0, // 100% renewable
        ).await?;

        let non_renewable_price = market.calculate_dynamic_price(
            EnergyType::NaturalGas,
            "Central",
            14,
            100.0,
            0.0, // 0% renewable
        ).await?;

        assert!(renewable_price > non_renewable_price);

        Ok(())
    }

    #[tokio::test]
    async fn test_trade_matching() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Register participants
        let producer = create_test_energy_producer();
        let consumer = create_test_energy_consumer();
        
        market.register_producer(producer).await?;
        market.register_consumer(consumer).await?;

        // Create buy and sell orders
        let sell_order = EnergyOrder {
            id: "sell_001".to_string(),
            participant_id: "producer_001".to_string(),
            order_type: OrderType::Sell,
            energy_type: EnergyType::Solar,
            quantity_mwh: 150.0,
            price_per_mwh: 95.0,
            delivery_start: Utc::now() + Duration::hours(1),
            delivery_end: Utc::now() + Duration::hours(25),
            renewable_percentage: 100.0,
            location: "Bangkok".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(2),
            status: OrderStatus::Active,
        };

        let buy_order = EnergyOrder {
            id: "buy_001".to_string(),
            participant_id: "consumer_001".to_string(),
            order_type: OrderType::Buy,
            energy_type: EnergyType::Solar,
            quantity_mwh: 100.0,
            price_per_mwh: 100.0,
            delivery_start: Utc::now() + Duration::hours(1),
            delivery_end: Utc::now() + Duration::hours(25),
            renewable_percentage: 80.0,
            location: "Central".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(2),
            status: OrderStatus::Active,
        };

        market.submit_order(sell_order).await?;
        market.submit_order(buy_order).await?;

        // Run matching algorithm
        let matches = market.match_orders().await?;
        assert!(!matches.is_empty());

        let trade_match = &matches[0];
        assert_eq!(trade_match.quantity_mwh, 100.0); // Consumer's quantity (smaller)
        assert!(trade_match.price_per_mwh >= 95.0 && trade_match.price_per_mwh <= 100.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_renewable_energy_incentives() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Test renewable energy pricing incentives
        let solar_incentive = market.calculate_renewable_incentive(
            EnergyType::Solar,
            100.0, // 100 MWh
            1.0,   // 100% renewable
        ).await?;

        let wind_incentive = market.calculate_renewable_incentive(
            EnergyType::Wind,
            100.0,
            1.0,
        ).await?;

        let gas_incentive = market.calculate_renewable_incentive(
            EnergyType::NaturalGas,
            100.0,
            0.0, // 0% renewable
        ).await?;

        // Renewable sources should get incentives
        assert!(solar_incentive > 0.0);
        assert!(wind_incentive > 0.0);
        assert_eq!(gas_incentive, 0.0);

        // Calculate carbon credits
        let carbon_credits = market.calculate_carbon_credits(
            EnergyType::Solar,
            100.0,
            1.0,
        ).await?;

        assert!(carbon_credits > 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_grid_stability_contributions() -> Result<()> {
        let config = create_test_market_config();
        let market = EnergyMarket::new(config).await?;

        // Test grid stability scoring
        let stability_score = market.calculate_grid_stability_contribution(
            EnergyType::Hydro, // Reliable baseload
            100.0,
            "Bangkok", // High demand region
            10, // Peak hour
        ).await?;

        let variable_score = market.calculate_grid_stability_contribution(
            EnergyType::Solar, // Variable output
            100.0,
            "Northern", // Lower demand region
            14, // Off-peak hour
        ).await?;

        // Baseload should contribute more to stability
        assert!(stability_score > variable_score);

        Ok(())
    }

    #[tokio::test]
    async fn test_trade_settlement() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Register participants
        let producer = create_test_energy_producer();
        let consumer = create_test_energy_consumer();
        
        market.register_producer(producer).await?;
        market.register_consumer(consumer).await?;

        // Create and execute trade
        let mut trade = create_test_energy_trade();
        trade.trade_status = TradeStatus::Active;
        let trade_id = trade.id.clone();

        market.create_trade(trade).await?;

        // Simulate energy delivery
        let delivery_report = EnergyDeliveryReport {
            trade_id: trade_id.clone(),
            delivered_amount_mwh: 98.0, // Slight shortfall
            delivery_start: Utc::now(),
            delivery_end: Utc::now() + Duration::hours(24),
            quality_metrics: QualityMetrics {
                frequency_compliance: 99.5,
                voltage_compliance: 99.8,
                power_factor: 0.95,
                harmonics_compliance: true,
            },
            grid_impact: GridImpact {
                stability_contribution: 8.5,
                congestion_cost: 2.0,
                loss_factor: 0.03,
            },
        };

        market.record_energy_delivery(delivery_report).await?;

        // Process settlement
        let settlement = market.calculate_settlement(&trade_id).await?;
        
        // Should apply penalty for shortfall
        assert!(settlement.final_amount < 10_000.0); // Original total price
        assert!(settlement.penalties > 0.0);

        // Complete settlement
        market.settle_trade(&trade_id, settlement).await?;

        let completed_trade = market.get_trade(&trade_id).await?.unwrap();
        assert_eq!(completed_trade.trade_status, TradeStatus::Completed);

        Ok(())
    }

    #[tokio::test]
    async fn test_market_analytics() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Register multiple participants
        for i in 0..5 {
            let mut producer = create_test_energy_producer();
            producer.id = format!("producer_{:03}", i);
            producer.name = format!("Producer {}", i);
            producer.energy_type = match i % 3 {
                0 => EnergyType::Solar,
                1 => EnergyType::Wind,
                _ => EnergyType::Hydro,
            };
            market.register_producer(producer).await?;
        }

        for i in 0..3 {
            let mut consumer = create_test_energy_consumer();
            consumer.id = format!("consumer_{:03}", i);
            consumer.name = format!("Consumer {}", i);
            market.register_consumer(consumer).await?;
        }

        // Create multiple trades
        for i in 0..10 {
            let mut trade = create_test_energy_trade();
            trade.id = format!("trade_{:03}", i);
            trade.producer_id = format!("producer_{:03}", i % 5);
            trade.consumer_id = format!("consumer_{:03}", i % 3);
            trade.energy_amount_mwh = 50.0 + (i as f64 * 10.0);
            trade.price_per_mwh = 80.0 + (i as f64 * 5.0);
            trade.total_price = trade.energy_amount_mwh * trade.price_per_mwh;
            market.create_trade(trade).await?;
        }

        // Generate market analytics
        let analytics = market.generate_market_analytics().await?;
        
        assert_eq!(analytics.total_participants, 8);
        assert_eq!(analytics.total_trades, 10);
        assert!(analytics.total_volume_mwh > 0.0);
        assert!(analytics.total_value > 0.0);
        assert!(analytics.average_price_per_mwh > 0.0);
        assert!(!analytics.energy_type_distribution.is_empty());
        assert!(!analytics.regional_distribution.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_thai_energy_regulations() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Test ERC compliance validation
        let producer = create_test_energy_producer();
        
        // Validate producer meets Thai ERC requirements
        let compliance = market.validate_erc_compliance(&producer).await?;
        assert!(compliance.is_compliant);
        assert!(compliance.license_valid);
        assert!(compliance.meets_technical_standards);

        // Test EGAT grid connection requirements
        let grid_compliance = market.validate_grid_connection(
            &producer.location.grid_connection,
            producer.capacity_mw,
        ).await?;
        assert!(grid_compliance.connection_approved);

        // Test MEA/PEA distribution requirements
        let distribution_compliance = market.validate_distribution_compliance(
            &producer.location.region,
            producer.capacity_mw,
        ).await?;
        assert!(distribution_compliance.meets_requirements);

        Ok(())
    }

    #[tokio::test]
    async fn test_cross_border_trading() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Create cross-border trade scenario
        let mut laos_producer = create_test_energy_producer();
        laos_producer.id = "laos_hydro_001".to_string();
        laos_producer.name = "Laos Hydro Plant".to_string();
        laos_producer.energy_type = EnergyType::Hydro;
        laos_producer.location.region = "International".to_string();
        laos_producer.location.province = "Laos".to_string();
        laos_producer.certification.license_number = "LAO-HYDRO-2024-001".to_string();

        let thai_consumer = create_test_energy_consumer();

        market.register_producer(laos_producer.clone()).await?;
        market.register_consumer(thai_consumer).await?;

        // Create cross-border trade
        let mut cross_border_trade = create_test_energy_trade();
        cross_border_trade.id = "cross_border_001".to_string();
        cross_border_trade.producer_id = laos_producer.id;
        cross_border_trade.energy_amount_mwh = 500.0; // Large hydro import

        // Validate international trading requirements
        let international_compliance = market.validate_international_trade(&cross_border_trade).await?;
        assert!(international_compliance.regulatory_approval_required);
        assert!(international_compliance.customs_documentation_required);

        market.create_trade(cross_border_trade).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_seasonal_energy_trading() -> Result<()> {
        let config = create_test_market_config();
        let mut market = EnergyMarket::new(config).await?;

        // Test dry season vs wet season pricing
        let dry_season_price = market.calculate_seasonal_price(
            EnergyType::Hydro,
            "Northern",
            "dry_season", // April-May
            100.0,
        ).await?;

        let wet_season_price = market.calculate_seasonal_price(
            EnergyType::Hydro,
            "Northern",
            "wet_season", // July-September
            100.0,
        ).await?;

        // Dry season should be more expensive for hydro
        assert!(dry_season_price > wet_season_price);

        // Test hot season solar production
        let hot_season_solar = market.calculate_seasonal_price(
            EnergyType::Solar,
            "Central",
            "hot_season", // March-May
            100.0,
        ).await?;

        assert!(hot_season_solar > 0.0);

        Ok(())
    }
}

#[cfg(test)]
mod energy_integration_tests {
    use super::*;
    use crate::blockchain::Blockchain;
    use crate::storage::StorageManager;

    #[tokio::test]
    async fn test_energy_trading_with_blockchain() -> Result<()> {
        // Setup blockchain and energy market
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        let market_config = create_test_market_config();
        let mut energy_market = EnergyMarket::new(market_config).await?;

        // Register participants
        let producer = create_test_energy_producer();
        let consumer = create_test_energy_consumer();
        
        energy_market.register_producer(producer.clone()).await?;
        energy_market.register_consumer(consumer.clone()).await?;

        // Create energy trade transaction
        let trade = create_test_energy_trade();
        let energy_tx = Transaction::new_energy_trade(
            producer.address,
            consumer.address,
            trade.energy_amount_mwh,
            trade.price_per_mwh,
            serde_json::to_string(&trade)?,
        )?;

        blockchain.add_pending_transaction(energy_tx).await?;

        // Create authority to validate block
        let authority = Authority {
            address: "egat_validator_001".to_string(),
            public_key: vec![1, 2, 3, 4],
            authority_type: ThaiAuthorityType::EGAT,
            license_number: "EGAT-VAL-2024-001".to_string(),
            organization: "EGAT Validation".to_string(),
            stake_amount: 2_000_000,
            joined_at: Utc::now(),
            reputation_score: 98.0,
            total_blocks_validated: 500,
            last_block_time: Some(Utc::now()),
            region: "Central".to_string(),
            is_active: true,
            uptime_percentage: 99.5,
            governance_participation: 90.0,
            energy_capacity_mw: 2000.0,
            grid_connections: vec!["EGAT_Main_Grid".to_string()],
        };

        // Create block with energy transaction
        let pending = blockchain.get_pending_transactions().await;
        let validator_info = crate::blockchain::ValidatorInfo {
            address: authority.address,
            stake: authority.stake_amount,
            reputation: authority.reputation_score,
            authority_type: Some(format!("{:?}", authority.authority_type)),
        };

        let block = Block::new(
            blockchain.get_latest_block_hash().await?,
            pending,
            blockchain.get_height().await? + 1,
            validator_info,
        )?;

        blockchain.add_block(block).await?;

        // Verify energy transaction was recorded on blockchain
        assert!(blockchain.get_height().await? > 0);
        
        // Verify trade was created in energy market
        let recorded_trade = energy_market.get_trade(&trade.id).await?;
        assert!(recorded_trade.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_multi_regional_energy_trading() -> Result<()> {
        let market_config = create_test_market_config();
        let mut energy_market = EnergyMarket::new(market_config).await?;

        // Create producers in different regions
        let regions = vec![
            ("Bangkok", EnergyType::Solar),
            ("Northern", EnergyType::Hydro),
            ("Northeastern", EnergyType::Wind),
            ("Southern", EnergyType::NaturalGas),
        ];

        for (i, (region, energy_type)) in regions.iter().enumerate() {
            let mut producer = create_test_energy_producer();
            producer.id = format!("producer_{}_{}", region.to_lowercase(), i);
            producer.name = format!("{} Producer {}", region, i);
            producer.energy_type = energy_type.clone();
            producer.location.region = region.to_string();
            
            energy_market.register_producer(producer).await?;
        }

        // Create consumers in each region
        for (i, (region, _)) in regions.iter().enumerate() {
            let mut consumer = create_test_energy_consumer();
            consumer.id = format!("consumer_{}_{}", region.to_lowercase(), i);
            consumer.name = format!("{} Consumer {}", region, i);
            consumer.location.region = region.to_string();
            
            energy_market.register_consumer(consumer).await?;
        }

        // Create inter-regional trades
        for i in 0..4 {
            let mut trade = create_test_energy_trade();
            trade.id = format!("inter_regional_trade_{}", i);
            trade.producer_id = format!("producer_{}_0", regions[i].0.to_lowercase());
            trade.consumer_id = format!("consumer_{}_0", regions[(i + 1) % 4].0.to_lowercase());
            
            energy_market.create_trade(trade).await?;
        }

        assert_eq!(energy_market.get_active_trades_count().await, 4);

        // Analyze regional trading patterns
        let regional_analytics = energy_market.generate_regional_analytics().await?;
        assert_eq!(regional_analytics.regions_active, 4);
        assert!(regional_analytics.inter_regional_volume > 0.0);

        Ok(())
    }
}
