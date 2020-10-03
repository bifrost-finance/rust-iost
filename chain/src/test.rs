use alloc::vec::Vec;
use alloc::{format, vec};

use crate::time_point::TimePoint;
use crate::{Action, AmountLimit, NumberBytes, Read, Result, SerializeData, Tx, Write};
// use base58::{FromBase58, ToBase58};
use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use keys::algorithm;

#[test]
fn print_hex() {
    // let raw_string = "jim".to_string().into_bytes();
    let raw_string = "lispczz5".to_string().into_bytes();
    dbg!(hex::encode(raw_string.as_slice()));

    dbg!(String::from_utf8(
        "fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e"
            .as_bytes()
            .to_vec()
    ));
}

#[test]
fn should_action_serialization_successful() {
    let action = Action::new(
        String::from("'token.iost'"),
        String::from("'transfer'"),
        String::from(r#"["iost","admin","lispczz","10.12034123",""]"#),
    );

    let result: Result<Vec<u8>> = action.to_serialize_data();
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data.len(), 77);
}

#[test]
fn should_amount_limit_serialization_successful() {
    let amount_limit = AmountLimit {
        token: "*".to_string(),
        value: "unlimited".to_string(),
    };
    let result: Result<Vec<u8>> = amount_limit.to_serialize_data();
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data.len(), 18);
}

use sha3::{Digest, Sha3_256};

#[test]
fn should_tx_serialization_and_sign_successful() {
    let action = Action::new(
        String::from("'token.iost'"),
        String::from("'transfer'"),
        String::from(r#"["iost","admin","lispczz","10.12034123",""]"#),
    );

    let amount_limit = AmountLimit {
        token: "*".to_string(),
        value: "unlimited".to_string(),
    };
    let time = Utc::now().timestamp_nanos();
    let expiration = time + Duration::seconds(10000).num_nanoseconds().unwrap();

    let tx = Tx {
        time: 1597313425200542000,
        expiration: 1597313515200542000,
        gas_ratio: 1.0,
        gas_limit: 1000000.0,
        delay: 0,
        chain_id: 1024,
        actions: vec![action.clone()],
        amount_limit: vec![amount_limit],
        publisher: "".to_string(),
        publisher_sigs: vec![],
        signers: vec![],
        signatures: vec![],
    };
    // dbg!(tx.num_bytes());
    let result = tx.to_serialize_data();
    assert!(result.is_ok());

    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();

    // write input message
    let data = result.unwrap();
    hasher.input(data);
    let result = hasher.result();
    // dbg!(result.as_slice());
    assert_eq!(
        "93c24341c06cd7a23023d278dd044bf736730ac5e32d432aff05a00ac3df85f8",
        hex::encode(result.as_slice())
    );
}
