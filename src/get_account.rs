use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::gas_info::GasInfo;
use crate::ram_info::RAMInfo;
use crate::permission::Permission;
use crate::group::Group;
use crate::frozen_balance::FrozenBalance;
use crate::vote_info::VoteInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccount {
    /// account name
    pub name: String,
    /// the balance of the account
    pub balance: f64,
    /// Gas information
    pub gas_info: GasInfo,
    /// Ram information
    pub ram_info: RAMInfo,
    /// permissions
    pub permissions: HashMap<String, Permission>,
    /// permission groups
    pub groups: HashMap<String, Group>,
    /// information on the frozen balance
    pub frozen_balances: Vec<FrozenBalance>,
    /// information of vote
    pub vote_infos: Vec<VoteInfo>
}

async fn get_account(domain: &str, account: &str, complete: bool) -> GetAccount {
    let url = format!("{}/getAccount/{}/{}", domain, account, complete);
    let res = reqwest::get(&url).await.unwrap()
        .json::<GetAccount>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn get_account_should_be_ok() {

        println!("{:#?}",get_account("http://api.iost.io","admin",true).await);

    }
}
