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
use std::io;
use std::io::{Read, Write};

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
