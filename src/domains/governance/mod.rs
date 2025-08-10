//! Governance Domain Module
//!
//! This bounded context handles governance tokens, staking, voting,
//! incentive mechanisms, and renewable energy certificates (RECs).

pub mod domain;
pub mod application;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{
    entities::{GovernanceToken, StakePosition, RECToken},
    services::{StakingService, IncentiveMechanismService, RECMarketplaceService},
    value_objects::{TokenAmount, StakeId, RECId, VotingPower},
};
