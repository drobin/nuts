// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

#[cfg(test)]
mod tests;

use std::convert::TryFrom;
use std::io::{self, Read, Write};

/// Trait that supports reading datatypes from a binary data stream.
///
/// Datatypes that implements this trait can be read from a binary data stream.
pub trait FromBinary {
    /// Reads data from the given `Read`.
    ///
    /// Reads as much as necessary from `r`. The method deserializes the
    /// instance and returns it.
    ///
    /// # Errors
    ///
    /// On conversion error an [`Error`] with kind [`ErrorKind::InvalidData`]
    /// should be returned.
    ///
    /// [`Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    /// [`ErrorKind::InvalidData`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.InvalidData
    fn from_binary(r: &mut dyn Read) -> io::Result<Self>
    where
        Self: Sized;
}

/// Deserializes an `u8` value.
///
/// Note that since this reads a single byte, no byte order conversions are
/// used. It is included for completeness.
impl FromBinary for u8 {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0; 1];

        r.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

/// Deserializes an `u16` value.
///
/// It reads the number in network byte order (big endian).
impl FromBinary for u16 {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0; 2];

        r.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}

/// Deserializes an `u32` value.
///
/// It reads the number in network byte order (big endian).
impl FromBinary for u32 {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0; 4];

        r.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
}

/// Deserializes an `u64` value.
///
/// It reads the number in network byte order (big endian).
impl FromBinary for u64 {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0; 8];

        r.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}

/// Deserializes an `usize` value.
///
/// It reads an `u32` value (4 bytes), thus the `usize` cannot be greater than
/// [`u32::MAX`].
///
/// [`u32::MAX`]: https://doc.rust-lang.org/std/u32/constant.MAX.html
impl FromBinary for usize {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        Ok(u32::from_binary(r)? as usize)
    }
}

/// Deserializes a vector which holds deserializable instances.
///
/// First is reads the length of the vector as a `usize`. Then is reads all
/// instances one by the other.
impl<D: FromBinary> FromBinary for Vec<D> {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let len = usize::from_binary(r)?;
        let mut vec = Vec::new();

        for _i in 0..len {
            vec.push(D::from_binary(r)?);
        }

        Ok(vec)
    }
}

/// Trait that supports writing datatypes into a binary data stream.
///
/// Datatypes that implements this trait can be serialized into a binary data
/// stream.
pub trait IntoBinary {
    /// Writes data into the given `Write`.
    ///
    /// Serializes this instance into its binary representation and writes the
    /// binary data into `w`.
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()>;
}

/// Serializes an `u8` value.
///
/// Note that since this writes a single byte, no byte order conversions are
/// used. It is included for completeness.
impl IntoBinary for u8 {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write_all(&[*self])?;
        Ok(())
    }
}

/// Serializes an `u16` value.
///
/// It writes the number in network byte order (big endian).
impl IntoBinary for u16 {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

/// Serializes an `u32` value.
///
/// It writes the number in network byte order (big endian).
impl IntoBinary for u32 {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

/// Serializes an `u64` value.
///
/// It writes the number in network byte order (big endian).
impl IntoBinary for u64 {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

/// Serializes an `usize` value.
///
/// It writes an `u32` value (4 bytes), thus the `usize` cannot be greater than
/// [`u32::MAX`].
///
/// # Errors
///
/// If you try to serialize a number greater than [`u32::MAX`], then an
/// [`Error`] of kind [`ErrorKind::WriteZero`] is generated.
///
/// [`u32::MAX`]: https://doc.rust-lang.org/std/u32/constant.MAX.html
/// [`Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
/// [`ErrorKind::WriteZero`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.WriteZero
impl IntoBinary for usize {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        match u32::try_from(*self) {
            Ok(n) => n.into_binary(w),
            Err(_) => {
                let msg = format!("at most {} elements are allowed", u32::MAX);
                Err(io::Error::new(io::ErrorKind::WriteZero, msg))
            }
        }
    }
}

/// Serializes a vector which holds serializable instances.
///
/// First is writes the length of the vector as a `usize`. Then is writes all
/// instances one by the other.
///
/// Due to the restrictions of the `usize` size to 4 bytes, the vector cannot
/// hold more than [`u32::MAX`] elements.
///
/// [`u32::MAX`]: https://doc.rust-lang.org/std/u32/constant.MAX.html
impl<B: IntoBinary> IntoBinary for Vec<B> {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        self.len().into_binary(w)?;

