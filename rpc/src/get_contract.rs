use crate::abi::ABI;
use crate::error::Error;
use crate::message::ErrorMessage;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Contract {
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

async fn get_contract(domain: &str, id: &str, by_longest_chain: bool) -> Result<Contract, Error> {
    let url = format!("{}/getContract/{}/{}", domain, id, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<Contract>().await.map_err(Error::Reqwest)?;
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
    async fn get_contract_should_be_ok() {
        let response = get_contract("http://api.iost.io","base.iost",true).await;
        assert!(response.is_ok());
    }
}
