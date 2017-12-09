#[derive(Serialize, Deserialize)]
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