        for item in self.iter() {
            item.into_binary(w)?;
        }

        Ok(())
    }
}

/// A [`Read`] extension that supports reading binary data using the
/// [`FromBinary`] trait.
///
/// # Examples
///
/// ```rust
/// use nuts::io::BinaryRead;
/// use std::io::Cursor;
///
/// let binary_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// let mut cursor = Cursor::new(&binary_data);
///
/// assert_eq!(cursor.read_binary::<u8>().unwrap(), 0x01);
/// assert_eq!(cursor.read_binary::<u16>().unwrap(), 0x0203);
/// assert_eq!(cursor.read_binary::<u32>().unwrap(), 0x04050607);
/// assert_eq!(cursor.read_binary::<u64>().unwrap(), 0x08090A0B0C0D0E0F);
/// ```
///
/// ```rust
/// use nuts::io::BinaryRead;
/// use std::io::Cursor;
///
/// let binary_data = [0, 0, 0, 3, 4, 5, 6];
/// let mut cursor = Cursor::new(&binary_data);
///
/// assert_eq!(cursor.read_binary::<Vec<u8>>().unwrap(), [4, 5, 6]);
/// ```
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`FromBinary`]: trait.FromBinary.html
pub trait BinaryRead: Read + Sized {
    /// Reads a datatype that implements the [`FromBinary`] trait.
    ///
    /// [`FromBinary`]: trait.FromBinary.html
    fn read_binary<B: FromBinary>(&mut self) -> io::Result<B> {
        B::from_binary(self)
    }
}

/// A [`Write`] extension that supports writing binary data using the
/// [`IntoBinary`] trait.
///
/// # Examples
///
/// ```rust
/// use nuts::io::BinaryWrite;
/// use std::io::Cursor;
///
/// let mut cursor = Cursor::new(Vec::new());
///
/// cursor.write_binary(&0x01u8).unwrap();
/// cursor.write_binary(&0x0203u16).unwrap();
/// cursor.write_binary(&0x04050607u32).unwrap();
/// cursor.write_binary(&0x08090A0B0C0D0E0Fu64).unwrap();
///
/// assert_eq!(cursor.into_inner(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// ```
///
/// ```rust
/// use nuts::io::BinaryWrite;
/// use std::io::Cursor;
///
/// let mut cursor = Cursor::new(Vec::new());
///
/// cursor.write_binary(&vec![4u8, 5u8, 6u8]).unwrap();
///
/// assert_eq!(cursor.into_inner(), [0, 0, 0, 3, 4, 5, 6]);
/// ```
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`IntoBinary`]: trait.IntoBinary.html
pub trait BinaryWrite: Write + Sized {
    /// Writes a datatype that implements the [`IntoBinary`] trait.
    ///
    /// [`IntoBinary`]: trait.IntoBinary.html
    fn write_binary(&mut self, b: &dyn IntoBinary) -> io::Result<()> {
        b.into_binary(self)
    }
}

/// All types that implement [`Read`] get methods defined in [`BinaryRead`] for
/// free.
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`BinaryRead`]: trait.BinaryRead.html
impl<R: Read + Sized> BinaryRead for R {}

/// All types that implement [`Write`] get methods defined in [`BinaryWrite`] for
/// free.
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`BinaryWrite`]: trait.BinaryWrite.html
impl<W: Write + Sized> BinaryWrite for W {}
