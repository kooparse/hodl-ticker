use reqwest;
use serde_json;
use crypto::Money;
use std::error;

pub fn get(uri: &str) -> Result<Vec<Money>, Box<error::Error>> {
    let mut resp = reqwest::get(uri)?;
    let body = &resp.text()?;
    let data: Vec<Money> = serde_json::from_str(&body)?;
    Ok(data)
}
