use crate::error::Error;
use crate::key_field::KeyField;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct BatchContractStoragePost {
    /// smart contract ID
    pub id: String,
    /// the key-fields which are queried，the order of return values is the same as the request
    pub key_fields: Vec<KeyField>,
    /// true - get data from the longest chain; false - get data from irreversible blocks
    pub by_longest_chain: bool
}

#[derive(Deserialize, Debug)]
pub struct BatchContractStorage {
    /// the stored data, returned in order as request
    pub datas: Vec<String>,
    /// the hash of block from which the data is from
    pub block_hash: String,
    /// the number of block from which the data is from
    pub block_number: String
}

async fn get_batch_contract_storage(new_post: BatchContractStoragePost) -> Result<BatchContractStorage, Error> {
    let req = reqwest::Client::new()
        .post("http://api.iost.io/getBatchContractStorage")
        .json(&new_post)
        .send()
        .await.map_err(Error::Reqwest)?;
    let status_code = req.status();
    if status_code == 200 {
        let response = req.json().await.map_err(Error::Reqwest)?;
        Ok(response)
    } else {
        let response = req.json().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(response))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn get_batch_contract_storage_should_be_ok() {
        let key = KeyField {
            key: "supply".to_string(),
            field: "TIiost".to_string()
        };

        let key1 = KeyField {
            key: "decimal".to_string(),
            field: "TIiost".to_string()
        };

        let new_post = BatchContractStoragePost {
            id: "token.iost".to_string(),
            key_fields: vec![key,key1],
            by_longest_chain: true
        };

        let res = get_batch_contract_storage(new_post).await;
        assert!(res.is_ok());
    }
}

