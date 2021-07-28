use crate::integration::{fetch_vacancies, Vacancy};
use std::sync::mpsc;
use crate::preloader;

// -w vacancy
pub async fn find_out_vacancies(vacancy: &str) -> (Vec<Vacancy>, i64) {
    let (tx, rx) = mpsc::channel();
    preloader::start(rx);
    let (vacancies, found) = fetch_vacancies(vacancy).await;
    let _result = tx.send(());

    (vacancies, found)
}