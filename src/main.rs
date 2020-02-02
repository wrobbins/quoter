use std::env;
mod yahoo;

fn main() {
    let symbols: Vec<String> = env::args().collect();

    let value: serde_json::Value = request(symbols).expect("failed to call Yahoo finance");

    let typed: yahoo::QuoteResponseWrapper =
        serde_json::from_value(value).expect("failed to deserialize to type");

    for result in typed.quoteResponse.result.iter() {
        println!(
            "{0} {1} {2}",
            format!("{:<5}", result.symbol),
            format!("{:>10}", format!("${:.2}", result.regularMarketPrice)),
            format!(
                "{:>10}",
                format!("{:.2}%", result.regularMarketChangePercent)
            )
        );
    }
}

fn request(symbols: Vec<String>) -> Result<serde_json::Value, std::io::Error> {
    let response = ureq::get("https://query1.finance.yahoo.com/v7/finance/quote")
        .query("symbols", &symbols.join(","))
        .query("fields", "regularMarketPrice,regularMarketChangePercent")
        .call();

    return response.into_json();
}
