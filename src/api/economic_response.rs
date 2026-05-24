use serde::Serialize;
use crate::model::{currency::Currency, economic_event::EventType};

#[derive(Serialize)]
pub struct EconomicResponse {
    pub currency: Currency,
    pub event_type: EventType,
}