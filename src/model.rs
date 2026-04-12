use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    id: String,
    symbol: String,
    amount: f64,
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