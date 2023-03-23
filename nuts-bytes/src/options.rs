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

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::reader::Reader;
use crate::writer::Writer;

#[derive(Debug)]
pub(crate) enum Int {
    Fix,
    Var,
}

/// Options to configure (de-) serialization.
#[derive(Debug)]
pub struct Options {
    int: Int,
    trailing: bool,
}

impl Options {
    /// Creates a new `Options` instance filled with default values:
    ///
    /// * integer encoding is set to [_varint_](Options::with_varint) encoding.
    /// * trailing bytes generates an [error](Options::fail_on_trailing).
    pub fn new() -> Options {
        Options {
            int: Int::Var,
            trailing: true,
        }
    }

    /// Sets the length encoding to be fixed.
    pub fn with_fixint(mut self) -> Self {
        self.int = Int::Fix;
        self
    }

    /// Use a variable length integer encoding.
    ///
    /// Algorithm is taken from the [_bincode_ project](https://docs.rs/bincode/latest/bincode/config/struct.VarintEncoding.html).
    /// Thank you guys for the input!
    ///
    /// > Encoding an unsigned integer v (of any type excepting u8) works as follows:
    /// >
    /// > 1. If `u < 251`, encode it as a single byte with that value.
    /// > 2. If `251 <= u < 2**16`, encode it as a literal byte 251, followed by a u16 with value `u `.
    /// > 3. If `2**16 <= u < 2**32`, encode it as a literal byte 252, followed by a u32 with value `u`.
    /// > 4. If `2**32 <= u < 2**64`, encode it as a literal byte 253, followed by a u64 with value `u`.
    /// > 5. If `2**64 <= u < 2**128`, encode it as a literal byte 254, followed by a u128 with value `u`.
    pub fn with_varint(mut self) -> Self {
        self.int = Int::Var;
        self
    }

    /// If enabled an error is generated if trailing (unread) bytes are available.
    pub fn fail_on_trailing(mut self) -> Self {
        self.trailing = true;
        self
    }

    /// If enabled ignore trailing (unread) bytes.
    pub fn ignore_trailing(mut self) -> Self {
        self.trailing = false;
        self
    }

    /// Creates a new [`Reader`] from this options.
    ///
    /// Use this reader to manually deserialize data.
    pub fn build_reader<'de>(self, bytes: &'de [u8]) -> Reader<'de> {
        Reader::new(self.int, bytes)
    }

    /// Creates a new [`Writer`] that writes into the given `vec`.
    pub fn build_vec_writer<'a>(self, vec: Vec<u8>) -> Writer<'a> {
        Writer::for_vec(self.int, vec)
    }

    /// Creates a new [`Writer`] that writes into the given `bytes` slice.
    pub fn build_slice_writer<'a>(self, bytes: &'a mut [u8]) -> Writer<'a> {
        Writer::for_slice(self.int, bytes)
    }

    /// Deserializes the given `bytes` slice into a data structure.
    ///
    /// # Errors
    ///
    /// If there are still unserialized data left in `bytes` after
    /// deserialization, an [`Error::TrailingBytes`] error is returned, if
    /// [`Options::ignore_trailing`] is not set.
    pub fn from_bytes<'a, T: Deserialize<'a>>(self, bytes: &'a [u8]) -> Result<T> {
        let mut reader = Reader::new(self.int, bytes);
        let value = T::deserialize(&mut reader)?;

        if !self.trailing || reader.remaining_bytes().is_empty() {
            Ok(value)
        } else {
            Err(Error::TrailingBytes)
        }
    }

    /// Serializes the given `value` into a byte stream.
    pub fn to_vec<T: Serialize>(self, value: &T) -> Result<Vec<u8>> {
        let mut writer = Writer::for_vec(self.int, vec![]);

        value.serialize(&mut writer)?;

        Ok(writer.into_vec())
    }

    /// Serializes the given `value` into the `bytes` slice.
    ///
    /// # Errors
    ///
    /// When there is not enough space available in `bytes` an
    /// [`Error::NoSpace`] error is returned.
    pub fn to_bytes<T: Serialize>(self, value: &T, bytes: &mut [u8]) -> Result<()> {
        let mut writer = Writer::for_slice(self.int, bytes);

        value.serialize(&mut writer)
    }
}
