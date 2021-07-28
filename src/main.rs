use crate::integration::Vacancy;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::mpsc::TryRecvError;

mod integration;
mod preloader;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    preloader::start(rx);
    let (vacancies, found) = integration::fetch_vacancies("java").await;
    tx.send(());

    for (i, a) in vacancies.iter().enumerate() {
        println!("{}. {}", i, a.name);
    }

    println!("found {}", found);
}
