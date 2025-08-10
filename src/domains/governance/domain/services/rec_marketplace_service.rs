//! REC Marketplace Service
//!
//! Domain service for handling Renewable Energy Certificate marketplace operations.

use crate::shared::domain::errors::DomainError;
use crate::domains::governance::domain::{
    entities::RECToken,
    value_objects::{RECId, EnergySourceType},
};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;

/// REC marketplace service
pub struct RECMarketplaceService {
    recs: RwLock<HashMap<String, RECToken>>,
}

impl RECMarketplaceService {
    pub async fn new() -> Result<Self, DomainError> {
        Ok(Self {
            recs: RwLock::new(HashMap::new()),
        })
    }

    pub async fn mint_rec(
        &self,
        energy_amount_kwh: f64,
        energy_source: EnergySourceType,
        generator_id: &str,
    ) -> Result<RECToken, DomainError> {
        let rec = RECToken::new(
            energy_amount_kwh,
            energy_source,
            generator_id.to_string(),
        )?;

        let mut recs = self.recs.write().await;
        recs.insert(rec.id().value().to_string(), rec.clone());

        info!(
            "Minted REC {} for {} kWh of {} energy by {}",
            rec.id(),
            energy_amount_kwh,
            rec.energy_source(),
            generator_id
        );

        Ok(rec)
    }

    pub async fn list_rec_for_sale(
        &self,
        rec_id: RECId,
        price: f64,
    ) -> Result<(), DomainError> {
        let mut recs = self.recs.write().await;
        
        let rec = recs
            .get_mut(rec_id.value())
            .ok_or_else(|| DomainError::aggregate_not_found(format!("REC not found: {}", rec_id)))?;
        
        rec.list_for_sale(price)?;

        info!("Listed REC {} for sale at {:.4} THB/kWh", rec_id, price);

        Ok(())
    }

    pub async fn purchase_rec(
        &self,
        rec_id: RECId,
        buyer_id: &str,
    ) -> Result<RECToken, DomainError> {
        let mut recs = self.recs.write().await;
        
        let rec = recs
            .get_mut(rec_id.value())
            .ok_or_else(|| DomainError::aggregate_not_found(format!("REC not found: {}", rec_id)))?;
        
        rec.transfer_ownership(buyer_id.to_string())?;

        info!("REC {} purchased by {}", rec_id, buyer_id);

        Ok(rec.clone())
    }

    pub async fn retire_rec(
        &self,
        rec_id: RECId,
        reason: &str,
    ) -> Result<(), DomainError> {
        let mut recs = self.recs.write().await;
        
        let rec = recs
            .get_mut(rec_id.value())
            .ok_or_else(|| DomainError::aggregate_not_found(format!("REC not found: {}", rec_id)))?;
        
        rec.retire(reason.to_string())?;

        info!("REC {} retired: {}", rec_id, reason);

        Ok(())
    }

    pub async fn get_rec(&self, rec_id: &RECId) -> Option<RECToken> {
        let recs = self.recs.read().await;
        recs.get(rec_id.value()).cloned()
    }

    pub async fn get_recs_by_generator(&self, generator_id: &str) -> Vec<RECToken> {
        let recs = self.recs.read().await;
        recs.values()
            .filter(|rec| rec.generator_id() == generator_id)
            .cloned()
            .collect()
    }

    pub async fn get_recs_for_sale(&self) -> Vec<RECToken> {
        let recs = self.recs.read().await;
        recs.values()
            .filter(|rec| matches!(rec.status(), crate::domains::governance::domain::entities::rec_token::RECStatus::ForSale))
            .cloned()
            .collect()
    }
}
