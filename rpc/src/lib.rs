#![allow(dead_code)]

extern crate alloc;

use async_trait::async_trait;
pub use crate::error::Error;
use crate::bytes::*;
pub use crate::get_node_info::NodeInfo;
pub use crate::get_chain_info::ChainInfo;
pub use crate::get_gas_ratio::GasRatio;
pub use crate::get_ram_info::RamInfo;
pub use crate::get_contract_storage::{ContractStorage, ContractStoragePost};
pub use crate::get_contract_storage_fields::{ContractStorageFields, ContractStorageFieldsPost};
pub use crate::get_batch_contract_storage::{BatchContractStorage, BatchContractStoragePost};
pub use crate::key_field::KeyField;
pub use crate::message::ErrorMessage;
use serde::{Serialize, Deserialize};
use crate::tx::Tx;
use crate::tx_response::TxResponse;

mod abi;
mod action;
mod amount_limit;
mod block;
mod bytes;
mod error;
mod frozen_balance;
mod gas_info;
mod get_block_by_hash;
mod get_batch_contract_storage;
mod get_contract_storage;
mod get_contract_storage_fields;
mod get_chain_info;
mod get_node_info;
mod net_work_info;
mod get_gas_ratio;
mod get_ram_info;
mod get_tx_by_hash;
mod get_token_balance;
mod get_producer_vote_info;
mod get_contract;
mod get_candidate_bonus;
mod get_voter_bonus;
mod get_token_info;
mod get_account;
mod group;
mod info;
mod item;
mod key_field;
mod message;
mod pledge_info;
mod permission;
mod ram_info;
mod receipts;
mod status;
mod status_code;
mod signature;
mod transaction;
mod tx;
mod tx_receipt;
mod tx_response;
mod unsigned_int;
mod vote_info;

struct IOST {
    host: String,
    client: reqwest::Client,
}

#[async_trait]
trait Client {
    fn new(host: &str) -> Self;

    async fn get<T>(&self, path: &str) -> Result<T, Error> where T: 'static + for<'de>Deserialize<'de>;

    async fn post<T, R>(&self, path: &str, param: R) -> Result<T, Error>
        where T: 'static + for<'de>Deserialize<'de>,
              R: Serialize + Send +Sync;
}

#[async_trait]
impl Client for IOST {

    fn new(host: &str) -> Self {
        Self {
            host: host.to_owned(),
            client: reqwest::Client::new()
        }
    }

    async fn get<T>(&self, path: &str) -> Result<T, Error> where T: 'static + for<'de>Deserialize<'de> {
        let url = format!("{}/{}", self.host, path);
        let response = self.client.get(&url).send().await.map_err(Error::Reqwest)?;
        if response.status() == 200 {
            let result = response.json::<T>().await.map_err(Error::Reqwest)?;
            Ok(result)
        } else {
            let rsp = response.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
            Err(Error::ErrorMessage(rsp))
        }
    }

    async fn post<T, R>(&self, path: &str, param: R) -> Result<T, Error>
        where T: 'static + for<'de> Deserialize<'de>,
              R: Serialize + Send + Sync
    {
        let url = format!("{}/{}", self.host, path);
        let req = reqwest::Client::new()
            .post(&url)
            .json(&param)
            .send()
            .await.map_err(Error::Reqwest)?;
        let code_status = req.status();
        if code_status == 200 {
            let response = req.json().await.map_err(Error::Reqwest)?;
            Ok(response)
        } else {
            let response = req.json().await.map_err(Error::Reqwest)?;
            Err(Error::ErrorMessage(response))
        }
    }
}

impl IOST {
    pub async fn get_node_info(&self) -> Result<NodeInfo, Error> {
        self.get("getNodeInfo").await
    }

    pub async fn get_chain_info(&self) -> Result<ChainInfo, Error> {
        self.get("getChainInfo").await
    }

    pub async fn get_gas_ratio(&self) -> Result<GasRatio, Error> {
        self.get("getGasRatio").await
    }

    pub async fn get_ram_info(&self) -> Result<RamInfo, Error> {
        self.get("getRAMInfo").await
    }

    pub async fn get_contract_storage(&self, par: ContractStoragePost) -> Result<ContractStorage, Error> {
        self.post("getContractStorage",&par).await
    }

    pub async fn get_contract_storage_fields(&self, par: ContractStorageFieldsPost) -> Result<ContractStorageFields, Error> {
        self.post("getContractStorageFields",&par).await
    }

    pub async fn get_batch_contract_storage(&self, par: BatchContractStoragePost) -> Result<BatchContractStorage, Error> {
        self.post("getBatchContractStorage", &par).await
    }

    pub async fn send_tx(&self, par: Tx) -> Result<TxResponse, Error> {
        self.post("sendTx", par).await
    }
}

/// Execute test file command "cargo test iost_basic_test -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    use crate::amount_limit::AmountLimit;
    use crate::signature::Signature;
    use crate::action::Action;

    #[tokio::test]
    async fn iost_basic_get_method_should_be_ok() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let result = iost.get_node_info().await;
        assert!(result.is_ok());
        let chain_result = iost.get_chain_info().await;
        assert!(chain_result.is_ok());
        let gas_result = iost.get_gas_ratio().await;
        assert!(gas_result.is_ok());
        let ram_result = iost.get_ram_info().await;
        assert!(ram_result.is_ok());
    }

    #[tokio::test]
    async fn iost_basic_post_method_should_be_ok() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let new_post = ContractStoragePost {
            id: "token.iost".to_string(),
            key: "TIiost".to_string(),
            field: "decimal".to_string(),
            by_longest_chain: true
        };
        let result = iost.get_contract_storage(new_post).await;
        assert!(result.is_ok());
        let new_post = ContractStorageFieldsPost {
            id: "token.iost".to_string(),
            key: "TIiost".to_string(),
            by_longest_chain: true
        };
        let field_result = iost.get_contract_storage_fields(new_post).await;
        assert!(field_result.is_ok());
        let key = KeyField {
            key: "supply".to_string(),
            field: "TIiost".to_string()
        };

        let key1 = KeyField {
            key: "decimal".to_string(),
            field: "TIiost".to_string()
        };

        let posts = BatchContractStoragePost {
            id: "token.iost".to_string(),
            key_fields: vec![key,key1],
            by_longest_chain: true
        };
        let storage_result = iost.get_batch_contract_storage(posts).await;
        assert!(storage_result.is_ok());
    }

    #[tokio::test]
    async fn test_send_tx_should_be_ok() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let action = Action{
            contract: "token.iost".to_string(),
            action_name: "transfer".to_string(),
            data: "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]".to_string()
        };

        let amount_limit = AmountLimit{
            token: "*".to_string(),
            value: "unlimited".to_string()
        };

        let signature = Signature{
            algorithm: "ED25519".to_string(),
            signature: "lDS+SdM+aiVHbDyXapvrsgyKxFg9mJuHWPZb/INBRWY=".to_string(),
            public_key: "/K1HM0OEbfJ4+D3BmalpLmb03WS7BeCz4nVHBNbDrx3/A31aN2RJNxyEKhv+VSoWctfevDNRnL1kadRVxSt8CA==".to_string()
        };

        let tx = Tx{
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1.0,
            gas_limit: 500000.0,
            delay: 0,
            chain_id: 1024,
            actions: vec![action],
            amount_limit: vec![amount_limit],
            publisher: "testaccount".to_string(),
            publisher_sigs: vec![signature],
            signers: vec![],
            signatures: vec![]
        };

        let tx_result = iost.send_tx(tx).await;
        dbg!(tx_result);
    }
}

