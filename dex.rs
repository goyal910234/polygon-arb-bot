use ethers::providers::{Provider, Http};
use ethers::contract::abigen;
use ethers::types::{U256, Address};
use std::sync::Arc;

abigen!(
    IUniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)
    ]"#
);

pub async fn get_price(
    provider: Arc<Provider<Http>>,
    router: Address,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
) -> anyhow::Result<f64> {
    let router = IUniswapV2Router::new(router, provider);
    let path = vec![token_in, token_out];
    let amounts = router.get_amounts_out(amount_in, path).call().await?;
    let price = amounts[1].as_u128() as f64 / 1e6; // assuming USDC decimals
    Ok(price)
}
