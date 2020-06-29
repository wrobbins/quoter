use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponseWrapper {
    pub quote_response: QuoteResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub result: Vec<Quote>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub regular_market_price: f32,
    pub symbol: String,
    pub regular_market_change_percent: f32
}
