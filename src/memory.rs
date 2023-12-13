// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

//! A sample [`Backend`](crate::backend::Backend) implementation which stores
//! the data in the memory.
//!
//! **This implementation is mainly used for demonstration, testing and
//! documentation.**
//!
//! It stores the content of the data blocks in a [hash](HashMap) indexed by
//! the [`Id`] of this backend, where the [id](crate::backend::Backend::Id) is
//! a simple `u32` value.
//!
//! When creating a [`MemoryBackend`] you can choose how the data are
//! encrypted. Choose the related options in
//! [`CreateOptionsBuilder`](crate::container::CreateOptionsBuilder) and the
//! [container](crate::container::Container) will pass (possibly) encrypted
//! data to this backend.
//!
//! ```rust
//! use nuts_container::container::*;
//! use nuts_container::memory::MemoryBackend;
//!
//! // Example creates an encrypted container with an attached MemoryBackend.
//!
//! let backend = MemoryBackend::new();
//! let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
//!
//! // Let's create an encrypted container (with aes128-ctr).
//! let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
//!     .with_password_callback(|| Ok(b"abc".to_vec()))
//!     .with_kdf(kdf.clone())
//!     .build::<MemoryBackend>()
//!     .unwrap();
//! let container = Container::<MemoryBackend>::create(backend, options).unwrap();
//! let info = container.info().unwrap();
//!
//! assert_eq!(info.cipher, Cipher::Aes128Ctr);
//! assert_eq!(info.kdf, kdf);
//! ```
//!
//! When you open a [`MemoryBackend`] you have no possibility to choose further
//! settings because (due to the nature of this volatile storage) nothing is
//! made persistent. On open, always an unencrypted
//! [container](crate::container::Container) is created.
//!
//! ```rust
//! use nuts_container::container::*;
//! use nuts_container::memory::MemoryBackend;
//!
//! // Example opens a container with an attached MemoryBackend,
//! // which is always unencrypted.
//!
//! let backend = MemoryBackend::new();
//!
//! // When opening a contaier with a MemoryBackend attached,
//! // the container is always unencrypted.
//! let options = OpenOptionsBuilder::new().build::<MemoryBackend>().unwrap();
//! let container = Container::<MemoryBackend>::open(backend, options).unwrap();
//! let info = container.info().unwrap();
//!
//! assert_eq!(info.cipher, Cipher::None);
//! assert_eq!(info.kdf, Kdf::None);
//! ```

#[cfg(test)]
mod tests;

