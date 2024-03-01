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

//! The backend of a container
//!
//! # Create a container
//!
//! The [`Create`] trait is used to create a new instance of a [`Backend`].
//!
//! Mainly, is performs the following tasks:
//!
//! 1. It creates the [`Backend::Settings`] of the backend. The settings
//!    contains runtime information which are stored in the header of the
//!    container. Later, when the container is opened again, the settings are
//!    extracted from the header and passed back to the backend.
//! 2. It creates the header of the backend instance using the [`HeaderSet`]
//!    trait. Its gets the binary data of the header (which already contains
//!    the binary encoded settings) and must store it at a suitable location.
//! 3. The final [`Create::build()`] call creates the backend instance, which
//!    is used by the container.
//!
//! # Open a container
//!
//! The [`Open`] trait is used to open an existing [`Backend`] instance.
//!
//! The container asks the trait for the binary header data using the
//! [`HeaderGet`] trait. The implementation should load it from a suitable
//! location.
//!
//! The final [`Open::build()`] call creates the backend instance, which is
//! used by the container.

use nuts_bytes::{FromBytes, ToBytes};
use std::error;
use std::fmt::{Debug, Display};
use std::str::FromStr;

// The maximun size of the header.
pub const HEADER_MAX_SIZE: usize = 512;

/// Trait identifies a block in the storage.
pub trait BlockId: Clone + Debug + Display + FromBytes + FromStr + PartialEq + ToBytes {
    /// Returns the number of bytes needed to store the id.
    fn size() -> usize;
}

/// Trait used to receive the header of a container.
///
/// The container uses the [`HeaderGet::get_header_bytes()`] method to ask the
/// backend for the header bytes. The container does not know where the backend
/// stores the header, that's why such a method is used. Not more than
/// [`HEADER_MAX_SIZE`] bytes can be stored in the header.
pub trait HeaderGet<B: Backend> {
    /// Receives the binary header data from the backend.
    ///
    /// The container uses this method to ask the backend for the header bytes.
    /// The container does not know where the backend stores the header, that's
    /// why such a method is used. Not more than [`HEADER_MAX_SIZE`] bytes can
    /// be stored in the header.
    ///
    /// The method should put the data into the `bytes` slice.
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), B::Err>;
}

/// Trait used to update the header of a container.
///
/// The container uses the [`HeaderSet::put_header_bytes()`] method to ask the
/// backend to put the given `bytes` into the header. The container does not
/// know where the backend stores the header, that's why such a trait is used.
/// Not more than [`HEADER_MAX_SIZE`] bytes can be stored in the header.
pub trait HeaderSet<B: Backend> {
    /// Puts the given `bytes` into the header of the backend.
    ///
    /// The container uses this method to ask the backend to put data into the
    /// header. The container does not know where the backend stores the
    /// header, that's why such a method is used. Not more than
    /// [`HEADER_MAX_SIZE`] bytes can be stored in the header.
    fn put_header_bytes(&mut self, bytes: &[u8; HEADER_MAX_SIZE]) -> Result<(), B::Err>;
}

/// Trait to configure the creation of a [`Backend`].
///
/// Should contain options used to create a specific backend. When a container
/// is created, this trait is used to create the related backend.
///
/// The [`Create::settings()`] method returns an instance of the settings of
/// this backend instance. The settings contains runtime configuration used by
/// the backend. The container stores the settings (possibly encrypted) in the
/// header of the container and should contain all settings/information needed
/// by the backend. It is loaded again from the header when the container is
/// opened.
///
/// Any type that implements this `Create` trait must also implement the
/// [`HeaderSet`] trait because the header of the container is created here.
///
/// Finally, the container calls [`Create::build()`] to create an instance of the
/// [`Backend`]. The resulting backend instance should be able to handle all
/// operations on it. The [`Create::build()`] method should validate all its
/// settings before returning the backend instance!
pub trait Create<B: Backend>: HeaderSet<B> {
    /// Returns the settings of this backend instance.
    ///
    /// The settings contains runtime configuration used by the backend. The
    /// container stores the settings (possibly encrypted) in the header of the
    /// container and should contain all settings/information needed by the
    /// backend. It is loaded again from the header when the container is
    /// opened.
    fn settings(&self) -> B::Settings;

