// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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
//! See the [`backend` crate](nuts_backend) documentation for details.
//!
//! ## Create a container
//!
//! The [`Container::create()`] method is used to create a new container. It
//! expects an instance of a type that implements the [`Create`] trait, which
//! acts as a builder for the related [`Backend`].
//!
//! Example:
//!
//! ```rust
//! use nuts_container::*;
//! use nuts_memory::MemoryBackend;
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
//! instance of a type that implements the [`Open`] trait, which acts as a
//! builder for the related [`Backend`].
//!
//! Example:
//!
//! ```rust
//! use nuts_container::*;
//! use nuts_memory::MemoryBackend;
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
//! use nuts_container::*;
//! use nuts_memory::MemoryBackend;
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
//! use nuts_container::*;
//! use nuts_memory::MemoryBackend;
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
//!   * _top-id_: The _top-id_ points to some kind of super-block. During
//!     [service-creation](Container::create_service) the super-block is
//!     aquired (if requested by the service) and its id (the _top-id_) is
//!     stored in the _secret_.
//!   * _settings of the backend_: The backend of the container stores its
//!     runtime information in the secret. It gets it back when opening the
//!     backend again. See [`Backend::Settings`] for more information.

mod buffer;
mod cipher;
mod digest;
mod error;
mod header;
mod info;
mod kdf;
mod migrate;
mod options;
mod ossl;
mod password;
mod service;
mod svec;
#[cfg(test)]
mod tests;

use log::debug;
use nuts_backend::{Backend, Create, Open, ReceiveHeader, HEADER_MAX_SIZE};
use std::{any, cmp};

use crate::cipher::CipherContext;
use crate::header::Header;
use crate::migrate::Migrator;
use crate::password::PasswordStore;

pub use buffer::BufferError;
pub use cipher::{Cipher, CipherError};
pub use digest::Digest;
pub use error::{ContainerResult, Error};
pub use header::HeaderError;
pub use info::Info;
pub use kdf::{Kdf, KdfError};
pub use migrate::{Migration, MigrationError};
pub use options::{CreateOptions, CreateOptionsBuilder, OpenOptions, OpenOptionsBuilder};
pub use password::PasswordError;
pub use service::{Service, ServiceFactory};

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
#[derive(Debug)]
pub struct Container<B: Backend> {
    backend: B,
    store: PasswordStore,
    header: Header<'static, B>,
    ctx: CipherContext,
}

impl<B: Backend> Container<B> {
    /// Creates a new container.
    ///
    /// This method expects two arguments:
    ///
    /// 1. `backend_options`, which is a type that implements the
    ///    [`Create`] trait. It acts as a builder for a concrete [`Backend`]
    ///    instance.
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
    pub fn create<C: Create<B>>(
        backend_options: C,
        options: CreateOptions,
    ) -> ContainerResult<Container<B>, B> {
        let mut header_bytes = [0; HEADER_MAX_SIZE];
        let settings = backend_options.settings();
        let header = Header::create(&options, settings)?;

        let callback = options.callback.clone();
        let mut store = PasswordStore::new(callback);

        header.write(&mut header_bytes, &mut store)?;

        let backend = map_err!(backend_options.build(header_bytes, options.overwrite))?;

        debug!(
            "Container created, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        let ctx = CipherContext::new(header.cipher());

        Ok(Container {
            backend,
            store,
            header,
            ctx,
        })
    }

    /// Creates a [service](Service) running on top of the given `container`.
    ///
    /// Basically, this method performs the following tasks:
    ///
    /// 1. The super-block is created, if [requested by the service](Service::need_top_id).
    /// 2. Uses [`ServiceFactory::create`] to create and return the service
    ///    instance.
    ///
    /// This should be the preferred way to create a nuts-service!
    pub fn create_service<F: ServiceFactory<B>>(
        mut container: Container<B>,
    ) -> Result<F::Service, F::Err> {
        // ensure that you are on the current revision
        container
            .header
            .latest_revision_or_err()
            .map_err(Error::<B>::Header)?;

        // ensure that the container does not already have a service
        container
            .header
            .accept_sid_for_create()
            .map_err(Error::<B>::Header)?;

        // aquire top-id (if requested)
        let top_id = if F::Service::need_top_id() {
            Some(container.aquire()?)
        } else {
            None
        };

        container.update_header(|header| {
            header.set_sid(F::Service::sid())?;

            if let Some(id) = top_id {
                header.set_top_id(id);
            }

            Ok(())
        })?;

        F::create(container)
    }

    /// Opens an existing container.
    ///
    /// This method expects two arguments:
    ///
    /// 1. `backend_options`, which is a type that implements the [`Open`]
    ///    trait. It acts as a builder for a concrete [`Backend`] instance.
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
    pub fn open<O: Open<B>>(
        mut backend_options: O,
        options: OpenOptions,
    ) -> ContainerResult<Container<B>, B> {
        let callback = options.callback.clone();
        let mut store = PasswordStore::new(callback);
        let migrator = Migrator::default();

        let mut header = Self::read_header(&mut backend_options, migrator, &mut store)?;
        let settings = header.settings().clone();
        let backend = map_err!(backend_options.build(settings))?;

        header.migrate()?;

        debug!(
            "Container opened, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        let ctx = CipherContext::new(header.cipher());

        Ok(Container {
            backend,
            store,
            header,
            ctx,
        })
    }

    /// Opens a [service](Service) running on top of an existing container.
    ///
    /// Basically, this method uses [`ServiceFactory::open`] to open and return
    /// the service instance.
    ///
    /// This should be the preferred way to open a nuts-service!
    pub fn open_service<F: ServiceFactory<B>>(
        mut container: Container<B>,
    ) -> Result<F::Service, F::Err> {
        let migration = F::Service::migration();
        let migrator = Migrator::default().with_migration(migration);

        container.header.set_migrator(migrator);
        container.header.migrate().map_err(Error::<B>::Header)?;
        container
            .header
            .accept_sid_for_open(F::Service::sid())
            .map_err(Error::Header)?;

        F::open(container)
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
            revision: self.header.revision(),
            cipher: self.header.cipher(),
            kdf: self.header.kdf().clone(),
            bsize_gross: self.backend.block_size(),
            bsize_net: self.block_size(),
        })
    }