use nuts_bytes::{FromBytes, ToBytes, Writer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{cmp, error, fmt, mem};

use crate::backend::{Backend, BlockId, Create, HeaderGet, HeaderSet, Open, HEADER_MAX_SIZE};
use crate::container::{Cipher, Kdf};

/// Error used by the memory backend.
#[derive(Debug)]
pub enum Error {
    /// Tried to read or write from/to an id, which does not exist.
    NoSuchId(Id),

    /// Failed to aquire the given id.
    AlreadAquired(Id),

    /// Failed to serialize binary data.
    Bytes(nuts_bytes::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoSuchId(id) => write!(fmt, "no such id: {}", id),
            Error::AlreadAquired(id) => write!(fmt, "already aquired: {}", id),
            Error::Bytes(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl error::Error for Error {}

impl From<nuts_bytes::Error> for Error {
    fn from(err: nuts_bytes::Error) -> Self {
        Error::Bytes(err)
    }
}

/// The [id](crate::backend::Backend::Id) of the memory backend.
#[derive(Clone, Copy, Debug, FromBytes, PartialEq, ToBytes)]
pub struct Id(u32);

impl fmt::Display for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl Id {
    fn null() -> Id {
        Id(u32::MAX)
    }
}

impl FromStr for Id {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        FromStr::from_str(s).map(|n| Id(n))
    }
}

impl BlockId for Id {
    fn null() -> Id {
        Id::null()
    }

    fn is_null(&self) -> bool {
        self.eq(&Id::null())
    }

    fn size() -> usize {
        mem::size_of::<u32>()
    }
}

/// The [`Backend`] implementation itself.
///
/// See the [module](crate::memory) documentation for details.
#[derive(Debug, PartialEq)]
pub struct MemoryBackend {
    bsize: u32,
    blocks: HashMap<u32, Vec<u8>>,
    header: Option<[u8; HEADER_MAX_SIZE]>,
}

impl MemoryBackend {
    /// Creates a new instance of the `MemoryBackend` type.
    ///
    /// The block-size is set to 512 bytes.
    pub fn new() -> MemoryBackend {
        Self::new_with_bsize(512)
    }

    /// Creates a new instance of the `MemoryBackend` type with the given
    /// block-size.
    pub fn new_with_bsize(bsize: u32) -> MemoryBackend {
        MemoryBackend {
            bsize,
            blocks: HashMap::new(),
            header: None,
        }
    }

    /// Returns the block size specified for this backend instance.
    pub fn block_size(&self) -> u32 {
        self.bsize
    }

    /// Receives the content of the block with the given `id`.
    ///
    /// Returns [`None`] if the block does not exist.
    pub fn get(&self, id: &Id) -> Option<&[u8]> {
        self.blocks.get(&id.0).map(|buf| buf.as_slice())
    }

    /// Inserts a new block.
    ///
    /// The block contains only zeros.
    ///
    /// Returns the id of the new block.
    pub fn insert(&mut self) -> Result<Id, Error> {
        self.insert_data(&[])
    }

    /// Inserts a new block with some initial data.
    ///
    /// Assigns the first [`block-size`](Self::block_size) bytes from `data` to
    /// the new block. If `data` does not have [`block-size`](Self::block_size)
    /// bytes, the new block is padded with zero bytes.
    ///
    /// Returns the id of the new block.
    pub fn insert_data(&mut self, data: &[u8]) -> Result<Id, Error> {
        let id = Id(self.max_id() + 1);
        let mut block = vec![0; self.bsize as usize];

        let n = cmp::min(block.len(), data.len());
        block[..n].copy_from_slice(&data[..n]);

        match self.blocks.insert(id.0, block) {
            Some(_) => Err(Error::AlreadAquired(id)),
            None => Ok(id),
        }
    }

    fn max_id(&self) -> u32 {
        *self.blocks.keys().max().unwrap_or(&0)
    }

    fn secret_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut writer = Writer::new(vec![]);

        writer.write(&1u32)?; // magic 1
        writer.write(&1u32)?; // magic 2
        writer.write::<Vec<u8>>(&vec![])?; // key
        writer.write::<Vec<u8>>(&vec![])?; // iv
        writer.write::<Vec<u8>>(&vec![])?; // userdata
        writer.write::<Vec<u8>>(&vec![])?; // settings

        Ok(writer.into_target())
    }

    fn header_bytes(&self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), Error> {
        let mut writer = Writer::new(bytes.as_mut_slice());

        writer.write(b"nuts-io")?; // magic

        writer.write(&0u32)?; // rev 0
        writer.write(&Cipher::None)?; // cipher
        writer.write::<Vec<u8>>(&vec![])?; // IV
        writer.write(&Kdf::None)?; // KDF
        writer.write(&self.secret_bytes()?)?; // secret

        Ok(())
    }
}

impl HeaderGet<Self> for MemoryBackend {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), Error> {
        match self.header.as_ref() {
            Some(source) => Ok(bytes.copy_from_slice(source)),
            None => self.header_bytes(bytes),
        }
    }
}

impl HeaderSet<Self> for MemoryBackend {
    fn put_header_bytes(&mut self, bytes: &[u8; HEADER_MAX_SIZE]) -> Result<(), Error> {
        self.header = Some(*bytes);
        Ok(())
    }
}

impl Create<Self> for MemoryBackend {
    fn settings(&self) -> () {
        ()
    }

    fn build(self) -> Result<MemoryBackend, Error> {
        Ok(self)
    }
}

impl Open<Self> for MemoryBackend {
    fn build(self, _settings: ()) -> Result<MemoryBackend, Error> {
        Ok(self)
    }
}

impl Backend for MemoryBackend {
    type CreateOptions = Self;
    type OpenOptions = Self;
    type Settings = ();
    type Err = Error;
    type Id = Id;
    type Info = ();

    fn info(&self) -> Result<(), Error> {
        Ok(())
    }

    fn block_size(&self) -> u32 {
        self.bsize
    }

    fn aquire(&mut self, buf: &[u8]) -> Result<Id, Error> {
        self.insert_data(buf)
    }

    fn release(&mut self, id: Id) -> Result<(), Error> {
        self.blocks.remove(&id.0);
        Ok(())
    }

    fn read(&mut self, id: &Id, buf: &mut [u8]) -> Result<usize, Error> {
        match self.blocks.get(&id.0) {
            Some(src) => {
                let len = cmp::min(src.len(), buf.len());

                let source = &src[..len];
                let target = &mut buf[..len];

                target.copy_from_slice(source);

                Ok(len)
            }
            None => Err(Error::NoSuchId(*id)),
        }
    }

    fn write(&mut self, id: &Id, buf: &[u8]) -> Result<usize, Error> {
        match self.blocks.get_mut(&id.0) {
            Some(target) => {
                let mut source = Cow::from(buf);
                let mut len = source.len();

                if len != self.bsize as usize {
                    len = cmp::min(source.len(), self.bsize as usize);
                    source.to_mut().resize(self.bsize as usize, 0);
                }

                target.copy_from_slice(&source);

                Ok(len)
            }
            None => Err(Error::NoSuchId(*id)),
        }
    }
}
