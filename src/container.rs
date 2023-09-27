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

//! # The basic container
//!
//! This module contains types for the basic container handling. A
//! [`Container`] acts like an encrypted block device, where you can read and
//! write encrypted blocks of data. See the [`Container`] documentation for
//! details.
//!
//! The [`Container`] has no knowlege about the storage layer. Every type that
//! implements the [`Backend`] trait can act as the storage layer for the
//! [`Container`]. The [`Container`] receives (possibly) encrypted data from
//! the backend and pushes (possibly) encrypted data back to the backend.
//! See the [`backend` module](crate::backend) documentation for details.
//!
//! ## Create a container
//!
//! The [`Container::create()`] method is used to create a new container. It
//! expects an instance of a type that implements the
//! [`Backend::CreateOptions`] trait, which acts as a builder for the related
//! [`Backend`].
//!
//! Example:
//!
//! ```rust
//! use nuts_container::container::*;
//! use nuts_container::memory::MemoryBackend;
//!
//! // Create a container with a memory backend.
//! let backend = MemoryBackend::new();
//!
//! // Let's create an encrypted container (with aes128-ctr).
//! // Because you are encrypting the container, you need to assign a
//! // password callback.
//! let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
//! let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
//!     .with_password_callback(|| Ok(b"abc".to_vec()))
//!     .with_kdf(kdf.clone())
//!     .build::<MemoryBackend>()
//!     .unwrap();
//!
//! // Create the container and fetch information.
//! // Here you can directly pass the backend instance to the create() method
//! // because MemoryBackend implements the Backend::CreateOptions trait.
//! let container = Container::<MemoryBackend>::create(backend, options).unwrap();
//! let info = container.info().unwrap();
//!
//! assert_eq!(info.cipher, Cipher::Aes128Ctr);
//! assert_eq!(info.kdf, kdf);
//! ```
//!
//! ## Open a container
//!
//! The [`Container::open()`] method is used to open a container. It expects an
//! instance of a type that implements the [`Backend::OpenOptions`] trait,
//! which acts as a builder for the related [`Backend`].
//!
//! Example:
//!
//! ```rust
//! use nuts_container::container::*;
//! use nuts_container::memory::MemoryBackend;
//!
//! let (backend, kdf) = {
//!     // In this example you create a container in a separate block.
//!     // So, the created container is closed again when leaving the scope.
//!     let backend = MemoryBackend::new();
//!     let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
//!     let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
//!         .with_password_callback(|| Ok(b"abc".to_vec()))
//!         .with_kdf(kdf.clone())
//!         .build::<MemoryBackend>()
//!         .unwrap();
//!
//!     // Create the container.
//!     let container = Container::<MemoryBackend>::create(backend, options).unwrap();
//!     let backend = container.into_backend();
//!
//!     (backend, kdf)
//! };
//!
//! // Open the container and fetch information.
//! // Here you can directly pass the backend instance to the open() method
//! // because MemoryBackend implements the Backend::OpenOptions trait.
//! let options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"abc".to_vec()))
//!     .build::<MemoryBackend>()
//!     .unwrap();
//! let container = Container::<MemoryBackend>::open(backend, options).unwrap();
//! let info = container.info().unwrap();
//!
//! assert_eq!(info.cipher, Cipher::Aes128Ctr);
//! assert_eq!(info.kdf, kdf);
//! ```
//!
//! ## Read from a container
//!
//! ```rust
//! use nuts_container::container::*;
//! use nuts_container::memory::MemoryBackend;
//!
//! // Create a container with a memory backend.
//! let mut backend = MemoryBackend::new();
//!
//! // Insert a block into the backend.
//! // Note that the insert() method is a part of the MemoryBackend and directly
//! // inserts a block into the backend (bypassing the crypto capabilities of the
//! // container).
//! let id = backend.insert().unwrap();
//!
//! // Create the container.
//! let options = CreateOptionsBuilder::new(Cipher::None)
//!     .build::<MemoryBackend>()
//!     .unwrap();
//! let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();
//!
//! // Read full block.
//! let mut buf = [b'x'; 512];
//! assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
//! assert_eq!(buf, [0; 512]);
//!
//! // Read block into a buffer which is smaller than the block-size.
//! // The buffer is filled with the first 400 bytes from the block.
//! let mut buf = [b'x'; 400];
//! assert_eq!(container.read(&id, &mut buf).unwrap(), 400);
//! assert_eq!(buf, [0; 400]);
//!
//! // Read block into a buffer which is bigger than the block-size.
//! // The first 512 bytes are filled with the content of the block,
//! // the remaining 8 bytes are not touched.
//! let mut buf = [b'x'; 520];
//! assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
//! assert_eq!(buf[..512], [0; 512]);
//! assert_eq!(buf[512..], [b'x'; 8]);
//! ```
//!
//! ## Write into a container
//!
//! ```rust
//! use nuts_container::container::*;
//! use nuts_container::memory::MemoryBackend;
//!
//! // In this example you create a container in a separate block.
//! // So, the created container is closed again when leaving the scope.
//! let mut backend = MemoryBackend::new();
//!
//! // Insert a block into the backend.
//! // Note that the insert() method is a part of the MemoryBackend and directly
//! // inserts a block into the backend (bypassing the crypto capabilities of the
//! // container).
//! let id = backend.insert().unwrap();
//!
//! // Create the container.
//! let options = CreateOptionsBuilder::new(Cipher::None)
//!     .build::<MemoryBackend>()
//!     .unwrap();
//! let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();
//!
//! // Write a full block. The whole block is filled with 'x'.
//! assert_eq!(container.write(&id, &[b'x'; 512]).unwrap(), 512);
//!
//! let mut buf = [0; 512];
//! assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
//! assert_eq!(buf, [b'x'; 512]);
//!
//! // Write a block from a buffer which is smaller than the block-size.
//! // The first bytes of the block are filled with the data from the buffer,
//! // the remaining space is padded with '0'.
//! assert_eq!(container.write(&id, &[b'x'; 400]).unwrap(), 400);
//!
//! let mut buf = [0; 512];
//! assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
//! assert_eq!(buf[..400], [b'x'; 400]);
//! assert_eq!(buf[400..], [0; 112]);
//!
//! // Write a block from a buffer which is bigger than the block-size.
//! // The block is filled with the first data from the buffer.
//! assert_eq!(container.write(&id, &[b'x'; 520]).unwrap(), 512);
//!
//! let mut buf = [0; 512];
//! assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
//! assert_eq!(buf, [b'x'; 512]);
//! ```
//!
//! ## The header of a container
//!
//! The header of the container stores all data necessary to open the container
//! again. There are:
//!
//! * The [`Cipher`]: The cipher defines the cipher used for encryption and
//!   decryption of the individual blocks of the container.
//!
//!   If the cipher is set to [`Cipher::None`], then encryption is disabled and
//!   all the data are stored unencrypted in the container.
//!
//!   If encryption is enabled (with a cipher which is not [`Cipher::None`]),
//!   then the blocks of the container are encrypted with a master-key stored
//!   and secured in the header of the container. This part of the header
//!   (other sensible data are also stored there) is called _secret_ and is
//!   encrypted with the wrapping-key derivited by the key derivation function
//!   ([`Kdf`]). So, with a user supplied passphrase you are derivating the
//!   wrapping-key, which decrypts the _secret_ part of the header, where the
//!   master-key (used for en-/decryption of the data blocks) is stored.
//!
//! * The key derivation function ([`Kdf`]) defines a way to create a key from
//!   a user supplied passphrase. In the next step this key is used to encrypt resp.
//!   decrypt the _secret_ part of the header.
//!
//! * The _secret_ is the encrypted part of the header and contains sensible
//!   data of the container. The secret is encrypted with a wrapping-key, which
//!   is the output of the [`Kdf`]. The _secret_ contains:
//!
//!   * _master-key_: The master-key is used for encryption of the blocks of
//!     the container.
//!
//!   * _userdata_: Any service running on top of the container can store
//!     individual, arbitrary data in the header. Usually the data are used for
//!     bootstrapping the service.
//!
//!   * _settings of the backend_: The backend of the container stores its
//!     runtime information in the secret. It gets it back when opening the
//!     backend again. See [`Backend::Settings`] for more information.

