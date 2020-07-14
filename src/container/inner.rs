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

use crate::header::Header;
use crate::io::IO;
use crate::result::Result;
use crate::secret::Secret;
use crate::types::{DiskType, Options, BLOCK_MIN_SIZE};

pub struct Inner {
    pub header: Header,
    pub secret: Secret,
    pub io: IO,
}

impl Inner {
    pub fn create(path: &str, options: &Options) -> Result<Inner> {
        let secret = Secret::create(options)?;
        let header = Inner::create_header(&secret, options)?;

        debug!("secret: {:?}", secret);
        debug!("header: {:?}", header);

        let mut fd = File::create(path)?;
        let mut io = IO::new(options.bsize(), options.blocks(), options.dtype, &mut fd)?;

        Inner::dump_header(&header, &mut io, &mut fd)?;

        let inner = Inner { header, secret, io };

        debug!(
            "allocating container, dtype = {}, bsize = {}, blocks = {}",
            inner.io.dtype, inner.io.bsize, inner.io.blocks
        );

        Ok(inner)
    }

    pub fn open(path: &str) -> Result<Inner> {
        let mut fd = File::open(path)?;
        let (header, secret) = Inner::open_header(&mut fd)?;
        let io = IO::new(secret.bsize, secret.blocks, secret.dtype, &mut fd)?;

        debug!("secret: {:?}", secret);
        debug!("header: {:?}", header);

        Ok(Inner { header, secret, io })
    }

    fn create_header(secret: &Secret, options: &Options) -> Result<Header> {
        let mut header = Header::create(options)?;

        header.write_secret(secret, &[9; 16])?;

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

    fn open_header(fd: &mut File) -> Result<(Header, Secret)> {
        // Create a temp. block with bsize = BLOCK_MIN_SIZE.
        // This is enough to read the header.
        let io = IO::new(BLOCK_MIN_SIZE, 1, DiskType::ThinZero, fd)?;

        // Read the binary header into `buf`.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        io.read(fd, &mut buf, 0)?;

        let header = Header::read(&buf).map(|(header, _)| header)?;
        let secret = header.read_secret(&[9; 16]).map(|(secret, _)| secret)?;

        header.validate()?;
        secret.validate(header.cipher, header.digest)?;

        Ok((header, secret))
    }
}
