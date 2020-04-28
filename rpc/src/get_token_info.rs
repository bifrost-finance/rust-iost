use crate::error::Error;
use crate::message::ErrorMessage;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct TokenInfo {
    /// token symbol
    pub symbol: String,
    /// token full name
    pub full_name: String,
    /// token issuer
    pub issuer: String,
    /// total amount of token supply, is the result of total_supply_float multiplied by decimal
    pub total_supply: String,
    /// current amount of token supply, is the result of current_supply_float multiplied by decimal
    pub current_supply: String,
    /// total amount of token supply
    pub total_supply_float: f64,
    /// current amount of token supply
    pub current_supply_float: f64,
    /// token decimal
    pub decimal: i32,
    /// whether the token can be transfered
    pub can_transfer: bool,
    /// whether the token can only be transfered by issuer
    pub only_issuer_can_transfer: bool
}

async fn get_token_info(domain: &str, symbol: &str, by_longest_chain: bool) -> Result<TokenInfo, Error> {
    let url = format!("{}/getTokenInfo/{}/{}", domain, symbol, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<TokenInfo>().await.map_err(Error::Reqwest)?;
        Ok(rsp)
    } else {
        let rsp = req.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(rsp))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_token_info_should_be_ok() {
        let response = get_token_info("http://api.iost.io","iost",true).await;
        assert!(response.is_ok());
    }
}