use reqwest::Client;
use std::env;
use crate::config::Config;
use dotenv::dotenv;

pub async fn call(c: Client, config: Config) -> Result<serde_json::Value, reqwest::Error>
{
    let currency = config.currency;
    let ids = config.assets.iter().map(|a| a.id.as_str()).collect::<Vec<_>>().join(",");
    let names = config.assets.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(",");
    let symbols = config.assets.iter().map(|a| a.symbol.as_str()).collect::<Vec<_>>().join(",");
    
    dotenv().ok();
    let api_key = env::var("COIN_GECKO_KEY").unwrap();
    let coin_gecko_url = format!("https://api.coingecko.com/api/v3/simple/price?vs_currencies={currency}&ids={ids}&names={names}&symbols={symbols}");
    let response: serde_json::Value = c
        .get(coin_gecko_url)
        .header("x-cg-demo-api-key", &api_key)
        .header("User-Agent", "crypto-monitor/1.0")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}