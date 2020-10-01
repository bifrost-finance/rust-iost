use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FrozenBalance {
    /// the amount
    pub amount: f64,
    /// the time when the amount is unfrozen
    pub time: String,
}
