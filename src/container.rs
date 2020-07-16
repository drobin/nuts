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

use log::debug;
use std::fs::File;
use std::ops;

use crate::error::{Error, InvalHeaderKind};
use crate::header::Header;
use crate::io::IO;
use crate::result::Result;
use crate::secret::Secret;
use crate::types::{Cipher, Digest, DiskType, Options, BLOCK_MIN_SIZE};

struct Inner {
    pub header: Header,
    pub secret: Secret,
    pub io: IO,
}

pub struct Container {
    password: Option<Vec<u8>>,
    inner: Option<Inner>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            password: None,
            inner: None,
        }
    }

    pub fn set_password(&mut self, password: &[u8]) {
        self.password = Some(password.to_vec());
    }

    pub fn create(&mut self, path: &str, options: &Options) -> Result<()> {
        if self.inner.is_none() {
            let password = self.password.as_ref().map(|p| p.as_slice());
            let secret = Secret::create(options)?;
            let header = Container::create_header(&secret, password, options)?;

            debug!("secret: {:?}", secret);
            debug!("header: {:?}", header);

            let mut fd = File::create(path)?;
            let mut io = IO::new(options.bsize(), options.blocks(), options.dtype, &mut fd)?;

            Container::dump_header(&header, &mut io, &mut fd)?;

            let inner = Inner { header, secret, io };

            debug!(
                "allocating container, dtype = {}, bsize = {}, blocks = {}",
                inner.io.dtype, inner.io.bsize, inner.io.blocks
            );

            self.inner = Some(inner);

            Ok(())
        } else {
            Err(Error::Opened)
        }
    }

    pub fn open(&mut self, path: &str) -> Result<()> {
        if self.inner.is_none() {
            let password = self.password.as_ref().map(|p| p.as_slice());
            let mut fd = File::open(path)?;
            let (header, secret) = Container::open_header(&mut fd, password)?;
            let io = IO::new(secret.bsize, secret.blocks, secret.dtype, &mut fd)?;

            debug!("secret: {:?}", secret);
            debug!("header: {:?}", header);

            self.inner = Some(Inner { header, secret, io });

            Ok(())
        } else {
            Err(Error::Opened)
        }
    }

    pub fn cipher(&self) -> Result<Cipher> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.header.cipher))
    }

    pub fn digest(&self) -> Result<Option<Digest>> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.header.digest))
    }

    pub fn dtype(&self) -> Result<DiskType> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.secret.dtype))
    }

    pub fn bsize(&self) -> Result<u32> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.secret.bsize))
    }

    pub fn blocks(&self) -> Result<u64> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.secret.blocks))
    }

    pub fn ablocks(&self) -> Result<u64> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.io.ablocks))
    }

    fn create_header(
        secret: &Secret,
        password: Option<&[u8]>,
        options: &Options,
    ) -> Result<Header> {
        let mut header = Header::create(options)?;
        let wrapping_key = Container::calculate_wrapping_key(&header, password)?;

        header.write_secret(secret, &wrapping_key)?;
        secret.validate(header.cipher, header.digest)?;
        header.validate()?;

        Ok(header)
    }

    fn dump_header(header: &Header, io: &mut IO, fd: &mut File) -> Result<u32> {
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        let offset = header.write(&mut buf)?;
        let end = offset as usize;

        io.write(&buf[..end], fd, 0)
    }

    fn open_header(fd: &mut File, password: Option<&[u8]>) -> Result<(Header, Secret)> {
        // Create a temp. block with bsize = BLOCK_MIN_SIZE.
        // This is enough to read the header.
        let io = IO::new(BLOCK_MIN_SIZE, 1, DiskType::ThinZero, fd)?;

        // Read the binary header into `buf`.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        io.read(fd, &mut buf, 0)?;

        let header = Header::read(&buf).map(|(header, _)| header)?;
        let wrapping_key = Container::calculate_wrapping_key(&header, password)?;

        let secret = header
            .read_secret(&wrapping_key)
            .map(|(secret, _)| secret)?;

        header.validate()?;
        secret.validate(header.cipher, header.digest)?;

        Ok((header, secret))
    }

    fn calculate_wrapping_key(header: &Header, password: Option<&[u8]>) -> Result<Vec<u8>> {
        let wrapping_key = if let Some(wkey) = header.wrapping_key.as_ref() {
            let digest = header
                .digest
                .ok_or(Error::InvalHeader(InvalHeaderKind::InvalDigest))?;
            let password = password.ok_or(Error::NoPassword)?;
            wkey.key(password, digest)?
        } else {
            vec![]
        };

        debug!("wrapping_key calculated, {} bytes", wrapping_key.len());

        Ok(wrapping_key)
    }
}

impl ops::Drop for Container {
    fn drop(&mut self) {
        if let Some(vec) = self.password.as_mut() {
            for e in vec.iter_mut() {
                *e = 0;
            }
        }
    }
}
