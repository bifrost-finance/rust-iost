use alloc::vec::Vec;
use core::str::from_utf8;

use codec::{Decode, Encode};

use crate::spv::{Head, Sign};

use super::super::NumberBytes;

#[derive(Clone, Debug, NumberBytes, PartialEq, Default, Encode, Decode)]
#[iost_root_path = "crate"]
pub struct BlockHead {
    pub version: i64,

    pub parent_hash: Vec<u8>,

    pub tx_merkle_hash: Vec<u8>,

    pub tx_receipt_merkle_hash: Vec<u8>,

    pub info: Vec<u8>,

    pub number: i64,

    pub witness: Vec<u8>,

    pub time: i64,

    pub hash: Vec<u8>,

    pub algorithm: u8,
    pub sig: Vec<u8>,
    pub pub_key: Vec<u8>,
}

impl BlockHead {
    pub fn parse_head(&self) -> Head {
        let mut head = Head {
            version: self.version,
            parent_hash: self.parent_hash.clone(),
            tx_merkle_hash: self.tx_merkle_hash.clone(),
            tx_receipt_merkle_hash: self.tx_receipt_merkle_hash.clone(),
            info: self.info.clone(),
            number: self.number,
            witness: "".to_string(),
            time: self.time,
        };
        head.witness = core::str::from_utf8(self.witness.as_slice())
            .unwrap()
            .to_string();
        return head;
    }

    pub fn parse_sign(&self) -> Sign {
        Sign {
            algorithm: self.algorithm,
            sig: from_utf8(self.sig.as_slice()).unwrap().to_string(),
            pub_key: from_utf8(self.pub_key.as_slice()).unwrap().to_string(),
        }
    }

    pub fn verify_self(&self) -> bool {
        let head = self.parse_head();
        let sign = self.parse_sign();
        return head.verify(sign);
    }
}
