use std::{env};
use crate::storage::storage::Storage;
use crate::storage::FileStorage;
use crate::model::Vacancy;
use chrono::Utc;

mod integration;
mod preloader;
mod commands;
mod storage;
mod model;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    let storage = FileStorage::new(String::from("data.txt"));

    if args.len() <= 1 {
        println!("unknown command");
    }

    let save_result = args.contains(&String::from("s"));

    if let Some(idx) = args.iter().position(|s| s.eq("w")) {
        let vacancy_to_find = args.get(idx + 1);
        let (_vacancies, found) = match vacancy_to_find {
            Some(s) => commands::find_out_vacancies(s).await,
            None => {
                println!("enter vacancy text");
                let empty: Vec<Vacancy> = Vec::new();
                (empty, 0 as i64)
            }
        };

        if !save_result {
            println!("found {} vacancies", found);
            return;
        }

        if let Some(v) = vacancy_to_find {
            let cur_date = Utc::now().date();
            let row = format!("date: {}, vacancy: {}, found: {}", cur_date, v, found);
            storage.save(row);

            println!("\nvacancies are saved in {}", storage.get_storage_name());
        }
    }
}
