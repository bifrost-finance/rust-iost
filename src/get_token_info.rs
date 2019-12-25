use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTokenInfo {
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

async fn get_token_info(domain: &str, symbol: &str, by_longest_chain: bool) -> GetTokenInfo {
    let url = format!("{}/getTokenInfo/{}/{}", domain, symbol, by_longest_chain);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetTokenInfo>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_token_info_should_be_ok() {

        println!("{:#?}",get_token_info("http://api.iost.io","iost",true).await)

    }
}