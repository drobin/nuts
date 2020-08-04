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
use log::debug;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

use crate::error::Error;
use crate::rand::random;
use crate::result::Result;
use crate::types::DiskType;

pub(crate) struct IO {
    pub bsize: u32,
    pub blocks: u64,
    pub ablocks: u64,
    pub dtype: DiskType,
}

impl IO {
    pub fn new<T>(bsize: u32, blocks: u64, dtype: DiskType, target: &mut T) -> Result<IO>
    where
        T: Write + Seek,
    {
        let pos = target.seek(SeekFrom::End(0))?;

        Ok(IO {
            bsize,
            blocks,
            ablocks: if bsize > 0 { pos / bsize as u64 } else { 0 },
            dtype,
        })
    }

    pub fn ensure_capacity<T>(&mut self, target: &mut T, blocks: u64) -> Result<()>
    where
        T: Write + Seek,
    {
        let blocks = std::cmp::min(blocks, self.blocks);

        match self.dtype {
            DiskType::FatZero | DiskType::FatRandom => {
                if blocks > 0 {
                    // Fat containers are extended to its size.
                    self.extend_container(target, self.blocks - self.ablocks)
                } else {
                    Ok(())
                }
            }
            DiskType::ThinZero | DiskType::ThinRandom => {
                if blocks > 0 && blocks > self.ablocks {
                    // This containers are extended to the requested block.
                    self.extend_container(target, blocks - self.ablocks)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn extend_container<T>(&mut self, target: &mut T, count: u64) -> Result<()>
    where
        T: Write + Seek,
    {
        debug!(
            "extending container by {} blocks, ablocks: {}",
            count, self.ablocks
        );

        self.seek(target, self.ablocks)?;

        let mut data = vec![0; self.bsize as usize];

        for _i in 0..count {
            if self.dtype == DiskType::FatRandom || self.dtype == DiskType::ThinRandom {
                random(&mut data)?;
            }

            target.write_all(&data)?;
            self.ablocks += 1;
        }

        debug!(
            "container extended by {} blocks, ablocks: {}",
            count, self.ablocks
        );

        Ok(())
    }

    pub fn read<T>(&self, source: &mut T, target: &mut [u8], id: u64) -> Result<u32>
    where
        T: Read + Seek,
    {
        self.assert_id(id)?;

        let len = std::cmp::min(target.len(), self.bsize as usize);
        let buf = target.get_mut(0..len).unwrap();

        if id < self.ablocks {
            // Read an allocated block.
            // Seek to the related position and read the buffer.

            self.seek(source, id)?;
            source.read_exact(buf)?;
        } else {
            // Read an existing but unallocated block.
            // Fill the target buffer with data which fits to the dtype.

            match self.dtype {
                DiskType::FatZero | DiskType::ThinZero => {
                    for e in buf.iter_mut() {
                        *e = 0;
                    }
                }
                DiskType::FatRandom | DiskType::ThinRandom => {
                    random(buf)?;
                }
            }
        }

        Ok(len as u32)
    }

    pub fn write<T>(&mut self, source: &[u8], target: &mut T, id: u64) -> Result<u32>
    where
        T: Write + Seek,
    {
        self.assert_id(id)?;
        self.ensure_capacity(target, id + 1)?;

        let len = std::cmp::min(source.len(), self.bsize as usize);
        let pad_len = self.bsize as usize - len;

        let buf = source.get(0..len).unwrap();

        let pad = match self.dtype {
            DiskType::FatZero | DiskType::ThinZero => vec![0; pad_len],
            DiskType::FatRandom | DiskType::ThinRandom => {
                let mut rnd: Vec<u8> = vec![0; pad_len];
                random(&mut rnd[..])?;
                rnd
            }
        };

        let block = [buf, &pad[..]].concat();

        self.seek(target, id)?;
        target.write_all(&block)?;

        Ok(self.bsize)
    }

    fn seek<T>(&self, fd: &mut T, id: u64) -> Result<()>
    where
        T: Seek,
    {
        let pos = id * self.bsize as u64;
        let pos2 = fd.seek(SeekFrom::Start(pos))?;

        if pos != pos2 {
            let err = std::io::Error::new(
                ErrorKind::UnexpectedEof,
                format!("failed to seek to position {}, is {}", pos, pos2),
            );
            Err(Error::IoError(err))
        } else {
            Ok(())
        }
    }

    fn assert_id(&self, id: u64) -> Result<()> {
        if id < self.blocks {
            Ok(())
        } else {
            let err =
                std::io::Error::new(ErrorKind::Other, format!("unable to locate block {}", id));
            Err(Error::IoError(err))
        }
    }
}

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
