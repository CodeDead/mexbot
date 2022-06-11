use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize)]
pub struct EconomicsResponse {
    #[serde(rename(serialize = "totalSupply", deserialize = "totalSupply"))]
    pub total_supply: Number,
    #[serde(rename(serialize = "circulatingSupply", deserialize = "circulatingSupply"))]
    pub circulating_supply: Number,
    pub price: Number,
    #[serde(rename(serialize = "marketCap", deserialize = "marketCap"))]
    pub market_cap: Number,
    pub volume24h: Number,
    #[serde(rename(serialize = "marketPairs", deserialize = "marketPairs"))]
    pub market_pairs: u32,
}

impl EconomicsResponse {
    /// Format an `EconomicsResponse` struct to a `String`
    ///
    /// # Returns
    /// A `String` representation of the `EconomicsResponse` struct
    pub fn to_string(&self) -> String {
        format!("```Total supply: {}\nCirculating supply: {}\nPrice: {}\nMarket cap: {}\n24h volume: {}\nMarket pairs: {}```",
        &self.total_supply,
        &self.circulating_supply,
        &self.price,
        &self.market_cap,
        &self.volume24h,
        &self.market_pairs)
    }
}
