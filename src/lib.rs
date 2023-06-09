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

//! A binary data format for [Serde].
//!
//! The `nuts-bytes` crate implements a [Serde] data format that converts into
//! a binary format. See the [Format specification] section for a detailed
//! description of the format.
//!
//! # Deserialization from a binary representation
//!
//! The deserialization describes the process of converting binary data into a
//! data type that implements the [`Deserialize`] trait.
//!
//! The [`Reader`] utility performs this deserialization by
//!
//! 1. taking binary data from a source type that implements the [`TakeBytes`]
//!    trait and finally
//! 2. performs the deserialization.
//!
//! The crate implements [`TakeBytes`] already for
//! [`&[u8]`](trait.TakeBytes.html#impl-TakeBytes%3C%27tb%3E-for-%26%27tb%20%5Bu8%5D).
//! It takes bytes from a [slice] of `u8` values.
//!
//! ## Deserialization example
//!
//! ```rust
//! use nuts_bytes::Reader;
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct SampleStruct {
//!     f1: u8,
//!     f2: u16,
//! };
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! enum SampleEnum {
//!     V1(u32)
//! }
//!
//! // deserialize a primitive (u32)
//! let mut reader = Reader::new([0x00, 0x00, 0x02, 0x9A].as_slice());
//!
//! let n: u32 = reader.deserialize().unwrap();
//! assert_eq!(n, 666);
//!
//! // deserialize a struct
//! let mut reader = Reader::new([0x07, 0x02, 0x9A, 0x00].as_slice());
//!
//! let sample: SampleStruct = reader.deserialize().unwrap();
//! assert_eq!(sample, SampleStruct{ f1: 7, f2: 666 });
//! assert_eq!(*reader.as_ref(), [0x00]); // Still one byte left
//!
//! // deserialize an enum
//! let mut reader = Reader::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x9A].as_slice());
//!
//! let sample: SampleEnum = reader.deserialize().unwrap();
//! assert_eq!(sample, SampleEnum::V1(666));
//!
//! // Not enough data available
//! let mut reader = Reader::new([0; 3].as_slice());
//! let err = reader.deserialize::<u32>().unwrap_err();
//!
//! assert_eq!(format!("{}", err), "No more bytes are available for reading.");
//! ```
//!
//! # Serialization into a binary representation
//!
//! The serialization describes the process of converting a data type that
//! implements the [`Serialize`] trait into its binary representation.
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
//!   slice, an [`Error::NoSpace`] error is raised.
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
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct SampleStruct {
//!     f1: u8,
//!     f2: u16,
//! };
//!
//! #[derive(Serialize)]
//! enum SampleEnum {
//!     V1(u32)
//! }
//!
//! // serialize a primitive (u32)
//! let mut writer = Writer::new(vec![]);
//! let n = writer.serialize(&666u32).unwrap();
//!
//! assert_eq!(n, 4); // 4 bytes written
//! assert_eq!(writer.into_target(), [0x00, 0x00, 0x02, 0x9A]);
//!
//! // serialize a struct
//! let sample = SampleStruct{ f1: 7, f2: 666 };
//! let mut writer = Writer::new(vec![]);
//! let n = writer.serialize(&sample).unwrap();
//!
//! assert_eq!(n, 3); // 3 bytes written
//! assert_eq!(writer.into_target(), [0x07, 0x02, 0x9A]);
//!
//! // serialize an enum
//! let sample = SampleEnum::V1(666);
//! let mut writer = Writer::new(vec![]);
//! let n = writer.serialize(&sample).unwrap();
//!
//! assert_eq!(n, 8); // 8 bytes written
//! assert_eq!(writer.into_target(), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x9A]);
//! ```
//!
//! ### Serialize into a slice
//!
//! ```rust
//! use nuts_bytes::Writer;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct SampleStruct {
//!     f1: u8,
//!     f2: u16,
//! };
//!
//! #[derive(Serialize)]
//! enum SampleEnum {
//!     V1(u32)
//! }
//!
//! // serialize a primitive (u32)
//! let mut buf = [0; 4];
//!
//! let mut writer = Writer::new(buf.as_mut_slice());
//! let n = writer.serialize(&666u32).unwrap();
//!
//! assert_eq!(n, 4); // 4 bytes written
//! assert_eq!(buf, [0x00, 0x00, 0x02, 0x9A]);
//!
//! // serialize a struct
//! let sample = SampleStruct{ f1: 7, f2: 666 };
//! let mut buf = [0; 4];
//!
//! let mut writer = Writer::new(buf.as_mut_slice());
//! let n = writer.serialize(&sample).unwrap();
//!
//! assert_eq!(n, 3); // 3 bytes written
//! assert_eq!(buf, [0x07, 0x02, 0x9A, 0x00]);
//!
//! // serialize an enum
//! let sample = SampleEnum::V1(666);
//! let mut buf = [0; 8];
//!
//! let mut writer = Writer::new(buf.as_mut_slice());
//! let n = writer.serialize(&sample).unwrap();
//!
//! assert_eq!(n, 8); // 8 bytes written
//! assert_eq!(buf, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x9A]);
//!
//! // Not enough space for serialization
//! let mut buf = [0; 3];
//!
//! let mut writer = Writer::new(buf.as_mut_slice());
//! let err = writer.serialize(&666u32).unwrap_err();
//!
//! assert_eq!(format!("{}", err), "no more space available for writing");
//! ```
//!
//! # Format specification
//!
//! The binary format is described [here](doc_format) in detail.
//!
//! [Serde]: https://www.serde.rs
//! [Format specification]: #format-specification

mod error;
mod reader;
mod source;
mod target;
mod writer;

#[cfg(doc)]
pub mod doc_format {
    //! Documentation: format specification
    //!
    #![doc = include_str!("../docs/format.md")]
}

#[cfg(doc)]
use serde::{Deserialize, Serialize};

pub use error::{Error, Result};
pub use reader::Reader;
pub use source::TakeBytes;
pub use target::PutBytes;
pub use writer::Writer;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(test)]
macro_rules! assert_error {
    ($err:expr, $type:ident :: $memb:ident) => {
        match $err {
            $type::$memb => {}
            _ => panic!("invalid error"),
        }
    };

    ($err:expr, $type:ident :: $memb:ident ( $(| $arg:ident | $assert:expr),+ ) ) => {
        match $err {
            $type::$memb($($arg),*) => {
                $(
                    assert!($assert);
                )*
            }
            _ => panic!("invalid error"),
        }
    };

    ($err:expr, $type:ident :: $memb:ident { $(| $arg:ident | $assert:expr),+ } ) => {
        match $err {
            $type::$memb{$($arg),*} => {
                $(
                    assert!($assert);
                )*
            }
            _ => panic!("invalid error"),
        }
    };
}

#[cfg(test)]
macro_rules! assert_error_eq {
    ($err:expr, $type:ident :: $memb:ident ( $(| $arg:ident | $val:expr),+ ) ) => {
        match $err {
            $type::$memb($($arg),*) => {
                $(
                    assert_eq!($arg, $val);
                )*
            }
            _ => panic!("invalid error"),
        }
    };

    ($err:expr, $type:ident :: $memb:ident { $(| $arg:ident | $val:expr),+ } ) => {
        match $err {
            $type::$memb{$($arg),*} => {
                $(
                    assert_eq!($arg, $val);
                )*
            }
            _ => panic!("invalid error"),
        }
    };
}

#[cfg(test)]
pub(crate) use {assert_error, assert_error_eq};
