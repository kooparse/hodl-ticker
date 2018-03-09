use helper::{format_bignum, format_percent, format_price};

#[derive(Serialize, Deserialize, Clone)]
pub struct Money {
    pub id: String,
    pub name: String,
    pub rank: String,
    pub price_usd: String,
    pub price_eur: Option<String>,
    pub percent_change_1h: String,
    pub percent_change_24h: String,
    pub percent_change_7d: String,
    pub market_cap_usd: String,
    pub market_cap_eur: Option<String>,
}

impl Money {
    pub fn get_price(&self) -> String {
        let price = match self.price_eur {
            Some(ref x) => x,
            None => &self.price_usd,
        };

        format_price(price)
    }

    pub fn get_market_cap(&self) -> String {
        let market_cap = match self.market_cap_eur {
            Some(ref x) => x,
            None => &self.market_cap_usd,
        };

        format_bignum(market_cap)
    }

    pub fn get_percent_1(&self) -> (String, bool) {
        format_percent(&self.percent_change_1h)
    }

    pub fn get_percent_24(&self) -> (String, bool) {
        format_percent(&self.percent_change_24h)
    }
}
