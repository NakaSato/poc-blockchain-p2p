//! Renewable Energy Certificate (REC) Token Entity
//!
//! Represents a renewable energy certificate as a non-fungible token.

use crate::shared::domain::{errors::DomainError, repository::AggregateRoot, events::DomainEvent};
use crate::domains::governance::domain::value_objects::{RECId, EnergySourceType};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RECStatus {
    Active,
    ForSale,
    Sold,
    Retired,
}

/// REC Token Entity
#[derive(Debug, Serialize, Deserialize)]
pub struct RECToken {
    id: RECId,
    energy_amount_kwh: f64,
    energy_source: EnergySourceType,
    generator_id: String,
    owner_id: String,
    status: RECStatus,
    minted_at: DateTime<Utc>,
    retired_at: Option<DateTime<Utc>>,
    retirement_reason: Option<String>,
    sale_price: Option<f64>,
    version: u64,
    #[serde(skip)]
    uncommitted_events: VecDeque<Box<dyn DomainEvent>>,
}

impl Clone for RECToken {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            energy_amount_kwh: self.energy_amount_kwh,
            energy_source: self.energy_source.clone(),
            generator_id: self.generator_id.clone(),
            owner_id: self.owner_id.clone(),
            status: self.status.clone(),
            minted_at: self.minted_at,
            retired_at: self.retired_at,
            retirement_reason: self.retirement_reason.clone(),
            sale_price: self.sale_price,
            version: self.version,
            uncommitted_events: VecDeque::new(),
        }
    }
}

impl RECToken {
    pub fn new(
        energy_amount_kwh: f64,
        energy_source: EnergySourceType,
        generator_id: String,
    ) -> Result<Self, DomainError> {
        if energy_amount_kwh <= 0.0 {
            return Err(DomainError::invalid_value("Energy amount must be positive"));
        }

        let id = RECId::generate();
        
        Ok(Self {
            id,
            energy_amount_kwh,
            energy_source,
            generator_id: generator_id.clone(),
            owner_id: generator_id, // Initially owned by generator
            status: RECStatus::Active,
            minted_at: Utc::now(),
            retired_at: None,
            retirement_reason: None,
            sale_price: None,
            version: 1,
            uncommitted_events: VecDeque::new(),
        })
    }

    pub fn id(&self) -> &RECId {
        &self.id
    }

    pub fn energy_amount_kwh(&self) -> f64 {
        self.energy_amount_kwh
    }

    pub fn energy_source(&self) -> &EnergySourceType {
        &self.energy_source
    }

    pub fn generator_id(&self) -> &str {
        &self.generator_id
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn status(&self) -> &RECStatus {
        &self.status
    }

    pub fn list_for_sale(&mut self, price: f64) -> Result<(), DomainError> {
        match self.status {
            RECStatus::Active => {
                if price <= 0.0 {
                    return Err(DomainError::invalid_value("Sale price must be positive"));
                }
                self.status = RECStatus::ForSale;
                self.sale_price = Some(price);
                self.version += 1;
                Ok(())
            }
            _ => Err(DomainError::invalid_operation("Can only list active RECs for sale"))
        }
    }

    pub fn transfer_ownership(&mut self, new_owner_id: String) -> Result<(), DomainError> {
        match self.status {
            RECStatus::ForSale => {
                self.owner_id = new_owner_id;
                self.status = RECStatus::Sold;
                self.version += 1;
                Ok(())
            }
            _ => Err(DomainError::invalid_operation("Can only transfer RECs that are for sale"))
        }
    }

    pub fn retire(&mut self, reason: String) -> Result<(), DomainError> {
        match self.status {
            RECStatus::Active | RECStatus::Sold => {
                self.status = RECStatus::Retired;
                self.retired_at = Some(Utc::now());
                self.retirement_reason = Some(reason);
                self.version += 1;
                Ok(())
            }
            _ => Err(DomainError::invalid_operation("Cannot retire REC in current status"))
        }
    }

    pub fn carbon_offset_kg(&self) -> f64 {
        self.energy_amount_kwh * self.energy_source.carbon_factor()
    }
}

impl AggregateRoot for RECToken {
    type Id = RECId;

    fn id(&self) -> &Self::Id {
        &self.id
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
        self.version += 1;
    }

    fn raise_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push_back(event);
    }
}
