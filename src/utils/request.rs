mod utils;

use reqwest::{Client, Result};
use serde::Serialize;

pub async fn send_post<T: Serialize>(url: &str, data: T) -> Result<String> {
    let client = Client::new();
    let response = client.post(url).json(&data).send().await?;
    Ok(response.text().await?)
}

pub async fn send_get(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    Ok(response.text().await?)
}
