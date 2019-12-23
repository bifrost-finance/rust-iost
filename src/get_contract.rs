use serde::{Serialize, Deserialize};
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::abi::ABI;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetContract {
    /// contract ID
    pub id: String,
    /// the code of the contract
    pub code: String,
    /// the language of the contract
    pub language: String,
    /// contract version
    pub version: String,
    /// the ABIs of the contract
    pub abis: Vec<ABI>
}

async fn get_contract(domain: &str, id: &str, by_longest_chain: bool) -> GetContract {
    let url = format!("{}/getContract/{}/{}", domain, id, by_longest_chain);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetContract>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_contract_should_be_ok() {

        println!("{:#?}", get_contract("http://api.iost.io","base.iost",true).await);

    }
}