use std::collections::HashSet;
use crypto::Money;
use prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use term::{Attr, color};

fn format_percent(percent: &str) -> prettytable::cell::Cell {
    let number: f32 = percent.parse().unwrap();
    if number >= 0.0 {
        return Cell::new(&format!("{}{} %", "+", percent))
            .with_style(Attr::ForegroundColor(color::GREEN));
    }

    Cell::new(&format!("{} %", percent))
        .with_style(Attr::ForegroundColor(color::RED))
}

fn divmod(n: i32, d: i32) -> (i32, i32) {
    (n / d, n % d)
}

fn format_price(nums: &str) -> String {
    let number: f32 = nums.parse().unwrap();
    let (dollars, cents) = divmod((number * 100.) as i32, 100);
    let mut s = String::new();
    for (i ,char) in dollars.to_string().chars().rev().enumerate() {
        if i % 3 == 0 && i != 0 {
            s.insert(0, ',');
        }
        s.insert(0, char);
    }
    format!("${}.{}", s, cents)
}

fn format_bignum(nums: &str) -> String {
    let number: f64 = nums.parse().unwrap();
    let mut number = number as i64;
    let suffixes = ["K", "M", "B", "T"];
    let mut suffix = "";
    if number > 1000 {
        for s in &suffixes {
            suffix = s;
            if number < 1_000_000 { break }
            number /= 1_000;
        }
    }
    format!("{:.3}{}", (number as f64) / 1000., suffix)
}

pub fn construct(data: Vec<Money>, desired: Vec<&str>, currency: String) -> Table {
    let mut des : HashSet<String> = HashSet::new();
    des.extend(desired.into_iter().map(|d| d.to_owned()));

    let mut table = Table::new();

    let headers = [
        "rank",
        "coin",
        &format!("price ({})", currency),
        "change (24h)",
        "change(1h)",
        &format!("market cap ({})", currency),
    ];

    let headers: Vec<Cell> = headers
        .iter()
        .map(|header| {
            Cell::new(&header.to_uppercase()).with_style(Attr::Bold).with_style(
                Attr::ForegroundColor(color::YELLOW),
            )
        })
        .collect();

    table.add_row(Row::new(headers));

    for item in data.iter() {
        if !des.is_empty() && !des.contains(&item.name) {
            continue;
        }

        let price = match item.price_eur {
            Some(ref x) => x,
            None => &item.price_usd,
        };

        let cap = match item.market_cap_eur {
            Some(ref x) => x,
            None => &item.market_cap_usd,
        };

        table.add_row(Row::new(vec![
            Cell::new(&item.rank),
            Cell::new(&item.name),
            Cell::new(&format_price(price))
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
            format_percent(&item.percent_change_24h),
            format_percent(&item.percent_change_1h),
            Cell::new(&format_bignum(cap)),
        ]));
    }

    table
}
