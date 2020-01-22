extern crate ureq;

fn main() {
    match request() {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error!: {}", e),
    }
}

fn request() -> Result<serde_json::Value, std::io::Error> {
    let response = ureq::get("https://query1.finance.yahoo.com/v7/finance/quote")
        .query("symbols", "fb,tsla,stmp")
        .query("fields", "regularMarketPrice,regularMarketChangePercent")
        .call();

    return response.into_json();
}
