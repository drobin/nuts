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
//! // Example creates an encrypted container with an attached MemoryBackend.
//!
//! use nuts::container::{Cipher, Container, CreateOptionsBuilder, Digest, Kdf};
//! use nuts::memory::MemoryBackend;
//!
//! let backend = MemoryBackend::new();
//! let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
//!
//! // Let's create an encrypted container (with aes128-ctr).
//! let options = CreateOptionsBuilder::<MemoryBackend>::new(backend, Cipher::Aes128Ctr)
//!     .with_password_callback(|| Ok(b"abc".to_vec()))
//!     .with_kdf(kdf.clone())
//!     .build()
//!     .unwrap();
//! let container = Container::create(options).unwrap();
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
//! // Example opens a container with an attached MemoryBackend,
//! // which is always unencrypted.
//!
//! use nuts::container::{Cipher, Container, Kdf, OpenOptionsBuilder};
//! use nuts::memory::MemoryBackend;
//!
//! let backend = MemoryBackend::new();
//!
//! // When opening a contaier with a MemoryBackend attached,
//! // the container is always unencrypted.
//! let options = OpenOptionsBuilder::<MemoryBackend>::new(backend)
//!     .build()
//!     .unwrap();
//! let container = Container::open(options).unwrap();
//! let info = container.info().unwrap();
//!
//! assert_eq!(info.cipher, Cipher::None);
//! assert_eq!(info.kdf, Kdf::None);
//! ```

#[cfg(test)]
mod tests;

use nuts_bytes::Writer;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{cmp, error, fmt, mem};

use crate::backend::{Backend, BlockId, Create, Open, HEADER_MAX_SIZE};
use crate::container::{Cipher, Kdf};

const BSIZE: u32 = 512;

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
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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
pub struct MemoryBackend(HashMap<u32, [u8; BSIZE as usize]>);

impl MemoryBackend {
    /// Creates a new instance of the `MemoryBackend` type.
    pub fn new() -> MemoryBackend {
        MemoryBackend(HashMap::new())
    }

    fn secret_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut writer = Writer::new(vec![]);

        writer.serialize(&1u32)?; // magic 1
        writer.serialize(&1u32)?; // magic 2
        writer.serialize::<Vec<u8>>(&vec![])?; // key
        writer.serialize::<Vec<u8>>(&vec![])?; // iv
        writer.serialize(&Option::<Id>::None)?; // top-id
        writer.serialize(&())?; // settings

        Ok(writer.into_target())
    }

    fn header_bytes(&self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), Error> {
        let mut writer = Writer::new(bytes.as_mut_slice());

        writer.write_bytes(b"nuts-io")?; // magic

        writer.serialize(&0u32)?; // rev 0
        writer.serialize(&Cipher::None)?; // cipher
        writer.serialize::<Vec<u8>>(&vec![])?; // IV
        writer.serialize(&Kdf::None)?; // KDF
        writer.serialize(&self.secret_bytes()?)?; // secret

        Ok(())
    }
}

impl Create<Self> for MemoryBackend {
    fn settings(&self) -> () {
        ()
    }

    fn put_header_bytes(&mut self, _bytes: &[u8; HEADER_MAX_SIZE]) -> Result<(), Error> {
        // ignore this call, you cannot make it persistent
        Ok(())
    }

    fn build(self) -> Result<MemoryBackend, Error> {
        Ok(self)
    }
}

impl Open<Self> for MemoryBackend {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), Error> {
        // let's generate some header-data on the fly
        self.header_bytes(bytes)
    }

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
        BSIZE
    }

    fn aquire(&mut self) -> Result<Id, Error> {
        let id = match self.0.keys().max() {
            Some(n) => n + 1,
            None => 0,
        };

        match self.0.insert(id, [0; BSIZE as usize]) {
            Some(_) => return Err(Error::AlreadAquired(Id(id))),
            None => Ok(Id(id)),
        }
    }

    fn release(&mut self, id: Id) -> Result<(), Error> {
        self.0.remove(&id.0);
        Ok(())
    }

    fn read(&mut self, id: &Id, buf: &mut [u8]) -> Result<usize, Error> {
        match self.0.get(&id.0) {
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
        match self.0.get_mut(&id.0) {
            Some(target) => {
                let mut source = Cow::from(buf);
                let mut len = source.len();

                if len != BSIZE as usize {
                    len = cmp::min(source.len(), BSIZE as usize);
                    source.to_mut().resize(BSIZE as usize, 0);
                }

                target.copy_from_slice(&source);

                Ok(len)
            }
            None => Err(Error::NoSuchId(*id)),
        }
    }
}
