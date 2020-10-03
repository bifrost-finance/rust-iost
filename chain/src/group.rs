use alloc::string::String;
use alloc::vec::Vec;

use crate::item::Item;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Group {
    /// name of the group
    pub name: String,
    /// information on the permission group
    pub items: Vec<Item>,
}
