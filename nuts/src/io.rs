// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use byteorder::{ByteOrder, NetworkEndian};
use std::cmp;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{self, Read, Write};

use crate::container::Container;
use crate::error::Error;
use crate::result::Result;
use crate::utils::SecureVec;

/// Trait that supports reading of basic datatypes.
///
/// The `ReadBasics` is extended from [`Read`] and reads `u8`, `u32` and `u64`
/// values from the underlying [`Read`] trait. The numbers are stored in
/// network byte order (big endian).
///
/// The trait is enabled to all types that implements [`Read`].
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait ReadBasics: Read {
    /// Reads an `u8` value from the underlying [`Read`] trait.
    ///
    /// Note that since this reads a single byte, no byte order conversions are
    /// used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::ReadBasics;
    /// use std::io::Cursor;
    ///
    /// let mut reader = Cursor::new(vec![6]);
    /// assert_eq!(reader.read_u8().unwrap(), 6);
    /// ```
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];

        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Read an `u32` value from the underlying [`Read`] trait.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::ReadBasics;
    /// use std::io::Cursor;
    ///
    /// let mut reader = Cursor::new(vec![0x00, 0x00, 0x12, 0x67]);
    /// assert_eq!(reader.read_u32().unwrap(), 4711);
    /// ```
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];

        self.read_exact(&mut buf)?;
        Ok(NetworkEndian::read_u32(&buf))
    }

    /// Read an `u64` value from the underlying [`Read`] trait.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::ReadBasics;
    /// use std::io::Cursor;
    ///
    /// let mut reader = Cursor::new(vec![0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83]);
    /// assert_eq!(reader.read_u64().unwrap(), 918733457491587);
    /// ```
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];

        self.read_exact(&mut buf)?;
        Ok(NetworkEndian::read_u64(&mut buf))
    }
}

/// Trait that supports reading extended datatypes.
///
/// The `ReadExt` is extended from [`Read`] and reads arrays and [`Vec`]s with
/// `u8` values from the underlying [`Read`] trait. All the data are stored in
/// network byte order (big endian).
///
/// The trait is enabled to all types that implements [`Read`].
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub trait ReadExt: ReadBasics {
    /// Reads an fixed sized array (containing `u8` values) from the the
    /// underlying [`Read`] trait.
    ///
    /// The array is returned as an [`Vec`] and has a size of `size` bytes.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::ReadExt;
    /// use std::io::Cursor;
    ///
    /// let mut reader = Cursor::new(vec![0x01, 0x02, 0x03]);
    /// assert_eq!(reader.read_array(3).unwrap(), [1, 2, 3]);
    /// ```
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    fn read_array(&mut self, size: u32) -> io::Result<Vec<u8>> {
        let mut arr = vec![0; size as usize];

        for elem in arr.iter_mut() {
            *elem = self.read_u8()?;
        }

        Ok(arr)
    }

    /// Reads a [`Vec`] (containing `u8` values) from the the underlying
    /// [`Read`] trait.
    ///
    /// The vector has a dynamic size and is additionally encoded in the
    /// [`Read`] stream.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::ReadExt;
    /// use std::io::Cursor;
    ///
    /// let mut reader = Cursor::new(vec![0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03]);
    /// assert_eq!(reader.read_vec().unwrap(), [1, 2, 3]);
    /// ```
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    fn read_vec(&mut self) -> io::Result<Vec<u8>> {
        self.read_u32().and_then(|u| self.read_array(u))
    }
}

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
        Ok(NetworkEndian::read_u16(&buf))
    }
}

/// Deserializes an `u32` value.
///
/// It reads the number in network byte order (big endian).
impl FromBinary for u32 {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0; 4];

        r.read_exact(&mut buf)?;
        Ok(NetworkEndian::read_u32(&buf))
    }
}

