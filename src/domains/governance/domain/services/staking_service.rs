//! Staking Service
//!
//! Domain service for handling token staking operations.

use crate::shared::domain::errors::DomainError;
use crate::domains::governance::domain::{
    entities::StakePosition,
    value_objects::{TokenAmount, StakeId},
};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Staking service for governance tokens
pub struct StakingService {
    stakes: RwLock<HashMap<String, StakePosition>>,
}

impl StakingService {
    pub async fn new() -> Result<Self, DomainError> {
        Ok(Self {
            stakes: RwLock::new(HashMap::new()),
        })
    }

    pub async fn stake_tokens(
        &self,
        staker_id: &str,
        amount: TokenAmount,
        lock_duration_days: u32,
    ) -> Result<StakePosition, DomainError> {
        let stake = StakePosition::new(
            staker_id.to_string(),
            amount,
            lock_duration_days,
        )?;

        let mut stakes = self.stakes.write().await;
        stakes.insert(stake.id().value().to_string(), stake.clone());

        Ok(stake)
    }

    pub async fn get_stake(&self, stake_id: &StakeId) -> Option<StakePosition> {
        let stakes = self.stakes.read().await;
        stakes.get(stake_id.value()).cloned()
    }

    pub async fn get_stakes_by_staker(&self, staker_id: &str) -> Vec<StakePosition> {
        let stakes = self.stakes.read().await;
        stakes
            .values()
            .filter(|stake| stake.staker_id() == staker_id)
            .cloned()
            .collect()
    }
}
