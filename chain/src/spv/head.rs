use alloc::string::{String, ToString};
use alloc::vec::Vec;

use base64;
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize};
use sha3::{Digest, Sha3_256};

use keys::algorithm;

use crate::spv::Sign;
use crate::verify::BlockHead;
use crate::{NumberBytes, Read, SerializeData, Write};

#[derive(Debug, Clone, NumberBytes, SerializeData, Write, Read)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[iost_root_path = "crate"]
pub struct Head {
    // #[cfg(feature = "std")]
    // #[serde(deserialize_with = "de_string_to_i64")]
    pub version: i64,

    // #[cfg(feature = "std")]
    // #[serde(rename = "parentHash")]
    // #[serde(deserialize_with = "base64_de_string_to_bytes")]
    pub parent_hash: Vec<u8>,

    // #[cfg(feature = "std")]
    // #[serde(rename = "txMerkleHash")]
    // #[serde(deserialize_with = "base64_de_string_to_bytes")]
    pub tx_merkle_hash: Vec<u8>,

    // #[cfg(feature = "std")]
    // #[serde(rename = "txReceiptMerkleHash")]
    // #[serde(deserialize_with = "base64_de_string_to_bytes")]
    pub tx_receipt_merkle_hash: Vec<u8>,

    // #[cfg(feature = "std")]
    // #[serde(deserialize_with = "base64_de_string_to_bytes")]
    pub info: Vec<u8>,

    // #[cfg(feature = "std")]
    // #[serde(deserialize_with = "de_string_to_i64")]
    pub number: i64,

    pub witness: String,

    // #[cfg(feature = "std")]
    // #[serde(deserialize_with = "de_string_to_i64")]
    pub time: i64,
}

impl Head {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        let head_bytes = self.to_serialize_data().unwrap();
        hasher.input(head_bytes);
        return hasher.result().to_vec();
    }

    pub fn verify(&self, sign: Sign) -> bool {
        let ed25519 = algorithm::new(algorithm::ED25519);
        let sign = base64::decode(sign.sig.as_str()).unwrap();
        let pub_key = bs58::decode(self.witness.as_str()).into_vec().unwrap();
        ed25519.verify(self.hash().as_slice(), pub_key.as_slice(), sign.as_slice())
    }
}

pub fn from_block_head(bh: &BlockHead) -> Head {
    let mut head = Head {
        version: bh.version,
        parent_hash: bh.parent_hash.clone(),
        tx_merkle_hash: bh.tx_merkle_hash.clone(),
        tx_receipt_merkle_hash: bh.tx_receipt_merkle_hash.clone(),
        info: bh.info.clone(),
        number: bh.number,
        witness: "".to_string(),
        time: bh.time,
    };
    head.witness = core::str::from_utf8(bh.witness.as_slice())
        .unwrap()
        .to_string();
    return head;
}

#[cfg(feature = "std")]
pub fn base64_de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    let res = base64::decode(s).unwrap();
    Ok(res.to_vec())
}

#[cfg(feature = "std")]
pub fn de_string_to_i64<'de, D>(de: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    let res = s.parse::<i64>().unwrap();
    Ok(res)
}
