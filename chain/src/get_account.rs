use crate::error::Error;
use crate::frozen_balance::FrozenBalance;
use crate::gas_info::GasInfo;
use crate::group::Group;
use crate::message::ErrorMessage;
use crate::permission::Permission;
use crate::ram_info::RAMInfo;
use crate::vote_info::VoteInfo;
use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Account {
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

async fn get_account(domain: &str, account: &str, complete: bool) -> Result<Account, Error> {
    let url = format!("{}/getAccount/{}/{}", domain, account, complete);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<Account>().await.map_err(Error::Reqwest)?;
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
    async fn get_account_should_be_ok() {
        let response = get_account("http://api.iost.io","admin",true).await;
        assert!(response.is_ok());
    }
}

