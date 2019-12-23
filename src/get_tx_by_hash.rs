use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::status::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxByHash {
    /// enum PENDING- transaction is cached, PACKED - transaction is in reversible blocks, IRREVERSIBLE - transaction is in irreversible blocks
    pub status: Status,
    /// Transaction data
    pub transaction: Transaction,
    /// the number of the block which the tx is in
    pub block_number: String
}

async fn get_tx_by_hash_info(domain: &str, hash: &str) -> GetTxByHash {
    let url = format!("{}/getTxByHash/{}", domain, hash);
    let res = reqwest::get(&url)
        .await.unwrap()
        .json::<GetTxByHash>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_tx_by_hash_info_should_be_ok (){

        println!("{:#?}", get_tx_by_hash_info("http://api.iost.io","Dj8bmA4Fx4LHrwLtDB6EEkNbBFU8biENxf55mNaJewYw").await);

    }
}
