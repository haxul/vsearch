use crate::integration::{Vacancy};
use std::{env};
use std::fs;
use std::io::Write;

mod integration;
mod preloader;
mod commands;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("unknown command");
    }

    let _save_result = args.contains(&String::from("-s"));

    if let Some(idx) = args.iter().position(|s| s.eq("w")) {
        let (vacancies, found) = match args.get(idx + 1) {
            Some(s) => commands::find_out_vacancies(s).await,
            None => {
                println!("enter vacancy text");
                let empty: Vec<Vacancy> = Vec::new();
                (empty, 0 as i64)
            }
        };

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("/home/haxul/Development/workspaces/rust/vsearch/data.txt")
            .unwrap();

        let strings: Vec<String> = vacancies.iter().map(|v| format!("{}", v)).collect();
        for s in strings {
            let _result = writeln!(file, "{}", s);
        }
    }
}
