use serde_json;

#[tokio::main]
async fn main() {
    let _result = request(String::from("java")).await;
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
        Ok(body) => serde_json::from_str(&body)
            .expect("some gets wrong during json parsing"),
        Err(e) => return Err(e)
    };

    println!("Body:\n{}", json);
    Ok(json)
}