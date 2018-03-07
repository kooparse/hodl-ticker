#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cursive;
extern crate cursive_table_view;
extern crate prettytable;
extern crate reqwest;
extern crate term;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod provider;
mod crypto;
mod layout;
mod helpers;

use cursive::Cursive;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use clap::App;
use std::io::stdout;
use std::io::Write;

use layout::Layout;

const SLEEP_TIMER: u64 = 5000;
// Limit our results to 10 crypto
const ENDPOINT: &str = "https://api.coinmarketcap.com/v1/ticker?limit=50";

fn make_uri(matches: &clap::ArgMatches) -> (String, String) {
    let currency: &str = matches.value_of("convert").unwrap_or("usd");
    (
        format!("{}&convert={}", ENDPOINT, currency),
        String::from(currency),
    )
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let (uri, currency) = make_uri(&matches);

    let mut desired = vec![];
    if let Some(vs) = matches.values_of("desired") {
        desired = vs.collect();
    }

    let data = provider::get(&uri).unwrap_or(vec![]);
    let mut layout = Layout::new(desired.clone(), currency.clone());

    layout.update(data);
    layout.run();

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || loop {
    //     let data = provider::get(&uri).unwrap_or(vec![]);
    //     cb_sink.send(Box::new(|s| {
    //         layout.update(data);
    //     }));
    //     tx.send(data).unwrap();
    //     thread::sleep(Duration::from_millis(SLEEP_TIMER));
    // });

    // loop {
    //     let data = rx.recv().unwrap();
    //     println!("{:?}", data);
    //     layout.update(data);
    // }
}
