CREATE TABLE opportunities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    buy_dex TEXT,
    sell_dex TEXT,
    buy_price REAL,
    sell_price REAL,
    profit REAL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);
