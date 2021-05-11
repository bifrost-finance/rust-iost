use alloc::string::ToString;
use core::fmt;

use crate::base58;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// Base58 encoding error
    Base58(base58::Error),
    /// secp256k1-related error
    Secp256k1(secp256k1::Error),

    ErrorEd25519,

    ErrorSecp256k1,
    /// hash error
    Hash(bitcoin_hashes::error::Error),
    /// verify failed
    VerifyFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Base58(ref e) => fmt::Display::fmt(e, f),
            Error::Secp256k1(ref e) => f.write_str(&e.to_string()),
            Error::ErrorEd25519 => f.write_str("Ed25519 failed"),
            Error::Hash(ref e) => f.write_str(&e.to_string()),
            Error::VerifyFailed => f.write_str("Verify failed"),
            Error::ErrorSecp256k1 => f.write_str("Secp256k1 failed"),
        }
    }
}

impl From<base58::Error> for Error {
    fn from(e: base58::Error) -> Error {
        Error::Base58(e)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(e: secp256k1::Error) -> Error {
        Error::Secp256k1(e)
    }
}

impl From<bitcoin_hashes::error::Error> for Error {
    fn from(e: bitcoin_hashes::error::Error) -> Error {
        Error::Hash(e)
    }
}
