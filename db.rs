use sqlx::{Sqlite, Pool};

static DB_URL: &str = "sqlite://arb_bot.db";

pub async fn init() -> anyhow::Result<()> {
    let pool = Pool::<Sqlite>::connect(DB_URL).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            buy_dex TEXT,
            sell_dex TEXT,
            buy_price REAL,
            sell_price REAL,
            profit REAL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await?;
    Ok(())
}

pub async fn log_opportunity(result: &crate::arbitrage::ArbResult) -> anyhow::Result<()> {
    let pool = Pool::<Sqlite>::connect(DB_URL).await?;
    sqlx::query(
        "INSERT INTO opportunities (buy_dex, sell_dex, buy_price, sell_price, profit)
         VALUES (?1, ?2, ?3, ?4, ?5)"
    )
    .bind(&result.buy_dex)
    .bind(&result.sell_dex)
    .bind(result.buy_price)
    .bind(result.sell_price)
    .bind(result.profit)
    .execute(&pool).await?;
    Ok(())
}
