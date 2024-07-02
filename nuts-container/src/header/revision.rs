// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

#[cfg(test)]
mod tests;

use crate::buffer::{Buffer, BufferError, BufferMut, FromBuffer, ToBuffer};
use crate::cipher::Cipher;
use crate::header::secret::Secret;
use crate::header::HeaderError;
use crate::kdf::Kdf;

const MAGIC: [u8; 7] = *b"nuts-io";

#[derive(Debug)]
pub struct Data {
    pub cipher: Cipher,
    pub iv: Vec<u8>,
    pub kdf: Kdf,
    pub secret: Secret,
}

#[derive(Debug)]
pub enum Revision {
    Rev0(Data),
}

impl Revision {
    pub fn rev0(cipher: Cipher, iv: Vec<u8>, kdf: Kdf, secret: Secret) -> Revision {
        Revision::Rev0(Data {
            cipher,
            iv,
            kdf,
            secret,
        })
    }

    pub fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Revision, HeaderError> {
        let magic = buf.get_array()?;

        if magic != MAGIC {
            return Err(HeaderError::InvalidHeader);
        }

        let b = buf.get_u32()?;

        if b != 0 {
            return Err(BufferError::InvalidIndex("Revision".to_string(), b).into());
        }

        let data = Data {
            cipher: Cipher::get_from_buffer(buf)?,
            iv: buf.get_vec::<8>()?,
            kdf: Kdf::get_from_buffer(buf)?,
            secret: Secret::from_buffer(buf)?,
        };

        Ok(Self::Rev0(data))
    }

    pub fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        buf.put_chunk(&MAGIC)?;

        let Revision::Rev0(data) = self;

        buf.put_u32(0)?;
        Cipher::put_into_buffer(&data.cipher, buf)?;
        buf.put_vec::<8>(&data.iv)?;
        Kdf::put_into_buffer(&data.kdf, buf)?;
        Secret::to_buffer(&data.secret, buf)?;

        Ok(())
    }
}
