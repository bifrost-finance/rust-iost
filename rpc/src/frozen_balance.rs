use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrozenBalance {
    /// the amount
    pub amount: f64,
    /// the time when the amount is unfrozen
    pub time: String
}