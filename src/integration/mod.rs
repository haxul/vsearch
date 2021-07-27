use serde_json;
use std::time::Instant;
use serde_json::{Value, Map};
use std::future::Future;

pub struct Vacancy {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) area: Option<String>,
    pub(crate) salary: Option<Salary>,
    pub(crate) created: Instant,
}

pub struct Salary {
    pub(crate) from: Option<i64>,
    pub(crate) to: Option<i64>,
    pub(crate) currency: String,
    pub(crate) is_gross: bool,
}

pub async fn fetch_vacancies(qr: &str) -> (Vec<Vacancy>, i64) {
    let response = fetch_vacancies_by_page(qr, 0).await;
    let mut vacancies: Vec<Vacancy> = Vec::new();
    if let Ok((mut vc, pages, mut page, found)) = response {
        vacancies.append(&mut vc);
        let mut futures = Vec::new();
        while page < pages - 1 {
            page = page + 1;
            let future = fetch_vacancies_by_page(qr, page);
            futures.push(future);
        }

        for future in futures {
            let result = future.await;
            if let Ok((mut vc, ..)) = result {
                vacancies.append(&mut vc);
            }
        }
        return (vacancies, found);
    };
    (vacancies, 0)
}

async fn fetch_vacancies_by_page(qr: &str, page: i64) -> Result<(Vec<Vacancy>, i64, i64, i64), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = &format!("https://api.hh.ru/vacancies?text={}&area=1&per_page=100&page={}", qr, page);
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

    let pages = match &json["pages"].as_i64() {
        None => 0,
        Some(a) => *a
    };
    let page = match &json["page"].as_i64() {
        None => 0,
        Some(a) => *a
    };

    let found = match &json["found"].as_i64() {
        None => 0,
        Some(a) => *a
    };

    let vacancies = extract_vacancies(json);
    Ok((vacancies, pages, page, found))
}

fn extract_vacancies(json: serde_json::Value) -> Vec<Vacancy> {
    let items = &json["items"];
    let mut vacancies: Vec<Vacancy> = Vec::new();

    for item in items.as_array().expect("must be array in remote service") {
        let item: &Map<String, Value> = item.as_object().unwrap();

        let name = item.get("name").unwrap();

        let area = item.get("area").unwrap();

        let area = match area.get("name") {
            None => None,
            Some(v) => match v.as_str() {
                None => None,
                Some(v) => Some(String::from(v))
            }
        };

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

        let salary: Option<Salary> = if !salary.is_null() {
            Some(Salary {
                from,
                to,
                currency,
                is_gross: gross,
            })
        } else {
            None
        };


        let vacancy = Vacancy {
            id,
            name,
            area,
            salary,
            created,
        };

        vacancies.push(vacancy);
    }

    vacancies
}