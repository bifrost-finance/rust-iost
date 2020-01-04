/// Count the number of bytes a type is expected to use.
pub trait NumberBytes {
    /// Count the number of bytes a type is expected to use.
    fn num_bytes(&self) -> usize;
}

/// Read bytes.
pub trait Read: Sized {
    /// Read bytes.
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError>;
}

/// Error that can be returned when reading bytes.
pub enum ReadError {
    /// Not enough bytes.
    NotEnoughBytes,
    /// Not support message type.
    NotSupportMessageType,
}

/// Write bytes.
pub trait Write: Sized {
    /// Write bytes.
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError>;
}

/// Error that can be returned when writing bytes.
#[derive(Debug, Clone, Copy)]
pub enum WriteError {
    /// Not enough space in the vector.
    NotEnoughSpace,
    /// Failed to parse an integer.
    TryFromIntError,
}

