#[derive(Clone)]
pub struct Currency<'a> {
    currency: &'a str,
    symbol: &'a str,
}

impl<'a> Currency<'a> {
    pub fn new(currency: &str) -> Currency {
        match currency {
            "eur" => Currency {
                currency,
                symbol: "€",
            },
            "gbp" => Currency {
                currency,
                symbol: "£",
            },
            "jpy" => Currency {
                currency,
                symbol: "¥",
            },
            "cny" => Currency {
                currency,
                symbol: "元",
            },
            _ => Currency {
                currency,
                symbol: "$",
            },
        }
    }

    pub fn get_name(&self) -> &str {
        self.currency
    }

    pub fn get_symbol(&self) -> &str {
        self.symbol
    }
}
