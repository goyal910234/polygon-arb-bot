mod config;
mod dex;
mod arbitrage;
mod db;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::load_config("config/config.toml")?;
    db::init().await?;

    loop {
        let opp = arbitrage::check_arbitrage(&cfg).await?;
        if let Some(result) = opp {
            db::log_opportunity(&result).await?;
            println!("Arbitrage Opportunity: {:?}", result);
        }
        sleep(Duration::from_secs(30)).await; // check every 30 sec
    }
}
