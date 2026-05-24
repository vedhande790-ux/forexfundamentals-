mod model;
mod ingestion;
mod engine;
mod api;

use crate::engine::{
    market_engine::MarketEngine,
    runtime::start_runtime,
    economic_engine::EconomicEngine,
    economic_runtime::start_economic_runtime,
};
use crate::api::{
    app_state::AppState,
    route::{get_prices, get_events},
};
use std::sync::{Arc, RwLock};
use axum::{routing::get, Router};
use tokio::task;

#[tokio::main]
async fn main() {
    let market_engine = MarketEngine::new();
    let shared_engine = Arc::new(RwLock::new(market_engine));

    let economic_engine = EconomicEngine::new();
    let shared_economic_engine = Arc::new(RwLock::new(economic_engine));

    let runtime_engine = shared_engine.clone();
    task::spawn(async move {
        start_runtime(runtime_engine).await;
    });

    let economic_runtime_engine = shared_economic_engine.clone();
    task::spawn(async move {
        start_economic_runtime(economic_runtime_engine).await;
    });

    let app_state = AppState {
        market_engine: shared_engine.clone(),
        economic_engine: shared_economic_engine.clone(),
    };

    let app = Router::new()
        .route("/prices", get(get_prices))
        .route("/events", get(get_events))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}