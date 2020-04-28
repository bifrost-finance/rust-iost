#![allow(unconditional_recursion)]
use serde::{Serialize, Deserialize, Serializer};
use iost_derive::{Read, Write};
use std::str::FromStr;
use crate::Error;

#[derive(Clone ,Default, Debug, Write, Read)]
#[iost_root_path = "crate"]

pub struct Signature {
    /// Encryption algorithm. Currently only "ed25519" and "secp256k1" are supported
    pub algorithm: String,
    /// After the contract is serialized, Sha3 is used for hash, and then private key is used for signature. Base64 encoding. See corresponding documents for details
    pub signature: String,
    /// The public key used by this signature. Base64 encoding
    pub public_key: String
}

impl Signature {
    pub fn as_bytes(&self) -> &[u8] {
        (&self.signature).as_bytes();
        (&self.algorithm).as_bytes();
        (&self.public_key).as_bytes()
    }

    pub const fn to_bytes(&self) -> [u8; 65] {
        self.to_bytes()
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.algorithm)
    }
}

impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorSignature;
        impl<'de> serde::de::Visitor<'de> for VisitorSignature {
            type Value = Signature;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                while let Some(field) = map.next_key()? {
                    match field {
                        "algorithm" => {
                            let _algorithm = map.next_value()?;
                        }
                        "signature" => {
                            let _signature = map.next_value()?;
                        }
                        "public_key" => {
                            let _public_key= map.next_value()?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let signature = Signature{
                    algorithm: "".to_string(),
                    signature: "".to_string(),
                    public_key: "".to_string()
                };
                Ok(signature)
            }

        }
        deserializer.deserialize_any(VisitorSignature)
    }
}

impl FromStr for Signature {
    type Err = Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Signature{
            algorithm: "".to_string(),
            signature: "".to_string(),
            public_key: "".to_string()
        })
    }
}