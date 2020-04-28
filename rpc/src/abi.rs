use crate::amount_limit::AmountLimit;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct ABI {
    /// interface name
    pub name: String,
    /// arguments of the interface
    pub args: Vec<String>,
    /// The limits on the amount
    pub amount_limit: Vec<AmountLimit>
}

