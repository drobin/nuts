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

use crate::cli::archive::open_archive;
use crate::say;
use crate::time::TimeFormat;

#[derive(Args, Debug)]
pub struct ArchiveInfoArgs {
    /// Specifies the format used for timestamps
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FORMAT",
        default_value = "local"
    )]
    time_format: TimeFormat,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveInfoArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let archive = open_archive(&self.container, self.verbose)?;
        let info = archive.info();

        let created = self.time_format.format(&info.created, "%c");
        let modified = self.time_format.format(&info.modified, "%c");

        say!("created:  {}", created);
        say!("modified: {}", modified);
        say!("blocks:   {}", info.blocks);
        say!("files:    {}", info.files);

        Ok(())
    }
}
