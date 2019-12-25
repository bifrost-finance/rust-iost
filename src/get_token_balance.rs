use serde::{Serialize, Deserialize};
use crate::frozen_balance::FrozenBalance;

#[derive(Serialize ,Deserialize, Debug)]
pub struct GetTokenBalance {

    pub balance: f64,

    pub frozen_balances: Vec<FrozenBalance>
}

async fn get_token_balance(domain: &str, account: &str, token: &str, by_longest_chain: bool) -> GetTokenBalance {
    let url = format!("{}/getTokenBalance/{}/{}/{}", domain, account, token, by_longest_chain);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetTokenBalance>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_token_balance_should_be_ok() {

        println!("{:#?}", get_token_balance("http://api.iost.io","admin","iost",true).await);

    }
}