mod cipher;
mod digest;
mod error;
mod header;
mod info;
mod kdf;
mod options;
mod ossl;
mod password;
mod svec;
#[cfg(test)]
mod tests;

use log::debug;
use std::borrow::Cow;
use std::{any, cmp};

use crate::backend::{Backend, BlockId, Create, HeaderGet, HeaderSet, Open, HEADER_MAX_SIZE};
use crate::container::header::Header;
use crate::container::password::PasswordStore;
use crate::container::svec::SecureVec;

pub use cipher::{Cipher, CipherError};
pub use digest::{Digest, DigestError};
pub use error::{ContainerResult, Error};
pub use header::HeaderError;
pub use info::Info;
pub use kdf::Kdf;
pub use options::{CreateOptions, CreateOptionsBuilder, OpenOptions, OpenOptionsBuilder};
pub use password::PasswordError;

macro_rules! map_err {
    ($result:expr) => {
        $result.map_err(|cause| Error::Backend(cause))
    };
}

/// The Container type.
///
/// A `Container` acts like an encrypted block device, where you can read and
/// write encrypted blocks of data.
///
/// To create a new container use the [`Container::create`] method. You can
/// open an existing container with the [`Container::open`] method. With the
/// [`Container::read`] and [`Container::write`] methods you can read data from
/// the container resp. write data into the container.
pub struct Container<B: Backend> {
    backend: B,
    store: PasswordStore,
    header: Header,
    buf_in: SecureVec,
    buf_out: SecureVec,
}

