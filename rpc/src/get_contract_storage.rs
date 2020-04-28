use crate::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ContractStoragePost {
    /// ID of the smart contract
    pub id: String,
    /// the key of StateDB
    pub key: String,
    /// the values from StateDB; if StateDB[key] is a map then it is required to configure field to obtain values of StateDB[key][field]
    pub field: String,
    /// true - get data from the longest chain; false - get data from irreversible blocks
    pub by_longest_chain: bool
}

#[derive(Deserialize, Debug)]
pub struct ContractStorage {
    /// the stored data
    pub data: String,
    /// the hash of block from which the data is from
    pub block_hash: String,
    /// the number of block from which the data is from
    pub block_number: String
}

async fn get_contract_storage(new_post: ContractStoragePost) -> Result<ContractStorage, Error>  {
    let res_status = reqwest::Client::new()
        .post("http://api.iost.io/getContractStorage")
        .json(&new_post)
        .send()
        .await.map_err(Error::Reqwest)?;

    if res_status.status() == 200 {
        let response = res_status.json().await.map_err(Error::Reqwest)?;
        Ok(response)
    } else {
        let response = res_status.json().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(response))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_contract_storage_should_be_ok () {
        let new_post = ContractStoragePost {
            id: "token.iost".to_string(),
            key: "TIiost".to_string(),
            field: "decimal".to_string(),
            by_longest_chain: true
        };

        let res = get_contract_storage(new_post).await;
        assert!(res.is_ok());
    }
}





