use crate::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ContractStorageFieldsPost {
    /// ID of the smart contract
    pub id: String,
    /// the key of StateDB
    pub key: String,
    /// true - get data from the longest chain; false - get data from irreversible blocks
    pub by_longest_chain: bool
}

#[derive(Deserialize,Debug)]
pub struct ContractStorageFields {

    pub fields: Vec<String>

}

async fn get_contract_storage_fields(new_post: ContractStorageFieldsPost) -> Result<ContractStorageFields, Error> {
    let req = reqwest::Client::new()
        .post("http://api.iost.io/getContractStorageFields")
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
    async fn get_contract_storage_fields_should_be_ok() {
        let new_post = ContractStorageFieldsPost {
            id: "token.iost".to_string(),
            key: "TIiost".to_string(),
            by_longest_chain: true
        };

        let res = get_contract_storage_fields(new_post).await;
        assert!(res.is_ok());
    }
}


