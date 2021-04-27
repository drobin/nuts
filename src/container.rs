// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

pub(crate) mod inner;

use log::debug;
use std::path::Path;

use crate::container::inner::Inner;
use crate::error::Error;
use crate::password::PasswordStore;
use crate::result::Result;
use crate::types::{Cipher, DiskType, Options, WrappingKey};

pub struct Container {
    store: PasswordStore,
    inner: Option<Inner>,
}

impl Container {
    /// Creates a new closed container.
    ///
    /// You need to call [`create()`] or [`open()`] to open the container. An
    /// operation on a closed container will raise an [`Error::Closed`] error.
    ///
    /// [`create()`]: #method.create
    /// [`open()`]: #method.open
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn new() -> Container {
        Container {
            store: PasswordStore::new(),
            inner: None,
        }
    }

    /// Assigns a password callback to the container.
    ///
    /// A password is needed, when encryption is enabled for the container.
    /// Based on the password a wrapping key is generated, which encrypts the
    /// secret part of the header. If encryption is enabled but no password
    /// callback is assigned, an [`Error::NoPassword`] error is raised. If
    /// encryption is disabled, no password is needed and an assigned callback
    /// is never called.
    ///
    /// The callback returns a [`Vec<u8>`] wrapped into a [`Result`]. On
    /// success the callback returns the password (represented as an
    /// [`Vec<u8>`]) wrapped into an [`Ok`]. On any failure an [`Err`] value
    /// must be returned.
    ///
    /// [`Vec<u8>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`Error::NoPassword`]: ../error/enum.Error.html#variant.NoPassword
    /// [`Result`]: ../result/type.Result.html
    /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    pub fn set_password_callback(&mut self, callback: impl Fn() -> Result<Vec<u8>> + 'static) {
        self.store.set_callback(callback);
    }

    /// Creates a new container.
    ///
    /// The new container is initialized with the given `options` and stored in
    /// a file located at `path`. In case of an invalid option, the container
    /// is not created.
    ///
    /// If encryption is turned on, you will be asked for a password over the
    /// [`password callback`]. The returned password is then used for
    /// encryption of the secure part of the header.
    ///
    /// **Note**, that you cannot overwrite an existing file! Passing an
    /// exising file to `path` will abort the operation.
    ///
    /// The header with the (possibly encrypted) secret is created and stored
    /// in the first block of the container. The header contains all
    /// information you need to open the container again.
    ///
    /// # Errors
    ///
    /// The container must be closed before calling this method. If you call
    /// this method on an open container, an [`Error::Opened`] error is
    /// returned.
    ///
    /// If encryption is enabled but no password callback is assigned or the
    /// assigned callback returns an error, an [`Error::NoPassword`] error is
    /// returned.
    ///
    /// Further errors are listed in the [`Error`] type.
    ///
    /// [`password callback`]: #method.set_password_callback
    /// [`Error`]: ../error/enum.Error.html
    /// [`Error::Opened`]: ../error/enum.Error.html#variant.Opened
    /// [`Error::NoPassword`]: ../error/enum.Error.html#variant.NoPassword
    pub fn create(&mut self, path: impl AsRef<Path>, options: Options) -> Result<()> {
        if self.inner.is_none() {
            let inner = Inner::create(&path, options, &mut self.store)?;

            debug!(
                "allocating container, dtype = {:?}, bsize = {}, blocks = {}",
                inner.header.dtype, inner.header.bsize, inner.header.blocks
            );

            self.inner = Some(inner);

            Ok(())
        } else {
            Err(Error::Opened)
        }
    }

