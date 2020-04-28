use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RAMInfo {
    /// RAM bytes available for use
    pub available: String,
    /// RAM bytes used
    pub used: String,
    /// RAM bytes total
    pub total: String
}
