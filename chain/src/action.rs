use serde::{Serialize, Deserialize};
use iost_derive::{Read, Write};

#[derive(Serialize, Deserialize, Read, Write, PartialEq, Debug)]
#[iost_root_path = "crate"]
pub struct Action {
    /// contract name
    pub contract: String,
    /// function name of the contract
    pub action_name: String,
    /// Specific parameters of the call. Put every parameter in an array, and JSON-serialize this array. It may looks like ["a_string", 13]
    pub data: String
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action() {
        let action = Action{
            contract: "iost".to_string(),
            action_name: "iost".to_string(),
            data: "iost".to_string()
        };
        dbg!(action);
    }
}
