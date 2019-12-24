use serde::{Serialize, Deserialize};
use crate::net_work_info::NetWork;
use reqwest::{Response};
use serde::de::DeserializeOwned;
use crate::message::ErrorMessage;
use crate::error::Error;

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

async fn get_node_info () -> Result<GetNodeInfo, Error>  {
    let req = reqwest::get("https://api.iost.io/getNodeInfo").await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<GetNodeInfo>().await.map_err(Error::Reqwest)?;
        Ok(rsp)
    } else {
        let rsp = req.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(rsp))
    }
}


//async fn test_status() -> Result<get_node_info::GetNodeInfo, ()> {
//    let body = reqwest::get("https://api.iost.io/getNodeInfo").await?;
//
//    if body.status() == 200 {
//        let res = body.json::<GetNodeInfo>()
//            .await?;
//        Ok(res);
//    }
//    println!("{:?}",body);
//}

//fn on_response(res: &Response) {
//    match res.error_for_status_ref() {
//        Ok(_res) => (),
//        Err(err) => {
//            // asserting a 400 as an example
//            // it could be any status between 400...599
//            assert_eq!(
//                err.status(),
//                Some(reqwest::StatusCode::BAD_REQUEST)
//            );
//        }
//    }
//}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_node_info_should_be_ok() {

        println!("{:#?}", get_node_info().await);

    }
}



