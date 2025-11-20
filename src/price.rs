use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use solana_client::client_error::reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct PriceInfo {
    #[serde(rename = "usdPrice")]
    pub usd_price: f64,
    #[serde(rename = "blockId")]
    pub block_id: i64,
    pub decimals: i64,
    #[serde(rename = "priceChange24h")]
    pub price_change24h: f64,
}

pub async fn get_price() -> anyhow::Result<(f64, f64)> {
    let url = "https://lite-api.jup.ag/price/v3?ids=So11111111111111111111111111111111111111112,oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp";
    
    let resp = match reqwest::get(url).await {
        Ok(response) => {
            if !response.status().is_success() {
                log::warn!("Price API returned status: {}", response.status());
                // Return default prices if API fails
                return Ok((0.01, 100.0)); // Default: ORE $0.01, SOL $100
            }
            match response.text().await {
                Ok(text) => {
                    if text.is_empty() {
                        log::warn!("Price API returned empty response");
                        return Ok((0.01, 100.0));
                    }
                    text
                }
                Err(e) => {
                    log::warn!("Failed to read price API response: {:?}", e);
                    return Ok((0.01, 100.0));
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to fetch prices from API: {:?}, using defaults", e);
            return Ok((0.01, 100.0)); // Default prices
        }
    };
    
    let prices: HashMap<String, PriceInfo> = match serde_json::from_str(&resp) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("Failed to parse price JSON: {:?}, response: {}", e, resp.chars().take(200).collect::<String>());
            return Ok((0.01, 100.0)); // Return default prices on parse error
        }
    };
    
    let ore_price = prices.get("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp");
    let sol_price = prices.get("So11111111111111111111111111111111111111112");
    if let (Some(ore), Some(sol)) = (ore_price, sol_price) {
        return Ok((ore.usd_price, sol.usd_price));
    }
    
    log::warn!("Price data missing for ORE or SOL, using defaults");
    Ok((0.01, 100.0)) // Default prices if data is missing
}
