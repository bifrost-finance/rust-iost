use alloc::string::String;
use alloc::vec::Vec;

use crate::item::Item;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Permission {
    /// permission name
    pub name: String,
    /// permission group names
    pub group_names: Vec<String>,
    /// permission information
    pub items: Vec<Item>,
    /// permission threshold
    pub threshold: String,
}
