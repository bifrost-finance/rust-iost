use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    /// permission name or key paid ID
    pub id: String,
    /// true - id is a key pair; false - id is a permission name
    pub is_key_pair: bool,
    /// permission weight
    pub weight: String,
    /// the permission
    pub permission: String
}
