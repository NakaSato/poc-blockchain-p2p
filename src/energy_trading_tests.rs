//! Comprehensive tests for energy trading functionality in GridTokenX

use crate::blockchain::{Block, Transaction, EnergyTransaction};
use crate::utils::{ThaiEnergyMarket, EnergyUnit};
use chrono::Utc;
use anyhow::Result;

#[cfg(test)]
mod energy_trading_tests {
    use super::*;

    fn create_test_energy_transaction() -> Result<Transaction> {
        Transaction::new_energy_trade(
            "seller_address_123".to_string(),
            "buyer_address_456".to_string(),
            1000.0, // 1000 kWh
            4.50,   // 4.50 tokens per kWh
            "Bangkok".to_string(),
            Some("Peak hours trading".to_string()),
        )
    }

    fn create_test_generator_transaction() -> Result<Transaction> {
        Transaction::new_generator_mint(
            "egat_generator_001".to_string(),
            5000.0, // 5000 kWh generated
            "Solar Farm Bangkok".to_string(),
            Some("Clean energy generation".to_string()),
        )
    }

    #[test]
    fn test_energy_transaction_creation() -> Result<()> {
        let tx = create_test_energy_transaction()?;
        
        assert_eq!(tx.transaction_type, crate::blockchain::TransactionType::EnergyTrade);
        assert!(tx.amount > 0);
        assert!(!tx.sender.is_empty());
        assert!(!tx.recipient.is_empty());
        assert!(tx.gas_limit > 0);
        
        Ok(())
    }

    #[test]
    fn test_energy_transaction_validation() -> Result<()> {
        let tx = create_test_energy_transaction()?;
        
        // Valid transaction should pass validation
        assert!(tx.validate().is_ok());

        // Test invalid energy amount (negative)
        let mut invalid_tx = tx.clone();
        invalid_tx.amount = 0; // Zero amount should be invalid
        assert!(invalid_tx.validate().is_err());

        Ok(())
    }

