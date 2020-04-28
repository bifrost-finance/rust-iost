use crate::error::Error;
use crate::message::ErrorMessage;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CandidateBonus {
    /// the bonus he can receive
    pub bonus: f64
}

async fn get_candidate_bonus(domain: &str, name: &str, by_longest_chain: bool) -> Result<CandidateBonus, Error> {
    let url = format!("{}/getCandidateBonus/{}/{}", domain, name, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<CandidateBonus>().await.map_err(Error::Reqwest)?;
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
    async fn get_candidate_bonus_should_be_ok() {
        let response = get_candidate_bonus("http://api.iost.io","erebus",true).await;
        assert!(response.is_ok());
    }
}
