use serde::Deserialize;
use crate::model::Asset;
use std::fs;
#[derive(Deserialize, Debug)]
pub struct Config {
    refresh_seconds: u8,
    currency: String,
    assets: Vec<Asset>,
}

impl Config {
    pub fn new(file_path: String) -> Config {
        let file = fs::read_to_string(file_path).expect("Problème dans la lecture du fichier");
        let config: Config = toml::from_str(&file).expect("Problème dans le toml");
        config
    }
}