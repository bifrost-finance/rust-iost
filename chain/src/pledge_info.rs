use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PledgeInfo {
    /// the account receiving the deposit
    pub pledger: String,
    /// 	the amount of the deposit
    pub amount: f64,
}
