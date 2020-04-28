use alloc::collections::BTreeMap;
use crate::error::Error;
use crate::message::ErrorMessage;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct VoterBonus {
    /// the total voting bonus he can receive
    pub bonus: f64,
    /// the bonus from every candidate
    pub detail: BTreeMap<String,f64>
}

async fn get_voter_bonus(domain: &str, name: &str, by_longest_chain: bool) -> Result<VoterBonus, Error> {
    let url = format!("{}/getVoterBonus/{}/{}", domain, name, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<VoterBonus>().await.map_err(Error::Reqwest)?;
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
    async fn get_voter_bonus_should_be_ok() {
        let response = get_voter_bonus("http://api.iost.io", "admin", true).await;
        assert!(response.is_ok());
    }
}

