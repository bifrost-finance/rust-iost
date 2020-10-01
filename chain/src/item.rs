use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Item {
    /// permission name or key paid ID
    pub id: String,
    /// true - id is a key pair; false - id is a permission name
    pub is_key_pair: bool,
    /// permission weight
    pub weight: String,
    /// the permission
    pub permission: String,
}
