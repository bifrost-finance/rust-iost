use byteorder::{BigEndian, WriteBytesExt};
use crate::unsigned_int::UnsignedInt;

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

macro_rules! impl_num {
    ($($t:ty, $s:expr)*) => ($(
        impl NumberBytes for $t
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                $s
            }
        }

        impl Read for $t {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                let width: usize = $s;
                let mut num = <Self as From<u8>>::from(0_u8);
                for i in 0..width {
                    match bytes.get(*pos) {
                        Some(b) => {
                            let shift = <Self as From<u8>>::from(i as u8).saturating_mul(<Self as From<u8>>::from(8_u8));
                            num |= <Self as From<u8>>::from(*b) << shift;
//                            num |= <Self as From<u8>>::from(*b) << width;
                        }
                        None => return Err(ReadError::NotEnoughBytes),
                    }
                    *pos = pos.saturating_add(1);
                }
                Ok(num)
            }
        }

        impl Write for $t
        {
            #[inline]
            fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
                let width: usize = $s;
                let ff = <Self as From<u8>>::from(0xff);
                for i in 0..width {
                    // TODO rework this to dynamically allocate?
                    match bytes.get_mut(*pos) {
                        Some(byte) => {
                            let shift = <Self as From<u8>>::from(i as u8).saturating_mul(<Self as From<u8>>::from(8_u8));
                            *byte = ((*self >> shift) & ff) as u8;
                        }
                        None => return Err(WriteError::NotEnoughSpace),
                    }
                    *pos = pos.saturating_add(1);
                }

                Ok(())
            }
        }
    )*)
}

impl_num!(
    u8, 1
    u16, 2
    i16, 2
    u32, 4
    i32, 4
    u64, 8
    i64, 8
);

impl NumberBytes for f32 {
    #[inline]
    fn num_bytes(&self) -> usize {
        4
    }
}

impl Read for f32 {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u32::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Write for f32 {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.to_bits().write(bytes, pos)
    }
}

impl NumberBytes for f64 {
    #[inline]
    fn num_bytes(&self) -> usize {
        8
    }
}

impl Read for f64 {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u64::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Write for f64 {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.to_bits().write(bytes, pos)
    }
}

impl NumberBytes for bool {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl Read for bool {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v == 1)
    }
}

impl Write for bool {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes, pos)
    }
}

impl NumberBytes for char {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl Read for char {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v as Self)
    }
}

impl Write for char {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        (*self as u8).write(bytes, pos)
    }
}

impl NumberBytes for usize {
    #[inline]
    fn num_bytes(&self) -> usize {
        UnsignedInt::from(*self).num_bytes()
    }
}

impl Read for usize {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
//        UnsignedInt::read(bytes, pos).map(core::convert::Into::into)
        UnsignedInt::read(bytes, pos).map(core::convert::Into::into)
    }
}

impl Write for usize {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        UnsignedInt::from(*self).write(bytes, pos)
    }
}

impl<T> NumberBytes for Vec<T>
    where
        T: NumberBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.as_slice().num_bytes()
    }
}

impl<T> Read for Vec<T>
    where
        T: Read + Default + Clone,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let capacity = usize::read(bytes, pos)?;

        let mut results = Self::new();
        results.resize(capacity, T::default());

        for item in &mut results {
            let r = T::read(bytes, pos)?;
            *item = r;
        }

        Ok(results)
    }
}

impl<T> Write for Vec<T>
    where
        T: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_slice().write(bytes, pos)
    }
}

impl<T> NumberBytes for &[T]
    where
        T: NumberBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = self.len().num_bytes();
        for item in self.iter() {
            count += item.num_bytes();
        }
        count
    }
}

impl<T> Write for &[T]
    where
        T: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.len().write(bytes, pos)?;
        for item in self.iter() {
            item.write(bytes, pos)?;
        }
        Ok(())
    }
}

impl<T> NumberBytes for Option<T>
    where
        T: NumberBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = self.is_some().num_bytes();
        if let Some(t) = self {
            count += t.num_bytes();
        }
        count
    }
}

impl<T> Read for Option<T>
    where
        T: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let is_some = bool::read(bytes, pos)?;
        if is_some {
            Ok(Some(T::read(bytes, pos)?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Write for Option<T>
    where
        T: Write + Default,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.is_some().write(bytes, pos)?;
        match self {
            Some(item) => item.write(bytes, pos),
            None => Ok(()),
        }
    }
}

impl NumberBytes for String {
    #[inline]
    fn num_bytes(&self) -> usize {
        self.as_str().num_bytes()
    }
}

impl Read for String {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        // TODO: may need to read this as a cstr
        let utf8 = Vec::<u8>::read(bytes, pos)?;
        let s = Self::from_utf8_lossy(&utf8);
        Ok(s.into_owned())
    }
}

impl Write for String {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

impl<'a> NumberBytes for &'a str {
    #[inline]
    fn num_bytes(&self) -> usize {
        let len = self.len();
        len.num_bytes() + len
    }
}

impl<'a> Write for &'a str {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use iost_derive::{Read, Write, NumberBytes};

    #[derive(Read, Write, NumberBytes, Debug)]
    #[iost_root_path="crate"]
    struct Thing {
        a: u64,
        b: u64,
        c: u32,
    }

    #[test]
    fn test2() {
        let bytes = &[
            10, 9, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 20, 4, 3, 2, 1, 1,
            1, 1, 1,
        ];

        let mut pos = 0;
        let a = u8::read(bytes, &mut pos);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test3() {
        let bytes = &mut [0u8; 100];

        let mut pos = 0;

        let a = 5u8.write(bytes, &mut pos);

        dbg!(a);

    }

    #[test]
    fn test4() {
        let thing = Thing{
            a: 1023,
            b: 2,
            c: 3
        };

        let mut bytes = [0u8; 100];
        let mut write_pos = 0;
        let a = thing.write(&mut bytes, &mut write_pos);
        dbg!(a);
    }

    #[test]
    fn big_endian_should_be_ok() {
        let mut wtr = Vec::new();
        dbg!(wtr.write_i64::<BigEndian>(1023).unwrap());
        dbg!(wtr);

        let s = String::from("iost");
        let length = s.len() as u32;
        let mut arr = Vec::new();
        arr.write_u32::<BigEndian>(length).unwrap();
        let mut bytes= s.into_bytes() ;
        arr.reverse();
        for i in arr {
            bytes.insert(0,i);
        }
        dbg!(bytes);

        let arr_t = vec!["iost","iost"];
        let le = arr_t.len() as u32;
        let mut a_ = Vec::new();
        let mut a_u = Vec::new();

        for i in arr_t {
            let mut a = i.as_bytes().to_vec();
            let i_len = i.len() as u32;
            a_.write_u32::<BigEndian>(i_len);
            a_.reverse();
            for k in &a_ {
                a.insert(0, *k);
            }
            dbg!(&a);
            for j in a {
                a_u.push(j);
            }
        }
        dbg!(a_u);
        println!("{}",le);
    }
}
