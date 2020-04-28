use serde::{Serialize, Deserialize};
use iost_derive::{Read, Write, NumberBytes};
use crate::bytes::{Write, NumberBytes};
use crate::Error;
use std::str::FromStr;

#[derive(Clone, Default, Debug, Read, Serialize, Write, PartialEq, NumberBytes)]
#[iost_root_path = "crate"]
pub struct Action {
    /// contract name
    pub contract: String,
    /// function name of the contract
    pub action_name: String,
    /// Specific parameters of the call. Put every parameter in an array, and JSON-serialize this array. It may looks like ["a_string", 13]
    pub data: String
}

impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorAction;
        impl<'de> serde::de::Visitor<'de> for VisitorAction {
            type Value = Action;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                while let Some(field) = map.next_key()? {
                    match field {
                        "contract" => {
                            let _contract = map.next_value()?;
                        }
                        "action_name" => {
                            let _action_name = map.next_value()?;
                        }
                        "data" => {
                            let _data= map.next_value()?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let action = Action {
                    contract: "".to_string(),
                    action_name: "".to_string(),
                    data: "".to_string()
                };
                Ok(action)
            }

        }
        deserializer.deserialize_any(VisitorAction)
    }
}

impl Action {
    pub fn new(contract: String, action_name: String, data: String) -> Self {
        Action {
            contract,
            action_name,
            data
        }
    }
}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f,"contract: {}\n\
            action_name: {}\n\
            data: {}",
            self.contract,
            self.action_name,
            self.data
        )
    }
}

pub trait ToAction: Write + NumberBytes {
    const NAME: u64;

    #[inline]
    fn to_action(
        &self,
        contract: String,
        action_name: String,
        data: String
    ) -> core::result::Result<Action, Error>{
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).unwrap();

        Ok(Action{
            contract,
            action_name,
            data: "".to_string()
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action{
            contract: s.to_string(),
            action_name: s.to_string(),
            data: s.to_string()
        })
    }
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

    #[test]
    fn test_action_deserialize_should_be_ok() {
        let action_str = r#"
        {
            "contract": "token.iost",
            "action_name": "transfer",
            "data": "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]"
        }
        "#;
        let result_action: Result<Action, _> = serde_json::from_str(action_str);
        assert!(result_action.is_err());
    }
}
