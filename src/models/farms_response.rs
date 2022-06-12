use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize)]
pub struct FarmsResponse {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub farm_type: String,
    pub address: String,
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub price: Number,
    #[serde(rename(serialize = "farmingId", deserialize = "farmingId"))]
    pub farming_id: String,
    #[serde(rename(serialize = "farmingSymbol", deserialize = "farmingSymbol"))]
    pub farming_symbol: String,
    #[serde(rename(serialize = "farmingName", deserialize = "farmingName"))]
    pub farming_name: String,
    #[serde(rename(serialize = "farmingPrice", deserialize = "farmingPrice"))]
    pub farming_price: Number,
    #[serde(rename(serialize = "farmedId", deserialize = "farmedId"))]
    pub farmed_id: String,
    #[serde(rename(serialize = "farmedSymbol", deserialize = "farmedSymbol"))]
    pub farmed_symbol: String,
    #[serde(rename(serialize = "farmedName", deserialize = "farmedName"))]
    pub farmed_name: String,
    #[serde(rename(serialize = "farmedPrice", deserialize = "farmedPrice"))]
    pub farmed_price: Number,
}
