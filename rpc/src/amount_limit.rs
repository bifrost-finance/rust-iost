use serde::{Serialize, Deserialize};
use iost_derive::{Read, Write};

#[derive(Clone , Default, Serialize, Debug, Write, Read)]
#[iost_root_path = "crate"]
pub struct AmountLimit {
    /// token name
    pub token: String,
    /// corresponding token limit
    pub value: String
}

impl<'de> serde::Deserialize<'de> for AmountLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorAmountLimit;
        impl<'de> serde::de::Visitor<'de> for VisitorAmountLimit{
            type Value = AmountLimit;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                while let Some(field) = map.next_key()? {
                    match field {
                        "token" => {
                            let _token = map.next_value()?;
                        }
                        "value" => {
                            let _value = map.next_value()?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let amount_limit = AmountLimit {
                    token: "".to_string(),
                    value: "".to_string()
                };
                Ok(amount_limit)
            }

        }
        deserializer.deserialize_any(VisitorAmountLimit)
    }
}
