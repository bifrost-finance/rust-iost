use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PledgeInfo {
    /// the account receiving the deposit
    pub pledger: String,
    /// 	the amount of the deposit
    pub amount: f64
}
