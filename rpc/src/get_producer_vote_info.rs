use crate::status::Status;
use crate::error::Error;
use crate::message::ErrorMessage;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ProducerVoteInfo {

    pub pubkey: String,

    pub loc: String,

    pub url: String,

    pub net_id: String,

    pub is_producer: bool,

    pub status: Status,

    pub online: bool,

    pub votes: i32
}

async fn get_producer_vote_info(domain: &str, id: &str, by_longest_chain: bool) -> Result<ProducerVoteInfo, Error> {
    let url = format!("{}/getProducerVoteInfo/{}/{}", domain, id, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<ProducerVoteInfo>().await.map_err(Error::Reqwest)?;
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
    async fn get_producer_vote_info_should_be_ok() {
        let response = get_producer_vote_info("http://api.iost.io","producerName",true).await;
        assert!(response.is_err());
    }
}

