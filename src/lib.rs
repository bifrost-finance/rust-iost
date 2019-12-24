use crate::error::Error;
use crate::get_node_info::GetNodeInfo;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use async_trait::async_trait;
use crate::get_chain_info::GetChainInfo;

mod get_node_info;
mod net_work_info;
mod get_chain_info;
mod get_gas_ratio;
mod get_ram_info;
mod get_tx_by_hash;
mod action;
mod amount_limit;
mod receipts;
mod status_code;
mod transaction;
mod tx_receipt;
mod group;
mod status;
mod get_block_by_hash;
mod block;
mod info;
mod get_account;
mod gas_info;
mod pledge_info;
mod ram_info;
mod permission;
mod item;
mod frozen_balance;
mod vote_info;
mod get_token_balance;
mod get_producer_vote_info;
mod get_contract;
mod abi;
mod get_candidate_bonus;
mod get_voter_bonus;
mod get_token_info;
mod error;
mod message;

struct IOST {
    host: String,
    client: reqwest::Client,
}

#[async_trait]
trait Client {
    fn new(host: &str) -> Self;

    async fn get<T>(&self, path: &str) -> Result<T, Error> where T: 'static + for<'de>Deserialize<'de>;

//    async fn post(&self, path: &str) -> Result<T, Error>;
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
        let result = response.json::<T>().await.map_err(Error::Reqwest)?;

        Ok(result)
    }

//    async fn post(&self, path: &str) -> Result<T, Error> {
//        unimplemented!()
//    }
}

impl IOST {
    pub async fn get_node_info(&self) -> Result<GetNodeInfo, Error> {
        self.get("getNodeInfo").await
    }

    pub async fn get_chain_info(&self) -> Result<GetChainInfo, Error> {
        self.get("getChanInfo").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn iost_basic_test() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let result = iost.get_node_info().await;
        dbg!(&result);
        let chain_result = iost.get_chain_info().await;
        dbg!(&chain_result);
    }
}