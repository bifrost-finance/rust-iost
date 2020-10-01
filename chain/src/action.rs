use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

use crate::Error::JsonParserError;
use crate::{AccountName, Error, NumberBytes, Read, ReadError, SerializeData, Write, WriteError};
use codec::{Decode, Encode};
use core::str::FromStr;
use lite_json::{JsonValue, Serialize};
#[cfg(feature = "std")]
use serde::{
    ser::{Error as SerError, SerializeStruct, Serializer},
    Deserialize, Serialize as SerSerialize,
};
#[cfg(feature = "std")]
use serde_json::to_string as json_to_string;

#[derive(Clone, Default, Debug, PartialEq, Encode, Decode, SerializeData)]
#[iost_root_path = "crate"]
pub struct Action {
    /// contract name
    pub contract: Vec<u8>,
    /// function name of the contract
    pub action_name: Vec<u8>,
    /// Specific parameters of the call. Put every parameter in an array, and JSON-serialize this array. It may looks like ["a_string", 13]
    pub data: Vec<u8>,
}

#[cfg(feature = "std")]
impl serde::ser::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Action", 3)?;
        state.serialize_field(
            "contract",
            String::from_utf8(self.contract.clone()).unwrap().as_str(),
        )?;
        state.serialize_field(
            "action_name",
            String::from_utf8(self.action_name.clone())
                .unwrap()
                .as_str(),
        )?;
        state.serialize_field(
            "data",
            String::from_utf8(self.data.clone()).unwrap().as_str(),
        )?;
        state.end()
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Debug)]
        struct VisitorAction;
        impl<'de> serde::de::Visitor<'de> for VisitorAction {
            type Value = Action;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::MapAccess<'de>,
            {
                let mut contract: Vec<u8> = vec![];
                let mut action_name: Vec<u8> = vec![];
                let mut data: Vec<u8> = vec![];
                while let Some(field) = map.next_key()? {
                    match field {
                        "contract" => {
                            let contract_value: String = map.next_value()?;
                            contract = contract_value.into_bytes();
                        }
                        "action_name" => {
                            let account_name_value: String = map.next_value()?;
                            action_name = account_name_value.into_bytes();
                        }
                        "data" => {
                            let data_value: String = map.next_value()?;
                            data = data_value.into_bytes();
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let action = Action {
                    contract,
                    action_name,
                    data,
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
            contract: contract.into_bytes(),
            action_name: action_name.into_bytes(),
            data: data.into_bytes(),
        }
    }

    // #[cfg(feature = "std")]
    pub fn from_str<T: AsRef<str>>(
        contract: T,
        action_name: T,
        action_transfer: ActionTransfer,
    ) -> crate::Result<Self> {
        // let data = serde_json::to_string(&action_transfer).unwrap();
        Ok(Action {
            contract: contract.as_ref().as_bytes().to_vec(),
            action_name: action_name.as_ref().as_bytes().to_vec(),
            data: action_transfer.no_std_serialize().as_bytes().to_vec(),
        })
    }

    // #[cfg(feature = "std")]
    pub fn transfer<T: AsRef<str>>(from: T, to: T, quantity: T, memo: T) -> crate::Result<Action> {
        let action_transfer = ActionTransfer::from_str(from, to, quantity, memo)?;
        Action::from_str("token.iost", "transfer", action_transfer)
    }

    pub fn from_shadow_action(shadow_action: ShadowAction) -> Action {
        Action {
            contract: shadow_action.contract.into_bytes(),
            action_name: shadow_action.action_name.into_bytes(),
            data: shadow_action.data.into_bytes(),
        }
    }

    pub fn no_std_serialize(&self) -> JsonValue {
        let shadow_action = ShadowAction::from_action(self).unwrap();
        let object = JsonValue::Object(vec![
            (
                "contract".chars().collect::<Vec<_>>(),
                JsonValue::String(shadow_action.contract.chars().collect()),
            ),
            (
                "action_name".chars().collect::<Vec<_>>(),
                JsonValue::String(shadow_action.action_name.chars().collect()),
            ),
            (
                "data".chars().collect::<Vec<_>>(),
                JsonValue::String(shadow_action.data.chars().collect()),
            ),
        ]);
        object
        // String::from_utf8(object.format(4)).unwrap()
    }
}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "contract: {}\n\
            action_name: {}\n\
            data: {}",
            String::from_utf8_lossy(self.contract.as_slice()),
            String::from_utf8_lossy(self.action_name.as_slice()),
            String::from_utf8_lossy(self.data.as_slice()),
        )
    }
}

impl Write for Action {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        let shadow_action = ShadowAction::from_action(self).unwrap();
        shadow_action
            .write(bytes, pos)
            .map_err(crate::Error::BytesWriteError);
        Ok(())
    }
}

impl Read for Action {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let shadow_action: ShadowAction = ShadowAction::read(bytes, pos)?;
        Ok(Action::from_shadow_action(shadow_action))
    }
}

impl NumberBytes for Action {
    fn num_bytes(&self) -> usize {
        let shadow_action = ShadowAction::from_action(self).unwrap();
        shadow_action.num_bytes()
    }
}

