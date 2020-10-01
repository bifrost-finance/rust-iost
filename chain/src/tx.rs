use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::Error::{BytesReadError, InvalidPublisherSignature, InvalidSignature};
use crate::{
    Action, AmountLimit, NumberBytes, Read, ReadError, SerializeData, Signature, Write, WriteError,
};
use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use keys::algorithm;
use lite_json::{JsonObject, JsonValue, NumberValue, Serialize};
#[cfg(feature = "std")]
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize as SerSerialize};
use sha3::{Digest, Sha3_256};

#[derive(Clone, Default, Debug, Read, Write, NumberBytes, SerializeData)]
#[cfg_attr(feature = "std", derive(Deserialize, SerSerialize))]
#[iost_root_path = "crate"]
pub struct Tx {
    /// Time of transaction. Unixepoch start in nanoseconds
    pub time: i64,
    /// Transaction expiration time. Unixepoch starts in nanoseconds. If the chunk node does not receive the transaction until after the expiration time, it will not execute
    pub expiration: i64,
    /// GAS multiplying rate. This transaction shall be paid according to the gas ratio of the default gas. The higher the multiplier, the higher the priority. The reasonable value range is [1.0, 100.0]
    pub gas_ratio: f64,
    /// The maximum allowed gas of the transaction, with a minimum setting of 50000
    pub gas_limit: f64,
    /// Used in delayed transactions. The number of nanoseconds to delay execution. Non delayed transaction set to 0
    pub delay: i64,
    /// Network ID
    pub chain_id: u32,
    /// Specific call in transaction
    pub actions: Vec<Action>,
    /// Token restrictions on transactions. You can specify multiple tokens and a corresponding number limit. If the transaction exceeds these limits, execution fails
    pub amount_limit: Vec<AmountLimit>,
    /// ID of the transaction sender
    pub publisher: String,
    /// Publisher's signature. The signing process is as follows. Publisher can provide multiple signatures with different permissions. You can refer to the documentation of the permission system
    pub publisher_sigs: Vec<Signature>,
    /// Signer ID other than publisher. It can be empty.
    pub signers: Vec<String>,
    /// Signature of signers. Each signer can have one or more signatures, so the length is not less than the length of signers
    pub signatures: Vec<Signature>,
}

