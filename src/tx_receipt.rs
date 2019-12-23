use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::status_code::StatusCode;
use crate::receipts::Receipt;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxReceipt {
    /// hash of the transaction
    pub tx_hash: String,
    /// GAS consumption of the transaction
    pub gas_usage: f64,
    /// RAM consumption for the transaction. map-key is account name, and value is RAM amount
    pub ram_usage: HashMap<String, i64>,
    /// Status of the transaction. SUCCESS; GAS_RUN_OUT - insufficient GAS;
    /// BALANCE_NOT_ENOUGH - insufficient balance; WRONG_PARAMETER;
    /// RUNTIME_ERROR - a run-time error; TIMEOUT; WRONG_TX_FORMAT;
    /// DUPLICCATE_SET_CODE - set code is duplicated unexpectedly;
    /// UNKNOWN_ERROR
    pub status_code: StatusCode,
    /// a message descripting status_code
    pub message: String,
    /// return values for each Action
    pub returns: Vec<String>,
    /// for event functions
    pub receipts: Vec<Receipt>
}

async fn get_tx_receipt_json (domain: &str, hash: &str) -> TxReceipt {
    let url = format!("{}/getTxReceiptByTxHash/{}", domain, hash);
    let res = reqwest::get(&url)
        .await.unwrap()
        .json::<TxReceipt>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_tx_receipt_json_should_be_ok() {

        println!("{:#?}", get_tx_receipt_json("http://api.iost.io", "Dj8bmA4Fx4LHrwLtDB6EEkNbBFU8biENxf55mNaJewYw").await);

    }

}
