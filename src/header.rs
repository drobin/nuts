// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use log::{error, warn};
use std::fmt;

use crate::binary;
use crate::error::{Error, InvalHeaderKind};
use crate::openssl;
use crate::result::Result;
use crate::types::{Cipher, Digest, Options, WrappingKey};
use crate::wkey::WrappingKeyData;

const MAGIC: [u8; 7] = [b'n', b'u', b't', b's', b'-', b'i', b'o'];

pub struct Header {
    pub revision: u8,
    pub cipher: Cipher,
    pub digest: Option<Digest>,
    pub wrapping_key: Option<WrappingKeyData>,
    pub hmac: Vec<u8>,
    pub secret: Vec<u8>,
}

impl Header {
    pub fn create(options: &Options) -> Result<Header> {
        let wkey_data = match options.wkey {
            Some(WrappingKey::Pbkdf2 {
                iterations,
                salt_len,
            }) => {
                let mut salt = vec![0; salt_len as usize];
                openssl::random(&mut salt)?;

                Some(WrappingKeyData::pbkdf2(iterations, &salt))
            }
            None => None,
        };

        Ok(Header {
            revision: 1,
            cipher: options.cipher,
            digest: options.md,
            wrapping_key: wkey_data,
            hmac: Vec::new(),
            secret: Vec::new(),
        })
    }

    pub fn read(source: &[u8]) -> Result<(Header, u32)> {
        let mut offset = 0;

        binary::read_array_as(source, &mut offset, 7, validate_magic)?;
        let revision = binary::read_u8_as(source, &mut offset, u8_to_revision)?;
        let cipher = binary::read_u8_as(source, &mut offset, u8_to_cipher)?;
        let digest = binary::read_u8_as(source, &mut offset, u8_to_digest)?;
        let wrapping_key = read_wrapping_key(source, &mut offset)?;
        let hmac = binary::read_vec(source, &mut offset)?;
        let secret = binary::read_vec(source, &mut offset)?;

        let header = Header {
            revision,
            cipher,
            digest,
            wrapping_key,
            hmac,
            secret,
        };

        Ok((header, offset))
    }

    pub fn write(&self, target: &mut [u8]) -> Result<u32> {
        let mut offset: u32 = 0;

        binary::write_array(target, &mut offset, &MAGIC)?;
        binary::write_u8_as(target, &mut offset, self.revision, revision_to_u8)?;
        binary::write_u8_as(target, &mut offset, self.cipher, cipher_to_u8)?;
        binary::write_u8_as(target, &mut offset, self.digest, digest_to_u8)?;
        write_wrapping_key(target, &mut offset, &self.wrapping_key)?;
        binary::write_vec(target, &mut offset, &self.hmac)?;
        binary::write_vec(target, &mut offset, &self.secret)?;

        Ok(offset)
    }

    pub fn validate(&self) -> Result<()> {
        Header::validate_revision(self.revision)?;
        self.validate_digest()?;
        self.validate_hmac()?;

        Ok(())
    }

    fn validate_revision(revision: u8) -> Result<()> {
        if revision == 1 {
            Ok(())
        } else {
            error!("invalid revision: {}", revision);
            Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
        }
    }

    fn validate_digest(&self) -> Result<()> {
        if self.cipher == Cipher::None && self.digest.is_some() {
            error!(
                "invalid digest {} for cipher {}",
                self.digest.unwrap(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalDigest))
        } else if self.cipher != Cipher::None && self.digest.is_none() {
            error!("invalid digest None for cipher {}", self.cipher);
            Err(Error::InvalHeader(InvalHeaderKind::InvalDigest))
        } else {
            Ok(())
        }
    }

    fn validate_hmac(&self) -> Result<()> {
        let size = match self.digest {
            Some(md) => md.size() as usize,
            None => 0,
        };

        if self.hmac.len() < size {
            error!(
                "invalid hmac, len: {}, expected: {} ({})",
                self.hmac.len(),
                size,
                digest_to_string(self.digest)
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalHmac))
        } else {
            if self.hmac.len() != size {
                warn!(
                    "lost hmac, len: {}, min: {} ({})",
                    self.hmac.len(),
                    size,
                    digest_to_string(self.digest)
                );
            }

            Ok(())
        }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let hmac = format!("<{} bytes>", self.hmac.len());
        let secret = format!("<{} bytes>", self.secret.len());

        fmt.debug_struct("Header")
            .field("revision", &self.revision)
            .field("cipher", &self.cipher)
            .field("digest", &self.digest)
            .field("wrapping_key", &self.wrapping_key)
            .field("hmac", &hmac)
            .field("secret", &secret)
            .finish()
    }
}

fn validate_magic(slice: &[u8]) -> Result<()> {
    if slice == MAGIC {
        Ok(())
    } else {
        error!("invalid magic: {:x?}", slice);
        Err(Error::InvalHeader(InvalHeaderKind::InvalMagic))
    }
}

fn u8_to_revision(revision: u8) -> Result<u8> {
    if revision == 1 {
        Ok(revision)
    } else {
        Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
    }
}

fn revision_to_u8(revision: u8) -> Result<u8> {
    if revision == 1 {
        Ok(revision)
    } else {
        Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
    }
}

fn u8_to_cipher(i: u8) -> Result<Cipher> {
    match i {
        0 => Ok(Cipher::None),
        1 => Ok(Cipher::Aes128Ctr),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalCipher)),
    }
}

fn cipher_to_u8(cipher: Cipher) -> Result<u8> {
    match cipher {
        Cipher::None => Ok(0),
        Cipher::Aes128Ctr => Ok(1),
    }
}

fn u8_to_digest(i: u8) -> Result<Option<Digest>> {
    match i {
        1 => Ok(Some(Digest::Sha1)),
        0xFF => Ok(None),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDigest)),
    }
}

fn digest_to_u8(digest: Option<Digest>) -> Result<u8> {
    match digest {
        Some(Digest::Sha1) => Ok(1),
        None => Ok(0xFF),
    }
}

fn read_wrapping_key(data: &[u8], offset: &mut u32) -> Result<Option<WrappingKeyData>> {
    let algorithm = binary::read_u8(data, offset)?;

    match algorithm {
        1 => {
            let iterations = binary::read_u32(data, offset)?;
            let salt = binary::read_vec(data, offset)?;

            Ok(Some(WrappingKeyData::pbkdf2(iterations, &salt)))
        }
        0xFF => Ok(None),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalWrappingKey)),
    }
}

fn write_wrapping_key(
    target: &mut [u8],
    offset: &mut u32,
    data: &Option<WrappingKeyData>,
) -> Result<()> {
    match data {
        Some(data) => {
            let WrappingKey::Pbkdf2 {
                iterations,
                salt_len,
            } = data.wkey;

            let salt = data.pbkdf2.as_ref().unwrap();
            assert_eq!(salt_len, salt.len() as u32);

            binary::write_u8(target, offset, 1)?;
            binary::write_u32(target, offset, iterations)?;
            binary::write_vec(target, offset, &salt)?;

            Ok(())
        }
        None => {
            binary::write_u8(target, offset, 0xFF)?;

            Ok(())
        }
    }
}

fn digest_to_string(digest: Option<Digest>) -> String {
    match digest {
        Some(md) => format!("{}", md),
        None => String::from("None"),
    }
}