fn expand<T>(x: &Vec<T>, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError>
where
    T: NumberBytes + Write,
{
    for item in x.iter() {
        item.num_bytes().write(bytes, pos)?;
        item.write(bytes, pos)?;
    }
    Ok(())
}

impl Tx {
    pub fn new(time: i64, expiration: i64, chain_id: u32, actions: Vec<Action>) -> Self {
        let amount_limit = AmountLimit {
            token: "*".to_string(),
            value: "unlimited".to_string(),
        };

        Tx {
            time,
            expiration,
            gas_ratio: 1.0,
            gas_limit: 1000000.0,
            delay: 0,
            chain_id,
            actions,
            amount_limit: vec![amount_limit],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![],
        }
    }

    #[cfg(feature = "std")]
    pub fn from_action(actions: Vec<Action>) -> Self {
        let amount_limit = AmountLimit {
            token: "*".to_string(),
            value: "unlimited".to_string(),
        };
        let time = Utc::now().timestamp_nanos();
        let expiration = time + Duration::seconds(10000).num_nanoseconds().unwrap();
        Tx {
            time,
            expiration,
            gas_ratio: 1.0,
            gas_limit: 1000000.0,
            delay: 0,
            chain_id: 1024,
            actions,
            amount_limit: vec![amount_limit],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![],
        }
    }

    pub fn no_std_serialize_vec(self) -> Vec<u8> {
        let object = JsonValue::Object(vec![
            (
                "time".chars().collect::<Vec<_>>(),
                JsonValue::Number(NumberValue {
                    integer: self.time,
                    fraction: 0,
                    fraction_length: 0,
                    exponent: 0,
                }),
            ),
            (
                "expiration".chars().collect::<Vec<_>>(),
                JsonValue::Number(NumberValue {
                    integer: self.expiration,
                    fraction: 0,
                    fraction_length: 0,
                    exponent: 0,
                }),
            ),
            (
                "gas_ratio".chars().collect::<Vec<_>>(),
                JsonValue::Number(NumberValue {
                    integer: self.gas_ratio as i64,
                    fraction: 0,
                    fraction_length: 0,
                    exponent: 0,
                }),
            ),
            (
                "gas_limit".chars().collect::<Vec<_>>(),
                JsonValue::Number(NumberValue {
                    integer: self.gas_limit as i64,
                    fraction: 0,
                    fraction_length: 0,
                    exponent: 0,
                }),
            ),
            (
                "delay".chars().collect::<Vec<_>>(),
                JsonValue::Number(NumberValue {
                    integer: self.delay,
                    fraction: 0,
                    fraction_length: 0,
                    exponent: 0,
                }),
            ),
            (
                "chain_id".chars().collect::<Vec<_>>(),
                JsonValue::Number(NumberValue {
                    integer: self.chain_id as i64,
                    fraction: 0,
                    fraction_length: 0,
                    exponent: 0,
                }),
            ),
            (
                "signers".chars().collect::<Vec<_>>(),
                JsonValue::Array(
                    self.signers
                        .iter()
                        .map(|e| JsonValue::String(e.chars().collect::<Vec<_>>()))
                        .collect(),
                ),
            ),
            (
                "actions".chars().collect::<Vec<_>>(),
                JsonValue::Array(self.actions.iter().map(|e| e.no_std_serialize()).collect()),
            ),
            (
                "amount_limit".chars().collect::<Vec<_>>(),
                JsonValue::Array(
                    self.amount_limit
                        .iter()
                        .map(|e| e.no_std_serialize())
                        .collect(),
                ),
            ),
            (
                "signatures".chars().collect::<Vec<_>>(),
                JsonValue::Array(
                    self.signatures
                        .iter()
                        .map(|e| e.no_std_serialize())
                        .collect(),
                ),
            ),
            (
                "publisher".chars().collect::<Vec<_>>(),
                JsonValue::String(self.publisher.chars().collect::<Vec<_>>()),
            ),
            (
                "publisher_sigs".chars().collect::<Vec<_>>(),
                JsonValue::Array(
                    self.publisher_sigs
                        .iter()
                        .map(|e| e.no_std_serialize())
                        .collect(),
                ),
            ),
        ]);
        object.format(4)
    }

    pub fn no_std_serialize(self) -> String {
        String::from_utf8(self.no_std_serialize_vec()).unwrap()
    }

    #[inline]
    fn number_bytes(&self, with_sign: bool) -> usize {
        let res = 48
            + self.signers.num_bytes()
            + self.signers.len() * 4
            + self.actions.num_bytes()
            + self.actions.len() * 4
            + self.amount_limit.num_bytes()
            + self.amount_limit.len() * 4;
        if with_sign {
            res + self.signatures.num_bytes() + self.signatures.len() * 4
        } else {
            res
        }
    }

    fn write(&self, bytes: &mut [u8], pos: &mut usize, with_sign: bool) -> Result<(), WriteError> {
        self.time.clone().write(bytes, pos);
        self.expiration.clone().write(bytes, pos);
        let mut ratio = (self.gas_ratio * 100.0) as i64;
        ratio.write(bytes, pos);
        let mut limit = (self.gas_limit * 100.0) as i64;
        limit.write(bytes, pos);
        self.delay.clone().write(bytes, pos);
        (self.chain_id.clone() as i32).write(bytes, pos);

        // reserved field
        0_i32.write(bytes, pos);

        self.signers.len().write(bytes, pos)?;
        expand::<String>(&self.signers, bytes, pos);
        self.actions.len().write(bytes, pos);
        expand::<Action>(&self.actions, bytes, pos);
        self.amount_limit.len().write(bytes, pos);
        expand::<AmountLimit>(&self.amount_limit, bytes, pos);
        if with_sign {
            self.signatures.len().write(bytes, pos);
            expand::<Signature>(&self.signatures, bytes, pos);
        }
        Ok(())
    }

    pub fn customized_to_serialize_data(&self, with_sign: bool) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.number_bytes(with_sign)];
        self.write(&mut data, &mut 0, with_sign)
            .map_err(crate::Error::BytesWriteError)
            .unwrap();
        Ok(data.to_vec())
    }

    pub fn sign(
        &mut self,
        account_name: String,
        sign_algorithm: &str,
        sec_key: &[u8],
    ) -> Result<(), WriteError> {
        self.publisher = account_name;

        if self.publisher_sigs.len() == 0 {
            let tx_bytes = self.customized_to_serialize_data(true).unwrap();
            // create a SHA3-256 object
            let mut hasher = Sha3_256::new();
            hasher.input(tx_bytes);
            let result = hasher.result();
            self.publisher_sigs =
                vec![Signature::sign(result.as_ref(), sign_algorithm, sec_key).unwrap()];
        }
        Ok(())
    }

    pub fn verify(&self) -> crate::Result<()> {
        for signature in &self.signatures {
            let tx_bytes = self.customized_to_serialize_data(false).unwrap();
            let mut hasher = Sha3_256::new();
            hasher.input(tx_bytes);
            let result = hasher.result();
            if !signature.verify(result.as_slice()) {
                return Err(InvalidSignature());
            }
        }
        for publisher_sig in &self.publisher_sigs {
            let tx_bytes = self.customized_to_serialize_data(true).unwrap();
            let mut hasher = Sha3_256::new();
            hasher.input(tx_bytes);
            let result = hasher.result();
            if !publisher_sig.verify(result.as_slice()) {
                return Err(InvalidPublisherSignature());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use base58::FromBase58;

    #[test]
    fn test_bytes_serialization() {
        let tx = Tx::from_action(vec![Action {
            contract: "token.iost".to_string().into_bytes(),
            action_name: "transfer".to_string().into_bytes(),
            data: r#"["iost","admin","lispczz3","100",""]"#.to_string().into_bytes(),
        }]);

        let mut data = tx.to_serialize_data().unwrap();
        // assert!(data.is_ok());
        dbg!(tx.num_bytes());
        dbg!(data.len());

        let other_tx = Tx::read(&mut data, &mut 0).unwrap();
        dbg!(other_tx);
    }

    #[test]
    fn test_send_tx() {
        let action = Action::transfer("lispczz4", "lispczz5", "10", "").unwrap();
        // let mut tx = Tx::from_action(vec![Action {
        //     contract: "token.iost".to_string().into_bytes(),
        //     action_name: "transfer".to_string().into_bytes(),
        //     data: r#"["iost","lispczz4","lispczz5","8",""]"#.to_string().into_bytes(),
        // }]);

        let mut tx = Tx::from_action(vec![action]);
        // let sec_key = "2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1".from_base58().unwrap();
        let sec_key = "xjggJ3TrLXz7qEwrGG3Rc4Fz59imjixhXpViq9W7Ncx"
            .from_base58()
            .unwrap();
        // let sec_key = base64::decode("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1").unwrap();
        tx.sign(
            "lispczz4".to_string(),
            algorithm::SECP256K1,
            sec_key.as_slice(),
        );
        let result = tx.verify();
        assert!(result.is_ok());

        // let tx_string = serde_json::to_string_pretty(&tx).unwrap();
        let tx_string = tx.no_std_serialize();
        // dbg!(tx_string);
        // dbg!(tx.no_std_serialize());
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("http://127.0.0.1:30001/sendTx")
            .body(tx_string)
            .send()
            .unwrap();
        dbg!(res.text());
    }

    #[test]
    fn test_tx() {
        let mut tx = Tx {
            time: 1598918258274417000,
            expiration: 1598918348274417000,
            gas_ratio: 1.0,
            gas_limit: 1000000.0,
            delay: 0,
            chain_id: 1024,
            actions: vec![Action {
                contract: "token.iost".to_string().into_bytes(),
                action_name: "transfer".to_string().into_bytes(),
                data: r#"["iost","admin","lispczz3","100",""]"#.to_string().into_bytes(),
            }],
            amount_limit: vec![AmountLimit {
                token: "*".to_string(),
                value: "unlimited".to_string(),
            }],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![],
        };
        // let data: Vec<u8> = tx.to_serialize_data().unwrap();
        let sec_key = "2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1".from_base58().unwrap();
        // let sec_key = base64::decode("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1").unwrap();
        tx.sign("admin".to_string(), algorithm::ED25519, sec_key.as_slice());
        // let s = String::from_utf8(data.clone());
        // dbg!(hex::encode(data.as_slice()));
        let result = tx.verify();
        assert!(result.is_ok());

        let tx_string = serde_json::to_string_pretty(&tx).unwrap();
        dbg!(tx_string);
        // create a SHA3-256 object
        // let mut hasher = Sha3_256::new();
        // "Fpl2AbiSgVxJzhOU1ASofiYoLf0uqXlIWz0hXroxd0i38BfJVErzVdR7mQP1SEXk1sKz98i+fPDyPmRY56WbDA=="

        // "6BK1LqmtXLqamvA6/MbylCpFJLDfPANE3BlQcoMWcMQ="
        // write input message
        // let data = result.unwrap();
        // hasher.input(data);
        // let result = hasher.result();
        // dbg!(result.as_slice());
        // assert_eq!(
        //     "93c24341c06cd7a23023d278dd044bf736730ac5e32d432aff05a00ac3df85f8",
        //     hex::encode(result.as_slice())
        // );
    }

    #[test]
    fn test_no_std_serialize() {
        let mut tx = Tx {
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1.0,
            gas_limit: 500000.0,
            delay: 0,
            chain_id: 1024,
            actions: vec![ Action {
                contract: "token.iost".to_string().into_bytes(),
                action_name: "transfer".to_string().into_bytes(),
                data: r#"["iost", "testaccount", "anothertest", "100", "this is an example transfer"]"#.to_string().into_bytes(),
            }],
            amount_limit: vec![ AmountLimit {
                token: "*".to_string(),
                value: "unlimited".to_string()
            }],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![]
        };
        let result = tx.no_std_serialize();
        // println!("{}", String::from_utf8_lossy(&result[..]));
        dbg!(result);
    }

    #[test]
    fn should_tx_sign_be_ok() {
        let mut tx = Tx {
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1.0,
            gas_limit: 500000.0,
            delay: 0,
            chain_id: 1024,
            actions: vec![ Action {
                contract: "token.iost".to_string().into_bytes(),
                action_name: "transfer".to_string().into_bytes(),
                data: "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]".to_string().into_bytes(),
            }],
            amount_limit: vec![ AmountLimit {
                token: "*".to_string(),
                value: "unlimited".to_string()
            }],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![]
        };

        let sec_key = base64::decode("gkpobuI3gbFGstgfdymLBQAGR67ulguDzNmLXEJSWaGUNL5J0z5qJUdsPJdqm+uyDIrEWD2Ym4dY9lv8g0FFZg==").unwrap();
        tx.sign(
            "testaccount".to_string(),
            algorithm::ED25519,
            sec_key.as_slice(),
        );
        assert!(tx.verify().is_ok());

        let tx_str = r#"
        {
            "time": 1544709662543340000,
            "expiration": 1544709692318715000,
            "gas_ratio": 1,
            "gas_limit": 500000,
            "delay": 0,
            "chain_id": 1024,
            "signers": [],
            "actions": [{
                "contract": "token.iost",
                "action_name": "transfer",
                "data": "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]"
            }],
            "amount_limit": [{
                "token": "*",
                "value": "unlimited"
            }],
            "signatures": [],
            "publisher": "testaccount",
            "publisher_sigs": [{
                "algorithm": "ED25519",
                "public_key": "lDS+SdM+aiVHbDyXapvrsgyKxFg9mJuHWPZb/INBRWY=",
                "signature": "/K1HM0OEbfJ4+D3BmalpLmb03WS7BeCz4nVHBNbDrx3/A31aN2RJNxyEKhv+VSoWctfevDNRnL1kadRVxSt8CA=="
            }]
        }
        "#;
        let tx_struct: Result<Tx, _> = serde_json::from_str(tx_str);

        assert!(tx_struct.is_ok());
        if let Ok(expected_tx) = tx_struct {
            // let result = tx.to_serialize_data();
            // assert!(result.is_ok());
            assert_eq!(tx.publisher_sigs.len(), 1);
            assert_eq!(tx.publisher_sigs.len(), expected_tx.publisher_sigs.len());
            assert_eq!(
                tx.publisher_sigs[0].public_key,
                expected_tx.publisher_sigs[0].public_key
            );
            assert_eq!(
                tx.publisher_sigs[0].signature,
                expected_tx.publisher_sigs[0].signature
            );
        }
    }
}
