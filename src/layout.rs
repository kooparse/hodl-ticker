use crypto::Money;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use term::{Attr, color};


pub fn construct(data: Vec<Money>, currency: String) -> Table {
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
            Cell::new(&price)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(&item.percent_change_24h),
            Cell::new(&item.percent_change_1h),
            Cell::new(&cap),
        ]));
    }

    table
}
