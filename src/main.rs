use std::{env};
use std::process::exit;
use crate::storage::storage::Storage;
use crate::storage::FileStorage;
use crate::model::Vacancy;

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
        let (vacancies, found) = match args.get(idx + 1) {
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

        let ok = storage.save(vacancies);
        if !ok {
            exit(1)
        }

        println!("\nvacancies are saved in {}", storage.get_storage_name());
    }
}
