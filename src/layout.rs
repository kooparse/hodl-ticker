use std::collections::HashSet;
use helpers;
use crypto::Money;
use provider;

use cursive_table_view;
use std::cmp::Ordering;
use cursive::Cursive;
use cursive::traits::*;
use cursive::align::HAlign;
use cursive::views::{Dialog, TextView, TextContent};
use cursive_table_view::{TableView, TableViewItem};

type MoneyTable = TableView<Money, Column>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Column {
    Rank,
    Coin,
    Price,
    Change24,
    Change1,
    MarketCap,
}

impl TableViewItem<Column> for Money {
    fn to_column(&self, column: Column) -> String {
        let price = self.get_price(true);
        let market_cap = self.get_mark_cap(true);
        let percent_24 = self.get_percent_24();
        let percent_1 = self.get_percent_1();

        let col = match column {
            Column::Rank => &self.rank,
            Column::Coin => &self.name,
            Column::Price => &price,
            Column::Change24 => &percent_24,
            Column::Change1 => &percent_1,
            Column::MarketCap => &market_cap,
        };
        col.to_string()
    }

    fn cmp(&self, other: &Self, column: Column) -> Ordering
    where
        Self: Sized,
    {
        match column {
            Column::Rank => {
                let rank = self.rank.parse::<i32>().unwrap();
                return rank.cmp(&other.rank.parse().unwrap());
            }
            Column::Price => self.get_price(false).cmp(&other.get_price(false)),
            _ => self.rank.cmp(&other.rank),
        }
    }
}

fn format_col(title: &str) -> String {
    format!("{}", title.to_uppercase())
}


pub fn create_table(data: Vec<Money>) -> MoneyTable {
    TableView::<Money, Column>::new()
        .column(Column::Rank, format_col("Rank"), |c| {
            c.align(HAlign::Left).width_percent(5)
        })
        .column(Column::Price, format!("Price").to_uppercase(), |c| {
            c.align(HAlign::Left).width_percent(20)
        })
        .column(Column::Change24, format_col("Change 24h"), |c| {
            c.align(HAlign::Right).width_percent(15)
        })
        .column(Column::Change1, format_col("Change 1h"), |c| {
            c.align(HAlign::Right).width_percent(15)
        })
        .column(Column::MarketCap, format_col("Market Cap"), |c| {
            c.align(HAlign::Right)
        })
        .items(data)
}

pub struct Layout {
    pub siv: Cursive,
    pub currency: String,
    pub content: TextContent,
}


impl Layout {
    pub fn new(desired: Vec<&str>, currency: String) -> Layout {
        let mut siv = Cursive::new();


        let content = TextContent::new("hey");
        let mut layout = Layout { siv, currency, content };

        layout.siv.set_fps(30);
        layout.siv.add_global_callback('q', |s| s.quit());
        layout.siv.add_layer(TextView::new_with_content(content.clone()));

        return layout;
    }

    // pub fn fill_table(&self, data: Vec<Money>) -> MoneyTable {
    //     create_table(data)
    // }

    pub fn run(&mut self) {
        self.siv.run();
    }

    pub fn update(&mut self, data: Vec<Money>) {
        // self.content = create_table(data);

        // self.siv
        //     .call_on_id("main_layout", move |view: &mut Dialog| {
        //         view.set_content(self.content.clone());
        //     });
    }
}
