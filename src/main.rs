extern crate reqwest;

use serde_json;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let result = request(String::from("java")).await;
}

async fn request(qr: String) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = &format!("https://api.hh.ru/vacancies?{}", qr);
    let res = client.get(url)
        .header("User-Agent", "vsearch")
        .send()
        .await?;

    let body = res.text().await;
    let json: serde_json::Value = match body {
        Ok(b) => serde_json::from_str(&b).unwrap(),
        Err(e) => return Err(e)
    };

    println!("Body:\n{}", json);
    Ok(json)
}