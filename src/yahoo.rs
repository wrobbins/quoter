#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponseWrapper {
    pub quoteResponse: QuoteResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub result: Vec<Quote>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub regularMarketPrice: f32,
    pub symbol: String,
    pub regularMarketChangePercent: f32
}
