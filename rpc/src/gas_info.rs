use crate::pledge_info::PledgeInfo;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GasInfo {
    /// Total gas for the moment
    pub current_total: f64,
    /// Gas available for trade
    pub transferable_gas: f64,
    /// Gas obtained from deposits
    pub pledge_gas: f64,
    /// The rate of gas increase, in gas per second
    pub increase_speed: f64,
    /// The upper limit of gas from token deposit
    pub limit: f64,
    /// The information on deposit made by other accounts, on behalf of the inquired account
    pub pledged_info: Vec<PledgeInfo>
}
