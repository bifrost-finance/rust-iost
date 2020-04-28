use crate::error::Error;
use crate::frozen_balance::FrozenBalance;
use crate::message::ErrorMessage;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct TokenBalance {
    /// balance
    pub balance: f64,
    /// frozen balances
    pub frozen_balances: Vec<FrozenBalance>
}

async fn get_token_balance(domain: &str, account: &str, token: &str, by_longest_chain: bool) -> Result<TokenBalance, Error> {
    let url = format!("{}/getTokenBalance/{}/{}/{}", domain, account, token, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<TokenBalance>().await.map_err(Error::Reqwest)?;
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
    async fn get_token_balance_should_be_ok() {
        let response = get_token_balance("http://api.iost.io","admin","iost",true).await;
        assert!(response.is_ok());
    }
}




