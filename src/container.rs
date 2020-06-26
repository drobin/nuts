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

use std::fs::File;

use crate::block::Block;
use crate::error::Error;
use crate::header::Header;
use crate::result::Result;
use crate::secret::Secret;
use crate::types::{Cipher, Digest, DiskType, Options, BLOCK_MIN_SIZE};

pub struct Container {
    cipher: Cipher,
    digest: Digest,
    block: Block,
    fd: File,
}

impl Container {
    pub fn create(path: &str, options: &Options) -> Result<Container> {
        let block = Block::new(options.bsize(), options.blocks(), 0, options.dtype);
        let mut fd = create_file(path)?;

        let mut header = Container::create_header(options);
        let secret = Container::create_secret(options);

        Container::dump_secret(&secret, &mut header)?;
        Container::dump_header(&header, &block, &mut fd)?;

        Ok(Container {
            cipher: options.cipher,
            digest: options.md,
            block,
            fd,
        })
    }

    pub fn open(path: &str) -> Result<Container> {
        let mut fd = open_file(path)?;
        let header = Container::open_header(&mut fd)?;
        let secret = Container::open_secret(&header)?;

        let block = Block::new(secret.bsize, secret.blocks, 0, secret.dtype);

        Ok(Container {
            cipher: header.cipher,
            digest: header.digest.unwrap(),
            block,
            fd,
        })
    }

    fn create_header(options: &Options) -> Header {
        let header = Header::new(options.cipher, options.md);

        // TODO generate keys if applicable

        header
    }

    fn create_secret(options: &Options) -> Secret {
        let mut secret = Secret::new();

        secret.dtype = options.dtype;
        secret.bsize = options.bsize();
        secret.blocks = options.blocks();

        // TODO generate keys if applicable

        secret
    }

    fn dump_header(header: &Header, block: &Block, fd: &mut File) -> Result<u32> {
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        let offset = header.write(&mut buf)?;
        let end = offset as usize;

        block.write(&buf[..end], fd, 0)
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
        let block = Block::new(BLOCK_MIN_SIZE, 1, 0, DiskType::ThinZero);

        // Read the binary header into `buf`.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        block.read(fd, &mut buf, 0)?;

        // Parse the header.
        Header::read(&buf).map(|(header, _)| header)
    }

    fn open_secret(header: &Header) -> Result<Secret> {
        Secret::read(&header.secret).map(|(secret, _)| secret)
    }

    pub fn cipher(&self) -> Cipher {
        self.cipher
    }

    pub fn digest(&self) -> Digest {
        self.digest
    }

    pub fn dtype(&self) -> DiskType {
        self.block.dtype
    }

    pub fn bsize(&self) -> u32 {
        self.block.bsize
    }

    pub fn blocks(&self) -> u64 {
        self.block.blocks
    }

    pub fn ablocks(&self) -> u64 {
        self.block.ablocks
    }
}

fn open_file(path: &str) -> Result<File> {
    File::open(path).or_else(|err| Err(Error::IoError(err)))
}

fn create_file(path: &str) -> Result<File> {
    File::create(path).or_else(|err| Err(Error::IoError(err)))
}
