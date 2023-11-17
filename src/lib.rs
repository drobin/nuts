// MIT License
//
// Copyright (c) 2023 Robin Doer
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

//! Conversion into a binary data format.
//!
//! The `nuts-bytes` crate implements a tool that converts structured data into
//! a binary format. See the [Format specification] section for a detailed
//! description of the format.
//!
//! # Deserialization from a binary representation
//!
//! The deserialization describes the process of converting binary data into a
//! data type that implements the [`FromBytes`] trait.
//!
//! The [`Reader`] utility performs this deserialization by
//!
//! 1. taking binary data from a source type that implements the [`TakeBytes`]
//!    trait and finally
//! 2. performs the [deserialization](Reader::read).
//!
//! The crate implements [`TakeBytes`] already for
//! [`&[u8]`](trait.TakeBytes.html#impl-TakeBytes%3C%27tb%3E-for-%26%27tb%20%5Bu8%5D).
//! It takes bytes from a [slice] of `u8` values.
//!
//! ## Deserialization example
//!
//! ```rust
//! use nuts_bytes::{Reader, ReaderError};
//!
//! // deserialize a primitive (u32)
//! let mut reader = Reader::<&[u8]>::new([0x00, 0x00, 0x02, 0x9A].as_slice());
//! let n: u32 = reader.read().unwrap();
//!
//! assert_eq!(n, 666);
//!
//! // Not enough data available
//! let mut reader = Reader::<&[u8]>::new([0; 3].as_slice());
//! let err = reader.read::<u32>().unwrap_err();
//!
//! assert!(matches!(err, ReaderError::Eof));
//! ```
//!
//! # Serialization into a binary representation
//!
//! The serialization describes the process of converting a data type that
//! implements the [`ToBytes`] trait into its binary representation.
//!
//! The [`Writer`] utility performs this serialization. It
//!
//! 1. performs the serialization and finally
//! 2. pushes the binary data into a target type, that implements the
//!    [`PutBytes`] trait.
//!
//! The crate implements [`PutBytes`] for the following types:
//!
//! * [`&mut [u8]`](trait.PutBytes.html#impl-PutBytes-for-%26mut%20%5Bu8%5D)
//!   Serialize into a [slice] of `u8` values. Not more than [`slice::len()`]
//!   bytes can be written. If the number of bytes exceeds the size of the
//!   slice, an [`PutBytesError::no_space()`] error is raised.
//! * [`Vec<u8>`](trait.PutBytes.html#impl-PutBytes-for-Vec<u8>)
//!   Serialize into a [`Vec`] of `u8` values. The binary data are appended to
//!   the [`Vec`].
//!
//! ## Serialization examples
//!
//! ### Serialize into a vec
//!
//! ```rust
//! use nuts_bytes::Writer;
//!
//! // serialize a primitive (u32)
//! let mut writer = Writer::<Vec<u8>>::new(vec![]);
//! writer.write(&666u32).unwrap();
//!
//! assert_eq!(writer.into_target(), [0x00, 0x00, 0x02, 0x9A]);
//! ```
//!
//! ### Serialize into a slice
//!
//! ```rust
//! use nuts_bytes::Writer;
//!
//! // serialize a primitive (u32)
//! let mut buf = [0; 4];
//! let mut writer = Writer::<&mut [u8]>::new(buf.as_mut_slice());
//! writer.write(&666u32).unwrap();
//!
//! assert_eq!(buf, [0x00, 0x00, 0x02, 0x9A]);
//!
//! // Not enough space for serialization
//! let mut buf = [0; 3];
//! let mut writer = Writer::<&mut [u8]>::new(buf.as_mut_slice());
//! let err = writer.write(&666u32).unwrap_err();
//!
//! assert_eq!(format!("{}", err), "no more space available for writing");
//! ```
//!
//! # Format specification
//!
//! The binary format is described [here](doc_format) in detail.
//!
//! [Format specification]: #format-specification

#[cfg(feature = "derive")]
mod derive;
mod from_bytes;
mod put_bytes;
mod reader;
mod take_bytes;
mod to_bytes;
mod writer;

#[cfg(doc)]
pub mod doc_format {
    //! Documentation: format specification
    //!
    #![doc = include_str!("../docs/format.md")]
}

#[cfg(all(doc, feature = "derive"))]
pub mod doc_derive {
    //! Derive macros available if nuts-bytes is built with
    //! `features = ["derive"]`.
    //!
    #![doc = include_str!("../docs/derive.md")]
}

#[cfg(feature = "derive")]
pub use derive::TakeDeriveError;
pub use from_bytes::{FromBytes, TakeCharError, TakeStringError};
#[cfg(feature = "derive")]
pub use nuts_bytes_derive::{FromBytes, ToBytes};
pub use put_bytes::{PutBytes, PutBytesError};
pub use reader::{Reader, ReaderError};
pub use take_bytes::{TakeBytes, TakeBytesError};
pub use to_bytes::ToBytes;
pub use writer::Writer;

// #[doc = include_str!("../README.md")]
// #[cfg(doctest)]
// pub struct ReadmeDoctests;
