// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use bytes::{Buf, BytesMut};
use log::{debug, trace};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use std::io::{Cursor, Read, Write};
use thiserror::Error;

enum BsonDisplay<'a> {
    DocRef(&'a bson::Document),
    BsonRef(&'a bson::Bson),
}

#[cfg(feature = "debug-condensed")]
impl<'a> BsonDisplay<'a> {
    fn as_document(&self) -> Option<&'a bson::Document> {
        match *self {
            Self::DocRef(doc) => Some(doc),
            Self::BsonRef(bson) => bson.as_document(),
        }
    }

    fn as_array(&self) -> Option<&'a Vec<bson::Bson>> {
        match *self {
            Self::DocRef(_) => None,
            Self::BsonRef(bson) => bson.as_array(),
        }
    }
}

#[cfg(feature = "debug-condensed")]
impl<'a> fmt::Display for BsonDisplay<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(doc) = self.as_document() {
            let mut first = true;

            fmt.write_str("{")?;

            for (key, value) in doc {
                if first {
                    first = false;
                    fmt.write_str(" ")?;
                } else {
                    fmt.write_str(", ")?;
                }

                write!(fmt, "\"{}\": {}", key, Self::BsonRef(value))?;
            }

            write!(fmt, "{}}}", if !first { " " } else { "" })
        } else if let Some(arr) = self.as_array() {
            write!(fmt, "[ <{} bytes> ]", arr.len())
        } else {
            match self {
                Self::DocRef(doc) => fmt::Display::fmt(doc, fmt),
                Self::BsonRef(bson) => fmt::Display::fmt(bson, fmt),
            }
        }
    }
}

#[cfg(not(feature = "debug-condensed"))]
impl<'a> fmt::Display for BsonDisplay<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DocRef(doc) => fmt::Display::fmt(doc, fmt),
            Self::BsonRef(bson) => fmt::Display::fmt(bson, fmt),
        }
    }
}

/// Error type used by the [`BsonReader`] and [`BsonWriter`] utilities.
#[derive(Error, Debug)]
pub enum BsonError {
    /// An error occured while deserializing BSON data.
    #[error(transparent)]
    Deserialize(#[from] bson::de::Error),

    /// An error occured while serializing BSON data.
    #[error(transparent)]
    Serialization(#[from] bson::ser::Error),

    /// An IO-error occured.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// The connection to the peer is closed but there are still buffered data.
    /// This means that the peer closed the connection while sending a frame.
    #[error("connection reset by peer")]
    ConnectionResetByPeer,
}

type BsonResult<T> = Result<T, BsonError>;

/// A utility to read [BSON](bson) encoded data from an arbitrary
/// [source](Read).
pub struct BsonReader<R> {
    source: R,
    buffer: BytesMut,
}

impl<R: Read> BsonReader<R> {
    /// Creates a new `BsonReader` instance, which reads from the given`source`.
    pub fn new(source: R) -> BsonReader<R> {
        BsonReader {
            source,
            buffer: BytesMut::new(),
        }
    }

    /// Reads a [BSON](bson) document from the attached source and deserialize
    /// it into the given type `T`.
    ///
    /// It reads data from the attached source until another BSON document is
    /// available. Next, the document is serialized into `T` and returned into
    /// a wrapped [`Some`] value. Returns [`None`] if the peer closes the
    /// connection.
    pub fn read<T: DeserializeOwned + fmt::Debug>(&mut self) -> BsonResult<Option<T>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            let n = self.read_next_chunk()?;

            if n == 0 {
                if self.buffer.is_empty() {
                    // No buffered data, clean shutdown.
                    return Ok(None);
                } else {
                    // There are still buffered data. This means that the peer
                    // closed the connection while sending a frame.
                    return Err(BsonError::ConnectionResetByPeer);
                }
            }
        }
    }

    fn parse_frame<T: DeserializeOwned + fmt::Debug>(&mut self) -> BsonResult<Option<T>> {
        if self.is_complete() {
            let mut cursor = Cursor::new(&self.buffer[..]);
            let bson = bson::from_reader(&mut cursor)?;

            trace!(
                "read doc {} bytes: {}",
                cursor.position(),
                &BsonDisplay::BsonRef(&bson)
            );

            let value = bson::from_bson(bson)?;

            let pos = cursor.position();
            self.buffer.advance(pos as usize);

            debug!("read deserialized: {:?}", value);

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn is_complete(&self) -> bool {
        if self.buffer.remaining() >= 4 {
            let mut slice = &self.buffer[..];
            let len = slice.get_u32_le() as usize;

            self.buffer.remaining() >= len
        } else {
            false
        }
    }

    fn read_next_chunk(&mut self) -> BsonResult<usize> {
        let mut buf = [0; 1024];
        let n = self.source.read(&mut buf)?;

        self.buffer.extend_from_slice(&buf[..n]);

        Ok(n)
    }
}

/// A utility to write [BSON](bson) encoded data into an arbitrary
/// [target](Write).
pub struct BsonWriter<W> {
    target: W,
}

impl<W: Write> BsonWriter<W> {
    /// Creates a new `BsonWriter` instance which writes into the given `target`.
    pub fn new(target: W) -> BsonWriter<W> {
        BsonWriter { target }
    }

    /// Deserializes the given `value` into a [BSON](bson) document and writes
    /// it into the attached target.
    pub fn write<T: Serialize + fmt::Debug>(&mut self, value: T) -> BsonResult<()> {
        debug!("write serialized: {:?}", value);

        let doc = bson::to_document(&value)?;
        trace!("write doc {}", BsonDisplay::DocRef(&doc));

        doc.to_writer(&mut self.target)?;
        self.target.flush()?;

        Ok(())
    }
}
