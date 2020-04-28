use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ChainInfo {
    /// Network name, such as "mainnet" or "testnet"
    pub net_name: String,
    /// iost protocol version
    pub protocol_version: String,
    /// iost chain id
    pub chain_id: i32,
    /// the lastest block height
    pub head_block: String,
    /// the hash of the lastest block
    pub head_block_hash: String,
    /// height of irreversible blocks
    pub lib_block: String,
    /// hash of irreversible blocks
    pub lib_block_hash: String,
    /// list of pubkeys for the current block production nodes
    pub witness_list: Vec<String>,
    /// list of pubkeys for the block production nodes of the last irreversible block time
    pub lib_witness_list: Vec<String>,
    /// list of pubkeys for the next round block production nodes
    pub pending_witness_list: Vec<String>,
    /// time of head block
    pub head_block_time: String,
    /// time of last irreversible block
    pub lib_block_time: String
}

async fn get_chain_info() -> ChainInfo {
    let res = reqwest::get("https://api.iost.io/getChainInfo").await.unwrap()
        .json::<ChainInfo>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_chain_info_should_be_ok() {

        println!("{:#?}", get_chain_info().await);

    }
}
