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

pub mod plugin;

use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::{error, result};

use nuts_bytes::{FromBytes, ToBytes};

/// The minimum size of a block.
pub const BLOCK_MIN_SIZE: u32 = 512;

/// Trait that needs to be implemented by [`Backend::CreateOptions`] and
/// [`Backend::OpenOptions`].
pub trait Options<B: Backend> {
    /// Validates this `Options` implementation.
    ///
    /// It should check whether all values are valid.
    ///
    /// # Errors
    ///
    /// In case of an invalid value a self-defined [`Backend::Err`] is returned.
    fn validate(&self) -> result::Result<(), B::Err>;
}

/// Trait identifies a block in the storage.
pub trait BlockId: Clone + Debug + Display + FromBytes + FromStr + PartialEq + ToBytes {
    /// Creates a null-id.
    ///
    /// A null-id does not point to a block. It points to nowhere.
    fn null() -> Self;

    /// Tests whether this id is a null-id.
    fn is_null(&self) -> bool;

    /// Returns the number of bytes needed to store the id.
    fn size() -> usize;
}

pub trait Backend
where
    Self: Sized,
{
    /// Options used to create a backend instance.
    ///
    /// Passed to [`Backend::create`] and customizes the backend.
    type CreateOptions: Options<Self>;

    /// Options used to open a backend instance.
    ///
    /// Passed to [`Backend::open`] and contains options used to open the
    /// backend.
    type OpenOptions: Options<Self>;

    /// Runtime configuration used by the backend.
    ///
    /// It should contain all settings/information needed by the backend. It is
    /// loaded from the header when the backend is [opened](Backend::open).
    ///
    /// The [`Backend::create`] method returns an instance of this type, so the
    /// container stores the settings (possibly encrypted) in the header of the
    /// container.
    ///
    /// When the container is [opened](nuts::container::Container::open), the
    /// [`Backend::open`] method is called to open the backend. Next, the
    /// container reads its header, extracts the settings from it and pass it
    /// down to the backend by calling the [`Backend::open_ready`] method.
    type Settings: Clone + FromBytes + ToBytes;

    /// The error type used by methods of this trait.
    type Err: error::Error + Send + Sync;

    /// The id identifies a block in the storage. It is used everywhere you
    /// need a pointer to a block.
    type Id: BlockId;

    /// Information of the backend.
    ///
    /// It includes information like public settings. The difference to
    /// [`Backend::Settings`] is that [`Backend::Settings`] might include
    /// sensible information which are removed from [`Backend::Info`].
    type Info;

    /// Creates a new instance of the backend.
    ///
    /// On success the backend is open and ready to read and write data.
    ///
    /// The method returns
    /// * The backend instance itself. The upper container uses this instance.
    /// * A [`Backend::Settings`] instance. The settings are stored in the
    /// header of the container and are extracted again when the container is
    /// opened again.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn create(options: Self::CreateOptions) -> result::Result<(Self, Self::Settings), Self::Err>;

    /// Opens an instance of the backend.
    ///
    /// On success the backend is open and ready to read and write data. The
    /// [settings](Backend::Settings) are sill missing. Once the backend is
    /// open, the upper container reads the header and extracts the settings
    /// from it. Finally it calls [`Backend::open_ready`] to assign the
    /// settings to its backend. Thats why there is one [`Backend::read`]
    /// attempt to the backend, which reads the header block. This read should
    /// succeed even without complete settings. Consider using default values
    /// for your settings.
    ///
    /// The method returns the backend instance itself.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn open(options: Self::OpenOptions) -> result::Result<Self, Self::Err>;

    /// Method is called by the container when the settings of the backend are
    /// available.
    ///
    /// Once the backend is [open](Backend::open), the upper container reads
    /// the header and extracts the settings from it. Finally it calls
    /// [`Backend::open_ready`] to assign the settings to its backend.
    fn open_ready(&mut self, settings: Self::Settings);

    /// Returns information from the backend.
    ///
    /// It includes information like public settings. The difference to
    /// [`Backend::Settings`] is that [`Backend::Settings`] might include
    /// sensible information which are removed from [`Backend::Info`].
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn info(&self) -> result::Result<Self::Info, Self::Err>;

    /// Returns the current block size.
    ///
    /// * If the backend was [created](Backend::create) you should be able to
    ///   provide the final block size because all options and settings are
    ///   available.
    /// * If the backend was [opened](Backend::open) and [Self::open_ready()]
    ///   was not invoked yet, the final block size might not be available. In
    ///   this case [`BLOCK_MIN_SIZE`] should be returned.
    fn block_size(&self) -> u32;

    /// Returns the if where the container stores the header.
    fn header_id(&self) -> Self::Id;

    /// Aquires a new block in the backend.
    ///
    /// Once aquired you should be able to [read](Backend::read) and
    /// [write](Backend::write) from/to it.
    ///
    /// By default an aquired block, which is not written yet, should return
    /// an all-zero buffer.
    ///
    /// Returns the [id](Backend::Id) if the block.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn aquire(&mut self) -> result::Result<Self::Id, Self::Err>;

    /// Releases a block again.
    ///
    /// A released block cannot be [read](Backend::read) and
    /// [written](Backend::write), the [id](Self::Id) cannot be used
    /// afterwards.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn release(&mut self, id: Self::Id) -> result::Result<(), Self::Err>;

    /// Reads a block from the backend.
    ///
    /// Reads the block with the given `id` and places the data in `buf`.
    ///
    /// You cannot read not more data than the
    /// [block-size](Backend::block_size) bytes. If `buf` is larger, than not
    /// the whole buffer is filled. In the other direction, if `buf` is not
    /// large enough to store the whole block, `buf` is filled with the first
    /// `buf.len()` bytes.
    ///
    /// The methods returns the number of bytes actually read, which cannot be
    /// greater than the [block-size](Backend::block_size).
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn read(&mut self, id: &Self::Id, buf: &mut [u8]) -> result::Result<usize, Self::Err>;

    /// Writes a block into the backend.
    ///
    /// Writes up to `buf.len()` bytes from the unencrypted `buf` buffer into
    /// the block with the given `id`.
    ///
    /// * If `buf` is not large enough to fill the whole block, the destination
    ///   block should be padded with all zeros.
    /// * If `buf` holds more data than the [block-size](Backend::block_size),
    ///   then only the first [block-size](Backend::block_size) bytes are
    ///   copied into the block.
    ///
    /// The method returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn write(&mut self, id: &Self::Id, buf: &[u8]) -> result::Result<usize, Self::Err>;
}
