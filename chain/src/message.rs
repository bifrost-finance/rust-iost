use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {

    pub code: i32,

    pub message: String
}