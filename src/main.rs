mod api;
mod config;
mod display;
mod model;
mod watchers;
use api::call;
use crate::config::Config;
use crate::model::PortfolioRow;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let file_path = ("./portfolio.toml").to_string();
    let config = Config::new(file_path);
    let client = reqwest::Client::new();
    let api_result = call(client, &config).await.expect("problem in the api return");
    let portfolio = PortfolioRow::build(&config.assets, &api_result, &config);
    display::render(&portfolio, &config.currency);
}
