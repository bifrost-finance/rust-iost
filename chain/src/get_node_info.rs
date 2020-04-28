#![allow(dead_code)]

use crate::net_work_info::NetWork;
use crate::message::ErrorMessage;
use crate::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
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

async fn get_node_info () -> Result<NodeInfo, Error>  {
    let req = reqwest::get("https://api.iost.io/getNodeInfo").await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<NodeInfo>().await.map_err(Error::Reqwest)?;
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
    async fn get_node_info_should_be_ok() {

        println!("{:#?}", get_node_info().await);

    }
}




