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