//! Governance Token Entity
//!
//! Represents the NRG governance token used for staking and voting.

use crate::shared::domain::{errors::DomainError, repository::AggregateRoot, events::DomainEvent};
use crate::domains::governance::domain::value_objects::TokenAmount;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

/// Governance Token Entity
#[derive(Debug, Serialize, Deserialize)]
pub struct GovernanceToken {
    symbol: String,
    total_supply: TokenAmount,
    circulating_supply: TokenAmount,
    created_at: DateTime<Utc>,
    version: u64,
    #[serde(skip)]
    uncommitted_events: VecDeque<Box<dyn DomainEvent>>,
}

impl Clone for GovernanceToken {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol.clone(),
            total_supply: self.total_supply.clone(),
            circulating_supply: self.circulating_supply.clone(),
            created_at: self.created_at,
            version: self.version,
            uncommitted_events: VecDeque::new(),
        }
    }
}

impl GovernanceToken {
    pub fn new(symbol: String, total_supply: TokenAmount) -> Result<Self, DomainError> {
        Ok(Self {
            symbol,
            total_supply: total_supply.clone(),
            circulating_supply: total_supply,
            created_at: Utc::now(),
            version: 1,
            uncommitted_events: VecDeque::new(),
        })
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn total_supply(&self) -> &TokenAmount {
        &self.total_supply
    }

    pub fn circulating_supply(&self) -> &TokenAmount {
        &self.circulating_supply
    }
}

impl AggregateRoot for GovernanceToken {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.symbol
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn uncommitted_events(&self) -> &VecDeque<Box<dyn DomainEvent>> {
        &self.uncommitted_events
    }

    fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    fn apply_event(&mut self, _event: Box<dyn DomainEvent>) {
        // Apply event logic would go here
        self.version += 1;
    }

    fn raise_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push_back(event);
    }
}
