use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountLimit {
    /// token name
    pub token: String,
    /// corresponding token limit
    pub value: String
}