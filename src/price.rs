use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use solana_client::client_error::reqwest;
use once_cell::sync::Lazy;

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

struct PriceCache {
    ore_price: f64,
    sol_price: f64,
    last_fetch: Instant,
}

static PRICE_CACHE: Lazy<Mutex<Option<PriceCache>>> = Lazy::new(|| Mutex::new(None));
const CACHE_DURATION: Duration = Duration::from_secs(30); // Cache prices for 30 seconds

pub async fn get_price() -> anyhow::Result<(f64, f64)> {
    // Check cache first
    {
        let cache_guard = PRICE_CACHE.lock().unwrap();
        if let Some(cache) = cache_guard.as_ref() {
            if cache.last_fetch.elapsed() < CACHE_DURATION {
                return Ok((cache.ore_price, cache.sol_price));
            }
        }
    }
    
    // Fetch new prices
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
        let result = (ore.usd_price, sol.usd_price);
        
        // Update cache
        {
            let mut cache_guard = PRICE_CACHE.lock().unwrap();
            *cache_guard = Some(PriceCache {
                ore_price: ore.usd_price,
                sol_price: sol.usd_price,
                last_fetch: Instant::now(),
            });
        }
        
        return Ok(result);
    }
    
    log::warn!("Price data missing for ORE or SOL, using defaults");
    let defaults = (0.01, 100.0);
    
    // Cache defaults too (but with shorter duration)
    {
        let mut cache_guard = PRICE_CACHE.lock().unwrap();
        *cache_guard = Some(PriceCache {
            ore_price: defaults.0,
            sol_price: defaults.1,
            last_fetch: Instant::now(),
        });
    }
    
    Ok(defaults) // Default prices if data is missing
}
