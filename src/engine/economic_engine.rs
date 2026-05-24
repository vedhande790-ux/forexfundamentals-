use std::collections::HashMap;
use crate::model::{
    currency::Currency,
    economic_event::{EconomicEvent, EventType},
};

pub struct EconomicEngine {
    economic_state: HashMap<Currency, HashMap<EventType, EconomicEvent>>,
}

impl EconomicEngine {
    pub fn new() -> Self {
        Self {
            economic_state: HashMap::new(),
        }
    }

    pub fn process_event(&mut self, event: EconomicEvent) {
        self.economic_state
            .entry(event.currency.clone())
            .or_insert(HashMap::new())
            .insert(event.event_type.clone(), event);
    }

    pub fn get_economic_state(&self) -> &HashMap<Currency, HashMap<EventType, EconomicEvent>> {
        &self.economic_state
    }
}
