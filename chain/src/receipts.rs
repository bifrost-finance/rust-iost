use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Receipt {
    /// ABI function name
    pub func_name: String,
    /// content
    pub content: String,
}
