use serde::{Serialize, Deserialize};
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::status::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProducerVoteInfo {

    pub pubkey: String,

    pub loc: String,

    pub url: String,

    pub net_id: String,

    pub is_producer: bool,

    pub status: Status,

    pub online: bool,

    pub votes: i32
}

async fn get_producer_vote_info(domain: &str, id: &str, by_longest_chain: bool) -> GetProducerVoteInfo {
    let url = format!("{}/getProducerVoteInfo/{}/{}", domain, id, by_longest_chain);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetProducerVoteInfo>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_producer_vote_info_should_be_ok() {

        println!("{:#?}", get_producer_vote_info("http://api.iost.io","duongkien",true).await);

    }
}