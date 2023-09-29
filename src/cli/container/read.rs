// MIT License
//
// Copyright (c) 2023 Robin Doer
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
use log::debug;
use nuts_container::backend::Backend;
use nuts_directory::DirectoryBackend;
use std::cmp;

use crate::cli::container::open_container;
use crate::format::Format;

#[derive(Args, Debug)]
pub struct ContainerReadArgs {
    /// The id of the block to read
    id: String,

    /// Specifies the format of the userdata dump
    #[clap(short, long, value_parser, default_value = "raw")]
    format: Format,

    /// Reads up to SIZE bytes. If not specified, reads the whole block
    #[clap(short, long, id = "SIZE")]
    max_bytes: Option<u64>,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ContainerReadArgs {
    pub fn run(&self) -> Result<()> {
        debug!("id: {}", self.id);
        debug!("format: {:?}", self.format);
        debug!("max_bytes: {:?}", self.max_bytes);
        debug!("container: {}", self.container);

        let mut container = open_container(&self.container)?;
        let id: <DirectoryBackend as Backend>::Id = self.id.parse()?;

        let max_bytes = self.max_bytes.unwrap_or(u64::MAX);
        let block_size = container.block_size();
        let max_bytes = cmp::min(max_bytes, block_size as u64) as usize;

        debug!("block_size: {} => max_bytes: {}", block_size, max_bytes);

        let mut buf = vec![0; max_bytes];
        let mut writer = self.format.create_writer();

        let n = container.read(&id, &mut buf)?;

        writer.print(&buf[..n])?;
        writer.flush()?;

        Ok(())
    }
}
