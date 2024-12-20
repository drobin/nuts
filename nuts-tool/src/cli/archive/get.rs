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

use anyhow::{anyhow, Result};
use clap::Args;
use log::{debug, trace};

use crate::cli::ctx::ArchiveContext;
use crate::format::Format;

#[derive(Args, Debug)]
pub struct ArchiveGetArgs {
    /// The  name of the entry to print
    name: String,

    /// Specifies the format of the output
    #[clap(short, long, value_parser, default_value = "raw")]
    format: Format,
}

impl ArchiveGetArgs {
    pub fn run(&self, ctx: &ArchiveContext) -> Result<()> {
        debug!("args: {:?}", self);

        let mut archive = ctx.open_archive()?;
        let block_size = archive.as_ref().block_size() as usize;

        let entry = match archive.lookup(&self.name) {
            Some(Ok(e)) => e,
            Some(Err(err)) => return Err(err.into()),
            None => return Err(anyhow!("no such entry: {}", self.name)),
        };

        if let Some(mut file) = entry.into_file() {
            let mut buf = vec![0; block_size];
            let mut writer = self.format.create_writer();

            loop {
                let n = file.read(&mut buf)?;
                trace!("{} bytes read from {}", n, self.name);

                if n > 0 {
                    if !ctx.quiet {
                        writer.print(&buf[..n])?;
                    }
                } else {
                    break;
                }
            }

            if !ctx.quiet {
                writer.flush()?;
            }
        }

        Ok(())
    }
}
