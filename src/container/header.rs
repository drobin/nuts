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

use std::io::{Cursor, Read, Write};

use crate::backend::Backend;
use crate::bytes::{self, FromBytesExt, ToBytesExt};
use crate::container::cipher::Cipher;

const MAGIC: [u8; 7] = *b"nuts-io";

#[derive(Debug)]
pub struct Header {
    pub(crate) cipher: Cipher,
}

impl Header {
    pub fn new() -> Header {
        Header {
            cipher: Cipher::None,
        }
    }

    pub fn read<B: Backend>(buf: &[u8]) -> bytes::Result<(Header, B::Settings)> {
        let mut cursor = Cursor::new(buf);
        let mut magic = [0; 7];

        cursor.read_exact(&mut magic)?;

        if magic != MAGIC {
            return Err(bytes::Error::invalid("magic mismatch"));
        }

        let cipher = cursor.from_bytes()?;
        let settings = cursor.from_bytes()?;

        Ok((Header { cipher }, settings))
    }

    pub fn write<B: Backend>(
        header: &Header,
        settings: &B::Settings,
        buf: &mut [u8],
    ) -> bytes::Result<()> {
        let mut cursor = Cursor::new(buf);

        cursor.write_all(&MAGIC).unwrap();
        cursor.to_bytes(&header.cipher)?;
        cursor.to_bytes(settings)?;
        cursor.flush()?;

        Ok(())
    }
}
