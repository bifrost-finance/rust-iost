use alloc::string::String;
use alloc::vec::Vec;

use crate::amount_limit::AmountLimit;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ABI {
    /// interface name
    pub name: String,
    /// arguments of the interface
    pub args: Vec<String>,
    /// The limits on the amount
    pub amount_limit: Vec<AmountLimit>,
}
