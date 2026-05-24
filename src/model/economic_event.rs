use serde::Serialize;
use crate::model::currency::Currency;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum EventType {
    CPI,
    NFP,
    InterestRate,
    PMI,
}

#[derive(Debug, Clone, Serialize)]
pub enum Impact {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct EconomicEvent {
    pub currency: Currency,
    pub event_type: EventType,
    pub actual: f64,
    pub forecast: f64,
    pub impact: Impact,
    pub timestamp: u64,
}