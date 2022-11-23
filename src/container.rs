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

mod cipher;
mod error;
mod header;
mod info;
mod options;

use log::debug;
use std::any;

use crate::backend::{Backend, BLOCK_MIN_SIZE};
use crate::container::header::Header;

pub use cipher::Cipher;
pub use error::{ContainerError, ContainerResult};
pub use info::Info;
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
#[derive(Debug)]
pub struct Container<B> {
    backend: B,
    header: Header,
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

        Self::write_header(&mut backend, &header, &settings)?;

        debug!(
            "Container created, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container { backend, header })
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
        let mut backend = map_err!(B::open(options.backend))?;
        let (header, settings) = Self::read_header(&mut backend)?;

        backend.open_ready(settings);

        debug!(
            "Container opened, backend: {}, header: {:?}",
            any::type_name::<B>(),
            header
        );

        Ok(Container { backend, header })
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
        map_err!(self.backend.read(id, buf))
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
    /// block is automatically padded:
    ///
    /// * If encryption is enabled, than the padding is filled with random
    ///   data.
    /// * If encryption is disabled, than the padding is initialized with
    ///   all zeros.
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
        map_err!(self.backend.write(id, buf))
    }

    fn read_header(backend: &mut B) -> ContainerResult<(Header, B::Settings), B> {
        let id = backend.header_id();
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        match backend.read(&id, &mut buf) {
            Ok(_) => Ok(Header::read::<B>(&buf)?),
            Err(cause) => Err(ContainerError::Backend(cause)),
        }
    }

    fn write_header(
        backend: &mut B,
        header: &Header,
        envelope: &B::Settings,
    ) -> ContainerResult<usize, B> {
        let id = backend.header_id();
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        header.write::<B>(&envelope, &mut buf)?;

        map_err!(backend.write(&id, &buf))
    }
}
