use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct PriceData{
    id: String,
    price: u128,
    vol: f64,
}

#[derive(Debug)]
pub struct PortfolioRow {
    symbol: String,
    amount: u8,
    price: u128,
    vol: u8,
    value: u128,
}