//! Incentive Mechanism Service
//!
//! Domain service for handling reward distribution and incentive mechanisms.

use crate::shared::domain::errors::DomainError;
use crate::domains::governance::domain::value_objects::TokenAmount;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;

/// Incentive mechanism service
pub struct IncentiveMechanismService {
    rewards: RwLock<HashMap<String, TokenAmount>>,
}

impl IncentiveMechanismService {
    pub async fn new() -> Result<Self, DomainError> {
        Ok(Self {
            rewards: RwLock::new(HashMap::new()),
        })
    }

    pub async fn reward_conservation(
        &self,
        trader_id: &str,
        energy_saved_kwh: f64,
    ) -> Result<TokenAmount, DomainError> {
        // Reward calculation: 0.1 NRG per kWh saved
        let reward_amount = TokenAmount::new(energy_saved_kwh * 0.1, 18)?;
        
        let mut rewards = self.rewards.write().await;
        let current_rewards = rewards
            .get(trader_id)
            .cloned()
            .unwrap_or_else(|| TokenAmount::new(0.0, 18).unwrap());
        
        let new_total = current_rewards.add(&reward_amount)?;
        rewards.insert(trader_id.to_string(), new_total.clone());

        info!(
            "Rewarded {} for conserving {} kWh energy (total: {})",
            trader_id, energy_saved_kwh, new_total
        );

        Ok(reward_amount)
    }

    pub async fn reward_demand_response(
        &self,
        trader_id: &str,
    ) -> Result<TokenAmount, DomainError> {
        // Fixed reward for demand response participation
        let reward_amount = TokenAmount::new(5.0, 18)?; // 5 NRG
        
        let mut rewards = self.rewards.write().await;
        let current_rewards = rewards
            .get(trader_id)
            .cloned()
            .unwrap_or_else(|| TokenAmount::new(0.0, 18).unwrap());
        
        let new_total = current_rewards.add(&reward_amount)?;
        rewards.insert(trader_id.to_string(), new_total.clone());

        info!(
            "Rewarded {} for demand response participation (total: {})",
            trader_id, new_total
        );

        Ok(reward_amount)
    }

    pub async fn reward_liquidity_provision(
        &self,
        trader_id: &str,
        volume_provided: f64,
    ) -> Result<TokenAmount, DomainError> {
        // Reward calculation: 0.01 NRG per kWh provided
        let reward_amount = TokenAmount::new(volume_provided * 0.01, 18)?;
        
        let mut rewards = self.rewards.write().await;
        let current_rewards = rewards
            .get(trader_id)
            .cloned()
            .unwrap_or_else(|| TokenAmount::new(0.0, 18).unwrap());
        
        let new_total = current_rewards.add(&reward_amount)?;
        rewards.insert(trader_id.to_string(), new_total.clone());

        info!(
            "Rewarded {} for providing {} kWh liquidity (total: {})",
            trader_id, volume_provided, new_total
        );

        Ok(reward_amount)
    }

    pub async fn get_total_rewards(&self, trader_id: &str) -> TokenAmount {
        let rewards = self.rewards.read().await;
        rewards
            .get(trader_id)
            .cloned()
            .unwrap_or_else(|| TokenAmount::new(0.0, 18).unwrap())
    }
}
