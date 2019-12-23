use serde::{Serialize, Deserialize};
use crate::net_work_info::NetWork;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNodeInfo {
    /// Building time of the 'server' binary
    pub build_time: String,
    /// Git hash of the 'iserver' binary
    pub git_hash: String,
    /// Current mode of the server. It can be one of 'ModeInit', 'ModeNormal' and 'ModeSync'
    pub mode: String,
    /// Network information of the node
    pub network: NetWork,
    /// the version of code
    pub code_version: String,
    /// the current timestamp of the server, unit is nano second
    pub server_time: String,
}

async fn get_node_info () -> GetNodeInfo {
    let  res = reqwest::get("https://api.iost.io/getNodeInfo").await.unwrap()
        .json::<GetNodeInfo>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_node_info_should_be_ok() {

        println!("{:#?}", get_node_info().await);

    }
}


