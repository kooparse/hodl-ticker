extern crate term;
extern crate reqwest;
extern crate prettytable;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod provider;
mod crypto;
mod layout;

use clap::App;

// Limit our results to 10 crypto
const ENDPOINT: &str = "https://api.coinmarketcap.com/v1/ticker?limit=10";

fn make_uri(matches: clap::ArgMatches) -> (String, String) {
    let currency: &str = matches.value_of("convert").unwrap_or("usd");
    (format!("{}&convert={}", ENDPOINT, currency), String::from(currency))
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let (uri, currency) = make_uri(matches);

    let data = provider::get(&uri).unwrap_or(vec![]);
    let l = layout::construct(data, currency);
    l.printstd();
}