impl<B: Backend> Container<B> {
    /// Creates a new container.
    ///
    /// This method expects two arguments:
    ///
    /// 1. `backend_options`, which is a type that implements the
    ///    [`Backend::CreateOptions`] trait. It acts as a builder for a
    ///    concrete [`Backend`] instance.
    /// 2. `options`, which is a builder of this `Container`. A
    ///    [`CreateOptions`] instance can be created with the
    ///    [`CreateOptionsBuilder`] utility.
    ///
    /// If encryption is turned on, you will be asked for a password over the
    /// [password callback](CreateOptionsBuilder::with_password_callback). The
    /// returned password is then used for encryption of the secure part of the
    /// header.
    ///
    /// The header with the (possibly encrypted) secret is created and passed
    /// to the [`Backend`]. The header contains all information you need to
    /// open the container again.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn create(
        mut backend_options: B::CreateOptions,
        options: CreateOptions,
    ) -> ContainerResult<Container<B>, B> {
        let header = Header::create(&options)?;
        let settings = backend_options.settings();

        let callback = options.callback.map(|cb| cb.clone());
        let mut store = PasswordStore::new(callback);

        Self::write_header(&mut backend_options, &header, settings, &mut store)?;
        let backend = map_err!(backend_options.build())?;

        debug!(
            "Container created, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container {
            backend,
            store,
            header,
            buf_in: vec![].into(),
            buf_out: vec![].into(),
        })
    }

    /// Opens an existing container.
    ///
    /// This method expects two arguments:
    ///
    /// 1. `backend_options`, which is a type that implements the
    ///    [`Backend::OpenOptions`] trait. It acts as a builder for a concrete
    ///    [`Backend`] instance.
    /// 2. `options`, which is a builder of this `Container`. A
    ///    [`OpenOptions`] instance can be created with the
    ///    [`OpenOptionsBuilder`] utility.
    ///
    /// If encryption is turned on for the container, you will be asked for a
    /// password over the
    /// [password callback](OpenOptionsBuilder::with_password_callback). The
    /// returned password is then used to decrypt the secure part of the header.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn open(
        mut backend_options: B::OpenOptions,
        options: OpenOptions,
    ) -> ContainerResult<Container<B>, B> {
        let callback = options.callback.map(|cb| cb.clone());
        let mut store = PasswordStore::new(callback);

        let (header, settings) = Self::read_header(&mut backend_options, &mut store)?;
        let backend = map_err!(backend_options.build(settings))?;

        debug!(
            "Container opened, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container {
            backend,
            store,
            header,
            buf_in: vec![].into(),
            buf_out: vec![].into(),
        })
    }

    /// Returns the backend of this container.
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Consumes this container, returning the inner backend.
    pub fn into_backend(self) -> B {
        self.backend
    }

    /// Returns information from the container.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn info(&self) -> ContainerResult<Info<B>, B> {
        let backend = map_err!(self.backend.info())?;

        Ok(Info {
            backend,
            cipher: self.header.cipher,
            kdf: self.header.kdf.clone(),
            bsize_gross: self.backend.block_size(),
            bsize_net: self.block_size(),
        })
    }

    /// Returns userdata assigned to the container.
    ///
    /// Userdata are arbitrary data stored in the (encrypted) header of the
    /// container. It can be used by a service running on top of a _nuts_
    /// container to store information about the service itself.
    pub fn userdata(&self) -> &[u8] {
        &self.header.userdata
    }

    /// Updates the userdata.
    ///
    /// Assigns a new set of new userdata to the container; any previous
    /// userdata are overwritten.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn update_userdata(&mut self, userdata: &[u8]) -> ContainerResult<(), B> {
        let (mut header, settings) = Self::read_header(&mut self.backend, &mut self.store)?;

        header.userdata.clear();
        header.userdata.extend_from_slice(userdata);

        Self::write_header(&mut self.backend, &header, settings, &mut self.store)?;
        self.header = header;

        Ok(())
    }

    /// The (net) block size specifies the number of userdata bytes you can
    /// store in a block. It can be less than the gross block size specified by
    /// the [backend](Backend::block_size)!
    ///
    /// Depending on the selected cipher, you need to store additional data in
    /// a block. I.e. an AE-cipher results into a tag, which needs to be stored
    /// additionally. Such data must be substracted from the gross block size
    /// and results into the net block size.
    pub fn block_size(&self) -> u32 {
        self.backend
            .block_size()
            .checked_sub(self.header.cipher.tag_size())
            .unwrap_or(0)
    }

    /// Aquires a new block in the backend.
    ///
    /// Once aquired you should be able to [read](Container::read) and
    /// [write](Container::write) from/to it.
    ///
    /// By default an aquired block, which is not written yet, returns an
    /// all-zero buffer.
    ///
    /// Returns the [id](Backend::Id) of the block.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn aquire(&mut self) -> ContainerResult<B::Id, B> {
        map_err!(self.backend.aquire())
    }

    /// Releases a block again.
    ///
    /// A released block cannot be [read](Container::read) and
    /// [written](Container::write), the [id](Backend::Id) cannot be used
    /// afterwards.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn release(&mut self, id: B::Id) -> ContainerResult<(), B> {
        map_err!(self.backend.release(id))
    }

    /// Reads a block from the container.
    ///
    /// Reads the block with the given `id` and places the decrypted data in
    /// `buf`.
    ///
    /// You cannot read not more data than [block-size](Backend::block_size)
    /// bytes. If `buf` is larger, than not the whole buffer is filled. In the
    /// other direction, if `buf` is not large enough to store the whole block,
    /// `buf` is filled with the first `buf.len()` bytes.
    ///
    /// The methods returns the number of bytes actually read, which cannot be
    /// greater than the [block-size](Backend::block_size).
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn read(&mut self, id: &B::Id, buf: &mut [u8]) -> ContainerResult<usize, B> {
        if id.is_null() {
            return Err(Error::NullId);
        }

        self.buf_in.resize(self.backend.block_size() as usize, 0);
        map_err!(self.backend.read(id, &mut self.buf_in))?;

        let key = &self.header.key;
        let iv = &self.header.iv;

        self.header
            .cipher
            .decrypt(&self.buf_in, &mut self.buf_out, key, iv)?;

        let n = cmp::min(self.buf_out.len(), buf.len());
        buf[..n].copy_from_slice(&self.buf_out[..n]);

        Ok(n)
    }

    /// Writes a block into the container.
    ///
    /// Encrypts the plain data from `buf` and writes the encrypted data into
    /// the block with the given `id`.
    ///
    /// Writes up to `buf.len()` bytes from the unencrypted `buf` buffer into
    /// the container.
    ///
    /// If `buf` is not large enough to fill the whole block, the destination
    /// block is automatically padded with all zeros.
    ///
    /// If `buf` holds more data than the block-size, then the first
    /// [block-size](Backend::block_size) bytes are copied into the block.
    ///
    /// The method returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`Error`] type.
    pub fn write(&mut self, id: &B::Id, buf: &[u8]) -> ContainerResult<usize, B> {
        if id.is_null() {
            return Err(Error::NullId);
        }

        let block_size = self.backend.block_size() as usize;
        let block_size_net = block_size - self.header.cipher.tag_size() as usize;

        let key = &self.header.key;
        let iv = &self.header.iv;

        let mut ptext = Cow::from(buf);
        let ptext_len = cmp::min(ptext.len(), block_size_net);

        if ptext.len() != block_size_net {
            // pad with 0 if not a complete block
            ptext.to_mut().resize(block_size_net, 0);
        }

        self.header
            .cipher
            .encrypt(&ptext, &mut self.buf_out, key, iv)?;

        match ptext {
            Cow::Owned(buf) => {
                // whiteout owned buffer
                let _: SecureVec = buf.into();
            }
            _ => {}
        };

        map_err!(self.backend.write(id, &self.buf_out)).map(|_| ptext_len)
    }

    fn read_header<H: HeaderGet<B>>(
        reader: &mut H,
        store: &mut PasswordStore,
    ) -> ContainerResult<(Header, B::Settings), B> {
        let mut buf = [0; HEADER_MAX_SIZE];

        match reader.get_header_bytes(&mut buf) {
            Ok(_) => Ok(Header::read::<B>(&buf, store)?),
            Err(cause) => Err(Error::Backend(cause)),
        }
    }

    fn write_header<H: HeaderSet<B>>(
        writer: &mut H,
        header: &Header,
        settings: B::Settings,
        store: &mut PasswordStore,
    ) -> ContainerResult<(), B> {
        let mut buf = [0; HEADER_MAX_SIZE];

        header.write::<B>(settings, &mut buf, store)?;
        map_err!(writer.put_header_bytes(&buf))
    }
}
