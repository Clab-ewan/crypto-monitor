use crate::api::CoinPrices;
use crate::config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub amount: f64,
}

#[derive(Debug)]
pub struct PortfolioRow {
    pub id: String,
    pub amount: f64,
    pub price: f64,
    pub curr_24h_change: f64,
    pub value: f64,
}

impl PortfolioRow {
    pub fn build(assets: &Vec<Asset>, api_result: &CoinPrices, config: &Config) -> Vec<PortfolioRow> {
        let mut portfolio: Vec<PortfolioRow> = Vec::new();
        for a in assets {
            let row: PortfolioRow = PortfolioRow {
                id: a.id.clone(),
                amount: a.amount,
                price: api_result[&a.id][&config.currency],
                curr_24h_change: api_result[&a.id][&format!("{}_24h_change", config.currency)],
                value: a.amount * api_result[&a.id][&config.currency],
            };
            portfolio.push(row);
        }
        portfolio
    }
}
