use currency::Currency;
use helper::{format_bignum, format_percent, format_price};

#[derive(Serialize, Deserialize, Clone)]
pub struct Money {
    pub id: String,
    pub name: String,
    pub rank: String,
    pub price_usd: Option<String>,
    pub price_eur: Option<String>,
    pub price_gbp: Option<String>,
    pub price_jpy: Option<String>,
    pub percent_change_1h: String,
    pub percent_change_24h: String,
    pub percent_change_7d: String,
    pub market_cap_usd: Option<String>,
    pub market_cap_eur: Option<String>,
    pub market_cap_gbp: Option<String>,
    pub market_cap_jpy: Option<String>,
}

impl Money {
    pub fn get_price(&self, currency: Currency) -> String {
        let price = match currency.get_name() {
            "eur" => self.price_eur.clone(),
            "gbp" => self.price_gbp.clone(),
            "jpy" => self.price_jpy.clone(),
            _ => self.price_usd.clone(),
        };

        let price = match price {
            Some(ref x) => x,
            None => "",
        };

        format_price(price)
    }

    pub fn get_market_cap(&self, currency: Currency) -> String {
        let market_cap = match currency.get_name() {
            "eur" => self.market_cap_eur.clone(),
            "gbp" => self.market_cap_gbp.clone(),
            "jpy" => self.market_cap_jpy.clone(),
            _ => self.market_cap_usd.clone(),
        };

        let market_cap = match market_cap {
            Some(ref x) => x,
            None => "",
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
