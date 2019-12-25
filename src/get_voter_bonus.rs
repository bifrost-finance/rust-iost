use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVoterBonus {
    /// the total voting bonus he can receive
    pub bonus: f64,
    /// the bonus from every candidate
    pub detail: HashMap<String,f64>
}

async fn get_voter_bonus(domain: &str, name: &str, by_longest_chain: bool) -> GetVoterBonus {
    let url = format!("{}/getVoterBonus/{}/{}", domain, name, by_longest_chain);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetVoterBonus>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_voter_bonus_should_be_ok() {

        println!("{:#?}",get_voter_bonus("http://api.iost.io", "admin", true).await);

    }
}
