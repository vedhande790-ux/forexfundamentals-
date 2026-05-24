use axum::{extract::State, Json};
use crate::api::{
    app_state::AppState,
    price_response::PriceResponse,
    economic_response::EconomicResponse,
};

pub async fn get_prices(State(state): State<AppState>) -> Json<Vec<PriceResponse>> {
    let engine = state.market_engine.read().unwrap();
    let market_state = engine.get_market_state();
    let mut response = Vec::new();

    for tick in market_state.values() {
        response.push(PriceResponse {
            symbol: tick.symbol.clone(),
            price: tick.price,
        });
    }

    Json(response)
}

pub async fn get_events(State(state): State<AppState>) -> Json<Vec<EconomicResponse>> {
    let engine = state.economic_engine.read().unwrap();
    let economic_state = engine.get_economic_state();
    let mut response = Vec::new();

    for currency_events in economic_state.values() {
        for event in currency_events.values() {
            response.push(EconomicResponse {
                currency: event.currency.clone(),
                event_type: event.event_type.clone(),
            });
        }
    }

    Json(response)
}