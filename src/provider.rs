use crate::crypto::Money;
use reqwest;
use serde_json;
use std::error::Error;

pub fn get(uri: &str) -> Result<Vec<Money>, Box<dyn Error>> {
    let resp = reqwest::blocking::get(uri)?;
    let body = &resp.text()?;
    let data: Vec<Money> = serde_json::from_str(body)?;
    Ok(data)
}
