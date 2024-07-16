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

impl Data {
    fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Data, HeaderError> {
        let cipher = Cipher::get_from_buffer(buf)?;
        let iv = buf.get_vec::<8>()?;
        let kdf = Kdf::get_from_buffer(buf)?;
        let secret = Secret::from_buffer(buf)?;

        Ok(Data {
            cipher,
            iv,
            kdf,
            secret,
        })
    }

    fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        Cipher::put_into_buffer(&self.cipher, buf)?;
        buf.put_vec::<8>(&self.iv)?;
        Kdf::put_into_buffer(&self.kdf, buf)?;
        Secret::to_buffer(&self.secret, buf)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Revision {
    Rev0(Data),
    Rev1(Data),
}

impl Revision {
    pub fn latest(cipher: Cipher, iv: Vec<u8>, kdf: Kdf, secret: Secret) -> Revision {
        Revision::Rev1(Data {
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

        match b {
            0 => Data::get_from_buffer(buf).map(Revision::Rev0),
            1 => Data::get_from_buffer(buf).map(Revision::Rev1),
            _ => Err(BufferError::InvalidIndex("Revision".to_string(), b).into()),
        }
    }

    pub fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        buf.put_chunk(&MAGIC)?;

        match self {
            Revision::Rev0(data) => {
                buf.put_u32(0)?;
                data.put_into_buffer(buf)
            }
            Revision::Rev1(data) => {
                buf.put_u32(1)?;
                data.put_into_buffer(buf)
            }
        }
    }
}
