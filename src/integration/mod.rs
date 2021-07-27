use serde_json;
use std::time::Instant;
use serde_json::{Value, Map};

pub struct Vacancy {
    id: String,
    name: String,
    area: String,
    salary: Option<Salary>,
    schedule: String,
    created: Instant,
}

pub struct Salary {
    from: Option<i64>,
    to: Option<i64>,
    currency: String,
    is_gross: bool,
}

pub async fn fetch_vacancies(qr: String) -> Result<Vec<Vacancy>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = &format!("https://api.hh.ru/vacancies?text={}", qr);
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

    let vacancies = extract_vacancies(json);
    Ok(vacancies)
}

fn extract_vacancies(json: serde_json::Value) -> Vec<Vacancy> {
    let items = &json["items"];
    let mut vacancies: Vec<Vacancy> = Vec::new();

    for item in items.as_array().expect("must be array in remote service") {

        let item: &Map<String, Value> = item.as_object().unwrap();

        let name = item.get("name").unwrap();

        let name = match name.as_str() {
            None => String::from("UNKNOWN"),
            Some(name) => String::from(name)
        };

        let id = item.get("id").unwrap();

        let id = match id.as_str() {
            None => String::from("UNKNOWN"),
            Some(id) => String::from(id)
        };

        let created = Instant::now();




        let salary = item.get("salary").unwrap();

        let from: Option<i64> = match salary.get("from") {
            None => None,
            Some(value) => value.as_i64()
        };

        let to: Option<i64> = match salary.get("to") {
            None => None,
            Some(value) => value.as_i64()
        };

        let currency = match salary.get("currency") {
            None => String::from("UNKNOWN"),
            Some(value) => match value.as_str() {
                None => String::from("UNKNOWN"),
                Some(v) => String::from(v),
            }
        };

        let gross = match salary.get("gross") {
            None => false,
            Some(v) => match v.as_bool() {
                Some(gross) => gross,
                None => false
            }
        };

        let _salary: Option<Salary> = if !salary.is_null() {
            Some(Salary {
                from,
                to,
                currency,
                is_gross: gross,
            })
        } else {
            None
        };


    }
    vec![]
}