    /// Returns the _top-id_ of the container.
    ///
    /// A service (running on top of the container) can use the _top-id_ as a
    /// starting point or some kind of _super-block_. The _top-id_ is stored
    /// encrypted in the header of the container. Calling this method will
    /// neither fetch nor create the _top-id_. It returns an entry, where you
    /// can decide what to do.
    pub fn top_id(&self) -> Option<&B::Id> {
        self.header.top_id()
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
            .saturating_sub(self.header.cipher().tag_size())
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
        let key = self.header.key();
        let iv = self.header.iv();

        self.ctx.copy_from_slice(self.block_size() as usize, &[]);
        let ctext = self.ctx.encrypt(key, iv)?;

        map_err!(self.backend.aquire(ctext))
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
        let ctext = self.ctx.inp_mut(self.backend.block_size() as usize);
        map_err!(self.backend.read(id, ctext))?;

        let key = self.header.key();
        let iv = self.header.iv();

        let ptext = self.ctx.decrypt(key, iv)?;

        let n = cmp::min(ptext.len(), buf.len());
        buf[..n].copy_from_slice(&ptext[..n]);

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
        let len = self.ctx.copy_from_slice(self.block_size() as usize, buf);

        let key = self.header.key();
        let iv = self.header.iv();

        let ctext = self.ctx.encrypt(key, iv)?;

        map_err!(self.backend.write(id, ctext)).map(|_| len)
    }

    fn read_header<H: ReceiveHeader<B>>(
        reader: &mut H,
        migrator: Migrator<'static>,
        store: &mut PasswordStore,
    ) -> ContainerResult<Header<'static, B>, B> {
        let mut buf = [0; HEADER_MAX_SIZE];

        match reader.get_header_bytes(&mut buf) {
            Ok(_) => {
                debug!("got {} header bytes", buf.len());
                Ok(Header::read(&buf, migrator, store)?)
            }
            Err(cause) => Err(Error::Backend(cause)),
        }
    }

    fn update_header<F: FnOnce(&mut Header<B>) -> Result<(), HeaderError>>(
        &mut self,
        f: F,
    ) -> ContainerResult<(), B> {
        let migrator = Migrator::default();
        let mut header = Self::read_header(&mut self.backend, migrator, &mut self.store)?;
        let mut header_bytes = [0; HEADER_MAX_SIZE];

        debug!("header before update: {:?}", header);

        f(&mut header)?;

        debug!("header after update: {:?}", header);

        header.write(&mut header_bytes, &mut self.store)?;
        map_err!(self.backend.write_header(&header_bytes))?;

        self.header = header;

        Ok(())
    }

    /// Deletes the entire container and all traces.
    ///
    /// The method must not fail!
    pub fn delete(self) {
        self.backend.delete()
    }
}
