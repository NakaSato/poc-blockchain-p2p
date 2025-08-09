//! GridTokenX Utility Module
//!
//! This module provides various utility functions for the GridTokenX blockchain,
//! including cryptographic operations, data validation, and helper functions.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Wallet structure containing public/private key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub address: String,
}

/// Energy measurement units and conversions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnergyUnit {
    Wh,  // Watt-hour
    KWh, // Kilowatt-hour
    MWh, // Megawatt-hour
    GWh, // Gigawatt-hour
}

/// Thai energy market utilities
pub struct ThaiEnergyMarket;

impl ThaiEnergyMarket {
    /// Check if current time is peak hours in Thailand
    pub fn is_peak_hours() -> bool {
        let now = Utc::now();
        let bangkok_time = now + chrono::Duration::hours(7); // UTC+7
        let hour = bangkok_time.hour();

        // Peak hours: 18:00-22:00 on weekdays
        if bangkok_time.weekday().number_from_monday() <= 5 {
            hour >= 18 && hour < 22
        } else {
            false // No peak hours on weekends by default
        }
    }

    /// Get current Thai electricity tariff multiplier
    pub fn get_tariff_multiplier(region: &str) -> f64 {
        match region.to_lowercase().as_str() {
            "bangkok" => 1.2,      // Higher rates in Bangkok
            "central" => 1.0,      // Base rate
            "northern" => 0.9,     // Lower rates in northern regions
            "northeastern" => 0.8, // Lowest rates in Isan
            "southern" => 1.1,     // Slightly higher in south
            _ => 1.0,              // Default base rate
        }
    }

    /// Validate Thai grid location format
    pub fn validate_grid_location(location: &str) -> bool {
        // Format: PROVINCE-AREA-SUBSTATION (e.g., "BKK-01-SUB001")
        let parts: Vec<&str> = location.split('-').collect();
        parts.len() == 3
            && parts[0].len() == 3
            && parts[1].len() == 2
            && parts[2].starts_with("SUB")
    }

    /// Get Thai province code from full name
    pub fn get_province_code(province_name: &str) -> Option<&'static str> {
        match province_name.to_lowercase().as_str() {
            "bangkok" => Some("BKK"),
            "samut prakan" => Some("SPK"),
            "nonthaburi" => Some("NTB"),
            "pathum thani" => Some("PTH"),
            "chiang mai" => Some("CNX"),
            "chiang rai" => Some("CEI"),
            "khon kaen" => Some("KKC"),
            "udon thani" => Some("UTH"),
            "songkhla" => Some("SGZ"),
            "phuket" => Some("HKT"),
            _ => None,
        }
    }
}

/// Generic utility functions
pub struct Utils;

impl Utils {
    /// Generate a random ID
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Get current timestamp
    pub fn current_timestamp() -> DateTime<Utc> {
        Utc::now()
    }

    /// Format bytes as human readable string
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

        if bytes == 0 {
            return "0 B".to_string();
        }

        let base = 1024u64;
        let unit_index = (bytes as f64).log(base as f64).floor() as usize;
        let unit_index = unit_index.min(UNITS.len() - 1);

        let size = bytes as f64 / base.pow(unit_index as u32) as f64;

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Format duration in human readable form
    pub fn format_duration(seconds: u64) -> String {
        if seconds < 60 {
            format!("{}s", seconds)
        } else if seconds < 3600 {
            format!("{}m {}s", seconds / 60, seconds % 60)
        } else if seconds < 86400 {
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            format!("{}h {}m", hours, minutes)
        } else {
            let days = seconds / 86400;
            let hours = (seconds % 86400) / 3600;
            format!("{}d {}h", days, hours)
        }
    }

    /// Generate secure random bytes
    pub fn secure_random_bytes(length: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut bytes = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes
    }

    /// Convert HashMap to sorted vector for deterministic serialization
    pub fn hashmap_to_sorted_vec<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)>
    where
        K: Ord,
    {
        let mut items: Vec<_> = map.iter().collect();
        items.sort_by_key(|(k, _)| *k);
        items
    }
}

/// Energy conversion utilities
pub struct EnergyConversion;