/// Deserializes an `u64` value.
///
/// It reads the number in network byte order (big endian).
impl FromBinary for u64 {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0; 8];

        r.read_exact(&mut buf)?;
        Ok(NetworkEndian::read_u64(&buf))
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
        let mut buf = [0; 2];

        NetworkEndian::write_u16(&mut buf, *self);
        w.write_all(&buf)?;

        Ok(())
    }
}

/// Serializes an `u32` value.
///
/// It writes the number in network byte order (big endian).
impl IntoBinary for u32 {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        let mut buf = [0; 4];

        NetworkEndian::write_u32(&mut buf, *self);
        w.write_all(&buf)?;

        Ok(())
    }
}

/// Serializes an `u64` value.
///
/// It writes the number in network byte order (big endian).
impl IntoBinary for u64 {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        let mut buf = [0; 8];

        NetworkEndian::write_u64(&mut buf, *self);
        w.write_all(&buf)?;

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

/// Trait that supports writing of basic datatypes.
///
/// The `WriteBasics` trait is extended from [`Write`] and writes `u8`, `u32`
/// and `u64` values into the underlying [`Write`] trait. The numbers are
/// stored in network byte order (big endian).
///
/// The trait is enabled to all types that implements [`Write`].
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
pub trait WriteBasics: Write {
    /// Writes an `u8` value into the underlying [`Write`] trait.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::WriteBasics;
    /// use std::io::Cursor;
    ///
    /// let mut writer = Cursor::new(vec![]);
    ///
    /// writer.write_u8(6).unwrap();
    /// assert_eq!(writer.into_inner(), [6]);
    /// ```
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
        self.write_all(&[n])
    }

    /// Writes an `u32` value into the underlying [`Write`] trait.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::WriteBasics;
    /// use std::io::Cursor;
    ///
    /// let mut writer = Cursor::new(vec![]);
    ///
    /// writer.write_u32(4711).unwrap();
    /// assert_eq!(writer.into_inner(), [0x00, 0x00, 0x12, 0x67]);
    /// ```
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_u32(&mut self, n: u32) -> io::Result<()> {
        let mut buf = [0; 4];

        NetworkEndian::write_u32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an `u64` value into the underlying [`Write`] trait.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::WriteBasics;
    /// use std::io::Cursor;
    ///
    /// let mut writer = Cursor::new(vec![]);
    ///
    /// writer.write_u64(918733457491587).unwrap();
    /// assert_eq!(writer.into_inner(), [0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83]);
    /// ```
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_u64(&mut self, n: u64) -> io::Result<()> {
        let mut buf = [0; 8];

        NetworkEndian::write_u64(&mut buf, n);
        self.write_all(&buf)
    }
}

/// Trait that supports writing extended datatypes.
///
/// The `WriteExt` trait is extended from [`Write`] and writes arrays and
/// [`Vec`]s with `u8` values into the underlying [`Write`] trait. All the data
/// are stored in network byte order (big endian).
///
/// The trait is enabled to all types that implements [`Write`].
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub trait WriteExt: WriteBasics {
    /// Writes an fixed sized array (containing `u8` values) into the the
    /// underlying [`Write`] trait.
    ///
    /// The array to be written is passed to the method as an slice.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::WriteExt;
    /// use std::io::Cursor;
    ///
    /// let mut writer = Cursor::new(vec![]);
    ///
    /// writer.write_array(&[1, 2, 3]).unwrap();
    /// assert_eq!(writer.into_inner(), [1, 2, 3]);
    /// ```
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_array(&mut self, arr: &[u8]) -> io::Result<()> {
        for elem in arr.iter() {
            self.write_u8(*elem)?;
        }

        Ok(())
    }

    /// Writes a vector (containing `u8` values) into the the underlying
    /// [`Write`] trait.
    ///
    /// The vector has a dynamic size and is additionally encoded in the
    /// [`Write`] stream.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuts::io::WriteExt;
    /// use std::io::Cursor;
    ///
    /// let mut writer = Cursor::new(vec![]);
    ///
    /// writer.write_vec(&[1, 2, 3]);
    /// assert_eq!(writer.into_inner(), [0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03]);
    /// ```
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_vec(&mut self, vec: &[u8]) -> io::Result<()> {
        self.write_u32(vec.len() as u32)
            .and_then(|()| self.write_array(vec))
    }
}

/// All types that implement [`Read`] get methods defined in [`ReadBasics`] for
/// free.
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`ReadBasics`]: trait.ReadBasics.html
impl<R: Read + ?Sized> ReadBasics for R {}

