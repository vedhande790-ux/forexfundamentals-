use serde::Serialize;

#[derive(Serialize)]

pub struct PriceResponse {

    pub symbol:
        String,

    pub price:
        f64,
}