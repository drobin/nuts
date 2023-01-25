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

use std::io::{Cursor, Read, Write};

use nuts_bytes::{FromBytes, FromBytesExt, ToBytes, ToBytesExt};

#[derive(Clone)]
pub struct Settings(Vec<u8>);

impl Settings {
    pub fn to_bytes<B: ToBytes>(b: &B) -> nuts_bytes::Result<Settings> {
        let mut cursor = Cursor::new(vec![]);

        cursor.to_bytes(b)?;
        cursor.flush()?;

        Ok(Settings(cursor.into_inner()))
    }

    pub fn from_bytes<B: FromBytes>(self) -> nuts_bytes::Result<B> {
        Cursor::new(&self.0).from_bytes()
    }
}

impl FromBytes for Settings {
    fn from_bytes<R: Read>(source: &mut R) -> nuts_bytes::Result<Self> {
        let mut vec = vec![];
        source.read_to_end(&mut vec).unwrap();

        Ok(Settings(vec))
    }
}

impl ToBytes for Settings {
    fn to_bytes<W: Write>(&self, target: &mut W) -> nuts_bytes::Result<()> {
        target.write_bytes(&self.0)
    }
}
