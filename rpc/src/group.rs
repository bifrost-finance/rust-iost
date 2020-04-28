use crate::item::Item;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    /// name of the group
    pub name: String,
    /// information on the permission group
    pub items: Vec<Item>
}