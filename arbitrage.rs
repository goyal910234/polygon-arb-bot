use crate::dex;
use crate::config::Config;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArbResult {
    pub buy_dex: String,
    pub sell_dex: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub profit: f64,
}

pub async fn check_arbitrage(cfg: &Config) -> anyhow::Result<Option<ArbResult>> {
    let provider = Arc::new(Provider::<Http>::try_from(cfg.rpc_url.clone())?);

    let weth: Address = cfg.tokens.weth.parse()?;
    let usdc: Address = cfg.tokens.usdc.parse()?;
    let amount_in = U256::exp10(18); // 1 WETH

    let price_dex1 = dex::get_price(provider.clone(), cfg.dexes.dex1_router.parse()?, weth, usdc, amount_in).await?;
    let price_dex2 = dex::get_price(provider.clone(), cfg.dexes.dex2_router.parse()?, weth, usdc, amount_in).await?;

    let gas_cost = 1.0; // simplified gas cost in USDC
    let profit1 = (price_dex2 - price_dex1) * cfg.trade_amount - gas_cost;
    let profit2 = (price_dex1 - price_dex2) * cfg.trade_amount - gas_cost;

    if profit1 > cfg.min_profit_threshold {
        return Ok(Some(ArbResult {
            buy_dex: "DEX1".to_string(),
            sell_dex: "DEX2".to_string(),
            buy_price: price_dex1,
            sell_price: price_dex2,
            profit: profit1,
        }));
    } else if profit2 > cfg.min_profit_threshold {
        return Ok(Some(ArbResult {
            buy_dex: "DEX2".to_string(),
            sell_dex: "DEX1".to_string(),
            buy_price: price_dex2,
            sell_price: price_dex1,
            profit: profit2,
        }));
    }

    Ok(None)
}