impl EnergyConversion {
    /// Convert between energy units
    pub fn convert(amount: f64, from: EnergyUnit, to: EnergyUnit) -> f64 {
        let wh_amount = match from {
            EnergyUnit::Wh => amount,
            EnergyUnit::KWh => amount * 1000.0,
            EnergyUnit::MWh => amount * 1_000_000.0,
            EnergyUnit::GWh => amount * 1_000_000_000.0,
        };

        match to {
            EnergyUnit::Wh => wh_amount,
            EnergyUnit::KWh => wh_amount / 1000.0,
            EnergyUnit::MWh => wh_amount / 1_000_000.0,
            EnergyUnit::GWh => wh_amount / 1_000_000_000.0,
        }
    }

    /// Convert energy to GridTokenX tokens (1 kWh = 1,000,000 micro-tokens)
    pub fn energy_to_tokens(kwh: f64) -> u64 {
        (kwh * 1_000_000.0) as u64
    }

    /// Convert GridTokenX tokens to energy
    pub fn tokens_to_energy(tokens: u64) -> f64 {
        tokens as f64 / 1_000_000.0
    }

    /// Calculate carbon footprint for energy source (kg CO2 per kWh)
    pub fn carbon_footprint(energy_kwh: f64, source: &str) -> f64 {
        let co2_per_kwh = match source.to_lowercase().as_str() {
            "coal" => 0.820,
            "natural_gas" => 0.490,
            "oil" => 0.778,
            "nuclear" => 0.012,
            "hydro" => 0.024,
            "wind" => 0.011,
            "solar" => 0.041,
            "biomass" => 0.230,
            "geothermal" => 0.038,
            _ => 0.500, // Average grid mix
        };

        energy_kwh * co2_per_kwh
    }
}

/// Data validation utilities
pub struct DataValidation;

impl DataValidation {
    /// Validate energy amount is within reasonable bounds
    pub fn validate_energy_amount(kwh: f64) -> Result<()> {
        if kwh <= 0.0 {
            return Err(anyhow!("Energy amount must be positive"));
        }

        if kwh > 100_000.0 {
            return Err(anyhow!("Energy amount exceeds maximum limit (100 MWh)"));
        }

        Ok(())
    }

    /// Validate price is within Thai market bounds
    pub fn validate_energy_price(price_per_kwh: u64) -> Result<()> {
        const MIN_PRICE: u64 = 500; // 0.5 tokens per kWh
        const MAX_PRICE: u64 = 20_000; // 20 tokens per kWh

        if price_per_kwh < MIN_PRICE {
            return Err(anyhow!("Price below minimum ({} tokens/kWh)", MIN_PRICE));
        }

        if price_per_kwh > MAX_PRICE {
            return Err(anyhow!("Price above maximum ({} tokens/kWh)", MAX_PRICE));
        }

        Ok(())
    }
}

/// Cryptographic utilities
pub mod crypto {
    use super::*;
    use rand::RngCore;

    /// Generate a new cryptographic keypair
    pub fn generate_keypair() -> Result<Wallet> {
        // Generate 32-byte keys for simplicity
        let mut private_key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut private_key);

        // Generate public key (simplified - in real implementation would use actual crypto)
        let mut public_key = vec![0u8; 32];
        for (i, &byte) in private_key.iter().enumerate() {
            public_key[i] = byte.wrapping_add(1);
        }

        // Generate address from public key hash
        let address = generate_address(&public_key);

        Ok(Wallet {
            public_key,
            private_key,
            address,
        })
    }

    /// Generate address from public key
    pub fn generate_address(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        hex::encode(&hash[..20]) // Take first 20 bytes
    }

    /// Sign data with private key (simplified implementation)
    pub fn sign_data(private_key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(private_key);
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    /// Verify signature (simplified implementation)
    pub fn verify_signature(public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<bool> {
        // Simplified verification - in real implementation would use proper crypto
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        hasher.update(data);
        let expected = hasher.finalize();

        Ok(expected.as_slice() == signature)
    }

    /// Hash data using SHA256
    pub fn hash_sha256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

