use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub min_profit_threshold: f64,
    pub trade_amount: f64,
    pub dexes: Dexes,
    pub tokens: Tokens,
}

#[derive(Debug, Deserialize)]
pub struct Dexes {
    pub dex1_router: String,
    pub dex2_router: String,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub weth: String,
    pub usdc: String,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let data = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&data)?;
    Ok(cfg)
}
