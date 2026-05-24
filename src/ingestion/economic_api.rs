use crate::model::{
    currency::Currency,
    economic_event::{EconomicEvent, EventType, Impact},
};

pub async fn fetch_economic_events() -> Vec<EconomicEvent> {
    vec![
        EconomicEvent {
            currency: Currency::USD,
            event_type: EventType::CPI,
            actual: 3.2,
            forecast: 3.0,
            impact: Impact::High,
            timestamp: 123456,
        },
        EconomicEvent {
            currency: Currency::USD,
            event_type: EventType::NFP,
            actual: 220000.0,
            forecast: 200000.0,
            impact: Impact::High,
            timestamp: 123457,
        },
    ]
}