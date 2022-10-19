// MIT License
//
// Copyright (c) 2022 Robin Doer
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

//! Transformation from/to binary streams.

#[cfg(test)]
mod tests;

use std::convert::TryInto;
use std::io::{self, ErrorKind, Read, Write};
use std::{error, fmt, mem, result};

/// Errors thrown by the `bytes` modules.
#[derive(Debug)]
pub enum Error {
    /// Failed to read the requested number of bytes. No more bytes are
    /// available for reading.
    Eof,

    /// Failed to write the whole buffer. The underlaying buffer is full.
    NoSpace,

    /// The structured data are invalid and cannot be read/written. The
    /// `String` contains an error message.
    Invalid(String),

    /// Any other error occured.
    Other(io::Error),
}

impl Error {
    pub fn invalid<M: AsRef<str>>(msg: M) -> Error {
        Error::Invalid(msg.as_ref().to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Eof => write!(fmt, "No more bytes are available for reading."),
            Error::NoSpace => write!(fmt, "Failed to write the whole buffer."),
            Error::Invalid(msg) => write!(fmt, "Invalid data: {}", msg),
            Error::Other(cause) => write!(fmt, "An unexpected IO error occured: {}", cause),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Other(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Self {
        match cause.kind() {
            ErrorKind::UnexpectedEof => Error::Eof,
            ErrorKind::WriteZero => Error::NoSpace,
            _ => Error::Other(cause),
        }
    }
}

/// The [`Result`] used by `bytes` module.
///
/// Maps the error to [`Error`].
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Error`]: enum.Error.html
pub type Result<T> = result::Result<T, Error>;

/// Trait that supports reading datatypes from a binary data stream.
///
/// Datatypes that implements this trait can be read from a binary data stream.
pub trait FromBytes: Sized {
    /// Reads data from the given `Read`.
    ///
    /// Reads as much as necessary from `source`. The method deserializes the
    /// instance and returns it.
    ///
    /// # Errors
    ///
    /// If not enough data are available an error of kind [`Error::Eof`] should
    /// be returned. For convinient [`Error`] implements
    /// [`From<std::io::Error>`], which converts a `std::io::Error` into an
    /// [`Error`].
    ///
    /// On conversion error an error of kind [`Error::Invalid`] should be
    /// returned.
    ///
    /// [`Error`]: enum.Error.html
    /// [`ErrorKind::Eof`]: enum.Error.html#variant.Eof
    /// [`ErrorKind::Invalid`]: enum.Error.html#variant.Invalid
    /// [`From<std::io::Error>`]: enum.Error.html#impl-From<Error>
    fn from_bytes<R: Read>(source: &mut R) -> Result<Self>;
}

/// Trait that supports writing datatypes into a binary data stream.
///
/// Datatypes that implements this trait can be serialized into a binary data
/// stream.
pub trait ToBytes {
    /// Writes data into the given `Write`.
    ///
    /// Serializes this instance into its binary representation and writes the
    /// binary data into `target`.
    ///
    /// # Errors
    ///
    /// If the target buffer is not large enough an error of kind
    /// [`Error::NoSpace`] should be returned. For convinient [`Error`]
    /// implements [`From<std::io::Error>`], which converts a `std::io::Error`
    /// into an [`Error`].
    ///
    /// [`Error`]: enum.Error.html
    /// [`ErrorKind::NoSpace`]: enum.Error.html#variant.NoSpace
    /// [`ErrorKind::Invalid`]: enum.Error.html#variant.Invalid
    /// [`From<std::io::Error>`]: enum.Error.html#impl-From<Error>
    fn to_bytes<W: Write>(&self, target: &mut W) -> Result<()>;
}

/// A [`Read`] extension that supports reading binary data using the
/// [`FromBytes`] trait.
///
/// # Examples
///
/// ```rust
/// use nuts::bytes::FromBytesExt;
/// use std::io::Cursor;
///
/// let binary_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// let mut cursor = Cursor::new(&binary_data);
///
/// assert_eq!(cursor.from_bytes::<u8>().unwrap(), 0x01);
/// assert_eq!(cursor.from_bytes::<u16>().unwrap(), 0x0203);
/// assert_eq!(cursor.from_bytes::<u32>().unwrap(), 0x04050607);
/// assert_eq!(cursor.from_bytes::<u64>().unwrap(), 0x08090A0B0C0D0E0F);
/// ```
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`FromBytes`]: trait.FromBytes.html
pub trait FromBytesExt: Read + Sized {
    fn from_bytes<B: FromBytes>(&mut self) -> Result<B> {
        B::from_bytes(self)
    }
}

/// A [`Write`] extension that supports writing binary data using the
/// [`ToBytes`] trait.
///
/// # Examples
///
/// ```rust
/// use nuts::bytes::ToBytesExt;
/// use std::io::Cursor;
///
/// let mut cursor = Cursor::new(Vec::new());
///
/// cursor.to_bytes(&0x01u8).unwrap();
/// cursor.to_bytes(&0x0203u16).unwrap();
/// cursor.to_bytes(&0x04050607u32).unwrap();
/// cursor.to_bytes(&0x08090A0B0C0D0E0Fu64).unwrap();
///
/// assert_eq!(cursor.into_inner(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// ```
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`ToBytes`]: trait.ToBytes.html
pub trait ToBytesExt: Write + Sized {
    fn to_bytes<B: ToBytes>(&mut self, b: &B) -> Result<()> {
        b.to_bytes(self)
    }
}

/// All types that implement [`Read`] get methods defined in [`FromBytesExt`] for
/// free.
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`FromBytesExt`]: trait.FromBytesExt.html
impl<R: Read + Sized> FromBytesExt for R {}

/// All types that implement [`Write`] get methods defined in [`ToBytesExt`] for
/// free.
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`ToBytesExt`]: trait.ToBytesExt.html
impl<W: Write + Sized> ToBytesExt for W {}

macro_rules! impl_from_bytes_for {
    ($type:ty) => {
        impl FromBytes for $type {
            fn from_bytes<R: Read>(source: &mut R) -> Result<Self> {
                const SIZE: usize = mem::size_of::<$type>();
                let mut buf = [0; SIZE];

                source.read_exact(&mut buf)?;

                Ok(<$type>::from_be_bytes(buf))
            }
        }
    };
}

impl_from_bytes_for!(u8);
impl_from_bytes_for!(u16);
impl_from_bytes_for!(u32);
impl_from_bytes_for!(u64);

impl FromBytes for Vec<u8> {
    fn from_bytes<R: Read>(source: &mut R) -> Result<Self> {
        let len = source.from_bytes::<u32>()? as usize;
        let mut buf = vec![0; len];

        source.read_exact(&mut buf[..])?;

        Ok(buf)
    }
}

macro_rules! impl_to_bytes_for {
    ($type:ty) => {
        impl ToBytes for $type {
            fn to_bytes<W: Write>(&self, target: &mut W) -> Result<()> {
                Ok(target.write_all(&self.to_be_bytes())?)
            }
        }
    };
}

impl_to_bytes_for!(u8);
impl_to_bytes_for!(u16);
impl_to_bytes_for!(u32);
impl_to_bytes_for!(u64);

impl ToBytes for &[u8] {
    fn to_bytes<W: Write>(&self, target: &mut W) -> Result<()> {
        match self.len().try_into() {
            Ok(len) => {
                target.to_bytes::<u32>(&len)?;
                target.write_all(self)?;

                Ok(())
            }
            Err(_) => Err(Error::invalid("vector too large")),
        }
    }
}