    /// Opens an existing container.
    ///
    /// Opens a container, which is stored in a file located in `path`.
    ///
    /// The first physical block contains the header, which stores all relevant
    /// data needed to open the container. If encryption is turned on, then you
    /// will be asked for a password over the [`password callback`].
    ///
    /// If [`Some`] userdata is passed to the method, the wrapped vector is
    /// filled with the userdata stored in the header. If no userdata are
    /// stored in the container, the wrapped vector will be empty. If a
    /// [`None`] value is passed to the `userdata` argument, the userdata
    /// stored in the header are ignored.
    ///
    /// # Errors
    ///
    /// The container must be closed before calling this method. If you call
    /// this method on an open container, an [`Error::Opened`] error is
    /// returned.
    ///
    /// If encryption is enabled but no password callback is assigned or the
    /// assigned callback returns an error, an [`Error::NoPassword`] error is
    /// returned.
    ///
    /// Further errors are listed in the [`Error`] type.
    ///
    /// [`password callback`]: #method.set_password_callback
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#Some.v
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#None.v
    /// [`Error`]: ../error/enum.Error.html
    /// [`Error::Opened`]: ../error/enum.Error.html#variant.Opened
    /// [`Error::NoPassword`]: ../error/enum.Error.html#variant.NoPassword
    pub fn open(&mut self, path: impl AsRef<Path>, userdata: Option<&mut Vec<u8>>) -> Result<()> {
        if self.inner.is_none() {
            let inner = Inner::open(&path, &mut self.store)?;

            if let Some(userdata) = userdata {
                userdata.clear();
                userdata.extend(&inner.header.userdata);
            };

            self.inner = Some(inner);

            Ok(())
        } else {
            Err(Error::Opened)
        }
    }

    /// Tests whether the container is open.
    ///
    /// The container is open, if [`create()`] or [`open()`] was called successfully.
    ///
    /// [`create()`]: #method.create
    /// [`open()`]: #method.open
    pub fn is_open(&self) -> bool {
        self.on_open(|_| Ok(true)).is_ok()
    }

    /// Returns the userdata stored in the header of the container.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed. Further errors are listed in the [`Error`] type.
    ///
    /// [`Error`]: ../error/enum.Error.html
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn get_userdata(&self) -> Result<&[u8]> {
        self.on_open(|inner| Ok(&inner.header.userdata[..]))
    }

    /// Updates the userdata stored in the header of the container.
    ///
    /// Passing an empty slice to `userdata` will clear the userdata stored in
    /// the container. You can receive the userdata over the [`open()`] call
    /// again.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed. Further errors are listed in the [`Error`] type.
    ///
    /// [`open()`]: #method.open
    /// [`Error`]: ../error/enum.Error.html
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn set_userdata(&mut self, userdata: &[u8]) -> Result<()> {
        if self.is_open() {
            let inner = self.inner.as_mut().unwrap();

            inner.header.userdata.clear();
            inner.header.userdata.extend(userdata);

            inner.flush_header(&mut self.store)?;

            Ok(())
        } else {
            Err(Error::Closed)
        }
    }

    /// Reads a block from the container.
    ///
    /// Reads the block with the given `id` and places the decrypted data in
    /// `buf`.
    ///
    /// You cannot read not more data than the [block-size] bytes. If `buf` is
    /// larger, than not the whole buffer is filled. In the other direction, if
    /// `buf` is not large enough to store the whole block, `buf` is filled
    /// with the first [`buf.len()`] bytes.
    ///
    /// The methods returns the number of bytes actually read, which cannot be
    /// greater than the [block-size].
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed. Further errors are listed in the [`Error`] type.
    ///
    /// [block-size]: #method.bsize
    /// [`buf.len()`]: https://doc.rust-lang.org/std/primitive.slice.html#method.len
    /// [`Error`]: ../error/enum.Error.html
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn read(&mut self, id: u64, buf: &mut [u8]) -> Result<u32> {
        self.on_open_mut(|inner| inner.read_block(buf, id + 1))
    }

    /// Writes a block into the container.
    ///
    /// Encrypts the plain data from `buf` and writes the encrypted data into
    /// the block with the given `id`.
    ///
    /// Writes up to [`buf.len()`] bytes from the unencrypted `buf` buffer into
    /// the container.
    ///
    /// If `buf` is not large enough to fill the while block, the destination
    /// block is automatically padded:
    ///
    /// * If you have a random [disk-type] ([`DiskType::ThinRandom`],
    ///   [`DiskType::FatRandom`]), than the padding is filled with random
    ///   data.
    /// * If you have a zero [disk-type] ([`DiskType::ThinZero`],
    ///   [`DiskType::FatZero`]), than the padding is initialized with zeros.
    ///
    /// If `buf` holds more data than the block-size, then the first
    /// [block-size] bytes are copied into the block.
    ///
    /// The method returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed. Further errors are listed in the [`Error`] type.
    ///
    /// [block-size]: #method.bsize
    /// [disk-type]: #method.dtype
    /// [`buf.len()`]: https://doc.rust-lang.org/std/primitive.slice.html#method.len
    /// [`DiskType::FatZero`]: ../types/enum.DiskType.html#variant.FatZero
    /// [`DiskType::FatRandom`]: ../types/enum.DiskType.html#variant.FatRandom
    /// [`DiskType::ThinZero`]: ../types/enum.DiskType.html#variant.ThinZero
    /// [`DiskType::ThinRandom`]: ../types/enum.DiskType.html#variant.ThinRandom
    /// [`Error`]: ../error/enum.Error.html
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn write(&mut self, id: u64, buf: &[u8]) -> Result<u32> {
        self.on_open_mut(|inner| inner.write_block(buf, id + 1))
    }

