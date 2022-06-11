use reqwest::Error;

use crate::models::{economics_response::EconomicsResponse, tokens_response::TokensResponse};

pub struct ElrondService {
    pub economics: String,
    pub tokens: String,
}

impl ElrondService {
    /// Create a new `ElrondService`
    ///
    /// # Arguments
    ///
    /// * economics - The API endpoint to retrieve MEX economics
    /// * tokens - The API endpoint to retrieve token information
    ///
    /// # Returns
    ///
    /// A new `ElrondService` struct
    pub fn new(economics: &str, tokens: &str) -> Self {
        Self {
            economics: String::from(economics),
            tokens: String::from(tokens),
        }
    }

    /// Get economics information
    ///
    /// # Returns
    ///
    /// An `EconomicsResponse` struct or an `Error` in case the API call was unsuccessful
    pub async fn get_economics(&self) -> Result<EconomicsResponse, Error> {
        match reqwest::get(&self.economics).await {
            Ok(d) => match d.json().await {
                Ok(d) => Ok(d),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }

    /// Get tokens information
    ///
    /// # Returns
    ///
    /// A `Vec` of `TokensResponse` structs or an `Error` in case the API call was unsuccessful
    pub async fn get_tokens(&self) -> Result<Vec<TokensResponse>, Error> {
        match reqwest::get(&self.tokens).await {
            Ok(d) => match d.json().await {
                Ok(d) => Ok(d),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
}
