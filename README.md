# Crypto News Aggregator

Made by SE-2320 BT2

A Rust-based web service that aggregates cryptocurrency news from multiple sources. Users can search by cryptocurrency symbol or name to get the latest news articles.

## Features

- **Multi-source aggregation**: Fetches news from NewsData and CoinMarketCap
- **Pagination support**: Browse through news articles page by page
- **Comprehensive coverage**: Supports 30+ major cryptocurrencies
- **Simple interface**: Clean, responsive web interface
- **Fast backend**: Built with Rust and Axum for high performance

## Tech Stack

**Backend:**
- Rust
- Axum (Web framework)
- Reqwest (HTTP client)
- Tokio (Async runtime)
- Serde (Serialization)

**Frontend:**
- HTML5
- CSS3
- Vanilla JavaScript

**APIs Integrated:**
- NewsData API
- CoinMarketCap API

## Installation

1. Clone the repository:

   git clone https://github.com/Marticat/Crypto_News.git

Edit .env and add your API keys:

NEWS_API=
COINMARKETCAP_API=

## Run the server:

cargo run

## USAGE
Enter a cryptocurrency symbol in the search box
![Снимок экрана 2025-04-05 221347](https://github.com/user-attachments/assets/7d9f0352-aa67-4e0c-95aa-b1855eca1270)

Click "Search" to view news articles


![Снимок экрана 2025-04-05 221226](https://github.com/user-attachments/assets/a794e25b-23a0-4277-a795-c53d59b22c1d)
## PROJECT STRUCTURE

crypto-news-aggregator/
├── src/
│ ├── main.rs # Backend server code
├── static/
│ ├── index.html # Frontend markup
│ ├── style.css # Styles
│ └── app.js # Frontend logic
├── .env # Environment variables
├── Cargo.toml # Rust dependencies
└── README.md # Documentation


License
MIT License - see LICENSE for details
