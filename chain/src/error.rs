use crate::{ErrorMessage, ParseNameError, ReadError, WriteError};
use alloc::string::String;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BytesReadError(ReadError),
    BytesWriteError(WriteError),

    JsonParserError(),
    ///Error response message
    ErrorMessage(ErrorMessage),

    ParseNameErr(ParseNameError),

    FixedParseOverflow(),
    FixedParseAbnormalChar(),
    FixedParseAmountFormat(),
    FixedParseDivideByZero(),
    FixedParseDoubleDot(),

    InvalidSignature(),
    InvalidPublisherSignature(),

    InvalidSPVStartBlock(i64),
    IOSTBlockError(),

    IOSTBlockVerifyError(String),
    IOSTInvalidBlockSignature(),
    IOSTUpdateEpochError(String),
    IOSTBlockWitnessError(String),
}
