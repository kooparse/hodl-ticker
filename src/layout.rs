use crate::cell::LayoutCell;
use crate::crypto::Money;
use crate::currency;
use std::collections::HashSet;

use prettytable::{Row, Table};

pub struct Layout {
    headers: Vec<String>,
    data: Vec<Money>,
    filter_list: HashSet<String>,
}

impl Layout {
    pub fn new(
        data: Vec<Money>,
        filter_list: Vec<&str>,
        currency: currency::Currency,
    ) -> Layout {
        let filter_list: HashSet<String> =
            filter_list.into_iter().map(|d| d.to_owned()).collect();

        let headers = [
            "rank",
            "coin",
            &format!("price ({})", currency.get_symbol()),
            "change (24h)",
            &format!("market cap ({})", currency.get_symbol()),
        ]
        .iter()
        .map(|item| item.to_uppercase())
        .collect::<Vec<String>>();

        Layout {
            headers,
            data,
            filter_list,
        }
    }

    pub fn print(&self) {
        self.get_table().print_tty(false);
    }

    pub fn get_table(&self) -> Table {
        let mut table = Table::new();
        let mut cell = LayoutCell::new();

        let headers = self
            .headers
            .iter()
            .map(|header| cell.set(header).bold().yellow().build())
            .collect();

        table.add_row(Row::new(headers));

        self.data
            .iter()
            .filter(|item| {
                !(!self.filter_list.is_empty()
                    && !self.filter_list.contains(&item.name))
            })
            .for_each(|item| {
                let rank: String = format!("{}", item.rank);
                let price: String = format!("{:.2}", item.current_price);
                let market_cap: String = item.get_market_cap();
                let percent_24 = item.get_percent_24();

                table.add_row(Row::new(vec![
                    cell.set_and_build(&rank),
                    cell.set_and_build(&item.name),
                    cell.set(&price).bold().blue().build(),
                    cell.percent_color(percent_24).build(),
                    cell.set_and_build(&market_cap),
                ]));
            });

        table
    }
}