/// All types that implement [`Read`] get methods defined in [`ReadExt`] for
/// free.
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`ReadExt`]: trait.ReadExt.html
impl<R: Read + ?Sized> ReadExt for R {}

/// All types that implement [`Write`] get methods defined in [`WriteBasics`] for
/// free.
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`WriteBasics`]: trait.WriteBasics.html
impl<W: Write + ?Sized> WriteBasics for W {}

/// All types that implement [`Write`] get methods defined in [`WriteExt`] for
/// free.
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`WriteExt`]: trait.WriteExt.html
impl<W: Write + ?Sized> WriteExt for W {}

/// Utility used to read a stream of data from a container.
///
/// Compared to [`Container::read()`] (which can read a single block), the
/// `Reader` can read a stream of data which are located in several blocks. The
/// `Reader` has a queue of blocks ids. You can put an block id on the back of
/// this queue using the [`push_id()`] method. The `Reader` utility
/// subsequently takes an block id from the front of the queue. For each id it
/// reads the content of the block from the container. If no more block ids are
/// available for reading, the `Reader` reports an _end of file_ event.
///
/// Additionally you can configure a maximum number of bytes to read using the
/// [`set_max_bytes()`] method. If the condition is reached (or you getting out
/// of block ids - whatever is reached first), the `Reader` stops reading data.
/// The maximum number of bytes has not to be a multiple of the [block size]!
/// So you can read a part of a block easly.
///
/// The `Reader` implements the [`Read`] trait; it is used to read data from
/// the container.
///
/// [`Container::read()`]: ../container/struct.Container.html#method.read
/// [`push_id()`]: #method.push_id
/// [`set_max_bytes()`]: #method.set_max_bytes
/// [block size]: ../container/struct.Container.html#method.bsize
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub struct Reader<'a> {
    container: &'a mut Container,
    queue: VecDeque<u64>,
    cache: SecureVec<u8>,
    cur_bytes: u64,
    max_bytes: u64,
}

impl<'a> Reader<'a> {
    /// Create a new `Reader` instance.
    ///
    /// The given `container` is used as the source.
    pub fn new(container: &mut Container) -> Reader {
        Reader {
            container,
            queue: VecDeque::new(),
            cache: secure_vec![],
            cur_bytes: 0,
            max_bytes: u64::MAX,
        }
    }

    /// Sets the maximum number of bytes to read.
    ///
    /// The utility stops reading data, if `max_bytes` bytes are actually read.
    /// If unset, the `Reader` stops when no more blocks are available.
    pub fn set_max_bytes(&mut self, max_bytes: u64) {
        self.max_bytes = max_bytes;
    }

    /// Queues the given `id`.
    ///
    /// Puts the given `id` on the back of a queue. The `Reader` subsequently
    /// takes an block id from the front of the queue and reads the content of
    /// the block from the container.
    ///
    /// The `Reader` does not check for duplicates. Pushing an `id` more than
    /// one time on the queue can lead into a situation, where a block is read
    /// twice (or more)!
    pub fn push_id(&mut self, id: u64) {
        self.queue.push_back(id);
    }

    fn fill_cache(&mut self) -> Result<()> {
        if self.cache.is_empty() {
            match self.queue.pop_front() {
                Some(id) => {
                    let bsize = self.container.bsize()?;

                    self.cache.resize(bsize as usize, 0);
                    self.container.read(id, &mut self.cache)?;
                }
                None => (),
            }
        };

        Ok(())
    }
}