    /// Create an instance of the [`Backend`].
    ///
    /// The container calls [`Create::build()`] to create an instance of the
    /// [`Backend`]. The resulting backend instance should be able to handle
    /// all operations on it. The [`Create::build()`] method should validate
    /// all its settings before returning the backend instance!
    fn build(self) -> Result<B, B::Err>;
}

/// Trait used to open a [`Backend`].
///
/// Should contain options used to open a specific backend. When a container
/// is opened, this trait is used to open the related backend.
///
/// Any type that implements this `Open` trait must also implement the
/// [`HeaderGet`] trait because the header of the container is read here.
///
/// Finally, the container calls [`Open::build()`] to create an instance of the
/// [`Backend`]. The settings of the backend (extracted from the header) are
/// passed to the [`Open::build()`] method and can be used to configure the
/// [`Backend`] instance. The resulting backend instance should be able to
/// handle all operations on it. The [`Open::build()`] method should validate
/// all its settings before returning the backend instance!
pub trait Open<B: Backend>: HeaderGet<B> {
    /// Create an instance of the [`Backend`].
    ///
    /// The container calls [`Create::build()`] to create an instance of the
    /// [`Backend`]. The settings of the backend (extracted from the header) are
    /// passed to the method and can be used to configure the [`Backend`]. The
    /// resulting backend instance should be able to handle all operations on
    /// it. The method should validate all its settings before returning the
    /// backend instance!
    fn build(self, settings: B::Settings) -> Result<B, B::Err>;
}

/// Trait that describes a backend of a container.
pub trait Backend: HeaderGet<Self> + HeaderSet<Self>
where
    Self: Sized,
{
    /// Runtime configuration used by the backend.
    ///
    /// It should contain all settings/information needed by the backend. It is
    /// loaded from the header when the backend is opened. See the [`Open`]
    /// trait for more information on how the backend is opened.
    ///
    /// The [`Create`] trait is used to create the settings of a backend.
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

    /// Returns information from the backend.
    ///
    /// It includes information like public settings. The difference to
    /// [`Backend::Settings`] is that [`Backend::Settings`] might include
    /// sensible information which are removed from [`Backend::Info`].
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn info(&self) -> Result<Self::Info, Self::Err>;

    /// Returns the number of bytes needed to store the id.
    fn id_size() -> usize;

    /// Returns the block size of the backend.
    fn block_size(&self) -> u32;

    /// Aquires a new block in the backend.
    ///
    /// Once aquired you should be able to [read](Backend::read) and
    /// [write](Backend::write) from/to it.
    ///
    /// `buf` contains the initial data, which should be copied into the block.
    ///
    /// * A `buf` which is not large enough to fill the whole block must be
    ///   rejected and an error must be returned.
    /// * If `buf` holds more data than the [block-size](Backend::block_size),
    ///   then only the first [block-size](Backend::block_size) bytes are
    ///   copied into the block.
    ///
    /// By default an aquired block, which is not written yet, should return
    /// an all-zero buffer.
    ///
    /// Returns the [id](Backend::Id) if the block.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn aquire(&mut self, buf: &[u8]) -> Result<Self::Id, Self::Err>;

    /// Releases a block again.
    ///
    /// A released block cannot be [read](Backend::read) and
    /// [written](Backend::write), the [id](Self::Id) cannot be used
    /// afterwards.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn release(&mut self, id: Self::Id) -> Result<(), Self::Err>;

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
    fn read(&mut self, id: &Self::Id, buf: &mut [u8]) -> Result<usize, Self::Err>;

    /// Writes a block into the backend.
    ///
    /// Writes up to `buf.len()` bytes from the unencrypted `buf` buffer into
    /// the block with the given `id`.
    ///
    /// * A `buf` which is not large enough to fill the whole block must be
    ///   rejected and an error must be returned.
    /// * If `buf` holds more data than the [block-size](Backend::block_size),
    ///   then only the first [block-size](Backend::block_size) bytes are
    ///   copied into the block.
    ///
    /// The method returns the number of bytes actually written.
    ///
    /// # Errors
    ///
    /// On any error a self-defined [`Backend::Err`] is returned.
    fn write(&mut self, id: &Self::Id, buf: &[u8]) -> Result<usize, Self::Err>;
}
