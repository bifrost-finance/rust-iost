use serde::{Serialize, Deserialize};
use crate::amount_limit::AmountLimit;

#[derive(Serialize, Deserialize, Debug)]
pub struct ABI {
    /// interface name
    pub name: String,
    /// arguments of the interface
    pub args: Vec<String>,
    /// The limits on the amount
    pub amount_limit: Vec<AmountLimit>
}
