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

use anyhow::Result;
use clap::Args;
use log::{debug, trace};
use std::cmp;
use std::io::{self, Read};

use crate::cli::ctx::ContainerContext;

fn fill_buf(buf: &mut [u8]) -> Result<usize> {
    let mut nread = 0;

    while nread < buf.len() {
        let n = io::stdin().read(&mut buf[nread..])?;
        trace!("read {} bytes: nread: {}, max: {}", n, nread, buf.len());

        if n > 0 {
            nread += n;
        } else {
            break;
        }
    }

    Ok(nread)
}

#[derive(Args, Debug)]
pub struct ContainerWriteArgs {
    /// The id of the block to write. If not specified, aquire a new block
    id: Option<String>,

    /// Writes up to SIZE bytes. If not specified, write the whole block
    #[clap(short, long, id = "SIZE")]
    max_bytes: Option<u64>,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ContainerWriteArgs {
    pub fn run(&self, ctx: &ContainerContext) -> Result<()> {
        debug!("args: {:?}", self);

        let mut container = ctx.open_container(&self.container)?;

        let block_size = container.block_size();
        let max_bytes = self.max_bytes.unwrap_or(u64::MAX);
        let max_bytes = cmp::min(max_bytes, block_size as u64) as usize;

        debug!("block_size: {} => max_bytes: {}", block_size, max_bytes);

        let id = match self.id.as_ref() {
            Some(s) => {
                let id = s.parse()?;
                debug!("use id from cmdline: {}", id);
                id
            }
            None => {
                let id = container.aquire()?;
                debug!("aquire new id: {}", id);
                id
            }
        };

        let mut buf = vec![0; max_bytes];
        let n = fill_buf(&mut buf)?;

        debug!("{} bytes read from stdin", n);

        container.write(&id, &buf[..n])?;

        println!("{} bytes written into {}", n, id);
        Ok(())
    }
}