#[cfg_attr(feature = "std", derive(SerSerialize, Deserialize))]
#[derive(Clone, Debug, Read, Write, NumberBytes, Default, SerializeData)]
#[iost_root_path = "crate"]
pub struct ActionTransfer {
    pub token_type: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub memo: String,
}

impl ActionTransfer {
    pub fn new(token_type: String, from: String, to: String, amount: String, memo: String) -> Self {
        ActionTransfer {
            token_type,
            from,
            to,
            amount,
            memo,
        }
    }

    pub fn from_str<T: AsRef<str>>(from: T, to: T, amount: T, memo: T) -> crate::Result<Self> {
        Ok(ActionTransfer {
            token_type: String::from("iost"),
            from: from.as_ref().to_string(),
            to: to.as_ref().to_string(),
            amount: amount.as_ref().to_string(),
            memo: memo.as_ref().to_string(),
        })
    }

    pub fn no_std_serialize(&self) -> String {
        let mut vec: Vec<JsonValue> = Vec::new();
        vec.push(JsonValue::String(
            self.token_type.chars().collect::<Vec<_>>(),
        ));
        vec.push(JsonValue::String(self.from.chars().collect::<Vec<_>>()));
        vec.push(JsonValue::String(self.to.chars().collect::<Vec<_>>()));
        vec.push(JsonValue::String(self.amount.chars().collect::<Vec<_>>()));
        vec.push(JsonValue::String(self.memo.chars().collect::<Vec<_>>()));
        let object = JsonValue::Array(vec);

        String::from_utf8(object.format(4)).unwrap()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Read, Write, NumberBytes, SerializeData)]
#[cfg_attr(feature = "std", derive(SerSerialize))]
#[iost_root_path = "crate"]
pub struct ShadowAction {
    /// contract name
    pub contract: String,
    /// function name of the contract
    pub action_name: String,
    /// Specific parameters of the call. Put every parameter in an array, and JSON-serialize this array. It may looks like ["a_string", 13]
    pub data: String,
}

impl ShadowAction {
    fn from_action(action: &Action) -> crate::Result<ShadowAction> {
        Ok(ShadowAction {
            contract: String::from_utf8(action.contract.clone()).unwrap(),
            action_name: String::from_utf8(action.action_name.clone()).unwrap(),
            data: String::from_utf8(action.data.clone()).unwrap(),
        })
    }
}

pub trait ToAction: Write + NumberBytes {
    const NAME: u64;

    #[inline]
    fn to_action(
        &self,
        contract: String,
        action_name: String,
        data: String,
    ) -> core::result::Result<Action, Error> {
        // let mut data = vec![0_u8; self.num_bytes()];
        // self.write(&mut data, &mut 0).unwrap();

        Ok(Action {
            contract: contract.into_bytes(),
            action_name: action_name.into_bytes(),
            data: data.into_bytes(),
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            contract: s.to_string().into_bytes(),
            action_name: s.to_string().into_bytes(),
            data: s.to_string().into_bytes(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action() {
        let action = Action {
            contract: "iost".to_string().into_bytes(),
            action_name: "iost".to_string().into_bytes(),
            data: "".to_string().into_bytes(),
        };
        let data = action.to_serialize_data().unwrap();

        let sa = ShadowAction {
            contract: "iost".to_string(),
            action_name: "iost".to_string(),
            data: "".to_string(),
        };
        let sa_data = sa.to_serialize_data().unwrap();
        let d_sa = ShadowAction::read(&sa_data, &mut 0).unwrap();
        assert_eq!(data.num_bytes(), sa_data.num_bytes());
        assert_eq!(hex::encode(data), hex::encode(sa_data));
        let other = Action {
            contract: "token.iost".to_string().into_bytes(),
            action_name: "transfer".to_string().into_bytes(),
            data: r#"["iost","admin","lispczz3","100",""]"#.to_string().into_bytes(),
        };
        let mut d = other.to_serialize_data().unwrap();
        let a = Action::read(d.as_ref(), &mut 0).unwrap();
        let s_a = ShadowAction::from_action(&a);
        assert!(s_a.is_ok());
    }

    #[test]
    fn action_serialization() {
        let action = Action {
            contract: "iost".to_string().into_bytes(),
            action_name: "iost".to_string().into_bytes(),
            data: "".to_string().into_bytes(),
        };
        assert_eq!(
            String::from_utf8(action.no_std_serialize().format(4)).unwrap(),
            r#"{
    "contract": "iost",
    "action_name": "iost",
    "data": ""
}"#
        );
    }

    #[test]
    fn test_action_deserialize_should_be_ok1() {
        let action_str = r#"
        {
            "contract": "token.iost",
            "action_name": "transfer",
            "data": "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]"
        }
        "#;
        let result_action: Result<Action, _> = serde_json::from_str(action_str);
        assert!(result_action.is_ok());
    }

    // #[test]
    // fn test_action_transfer() {
    //     let action = Action::transfer("lispczz4", "lispczz5", "10", "").unwrap();
    //     let data = action.no_std_serialize();
    //     dbg!(data);
    // }
}