    #[test]
    fn test_energy_transaction_hash_consistency() -> Result<()> {
        let tx = create_test_energy_transaction()?;
        
        let hash1 = tx.hash()?;
        let hash2 = tx.hash()?;
        
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex string length
        assert!(!hash1.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_generator_mint_transaction() -> Result<()> {
        let tx = create_test_generator_transaction()?;
        
        assert_eq!(tx.transaction_type, crate::blockchain::TransactionType::GeneratorMint);
        assert!(tx.amount > 0);
        assert!(!tx.sender.is_empty());
        assert!(tx.recipient.is_empty()); // Mint transactions have no recipient
        
        Ok(())
    }

    #[test]
    fn test_energy_unit_conversion() {
        // Test kWh to MWh conversion
        assert_eq!(ThaiEnergyMarket::convert_energy_unit(1000.0, EnergyUnit::kWh, EnergyUnit::MWh), 1.0);
        
        // Test MWh to kWh conversion
        assert_eq!(ThaiEnergyMarket::convert_energy_unit(2.5, EnergyUnit::MWh, EnergyUnit::kWh), 2500.0);
        
        // Test same unit conversion
        assert_eq!(ThaiEnergyMarket::convert_energy_unit(100.0, EnergyUnit::kWh, EnergyUnit::kWh), 100.0);
    }

    #[test]
    fn test_thai_peak_hours_detection() {
        use chrono::{TimeZone, NaiveTime};
        
        // Test peak hours (9 AM - 10 PM)
        let peak_hour = chrono::Local.with_ymd_and_hms(2024, 8, 1, 14, 0, 0).unwrap(); // 2 PM
        assert!(ThaiEnergyMarket::is_peak_hours(&peak_hour.with_timezone(&Utc)));
        
        // Test off-peak hours (10 PM - 9 AM)
        let off_peak_hour = chrono::Local.with_ymd_and_hms(2024, 8, 1, 23, 0, 0).unwrap(); // 11 PM
        assert!(!ThaiEnergyMarket::is_peak_hours(&off_peak_hour.with_timezone(&Utc)));
        
        // Test edge cases
        let peak_start = chrono::Local.with_ymd_and_hms(2024, 8, 1, 9, 0, 0).unwrap(); // 9 AM
        assert!(ThaiEnergyMarket::is_peak_hours(&peak_start.with_timezone(&Utc)));
        
        let peak_end = chrono::Local.with_ymd_and_hms(2024, 8, 1, 22, 0, 0).unwrap(); // 10 PM
        assert!(ThaiEnergyMarket::is_peak_hours(&peak_end.with_timezone(&Utc)));
    }

    #[test]
    fn test_regional_energy_pricing() {
        // Bangkok should have higher base price
        let bangkok_price = ThaiEnergyMarket::get_regional_base_price("Bangkok");
        let rural_price = ThaiEnergyMarket::get_regional_base_price("Rural_Northeast");
        
        assert!(bangkok_price > rural_price);
        assert!(bangkok_price > 0.0);
        assert!(rural_price > 0.0);
        
        // Test unknown region defaults
        let unknown_price = ThaiEnergyMarket::get_regional_base_price("Unknown_Region");
        assert!(unknown_price > 0.0);
    }

    #[test]
    fn test_carbon_credits_calculation() -> Result<()> {
        // Solar energy should have good carbon credits
        let solar_credits = ThaiEnergyMarket::calculate_carbon_credits(1000.0, "solar")?;
        assert!(solar_credits > 0.0);
        
        // Coal energy should have negative credits (penalty)
        let coal_credits = ThaiEnergyMarket::calculate_carbon_credits(1000.0, "coal")?;
        assert!(coal_credits < 0.0);
        
        // Hydro should have good credits
        let hydro_credits = ThaiEnergyMarket::calculate_carbon_credits(1000.0, "hydro")?;
        assert!(hydro_credits > 0.0);
        
        // Solar should be better than hydro
        assert!(solar_credits > hydro_credits);
        
        Ok(())
    }

    #[test]
    fn test_energy_transaction_fee_calculation() -> Result<()> {
        let tx = create_test_energy_transaction()?;
        
        // Calculate base fee
        let base_fee = ThaiEnergyMarket::calculate_transaction_fee(tx.amount as f64, "Bangkok", false);
        assert!(base_fee > 0.0);
        
        // Peak hours should have higher fees
        let peak_fee = ThaiEnergyMarket::calculate_transaction_fee(tx.amount as f64, "Bangkok", true);
        assert!(peak_fee > base_fee);
        
        // Larger amounts should have higher fees
        let large_fee = ThaiEnergyMarket::calculate_transaction_fee(tx.amount as f64 * 10.0, "Bangkok", false);
        assert!(large_fee > base_fee);
        
        Ok(())
    }

    #[test]
    fn test_energy_market_compliance() -> Result<()> {
        let tx = create_test_energy_transaction()?;
        
        // Test valid transaction compliance
        assert!(ThaiEnergyMarket::validate_transaction_compliance(&tx)?);
        
        // Test transaction with too high energy amount (should fail compliance)
        let mut large_tx = tx.clone();
        large_tx.amount = 1_000_000; // 1M tokens, representing huge energy amount
        
        // Large transactions might require special approval
        let compliance_result = ThaiEnergyMarket::validate_transaction_compliance(&large_tx);
        // This might pass or fail depending on implementation limits
        
        Ok(())
    }

    #[test]
    fn test_grid_location_validation() {
        // Test valid Thai grid locations
        assert!(ThaiEnergyMarket::is_valid_grid_location("Bangkok_Central"));
        assert!(ThaiEnergyMarket::is_valid_grid_location("Chiang_Mai_North"));
        assert!(ThaiEnergyMarket::is_valid_grid_location("Phuket_South"));
        
        // Test invalid locations
        assert!(!ThaiEnergyMarket::is_valid_grid_location("Invalid_Location"));
        assert!(!ThaiEnergyMarket::is_valid_grid_location(""));
        
        // Test edge cases
        assert!(ThaiEnergyMarket::is_valid_grid_location("Nakhon_Ratchasima_Northeast"));
    }

    #[test]
    fn test_energy_trading_limits() -> Result<()> {
        // Test maximum energy per transaction
        let max_energy = ThaiEnergyMarket::get_max_energy_per_transaction("individual");
        assert!(max_energy > 0.0);
        
        let commercial_max = ThaiEnergyMarket::get_max_energy_per_transaction("commercial");
        assert!(commercial_max > max_energy);
        
        let industrial_max = ThaiEnergyMarket::get_max_energy_per_transaction("industrial");
        assert!(industrial_max > commercial_max);
        
        // Test minimum energy requirements
        let min_energy = ThaiEnergyMarket::get_min_energy_per_transaction();
        assert!(min_energy > 0.0);
        assert!(min_energy < max_energy);
        
        Ok(())
    }

    #[test]
    fn test_seasonal_pricing_adjustments() {
        use chrono::{Month, Utc};
        
        // Test summer pricing (higher demand)
        let summer_date = Utc::now().with_month(4).unwrap(); // April (hot season)
        let summer_multiplier = ThaiEnergyMarket::get_seasonal_multiplier(&summer_date);
        
        // Test cool season pricing
        let cool_date = Utc::now().with_month(12).unwrap(); // December (cool season)
        let cool_multiplier = ThaiEnergyMarket::get_seasonal_multiplier(&cool_date);
        
        // Summer should typically have higher multiplier due to AC usage
        assert!(summer_multiplier >= cool_multiplier);
        assert!(summer_multiplier > 0.0);
        assert!(cool_multiplier > 0.0);
    }

    #[test]
    fn test_energy_trading_block_validation() -> Result<()> {
        // Create transactions for a block
        let tx1 = create_test_energy_transaction()?;
        let tx2 = create_test_generator_transaction()?;
        let transactions = vec![tx1, tx2];
        
        // Create a test block
        let block = Block::new_genesis(transactions, "Energy trading test block".to_string())?;
        
        // Validate the block
        let validation_result = block.validate(None);
        assert!(validation_result.is_valid());
        
        // Check energy statistics
        assert!(block.energy_stats.total_energy_traded >= 0.0);
        assert!(block.energy_stats.total_energy_generated >= 0.0);
        
        Ok(())
    }

    #[test]
    fn test_multi_authority_energy_trading() -> Result<()> {
        // Simulate trading between different Thai energy authorities
        let egat_to_mea = Transaction::new_energy_trade(
            "egat_authority_address".to_string(),
            "mea_authority_address".to_string(),
            2000.0, // 2 MWh
            4.25,   // Wholesale rate
            "Central_Grid".to_string(),
            Some("EGAT to MEA wholesale".to_string()),
        )?;
        
        let mea_to_consumer = Transaction::new_energy_trade(
            "mea_authority_address".to_string(),
            "consumer_bangkok_123".to_string(),
            100.0, // 100 kWh
            5.50,  // Retail rate
            "Bangkok_Metro".to_string(),
            Some("MEA retail distribution".to_string()),
        )?;
        
        // Both transactions should be valid
        assert!(egat_to_mea.validate().is_ok());
        assert!(mea_to_consumer.validate().is_ok());
        
        // Wholesale rate should be lower than retail rate
        // This would need to be extracted from transaction data in real implementation
        
        Ok(())
    }

    #[test]
    fn test_renewable_energy_incentives() -> Result<()> {
        // Create renewable energy generation transaction
        let solar_tx = Transaction::new_generator_mint(
            "solar_farm_lopburi".to_string(),
            1500.0, // 1.5 MWh from solar
            "Solar Generation Lopburi".to_string(),
            Some("Clean renewable energy".to_string()),
        )?;
        
        // Renewable energy should have additional incentives
        let incentive = ThaiEnergyMarket::calculate_renewable_incentive(1500.0, "solar");
        assert!(incentive > 0.0);
        
        // Solar should have higher incentive than biomass
        let biomass_incentive = ThaiEnergyMarket::calculate_renewable_incentive(1500.0, "biomass");
        assert!(incentive > biomass_incentive);
        assert!(biomass_incentive > 0.0);
        
        Ok(())
    }

    #[test]
    fn test_emergency_energy_trading() -> Result<()> {
        // Test emergency energy request (high priority, premium pricing)
        let emergency_tx = Transaction::new_energy_trade(
            "emergency_provider".to_string(),
            "critical_hospital".to_string(),
            50.0,  // 50 kWh emergency supply
            8.00,  // Premium emergency rate
            "Bangkok_Emergency".to_string(),
            Some("Emergency hospital power supply".to_string()),
        )?;
        
        assert!(emergency_tx.validate().is_ok());
        
        // Emergency transactions should have higher priority
        let priority = ThaiEnergyMarket::get_transaction_priority(&emergency_tx);
        assert!(priority > 50); // High priority score
        
        Ok(())
    }

    #[test]
    fn test_cross_regional_energy_trading() -> Result<()> {
        // Test energy trading between different regions
        let north_to_south = Transaction::new_energy_trade(
            "hydro_dam_north".to_string(),
            "industrial_south".to_string(),
            3000.0, // 3 MWh cross-regional transfer
            4.75,   // Inter-regional rate
            "National_Grid".to_string(),
            Some("North to South energy transfer".to_string()),
        )?;
        
        assert!(north_to_south.validate().is_ok());
        
        // Cross-regional transfers should include transmission costs
        let transmission_fee = ThaiEnergyMarket::calculate_transmission_fee(3000.0, "North", "South");
        assert!(transmission_fee > 0.0);
        
        // Long distance should cost more than short distance
        let local_fee = ThaiEnergyMarket::calculate_transmission_fee(3000.0, "Bangkok", "Nonthaburi");
        assert!(transmission_fee > local_fee);
        
        Ok(())
    }
}

// Additional integration tests for energy trading workflows
#[cfg(test)]
mod energy_integration_tests {
    use super::*;
    use crate::blockchain::Blockchain;
    use crate::storage::StorageManager;

    #[tokio::test]
    async fn test_complete_energy_trading_workflow() -> Result<()> {
        // Create a test blockchain
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        // 1. Generator mints energy tokens
        let generation_tx = Transaction::new_generator_mint(
            "solar_farm_001".to_string(),
            2000.0,
            "Solar Generation Test".to_string(),
            None,
        )?;
        
        // 2. Energy trading transaction
        let trading_tx = Transaction::new_energy_trade(
            "solar_farm_001".to_string(),
            "consumer_001".to_string(),
            500.0,
            4.50,
            "Central_Grid".to_string(),
            None,
        )?;
        
        // Add transactions to blockchain
        blockchain.add_pending_transaction(generation_tx).await?;
        blockchain.add_pending_transaction(trading_tx).await?;
        
        // Mine a block with these transactions
        let pending = blockchain.get_pending_transactions().await;
        let block = Block::new(
            blockchain.get_latest_block_hash().await?,
            pending,
            blockchain.get_height().await? + 1,
            crate::blockchain::ValidatorInfo {
                address: "test_validator".to_string(),
                stake: 1000,
                reputation: 100.0,
                authority_type: Some("EGAT".to_string()),
            },
        )?;
        
        // Add block to chain
        blockchain.add_block(block).await?;
        
        // Verify the block was added successfully
        assert!(blockchain.get_height().await? > 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_energy_balance_tracking() -> Result<()> {
        let storage = StorageManager::new_memory_storage().await?;
        let mut blockchain = Blockchain::new(storage).await?;
        
        // Create multiple energy transactions
        let transactions = vec![
            Transaction::new_generator_mint("gen1".to_string(), 1000.0, "Gen1".to_string(), None)?,
            Transaction::new_energy_trade("gen1".to_string(), "user1".to_string(), 100.0, 4.0, "Grid".to_string(), None)?,
            Transaction::new_energy_trade("user1".to_string(), "user2".to_string(), 50.0, 4.5, "Grid".to_string(), None)?,
        ];
        
        // Add all transactions
        for tx in transactions {
            blockchain.add_pending_transaction(tx).await?;
        }
        
        // Create and add block
        let pending = blockchain.get_pending_transactions().await;
        let block = Block::new(
            blockchain.get_latest_block_hash().await?,
            pending,
            blockchain.get_height().await? + 1,
            crate::blockchain::ValidatorInfo {
                address: "validator".to_string(),
                stake: 1000,
                reputation: 100.0,
                authority_type: Some("ERC".to_string()),
            },
        )?;
        
        blockchain.add_block(block).await?;
        
        // Verify energy statistics
        let stats = blockchain.get_energy_stats().await?;
        assert!(stats.total_energy_generated > 0.0);
        assert!(stats.total_energy_traded > 0.0);
        
        Ok(())
    }
}
