use reqwest::Client;
use crate::config::Config;

async fn call(client: Client, config: Config) -> () {
    let coin_gecko_url = "https://pro-api.coingecko.com/api/v3/simple/price";
    let header = "x-cg-pro-api-key";
    let api_key = "";
}