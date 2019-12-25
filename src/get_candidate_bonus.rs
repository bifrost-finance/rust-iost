use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCandidateBonus {
    /// the bonus he can receive
    pub bonus: f64
}

async fn get_candidate_bonus(domain: &str, name: &str, by_longest_chain: bool) -> GetCandidateBonus {

    let url = format!("{}/getCandidateBonus/{}/{}", domain, name, by_longest_chain);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetCandidateBonus>()
        .await.unwrap();
    res

}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_candidate_bonus_should_be_ok() {

        println!("{:#?}", get_candidate_bonus("http://api.iost.io","erebus",true).await);

    }
}