use anyhow::Result;

pub mod poa;

pub use poa::{POAConsensusEngine, Authority, ThaiAuthorityType};

/// Consensus engine trait for different consensus mechanisms
pub trait ConsensusEngine {
    /// Validate a block according to consensus rules
    fn validate_block(&self, block: &crate::blockchain::Block, previous_block: Option<&crate::blockchain::Block>) -> Result<bool>;
    
    /// Select the next validator/authority for block production
    fn select_next_validator(&self) -> Result<String>;
    
    /// Check if an authority is authorized to produce a block
    fn is_authorized(&self, authority: &str, block_height: u64) -> Result<bool>;
    
    /// Get consensus configuration
    fn get_config(&self) -> Result<ConsensusConfig>;
}

/// Consensus configuration structure
#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    pub block_time: u64,           // Block time in seconds
    pub round_timeout: u64,        // Round timeout in seconds  
    pub max_validators: u64,       // Maximum number of validators
    pub consensus_type: String,    // Type of consensus (POA, POW, etc.)
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            block_time: 15,
            round_timeout: 30,
            max_validators: 10,
            consensus_type: "poa".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consensus_config_default() {
        let config = ConsensusConfig::default();
        assert_eq!(config.block_time, 15);
        assert_eq!(config.round_timeout, 30);
        assert_eq!(config.max_validators, 10);
        assert_eq!(config.consensus_type, "poa");
    }
    
    #[test]
    fn test_consensus_config_clone() {
        let config = ConsensusConfig::default();
        let cloned = config.clone();
        assert_eq!(config.block_time, cloned.block_time);
        assert_eq!(config.consensus_type, cloned.consensus_type);
    }
}
