use serde::{Serialize, Deserialize};
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::status::Status;
use crate::block::Block;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockByHash {
    /// PENDING - block is in cache; IRREVERSIBLE - block is irreversible.
    pub status: Status,
    /// a Block struct
    pub block: Block
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockByNumber {
    /// PENDING - block is in cache; IRREVERSIBLE - block is irreversible.
    pub status: Status,
    /// a Block struct
    pub block: Block
}

async fn get_block_by_hash(domain: &str, hash: &str, complete: bool) -> GetBlockByHash {
    let url = format!("{}/getBlockByHash/{}/{}", domain, hash, complete);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetBlockByHash>()
        .await.unwrap();
    res
}

async fn get_block_by_number(domain: &str, number: i32, complete: bool) -> GetBlockByNumber {
    let url = format!("{}/getBlockByNumber/{}/{}", domain, number, complete);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetBlockByNumber>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_block_by_hash_should_be_ok() {

        println!("{:#?}", get_block_by_hash("http://api.iost.io","GexerugLra5qBArG4vqCAFNX1F7WzLzpPdmzcjLBAi3k",false).await);

    }

    #[tokio::test]
    async fn get_block_by_number_should_be_ok() {

        println!("{:#?}", get_block_by_number("http://api.iost.io",3,false).await);

    }
}