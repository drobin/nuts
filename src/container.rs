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
use crate::types::{Cipher, Digest, DiskType, Options, BLOCK_MIN_SIZE};

pub struct Container {
    cipher: Cipher,
    digest: Digest,
    block: Block,
    fd: File,
}

impl Container {
    pub fn create(path: &str, options: &Options) -> Result<Container> {
        let mut container = Container {
            cipher: options.cipher,
            digest: options.md,
            block: Block::new(options.bsize(), options.blocks(), 0, options.dtype),
            fd: create_file(path)?,
        };

        // Create the header, written into `buf`.
        let header = Header::new(options.cipher, options.md);
        let mut buf = [0; 512];

        // Dump header into `buf`,
        // `buf` is written into the first block of the container.
        header.write(&mut buf)?;
        container.block.write(&buf, &mut container.fd, 0)?;

        Ok(container)
    }

    pub fn open(path: &str) -> Result<Container> {
        let mut fd = open_file(path)?;

        // Create a temp. block with bsize = BLOCK_MIN_SIZE.
        // This is enough to read the header.
        // Binary header is dumped into `buf`.
        let block = Block::new(BLOCK_MIN_SIZE, 1, 0, DiskType::ThinZero);

        // Read the binary header into `buf`.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        block.read(&mut fd, &mut buf, 0)?;

        // Parse the header.
        let mut offset = 0;
        let header = Header::read(&buf, &mut offset)?;

        Ok(Container {
            cipher: header.cipher,
            digest: header.digest.unwrap(),
            block: Block::new(BLOCK_MIN_SIZE, 0, 0, DiskType::FatRandom),
            fd: fd,
        })
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
