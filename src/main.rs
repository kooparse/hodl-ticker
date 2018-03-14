#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate prettytable;
extern crate reqwest;
extern crate term;
extern crate termion;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod provider;
mod crypto;
mod layout;
mod cell;
mod helper;

use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use clap::App;
use layout::Layout;

// Limit our results to 10 crypto
const ENDPOINT: &str = "https://api.coinmarketcap.com/v1/ticker";

fn make_uri(matches: &clap::ArgMatches) -> (String, String) {
    let currency: &str = matches.value_of("convert").unwrap_or("usd");
    let limit: &str = matches.value_of("limit").unwrap_or("10");
    (
        format!("{}?limit={}&convert={}", ENDPOINT, limit, currency),
        String::from(currency),
    )
}

fn clear_screen() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

fn erase_screen() {
    print!(
        "{}{}",
        termion::clear::AfterCursor,
        termion::cursor::Goto(1, 1)
    );
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let (uri, currency) = make_uri(&matches);
    let (tx, rx) = mpsc::channel();

    let mut filter_list = vec![];
    if let Some(list) = matches.values_of("filter") {
        filter_list = list.collect();
    }

    // Clear terminal screen
    clear_screen();

    if !matches.is_present("watch") {
        let data = provider::get(&uri.clone()).unwrap_or_else(|_| vec![]);
        let layout = Layout::new(data, filter_list.clone(), &currency);
        return layout.print();
    }

    thread::spawn(move || loop {
        let data = provider::get(&uri.clone()).unwrap_or_else(|_| vec![]);
        tx.send(data).unwrap();
        // Every 5 sec
        thread::sleep(Duration::new(5, 0));
    });

    for data in rx {
        erase_screen();
        let layout = Layout::new(data, filter_list.clone(), &currency);
        layout.print();
    }
}
