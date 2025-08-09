//! Get Market Depth Query
//!
//! Query for retrieving current market depth information.

use crate::shared::{
    domain::errors::DomainError,
    application::{Query, QueryHandler},
};
use crate::domains::energy_trading::domain::{
    services::EnergyTradingDomainService,
    aggregates::MarketDepth,
};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketDepthQuery {
    pub market_name: String,
}

impl Query for GetMarketDepthQuery {
    type Result = GetMarketDepthResult;
    
    fn query_type(&self) -> &'static str {
        "GetMarketDepth"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketDepthResult {
    pub market_name: String,
    pub market_depth: MarketDepth,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct GetMarketDepthHandler {
    energy_trading_service: Arc<EnergyTradingDomainService>,
}

impl GetMarketDepthHandler {
    pub fn new(energy_trading_service: Arc<EnergyTradingDomainService>) -> Self {
        Self {
            energy_trading_service,
        }
    }
}

#[async_trait]
impl QueryHandler<GetMarketDepthQuery> for GetMarketDepthHandler {
    async fn handle(&self, query: GetMarketDepthQuery) -> Result<GetMarketDepthResult, DomainError> {
        let market_depth = self.energy_trading_service
            .get_market_depth(&query.market_name)
            .await?;
        
        Ok(GetMarketDepthResult {
            market_name: query.market_name,
            market_depth,
            timestamp: chrono::Utc::now(),
        })
    }
}
