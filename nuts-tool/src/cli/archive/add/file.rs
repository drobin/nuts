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
use log::debug;
use nuts_archive::Archive;
use std::io::{self, Read};

use crate::cli::archive::add::{TimestampArgs, TSTAMP_HELP};
use crate::cli::open_container;

#[derive(Args, Debug)]
#[clap(after_help(TSTAMP_HELP))]
pub struct ArchiveAddFileArgs {
    /// Name of the file.
    name: String,

    #[clap(flatten)]
    timestamps: TimestampArgs,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveAddFileArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let container = open_container(&self.container, self.verbose)?;
        let mut archive = Archive::open(container)?;
        let block_size = archive.as_ref().block_size() as usize;

        let mut builder = archive.append_file(&self.name);

        if let Some(created) = self.timestamps.created {
            builder.set_created(created);
        }

        if let Some(changed) = self.timestamps.changed {
            builder.set_changed(changed);
        }

        if let Some(modified) = self.timestamps.modified {
            builder.set_modified(modified);
        }

        let mut entry = builder.build()?;
        let mut buf = vec![0; block_size];

        loop {
            let n = io::stdin().read(&mut buf)?;
            debug!("{} bytes read from stdin", n);

            if n > 0 {
                entry.write_all(&buf[..n])?;
            } else {
                break;
            }
        }

        Ok(())
    }
}
