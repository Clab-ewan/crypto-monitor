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
    id: String,
    amount: f64,
    price: f64,
    curr_24h_change: f64,
    value: f64,
}

impl PortfolioRow {
    pub fn build(assets: &Vec<Asset>, api_res: CoinPrices, config: Config) -> Vec<PortfolioRow> {
        let mut portfolio: Vec<PortfolioRow> = Vec::new();
        for a in assets {
            let row: PortfolioRow = PortfolioRow {
                id: a.id.clone(),
                amount: a.amount,
                price: api_res[&a.id][&config.currency],
                curr_24h_change: api_res[&a.id][&format!("{}_24h_change", config.currency)],
                value: a.amount * api_res[&a.id][&config.currency],
            };
            portfolio.push(row);
        }
        portfolio
    }
}
