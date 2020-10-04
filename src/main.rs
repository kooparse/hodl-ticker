mod cell;
mod crypto;
mod currency;
mod helper;
mod layout;
mod provider;

use crate::currency::Currency;
use crate::layout::Layout;
use clap::{load_yaml, App};
use std::error::Error;
use termion;

const DEFAULT_CURRENCY: &str = "usd";
// Limit our results to 10 crypto
const ENDPOINT: &str = "https://api.coingecko.com/api/v3/coins/markets";

fn make_uri(currency: &Currency, limit: &str) -> String {
    format!(
        "{}?per_page={}&vs_currency={}",
        ENDPOINT,
        limit,
        currency.get_name()
    )
}

fn clear_screen() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let limit: &str = matches.value_of("limit").unwrap_or("10");
    let currency =
        Currency::new(matches.value_of("convert").unwrap_or(DEFAULT_CURRENCY));
    let uri = make_uri(&currency, limit);

    let mut filter_list = vec![];
    if let Some(list) = matches.values_of("filter") {
        filter_list = list.collect();
    }

    // Clear terminal screen
    clear_screen();

    let data = provider::get(&uri)?;
    let layout = Layout::new(data, filter_list, currency);
    layout.print();

    Ok(())
}
