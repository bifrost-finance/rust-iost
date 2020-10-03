use crate::{ErrorMessage, ParseNameError, ReadError, WriteError};

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
}
