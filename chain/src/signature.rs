#![allow(unconditional_recursion)]
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::{Error, NumberBytes, Read, Write};
use core::str::FromStr;
use keys::algorithm;
use lite_json::{JsonObject, JsonValue, Serialize};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize as SerSerialize, Serializer};

#[derive(Clone, Default, Debug, NumberBytes, Write, Read)]
#[cfg_attr(feature = "std", derive(SerSerialize))]
#[iost_root_path = "crate"]
pub struct Signature {
    /// Encryption algorithm. Currently only "ed25519" and "secp256k1" are supported
    pub algorithm: String,
    /// After the contract is serialized, Sha3 is used for hash, and then private key is used for signature. Base64 encoding. See corresponding documents for details
    pub signature: String,
    /// The public key used by this signature. Base64 encoding
    pub public_key: String,
}

impl Signature {
    pub fn sign(message: &[u8], sign_algorithm: &str, sec_key: &[u8]) -> crate::Result<Signature> {
        let algorithm = algorithm::new(sign_algorithm);
        let pub_key = algorithm.get_pub_key(sec_key).unwrap();
        let result = algorithm.sign(message, sec_key);
        Ok(Signature {
            algorithm: sign_algorithm.to_string(),
            signature: base64::encode(result),
            public_key: base64::encode(pub_key),
        })
    }

    pub fn verify(&self, message: &[u8]) -> bool {
        let algorithm = algorithm::new(self.algorithm.as_str());
        let pub_key = base64::decode(self.public_key.as_str()).unwrap();
        let sig = base64::decode(self.signature.as_str()).unwrap();
        algorithm.verify(message, pub_key.as_slice(), sig.as_slice())
    }

    pub fn no_std_serialize(&self) -> JsonValue {
        let object = JsonValue::Object(vec![
            (
                "algorithm".chars().collect::<Vec<_>>(),
                JsonValue::String(self.algorithm.chars().collect()),
            ),
            (
                "signature".chars().collect::<Vec<_>>(),
                JsonValue::String(self.signature.chars().collect()),
            ),
            (
                "public_key".chars().collect::<Vec<_>>(),
                JsonValue::String(self.public_key.chars().collect()),
            ),
        ]);
        object
        // String::from_utf8(object.format(4)).unwrap()
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Debug)]
        struct VisitorSignature;
        impl<'de> serde::de::Visitor<'de> for VisitorSignature {
            type Value = Signature;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::MapAccess<'de>,
            {
                let mut algorithm = String::from("");
                let mut signature = String::from("");
                let mut public_key = String::from("");
                while let Some(field) = map.next_key()? {
                    match field {
                        "algorithm" => {
                            algorithm = map.next_value()?;
                        }
                        "signature" => {
                            signature = map.next_value()?;
                        }
                        "public_key" => {
                            public_key = map.next_value()?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let signature = Signature {
                    algorithm,
                    signature,
                    public_key,
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
        Ok(Signature {
            algorithm: "".to_string(),
            signature: "".to_string(),
            public_key: "".to_string(),
        })
    }
}
