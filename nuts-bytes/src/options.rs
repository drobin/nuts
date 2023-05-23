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
use crate::source::{BufferSource, TakeBytes};
use crate::target::{BufferTarget, PutBytes, VecTarget};
use crate::writer::Writer;

/// Options to configure (de-) serialization.
#[derive(Debug)]
pub struct Options {
    trailing: bool,
}

impl Options {
    /// Creates a new `Options` instance filled with default values:
    ///
    /// * trailing bytes generates an [error](Options::fail_on_trailing).
    pub fn new() -> Options {
        Options { trailing: true }
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
    /// Binary data are taken from the given `source` which must implement the
    /// [`TakeBytes`] trait.
    ///
    /// Use this reader to manually deserialize data.
    pub fn build_reader<'de, 'tb, T: TakeBytes<'tb>>(self, source: T) -> Reader<T> {
        Reader::new(source)
    }

    /// Creates a new [`Writer`] from this options.
    ///
    /// Binary data are writtem into the given `target` which must implement the
    /// [`PutBytes`] trait.
    pub fn build_writer<T: PutBytes>(self, target: T) -> Writer<T> {
        Writer::new(target)
    }

    /// Deserializes the given `bytes` slice into a data structure.
    ///
    /// # Errors
    ///
    /// If there are still unserialized data left in `bytes` after
    /// deserialization, an [`Error::TrailingBytes`] error is returned, if
    /// [`Options::ignore_trailing`] is not set.
    pub fn from_bytes<'a, T: Deserialize<'a>>(self, bytes: &'a [u8]) -> Result<T> {
        let mut reader = Reader::new(BufferSource::new(bytes));
        let value = T::deserialize(&mut reader)?;

        if !self.trailing || !reader.as_ref().have_remaining_bytes() {
            Ok(value)
        } else {
            Err(Error::TrailingBytes)
        }
    }

    /// Serializes the given `value` into a byte stream.
    pub fn to_vec<T: Serialize>(self, value: &T) -> Result<Vec<u8>> {
        let mut writer = self.build_writer(VecTarget::new(vec![]));

        value.serialize(&mut writer)?;

        Ok(writer.into_target().into_vec())
    }

    /// Serializes the given `value` into the `bytes` slice.
    ///
    /// Returns the number of bytes written into `bytes`.
    ///
    /// # Errors
    ///
    /// When there is not enough space available in `bytes` an
    /// [`Error::NoSpace`] error is returned.
    pub fn to_bytes<T: Serialize>(self, value: &T, bytes: &mut [u8]) -> Result<usize> {
        let mut writer = self.build_writer(BufferTarget::new(bytes));

        value.serialize(&mut writer)
    }
}
