use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    /// error code status
    pub code: i32,
    /// error message
    pub message: String
}
