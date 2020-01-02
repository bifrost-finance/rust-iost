//extern crate crypto;
//use crypto::*;
//use ed25519::{keypair, signature, verify, exchange};
//use curve25519::{curve25519_base, curve25519};
//use digest::Digest;
//use sha2::Sha512;
//fn main() {
//    let seed: &[u8] = &[0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31, 0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e];//32位
//
//    // KEYGEN
//    let (private_key, public_key) = keypair(seed); //[U8,64]
//
//    let message = b"This is my message!";
//
//    //私钥签名
//    let sig = signature(message, &private_key); //[U8,64]
//
//    //private_key
//    println!("private_key: {:?} ", private_key.to_vec());
//    println!("private_key_len :{:? }", private_key.len());
//
//    // public_key
//    println!("public_key :{:?}", public_key.to_vec());
//    println!("public_key_len :{:?}", public_key.len());
//
//    //signature
//    println!("signature:{:?}", sig.to_vec());
//    println!("signature_len:{:?}", sig.len());
//
//    // verify
//    println!("验证是否成功：{:?} ",verify(message, &public_key, &sig));
//}