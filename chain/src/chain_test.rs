use alloc::string::String;
use core::{iter::FromIterator, str::FromStr};
use lite_json::{parse_json, JsonValue};

const CHAIN_ID: [char; 8] = ['c', 'h', 'a', 'i', 'n', '_', 'i', 'd']; // key chain_id
const HEAD_BLOCK_HASH: [char; 15] = [
    'h', 'e', 'a', 'd', '_', 'b', 'l', 'o', 'c', 'k', '_', 'h', 'a', 's', 'h',
]; // key head_block_hash
#[test]
fn test() {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("http://127.0.0.1:30001/getChainInfo")
        .send()
        .unwrap();
    let mut chain_id = 0;
    let mut head_block_hash = Default::default();
    let node_info = parse_json(res.text().unwrap().as_str()).unwrap();
    match node_info {
        JsonValue::Object(ref json) => {
            for item in json.iter() {
                if item.0 == CHAIN_ID {
                    chain_id = {
                        match item.1.clone() {
                            JsonValue::Number(numberValue) => numberValue.to_f64() as i32,
                            _ => -1,
                        }
                    };
                }
                if item.0 == HEAD_BLOCK_HASH {
                    head_block_hash = {
                        match item.1 {
                            JsonValue::String(ref chars) => String::from_iter(chars.iter()),
                            _ => "".to_string(),
                        }
                    };
                }
            }
        }
        _ => (),
    }
    dbg!(head_block_hash);
}