    /// Returns the [`Cipher`] used by the container.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [`Cipher`]: ../types/enum.Cipher.html
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn cipher(&self) -> Result<Cipher> {
        self.on_open(|inner| Ok(inner.header.cipher))
    }

    /// Returns the [`DiskType`] used by the container.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [`DiskType`]: ../types/enum.DiskType.html
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn dtype(&self) -> Result<DiskType> {
        self.on_open(|inner| Ok(inner.header.dtype))
    }

    /// Returns the gross block size of the container.
    ///
    /// The gross block size is the block size you pass to [`create()`] when
    /// creating a container. The [net block size] is the number of bytes you
    /// can actually store in a block. Depending on the selected cipher, you
    /// might store additionally information in the block, thus the (net)
    /// number of bytes you can store in a block is smaller than the gross
    /// block size.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [net block size]: #method.bsize
    /// [`create()`]: #method.create
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn bsize_gross(&self) -> Result<u32> {
        self.on_open(|inner| Ok(inner.header.bsize))
    }

    /// Returns the (net) block size of the container.
    ///
    /// The net block size is the number of bytes you can store in a block. It
    /// can be less than the gross block size you pass to [`create()`] when
    /// creating a container.
    ///
    /// Depending on the selected cipher, you need to store additional data in
    /// a block. I.e. an AE-cipher results into a tag, which needs to be stored
    /// additionally. Such data must be substracted from the gross block size
    /// and results into the net block size.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [`create()`]: #method.create
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn bsize(&self) -> Result<u32> {
        self.on_open(|inner| Ok(inner.header.bsize_net()))
    }

    /// Returns the number of blocks which can be allocated for the container.
    ///
    /// Multiplied with the [`block size`] it gives you the size of the
    /// container.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [`block size`]: #method.bsize
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn blocks(&self) -> Result<u64> {
        self.on_open(|inner| Ok(inner.header.blocks))
    }

    /// Returns the wrapping key algorithm of the container.
    ///
    /// If encryption is enabled (the cipher is set to something other than
    /// [`Cipher::None`]), the wrapping key algorithm is wrapped into a
    /// [`Some`] value. If the cipher is set to [`Cipher::None`], no wrapping
    /// key is used and a [`None`] value is returned.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [`Cipher::None`]: ../types/enum.Cipher.html#variant.None
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#Some.v
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#None.v
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn wrapping_key(&self) -> Result<Option<WrappingKey>> {
        self.on_open(|inner| Ok(inner.header.wrapping_key.clone()))
    }

    /// Returns the number of currently allocated blocks of the container.
    ///
    /// This is the number of blocks, which are physically available. It
    /// depends on the [`DiskType`] of the container. If you have a fat
    /// container, then all blocks are allocated during creation of the
    /// container, and the number of allocated blocks is equal to the
    /// [`number of blocks`]. If you have a thin container, then the number of
    /// allocated block can be increased during the lifetime of the container -
    /// depending on which blocks are written.
    ///
    /// # Errors
    ///
    /// The method will return an [`Error::Closed`] error, if the container is
    /// closed.
    ///
    /// [`DiskType`]: ../types/enum.DiskType.html
    /// [`number of blocks`]: #method.blocks
    /// [`Error::Closed`]: ../error/enum.Error.html#variant.Closed
    pub fn ablocks(&self) -> Result<u64> {
        self.on_open(|inner| Ok(inner.ablocks))
    }

    fn on_open<'a, R>(&'a self, f: impl Fn(&'a Inner) -> Result<R>) -> Result<R> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| f(inner))
    }

    fn on_open_mut<'a, R>(
        &'a mut self,
        mut f: impl FnMut(&'a mut Inner) -> Result<R>,
    ) -> Result<R> {
        self.inner
            .as_mut()
            .map_or(Err(Error::Closed), |inner| f(inner))
    }
}
