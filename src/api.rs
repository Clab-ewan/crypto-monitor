use crate::config::Config;
use reqwest::Client;
use std::collections::HashMap;
use std::env;

// CoinGecko /simple/price returns: { "bitcoin": { "usd": 30000.0, "usd_24h_change": 2.5 }, ... }
pub type CoinPrices = HashMap<String, HashMap<String, f64>>;

pub async fn call(c: Client, config: &Config) -> Result<CoinPrices, Box<dyn std::error::Error>> {
    let currency = &config.currency;
    let ids = config
        .assets
        .iter()
        .map(|a| a.id.as_str())
        .collect::<Vec<_>>()
        .join(",");

    let api_key = env::var("COIN_GECKO_KEY")?;
    let coin_gecko_url = format!(
        "https://api.coingecko.com/api/v3/simple/price?vs_currencies={currency}&ids={ids}&include_24hr_change=true"
    );
    let result = c
        .get(coin_gecko_url)
        .header("x-cg-demo-api-key", &api_key)
        .header("User-Agent", "crypto-monitor/1.0")
        .send()
        .await?
        .json::<CoinPrices>()
        .await?;

    Ok(result)
}
