use serde::{Deserialize, Serialize};

use crate::helper::{format_bignum, format_percent};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Money {
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "market_cap_rank"))]
    pub rank: i32,
    pub current_price: f32,
    pub price_change_percentage_24h: f32,
    pub market_cap: f32,
}

impl Money {
    pub fn get_market_cap(&self) -> String {
        format_bignum(self.market_cap)
    }

    pub fn get_percent_24(&self) -> (String, bool) {
        format_percent(self.price_change_percentage_24h)
    }
}
