use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration};
use crate::ingestion::economic_api::fetch_economic_events;
use crate::engine::economic_engine::EconomicEngine;

pub async fn start_economic_runtime(engine: Arc<RwLock<EconomicEngine>>) {
    loop {
        let events = fetch_economic_events().await;

        {
            let mut engine = engine.write().unwrap();
            for event in events {
                engine.process_event(event);
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}