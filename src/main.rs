mod yahoo;
use clap::{App, Arg};
use colored::*;

fn main() {
    let matches = App::new("Quoter")
        .about("Stock quotes on the CLI")
        .arg(
            Arg::with_name("symbols")
                .long("symbols")
                .short("s")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .help("space separated list of stock symbols"),
        )
        .get_matches();

    let symbols = matches.values_of("symbols").unwrap().collect();

    let value: serde_json::Value = request(symbols).expect("failed to call Yahoo finance");

    let mut typed: yahoo::QuoteResponseWrapper =
        serde_json::from_value(value).expect("failed to deserialize to type");

    typed
        .quoteResponse
        .result
        .sort_unstable_by(|l, r| l.symbol.cmp(&r.symbol));

    for result in typed.quoteResponse.result.iter() {
        let color = get_change_color(result.regularMarketChangePercent);
        println!(
            "{0} {1} {2}",
            format!("{:<5}", result.symbol),
            format!("{:>10}", format!("${:.2}", result.regularMarketPrice)),
            format!(
                "{:>10}",
                format!("{:.2}%", result.regularMarketChangePercent).color(color)
            )
        );
    }
}

fn get_change_color(change: f32) -> &'static str {
    if change > 0.0 {
        return "green";
    }
    return "red";
}

fn request(symbols: Vec<&str>) -> Result<serde_json::Value, std::io::Error> {
    let response = ureq::get("https://query1.finance.yahoo.com/v7/finance/quote")
        .query("symbols", &symbols.join(","))
        .query("fields", "regularMarketPrice,regularMarketChangePercent")
        .call();

    return response.into_json();
}
