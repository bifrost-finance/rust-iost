use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    /// Encryption algorithm. Currently only "ed25519" and "secp256k1" are supported
    pub algorithm: String,
    /// After the contract is serialized, Sha3 is used for hash, and then private key is used for signature. Base64 encoding. See corresponding documents for details
    pub signature: String,
    /// The public key used by this signature. Base64 encoding
    pub public_key: String
}