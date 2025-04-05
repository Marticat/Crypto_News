
use axum::{extract::Query, response::{Html, Json}, routing::get, Router};
use dotenv::dotenv;
use reqwest as http_client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoNewsItem {
    pub headline: String,
    pub article_url: String,
    pub origin: String,
    pub published_at: String,
}

#[derive(Deserialize)]
struct SearchParams {
    crypto_query: String,
}

fn map_symbol_to_name(symbol: &str) -> &str {
    let normalized = symbol.to_uppercase();
    match normalized.as_str() {
        "BTC" => "Bitcoin",
        "ETH" => "Ethereum",
        "AVAX" => "Avalanche",
        "DOT" => "Polkadot",
        "LTC" => "Litecoin",
        "LINK" => "Chainlink",
        "DOGE" => "Dogecoin",
        "UNI" => "Uniswap",
        "SHIB" => "Shiba Inu",
        "TRX" => "TRON",
        "USDT" => "Tether",
        "USDC" => "USD Coin",
        "BNB" => "Binance Coin",
        "VET" => "VeChain",
        "XTZ" => "Tezos",
        "ALGO" => "Algorand",
        "FIL" => "Filecoin",
        "MATIC" => "Polygon",
        "ICP" => "Internet Computer",
        "XLM" => "Stellar",
        "NEAR" => "NEAR Protocol",
        "ETC" => "Ethereum Classic",
        "FTM" => "Fantom",
        "CRO" => "Cronos",
        "RUB" => "Russian Ruble",
        "KZT" => "Kazakhstani Tenge",
        "USD" => "US Dollar",
        "EUR" => "Euro",
        _ => symbol,
    }
}

async fn get_newsdata_articles(query: &str) -> Vec<CryptoNewsItem> {
    let api_key = match env::var("NEWS_API") {
        Ok(key) => key,
        Err(_) => return Vec::new(),
    };
    
    let search_query = map_symbol_to_name(query).to_lowercase();
    let endpoint = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en",
        api_key, search_query
    );

    let response = match http_client::get(&endpoint).await {
        Ok(res) => res,
        Err(_) => return Vec::new(),
    };

    let data: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(_) => return Vec::new(),
    };

    let mut articles = Vec::new();
    
    if let Some(results) = data["results"].as_array() {
        for entry in results.iter().take(11) {
            articles.push(CryptoNewsItem {
                headline: entry["title"].as_str().unwrap_or_default().to_string(),
                article_url: entry["link"].as_str().unwrap_or_default().to_string(),
                origin: entry["source_id"]
                    .as_str()
                    .unwrap_or("NewsData")
                    .to_string(),
                published_at: entry["pubDate"].as_str().unwrap_or_default().to_string(),
            });
        }
    }
    
    articles
}

async fn get_cmc_data(query: &str) -> Vec<CryptoNewsItem> {
    let api_key = match env::var("COINMARKETCAP_API") {
        Ok(key) => key,
        Err(_) => return Vec::new(),
    };

    let client = http_client::Client::new();
    let response = match client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/info")
        .query(&[("symbol", query)])
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

    let json_data: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(_) => return Vec::new(),
    };

    let mut cmc_articles = Vec::new();

    if let Some(crypto_data) = json_data["data"][query].as_object() {
        if let Some(website) = crypto_data["urls"]["website"][0].as_str() {
            cmc_articles.push(CryptoNewsItem {
                headline: format!("{} Overview", query),
                article_url: website.to_string(),
                origin: "CoinMarketCap".into(),
                published_at: chrono::Utc::now().to_rfc3339(),
            });
        }
    }

    cmc_articles
}

async fn handle_news_request(Query(params): Query<SearchParams>) -> Json<Vec<CryptoNewsItem>> {
    let query_term = params.crypto_query.clone();
    
    let (news_data, cmc_data) = tokio::join!(
        get_newsdata_articles(&query_term),
        get_cmc_data(&query_term)
    );

    let mut combined_results = news_data;
    combined_results.extend(cmc_data);
    
    Json(combined_results)
}

#[tokio::main]
async fn start_server() {
    dotenv().ok();
    
    let app_router = Router::new()
        .route("/news", get(handle_news_request))
        .route("/", get(|| async { Html(include_str!("../static/index.html")) }))
        .route("/style.css", get(|| async {
            ([("Content-Type", "text/css")], include_str!("../static/style.css"))
        }))
        .route("/app.js", get(|| async {
            Html(include_str!("../static/script.js"))
        }));

    let server_address = "127.0.0.1:8000".parse().unwrap();
    println!("Less go http://{}", server_address);
    
    axum::Server::bind(&server_address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}

fn main() {
    start_server();
}
