use crypto::Money;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use term::{Attr, color};


pub fn construct(data: Vec<Money>, currency: String) -> Table {
    let mut table = Table::new();

    let headers = [
        "Rank",
        "Coin",
        "Price (USD)",
        "Change (24H)",
        "Change(1H)",
        "Market Cap (USD)",
    ];

    let headers: Vec<Cell> = headers
        .iter()
        .map(|header| {
            Cell::new(&header).with_style(Attr::Bold).with_style(
                Attr::ForegroundColor(color::YELLOW),
            )
        })
        .collect();

    table.add_row(Row::new(headers));

    for item in data.iter() {
        table.add_row(Row::new(vec![
            Cell::new(&item.rank),
            Cell::new(&item.name),
            Cell::new(&item.price_usd)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(&item.percent_change_24h),
            Cell::new(&item.percent_change_1h),
            Cell::new(&item.market_cap_usd),
        ]));
    }

    table
}
