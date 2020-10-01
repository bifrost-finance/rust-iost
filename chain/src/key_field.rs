use alloc::string::String;

#[cfg(feature = "std")]
use serde::Serialize;

#[cfg_attr(feature = "std", derive(Serialize))]
pub struct KeyField {
    /// the key of StateDB
    pub key: String,
    /// the values from StateDB; if StateDB[key] is a map then it is required to configure field to obtain values of StateDB[key][field]
    pub field: String,
}
