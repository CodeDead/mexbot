use serde::{Serialize, Deserialize};
use serde_json::Number;

#[derive(Serialize, Deserialize)]
pub struct TokensResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub price: Number,
}
