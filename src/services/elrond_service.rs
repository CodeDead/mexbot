use reqwest::Error;

use crate::models::{
    economics_response::EconomicsResponse, farms_response::FarmsResponse,
    tokens_response::TokensResponse,
};

pub struct ElrondService {
    pub economics: String,
    pub tokens: String,
    pub farms: String,
}

impl ElrondService {
    /// Create a new `ElrondService`
    ///
    /// # Arguments
    ///
    /// * `economics` - The API endpoint to retrieve MEX economics
    /// * `tokens` - The API endpoint to retrieve token information
    /// * `farms` - The API endpoint to retrieve farm information
    ///
    /// # Returns
    ///
    /// A new `ElrondService` struct
    pub fn new(economics: &str, tokens: &str, farms: &str) -> Self {
        Self {
            economics: String::from(economics),
            tokens: String::from(tokens),
            farms: String::from(farms),
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

    /// Get the farms information
    ///
    /// # Returns
    /// A `Vec` of `FarmsResponse` structs or an `Error` in case the API call was unsuccessful
    pub async fn get_farms(&self) -> Result<Vec<FarmsResponse>, Error> {
        match reqwest::get(&self.farms).await {
            Ok(d) => match d.json().await {
                Ok(d) => Ok(d),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
}
