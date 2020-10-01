// use alloc::string::String;
// use alloc::vec::Vec;
// #[cfg(feature = "std")]
// use serde::Deserialize;
//
// #[derive(Deserialize, Debug, Default)]
// pub struct ChainInfo {
//     /// Network name, such as "mainnet" or "testnet"
//     pub net_name: String,
//     /// iost protocol version
//     pub protocol_version: String,
//     /// iost chain id
//     pub chain_id: i32,
//     /// the lastest block height
//     pub head_block: String,
//     /// the hash of the lastest block
//     pub head_block_hash: String,
//     /// height of irreversible blocks
//     pub lib_block: String,
//     /// hash of irreversible blocks
//     pub lib_block_hash: String,
//     /// list of pubkeys for the current block production nodes
//     pub witness_list: Vec<String>,
//     /// list of pubkeys for the block production nodes of the last irreversible block time
//     pub lib_witness_list: Vec<String>,
//     /// list of pubkeys for the next round block production nodes
//     pub pending_witness_list: Vec<String>,
//     /// time of head block
//     pub head_block_time: String,
//     /// time of last irreversible block
//     pub lib_block_time: String,
// }
//
// fn get_chain_info() -> ChainInfo {
//     // let res = reqwest::get("https://api.iost.io/getChainInfo").await.unwrap()
//     //     .json::<ChainInfo>()
//     //     .await.unwrap();
//
//     let mut chain_info = ChainInfo::default();
//     // let client = reqwest::Client::new();
//     // let res = client.get("https://api.iost.io/getChainInfo")
//     //     // .header(UserAgent::new("foo"))
//     //     .send()?;
//     let body: ChainInfo = reqwest::blocking::get("http://127.0.0.1:30001/getChainInfo")
//         .unwrap()
//         .json()
//         .unwrap();
//     dbg!(body);
//     chain_info
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn get_chain_info_should_be_ok() {
//         println!("{:#?}", get_chain_info());
//     }
// }
