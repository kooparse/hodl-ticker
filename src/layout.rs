use std::collections::HashSet;
use crypto::Money;
use cell::LayoutCell;
use currency;
use currency::Currency;

use prettytable::Table;
use prettytable::row::Row;

pub struct Layout<'a> {
    headers: Vec<String>,
    data: Vec<Money>,
    filter_list: HashSet<String>,
    currency: Currency<'a>,
}

impl<'a> Layout<'a> {
    pub fn new(
        data: Vec<Money>,
        filter_list: Vec<&str>,
        currency: currency::Currency<'a>,
    ) -> Layout<'a> {
        let filter_list: HashSet<String> =
            filter_list.into_iter().map(|d| d.to_owned()).collect();

        let headers = [
            "rank",
            "coin",
            &format!("price ({})", currency.get_symbol()),
            "change (24h)",
            "change(1h)",
            &format!("market cap ({})", currency.get_symbol()),
        ].iter()
            .map(|item| item.to_uppercase())
            .collect::<Vec<String>>();

        Layout {
            headers,
            data,
            filter_list,
            currency,
        }
    }

    pub fn print(&self) {
        self.get_table().print_tty(false)
    }

    pub fn get_table(&self) -> Table {
        let mut table = Table::new();
        let mut cell = LayoutCell::new();

        let headers = self.headers
            .iter()
            .map(|header| cell.set(header).bold().yellow().build())
            .collect();

        table.add_row(Row::new(headers));

        for item in &self.data {
            if !&self.filter_list.is_empty()
                && !&self.filter_list.contains(&item.name)
            {
                continue;
            }

            let price = item.get_price(self.currency.clone());
            let market_cap = item.get_market_cap(self.currency.clone());
            let percent_24 = item.get_percent_24();
            let percent_1 = item.get_percent_1();

            table.add_row(Row::new(vec![
                cell.set_and_build(&item.rank),
                cell.set_and_build(&item.name),
                cell.set(&price).bold().blue().build(),
                cell.percent_color(percent_24).build(),
                cell.percent_color(percent_1).build(),
                cell.set_and_build(&market_cap),
            ]));
        }

        table
    }
}
