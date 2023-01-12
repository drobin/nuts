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

mod cipher;
mod digest;
mod error;
mod header;
mod info;
mod kdf;
mod options;
mod password;

use log::debug;
use std::borrow::Cow;
use std::{any, cmp};

use nuts_backend::{Backend, BlockId, BLOCK_MIN_SIZE};

use crate::container::cipher::CipherCtx;
use crate::container::header::Header;
use crate::container::password::PasswordStore;
use crate::svec::SecureVec;

pub use cipher::Cipher;
pub use digest::Digest;
pub use error::{ContainerError, ContainerResult};
pub use info::Info;
pub use kdf::Kdf;
pub use options::{CreateOptions, CreateOptionsBuilder, OpenOptions, OpenOptionsBuilder};

macro_rules! map_err {
    ($result:expr) => {
        $result.map_err(|cause| ContainerError::Backend(cause))
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
pub struct Container<B> {
    backend: B,
    header: Header,
    ctx: CipherCtx,
}

impl<B: Backend> Container<B> {
    /// Creates a new container.
    ///
    /// The new container is initialized with the given `options`. In case of
    /// an invalid option, the container is not created.
    ///
    // If encryption is turned on, you will be asked for a password over the
    // [`password callback`]. The returned password is then used for
    // encryption of the secure part of the header.
    //
    /// The header with the (possibly encrypted) secret is created and stored
    /// in the header-block of the container. The header contains all
    /// information you need to open the container again.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`ContainerError`] type.
    // If encryption is enabled but no password callback is assigned or the
    // assigned callback returns an error, an [`Error::NoPassword`] error is
    // returned.
    //
    // Further errors are listed in the [`Error`] type.
    pub fn create(options: CreateOptions<B>) -> ContainerResult<Container<B>, B> {
        let header = Header::create(&options)?;
        let (mut backend, settings) = map_err!(B::create(options.backend))?;
        let ctx = CipherCtx::new(header.cipher, backend.block_size())?;

        let callback = options.callback.map(|cb| cb.clone());
        let mut store = PasswordStore::new(callback);

        Self::write_header(&mut backend, &header, &settings, &mut store)?;

        debug!(
            "Container created, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container {
            backend,
            header,
            ctx,
        })
    }

    /// Opens an existing container.
    ///
    /// The `options` argument are options used to open the container. Use the
    /// [`OpenOptionsBuilder`] utility to create such an instance.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`ContainerError`] type.
    // If encryption is enabled but no password callback is assigned or the
    // assigned callback returns an error, an [`Error::NoPassword`] error is
    // returned.
    ///
    // Further errors are listed in the [`Error`] type.
    pub fn open(options: OpenOptions<B>) -> ContainerResult<Container<B>, B> {
        let callback = options.callback.map(|cb| cb.clone());
        let mut store = PasswordStore::new(callback);

        let mut backend = map_err!(B::open(options.backend))?;
        let (header, settings) = Self::read_header(&mut backend, &mut store)?;

        backend.open_ready(settings);

        let ctx = CipherCtx::new(header.cipher, backend.block_size())?;

        debug!(
            "Container opened, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container {
            backend,
            header,
            ctx,
        })
    }

    /// Returns the backend of this container.
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns information from the container.
    ///
    /// # Errors
    ///
    /// Errors are listed in the [`ContainerError`] type.
    pub fn info(&self) -> ContainerResult<Info<B>, B> {
        let backend = map_err!(self.backend.info())?;

        Ok(Info {
            backend,
            cipher: self.header.cipher,
            kdf: self.header.kdf.clone(),
        })
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
    /// Errors are listed in the [`ContainerError`] type.
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
    /// Errors are listed in the [`ContainerError`] type.
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
    /// Errors are listed in the [`ContainerError`] type.
    pub fn read(&mut self, id: &B::Id, buf: &mut [u8]) -> ContainerResult<usize, B> {
        if id.is_null() {
            return Err(ContainerError::NullId);
        }

        let mut ctext = vec![0; self.backend.block_size() as usize];
        let n = map_err!(self.backend.read(id, &mut ctext))?;

        let key = &self.header.key;
        let iv = &self.header.iv;
        let ptext = self.ctx.decrypt(key, iv, &ctext[..n])?;

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
    /// Errors are listed in the [`ContainerError`] type.
    pub fn write(&mut self, id: &B::Id, buf: &[u8]) -> ContainerResult<usize, B> {
        if id.is_null() {
            return Err(ContainerError::NullId);
        }

        let block_size = self.backend.block_size() as usize;
        let key = &self.header.key;
        let iv = &self.header.iv;

        let mut ptext = Cow::from(buf);

        if ptext.len() < block_size {
            // pad with 0 if not a complete block
            ptext.to_mut().resize(block_size, 0);
        }

        let result = self.ctx.encrypt(key, iv, &ptext);

        match ptext {
            Cow::Owned(buf) => {
                SecureVec::from_vec(buf);
            }
            _ => {}
        };

        let ctext = result?;
        map_err!(self.backend.write(id, ctext))
    }

    fn read_header(
        backend: &mut B,
        store: &mut PasswordStore,
    ) -> ContainerResult<(Header, B::Settings), B> {
        let id = backend.header_id();
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        match backend.read(&id, &mut buf) {
            Ok(_) => Ok(Header::read(&buf, store)?),
            Err(cause) => Err(ContainerError::Backend(cause)),
        }
    }

    fn write_header(
        backend: &mut B,
        header: &Header,
        envelope: &B::Settings,
        store: &mut PasswordStore,
    ) -> ContainerResult<usize, B> {
        let id = backend.header_id();
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        header.write::<B>(&envelope, &mut buf, store)?;

        map_err!(backend.write(&id, &buf))
    }
}
