use crate::block::Block;
use crate::error::Error;
use crate::message::ErrorMessage;
use crate::status::Status;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockByHash {
    /// PENDING - block is in cache; IRREVERSIBLE - block is irreversible.
    pub status: Status,
    /// a Block struct
    pub block: Block
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockByNumber {
    /// PENDING - block is in cache; IRREVERSIBLE - block is irreversible.
    pub status: Status,
    /// a Block struct
    pub block: Block
}

async fn get_block_by_hash(domain: &str, hash: &str, complete: bool) -> Result<BlockByHash, Error> {
    let url = format!("{}/getBlockByHash/{}/{}", domain, hash, complete);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<BlockByHash>().await.map_err(Error::Reqwest)?;
        Ok(rsp)
    } else {
        let rsp = req.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(rsp))
    }
}

async fn get_block_by_number(domain: &str, number: i32, complete: bool) -> Result<BlockByNumber, Error> {
    let url = format!("{}/getBlockByNumber/{}/{}", domain, number, complete);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<BlockByNumber>().await.map_err(Error::Reqwest)?;
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
    async fn get_block_by_hash_should_be_ok() {
        let response = get_block_by_hash("http://api.iost.io","GexerugLra5qBArG4vqCAFNX1F7WzLzpPdmzcjLBAi3k",false).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_block_by_number_should_be_ok() {
        let response = get_block_by_number("http://api.iost.io",3,false).await;
        assert!(response.is_ok());
    }
}
