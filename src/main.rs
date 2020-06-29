mod quoter_config;
mod yahoo;
use clap::{App, Arg};
use colored::*;
use std::io::Error;

fn main() {
    let mut app = App::new("Quoter")
        .about("Stock quotes on the CLI")
        .usage("use the -s option to provide symbols or -cc to create a config file")
        .arg(
            Arg::with_name("symbols")
                .long("symbols")
                .short("s")
                .multiple(true)
                .takes_value(true)
                .required(false)
                .help("space separated list of stock symbols"),
        );

    let matches = app
        .get_matches_from_safe_borrow(::std::env::args_os())
        .unwrap();

    let mut symbols: Vec<&str> = Vec::new();

    if matches.is_present("symbols") {
        let mut cli_symbols = matches.values_of("symbols").unwrap().collect();
        symbols.append(&mut cli_symbols);
    }

    let config_symbols = quoter_config::read_quotes();
    for configured_symbol in config_symbols.iter() {
        symbols.push(&configured_symbol.symbol);
    }

    if symbols.is_empty() {
        app.print_help()
            .expect("well this is awkward - help is not working");
        std::process::exit(1);
    }

    let value: serde_json::Value = request(symbols).expect("failed to call Yahoo finance");

    let mut typed: yahoo::QuoteResponseWrapper =
        serde_json::from_value(value).expect("failed to deserialize to type");

    typed
        .quote_response
        .result
        .sort_unstable_by(|l, r| l.symbol.cmp(&r.symbol));

    for result in typed.quote_response.result.iter() {
        let color = get_change_color(result.regular_market_change_percent);
        println!(
            "{0} {1} {2}",
            format!("{:<5}", result.symbol),
            format!("{:>10}", format!("${:.2}", result.regular_market_price)),
            format!(
                "{:>10}",
                format!("{:.2}%", result.regular_market_change_percent).color(color)
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

fn request(symbols: Vec<&str>) -> Result<serde_json::Value, Error> {
    let response = ureq::get("https://query1.finance.yahoo.com/v7/finance/quote")
        .query("symbols", &symbols.join(","))
        .query("fields", "regularMarketPrice,regularMarketChangePercent")
        .call();

    return response.into_json();
}
