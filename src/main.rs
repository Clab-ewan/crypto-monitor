mod api;
mod config;
mod model;
mod display;
mod watchers;
use crate::config::{Config};

fn main() {
    let file_path = ("./portfolio.toml").to_string();
    let config = Config::new(file_path);
    // println!("{:?}", config);
}
