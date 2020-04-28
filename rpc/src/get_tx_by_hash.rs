use crate::error::Error;
use crate::message::ErrorMessage;
use crate::status::Status;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxByHash {
    /// enum PENDING- transaction is cached, PACKED - transaction is in reversible blocks, IRREVERSIBLE - transaction is in irreversible blocks
    pub status: Status,
    /// Transaction data
    pub transaction: Transaction,
    /// the number of the block which the tx is in
    pub block_number: String
}

async fn get_tx_by_hash_info(domain: &str, hash: &str) -> Result<GetTxByHash, Error> {
    let url = format!("{}/getTxByHash/{}", domain, hash);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<GetTxByHash>().await.map_err(Error::Reqwest)?;
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
    async fn get_tx_by_hash_info_should_be_ok (){
        let response: Result<GetTxByHash, Error> = get_tx_by_hash_info("http://api.iost.io","Dj8bmA4Fx4LHrwLtDB6EEkNbBFU8biENxf55mNaJewYw").await;
        assert!(response.is_ok());
    }
}