impl<'a> Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fill_cache()?;

        let remaining_bytes = (self.max_bytes - self.cur_bytes) as usize;
        let nbytes = cmp::min(cmp::min(self.cache.len(), buf.len()), remaining_bytes);
        let source = &self.cache[..nbytes];
        let target = &mut buf[..nbytes];

        target.copy_from_slice(source);
        self.cache.drain(..nbytes);
        self.cur_bytes += nbytes as u64;

        Ok(nbytes)
    }
}

/// Utility used to write a stream of data into a container.
///
/// Compared to [`Container::write()`] (which can update a single block), the
/// `Writer` can write a stream of data devided into several blocks; if one
/// block is full, the `Writer` switches to another block and continues
/// writing. The [`push_id()`] method is used to tell the `Writer` which blocks
/// can be written.
///
/// The `Writer` implements the [`Write`] trait; it is used to push data into
/// the container. Never forget to [`flush()`] the `Writer`! The [`flush()`]
/// call makes sure that the remaining data are padded and written into the
/// container.
///
/// [`Container::write()`]: ../container/struct.Container.html#method.write
/// [`push_id()`]: #method.push_id
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`flush()`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
pub struct Writer<'a> {
    container: &'a mut Container,
    queue: VecDeque<u64>,
    cache: SecureVec<u8>,
    blocks: u64,
}

impl<'a> Writer<'a> {
    /// Create a new `Writer` instance.
    ///
    /// The given `container` is used as the target.
    pub fn new(container: &mut Container) -> Writer {
        Writer {
            container,
            queue: VecDeque::new(),
            cache: secure_vec![],
            blocks: 0,
        }
    }

    /// Queues the given `id`.
    ///
    /// Puts the given `id` on the back of a queue. Whenever the `Writer`
    /// switches to another block, it takes an id from the front of the queue.
    /// If the queue is empty but the `Writer` still needs another block, then
    /// the [`write()`] or [`flush()`] operation is aborted. There can be also
    /// more ids on the queue than required. In this case the remaining blocks
    /// are ignored.
    ///
    /// The `Writer` does not check for duplicates. Pushing an `id` more than
    /// one time on the queue can lead into a situation, where a block is
    /// updated twice (or more)!
    ///
    /// [`write()`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write
    /// [`flush()`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
    pub fn push_id(&mut self, id: u64) {
        self.queue.push_back(id);
    }

    fn pop_id(&mut self) -> Result<u64> {
        self.queue.pop_front().ok_or_else(|| {
            let msg = "no more blocks available";
            let err = io::Error::new(io::ErrorKind::Other, msg);
            Error::IoError(err)
        })
    }

    /// Returns the number blocks actually written.
    pub fn blocks(&self) -> u64 {
        self.blocks
    }

    fn fill_cache(&mut self, buf: &[u8]) -> Result<usize> {
        let bsize = self.container.bsize()? as usize;
        let nbytes = cmp::min(bsize - self.cache.len(), buf.len());

        self.cache.extend(buf[..nbytes].iter());

        Ok(nbytes)
    }

    fn flush_cache(&mut self, force: bool) -> Result<()> {
        let bsize = self.container.bsize()? as usize;

        if self.cache.len() >= bsize || force {
            let nbytes = cmp::min(bsize, self.cache.len());
            let id = self.pop_id()?;

            self.container.write(id, &self.cache[..nbytes])?;
            self.cache.drain(..nbytes);

            self.blocks += 1;
        }

        Ok(())
    }
}

impl<'a> Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut nbytes = 0;

        while nbytes < buf.len() {
            nbytes += self.fill_cache(&buf[nbytes..])?;
            self.flush_cache(false)?;
        }

        Ok(nbytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        while !self.cache.is_empty() {
            self.flush_cache(true)?;
        }

        Ok(())
    }
}
