extern crate reqwest;

use std::collections::HashMap;


#[tokio::main]
async fn main() {
    let result = request(String::from("java")).await;

}

async fn request(qr: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = &format!("https://api.hh.ru/vacancies?{}", qr);
    let res = client.get(url)
        .header("User-Agent", "vsearch")
        .send()
        .await?;

    let body: String = res.text().await?;
    println!("Body:\n{}", body);

    Ok(body)
}