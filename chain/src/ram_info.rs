use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RAMInfo {
    /// RAM bytes available for use
    pub available: String,
    /// RAM bytes used
    pub used: String,
    /// RAM bytes total
    pub total: String,
}
