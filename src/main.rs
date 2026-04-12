mod api;
mod config;
mod display;
mod model;
mod watchers;
use api::call;
use crate::config::Config;

#[tokio::main]
async fn main() {
    let file_path = ("./portfolio.toml").to_string();
    let config = Config::new(file_path);
    let client = reqwest::Client::new();
    let result = call(client, config).await.expect("problem in the api return");
    println!("{:?}", result);
}
