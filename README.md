# 🔗 Polygon Arbitrage Opportunity Detector Bot

## 📌 Overview
A Rust bot that detects arbitrage opportunities between two DEXes (QuickSwap & SushiSwap) on the Polygon blockchain.  
It compares token prices (e.g., WETH/USDC), simulates trade profit, and logs opportunities to SQLite.

---

## 🚀 Tech Stack
- **Rust** + Tokio (async runtime)
- **ethers-rs** (Polygon RPC & smart contract calls)
- **SQLx + SQLite** (database logging)
- **TOML config**

---

## ⚙️ Setup Instructions

### 1. Clone Repo
```bash
git clone https://github.com/YOUR_USERNAME/polygon-arb-bot.git
cd polygon-arb-bot
