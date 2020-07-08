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
use crate::types::{Cipher, Digest, DiskType, Options, BLOCK_MIN_SIZE};

pub struct Container {
    cipher: Cipher,
    digest: Option<Digest>,
    io: IO,
    fd: File,
}

impl Container {
    pub fn create(path: &str, options: &Options) -> Result<Container> {
        let mut header = Container::create_header(options)?;
        let secret = Secret::create(options)?;

        debug!("header: {:?}", header);
        debug!("secret: {:?}", secret);

        header.validate()?;
        secret.validate(header.cipher, header.digest)?;

        let mut fd = File::create(path)?;
        let mut io = IO::new(options.bsize(), options.blocks(), options.dtype, &mut fd)?;

        io.ensure_capacity(&mut fd, 1)?;
        Container::dump_secret(&secret, &mut header)?;
        Container::dump_header(&header, &mut io, &mut fd)?;

        let container = Container {
            cipher: options.cipher,
            digest: options.md,
            io,
            fd,
        };

        debug!(
            "allocating container, dtype = {}, bsize = {}, blocks = {}",
            container.io.dtype, container.io.bsize, container.io.blocks
        );

        Ok(container)
    }

    pub fn open(path: &str) -> Result<Container> {
        let mut fd = File::open(path)?;
        let header = Container::open_header(&mut fd)?;
        let secret = Container::open_secret(&header)?;

        debug!("header: {:?}", header);
        debug!("secret: {:?}", secret);

        header.validate()?;
        secret.validate(header.cipher, header.digest)?;

        let io = IO::new(secret.bsize, secret.blocks, secret.dtype, &mut fd)?;

        Ok(Container {
            cipher: header.cipher,
            digest: header.digest,
            io,
            fd,
        })
    }

    fn create_header(options: &Options) -> Result<Header> {
        let header = Header::create(options);

        // TODO generate keys if applicable

        header
    }

    fn dump_header(header: &Header, io: &mut IO, fd: &mut File) -> Result<u32> {
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        let offset = header.write(&mut buf)?;
        let end = offset as usize;

        io.write(&buf[..end], fd, 0)
    }

    fn dump_secret(secret: &Secret, header: &mut Header) -> Result<u32> {
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        let result = secret.write(&mut buf);

        if let Ok(offset) = result {
            let end = offset as usize;
            header.secret.clear();
            header.secret.extend_from_slice(&buf[..end]);
        }

        // In any case clear the buffer, which contains the secret.
        for elem in buf.iter_mut() {
            *elem = 0;
        }

        result
    }

    fn open_header(fd: &mut File) -> Result<Header> {
        // Create a temp. block with bsize = BLOCK_MIN_SIZE.
        // This is enough to read the header.
        // Binary header is dumped into `buf`.
        let io = IO::new(BLOCK_MIN_SIZE, 1, DiskType::ThinZero, fd)?;

        // Read the binary header into `buf`.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        io.read(fd, &mut buf, 0)?;

        // Parse the header.
        Header::read(&buf).map(|(header, _)| header)
    }

    fn open_secret(header: &Header) -> Result<Secret> {
        Secret::read(&header.secret).map(|(secret, _)| secret)
    }

    pub fn cipher(&self) -> Cipher {
        self.cipher
    }

    pub fn digest(&self) -> Option<Digest> {
        self.digest
    }

    pub fn dtype(&self) -> DiskType {
        self.io.dtype
    }

    pub fn bsize(&self) -> u32 {
        self.io.bsize
    }

    pub fn blocks(&self) -> u64 {
        self.io.blocks
    }

    pub fn ablocks(&self) -> u64 {
        self.io.ablocks
    }
}
