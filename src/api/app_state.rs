use std::sync::{Arc, RwLock};
use crate::engine::{market_engine::MarketEngine, economic_engine::EconomicEngine};

#[derive(Clone)]
pub struct AppState {
    pub market_engine: Arc<RwLock<MarketEngine>>,
    pub economic_engine: Arc<RwLock<EconomicEngine>>,
}