//! Shared Value Objects
//!
//! This module contains value objects that are used across multiple bounded contexts.
//! These represent concepts that are shared in the ubiquitous language of GridTokenX.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Base trait for all value objects
pub trait ValueObject: Send + Sync + Clone + PartialEq + std::fmt::Debug {}

/// Generic hash value used throughout the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(String);

impl Hash {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            return Err(anyhow!("Hash cannot be empty"));
        }
        
        // Validate hex format
        if !value.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow!("Hash must be valid hexadecimal"));
        }
        
        Ok(Hash(value.to_lowercase()))
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Hash(hex::encode(bytes))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        hex::decode(&self.0).map_err(|e| anyhow!("Invalid hash format: {}", e))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Hash {
    type Error = anyhow::Error;
    
    fn try_from(value: String) -> Result<Self> {
        Hash::new(value)
    }
}

impl From<Hash> for String {
    fn from(hash: Hash) -> Self {
        hash.0
    }
}

/// Cryptographic signature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(String);

impl Signature {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            return Err(anyhow!("Signature cannot be empty"));
        }
        
        // Validate hex format
        if !value.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow!("Signature must be valid hexadecimal"));
        }
        
        Ok(Signature(value.to_lowercase()))
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Signature(hex::encode(bytes))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        hex::decode(&self.0).map_err(|e| anyhow!("Invalid signature format: {}", e))
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Timestamp value object with validation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Timestamp(Utc::now())
    }
    
    pub fn new(datetime: DateTime<Utc>) -> Result<Self> {
        // Validate that timestamp is not too far in the future (prevent clock drift issues)
        let now = Utc::now();
        let max_future = now + chrono::Duration::hours(1);
        
        if datetime > max_future {
            return Err(anyhow!("Timestamp cannot be more than 1 hour in the future"));
        }
        
        Ok(Timestamp(datetime))
    }
    
    pub fn from_timestamp(timestamp: i64) -> Result<Self> {
        let datetime = DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| anyhow!("Invalid timestamp: {}", timestamp))?;
        Self::new(datetime)
    }
    
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
    
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }
    
    pub fn is_before(&self, other: &Timestamp) -> bool {
        self.0 < other.0
    }
    
    pub fn is_after(&self, other: &Timestamp) -> bool {
        self.0 > other.0
    }
    
    pub fn duration_since(&self, other: &Timestamp) -> chrono::Duration {
        self.0 - other.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S UTC"))
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

/// Generic amount value object with validation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(u64);

impl Amount {
    pub fn new(value: u64) -> Result<Self> {
        if value == 0 {
            return Err(anyhow!("Amount must be greater than zero"));
        }
        Ok(Amount(value))
    }
    
    pub fn zero() -> Self {
        Amount(0)
    }
    
    pub fn value(&self) -> u64 {
        self.0
    }
    
    pub fn add(&self, other: &Amount) -> Result<Amount> {
        self.0
            .checked_add(other.0)
            .map(Amount)
            .ok_or_else(|| anyhow!("Amount overflow"))
    }
    
    pub fn subtract(&self, other: &Amount) -> Result<Amount> {
        if self.0 < other.0 {
            return Err(anyhow!("Cannot subtract larger amount from smaller"));
        }
        Ok(Amount(self.0 - other.0))
    }
    
    pub fn multiply(&self, factor: u64) -> Result<Amount> {
        self.0
            .checked_mul(factor)
            .map(Amount)
            .ok_or_else(|| anyhow!("Amount overflow"))
    }
    
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u64> for Amount {
    type Error = anyhow::Error;
    
    fn try_from(value: u64) -> Result<Self> {
        Amount::new(value)
    }
}

impl From<Amount> for u64 {
    fn from(amount: Amount) -> Self {
        amount.0
    }
}

/// Address value object for blockchain addresses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            return Err(anyhow!("Address cannot be empty"));
        }
        
        // Basic validation - addresses should be hex encoded
        if value.len() < 20 || value.len() > 64 {
            return Err(anyhow!("Address length must be between 20 and 64 characters"));
        }
        
        if !value.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow!("Address must be valid hexadecimal"));
        }
        
        Ok(Address(value.to_lowercase()))
    }
    
    pub fn from_public_key(public_key: &[u8]) -> Self {
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(public_key);
        // Take last 20 bytes for address (like Ethereum)
        let address_bytes = &hash[12..];
        Address(hex::encode(address_bytes))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        hex::decode(&self.0).map_err(|e| anyhow!("Invalid address format: {}", e))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", self.0)
    }
}

impl TryFrom<String> for Address {
    type Error = anyhow::Error;
    
    fn try_from(value: String) -> Result<Self> {
        // Remove 0x prefix if present
        let clean_value = if value.starts_with("0x") {
            value[2..].to_string()
        } else {
            value
        };
        Address::new(clean_value)
    }
}

/// Identifier value object for unique IDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Id(uuid::Uuid::new_v4().to_string())
    }
    
    pub fn from_string(value: String) -> Result<Self> {
        if value.is_empty() {
            return Err(anyhow!("ID cannot be empty"));
        }
        
        // Validate UUID format
        uuid::Uuid::parse_str(&value)
            .map_err(|_| anyhow!("Invalid UUID format"))?;
        
        Ok(Id(value))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<String> for Id {
    type Error = anyhow::Error;
    
    fn try_from(value: String) -> Result<Self> {
        Id::from_string(value)
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0
    }
}
