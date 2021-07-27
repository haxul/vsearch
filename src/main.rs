mod integration;


#[tokio::main]
async fn main() {

    let _result = integration::fetch_vacancies(String::from("java")).await;
}
