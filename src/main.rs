use crate::integration::Vacancy;
use tokio::task::block_in_place;

mod integration;


#[tokio::main]
async fn main() {
    let (vacancies, found) = integration::fetch_vacancies("java").await;
    for (i, a) in vacancies.iter().enumerate() {
        println!("{}. {}", i, a.name);
    }

    println!("found {}", found);
}
