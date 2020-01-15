fn main() {
    match request(){
        Ok(json) => println!("{}",json),
        Err(e) => println!("Error!: {}", e)
    }
}

fn request() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://query1.finance.yahoo.com/v7/finance/quote")
        .query(&[("symbols", "fb")])
        .query(&[("fields", "regularMarketPrice,regularMarketChangePercent")])
        .send()?;

    let body = response.text()?;

    return Ok(body);
}
