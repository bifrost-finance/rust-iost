use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ErrorMessage {
    /// error code status
    pub code: i32,
    /// error message
    pub message: String,
}
