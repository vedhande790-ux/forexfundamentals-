use crate::models::{currency::Currency, economic_event::EventType};

pub struct EconomicResponse {
    pub currency: Currency,
    pub event_type: EventType,
}