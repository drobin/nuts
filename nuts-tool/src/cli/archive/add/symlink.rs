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

use crate::cli::archive::add::{TimestampArgs, TSTAMP_HELP};
use crate::cli::ctx::ArchiveContext;

#[derive(Args, Debug)]
#[clap(after_help(TSTAMP_HELP))]
pub struct ArchiveAddSymlinkArgs {
    /// Name of the symlink.
    name: String,

    /// Target of the symlink.
    target: String,

    #[clap(flatten)]
    timestamps: TimestampArgs,
}

impl ArchiveAddSymlinkArgs {
    pub fn run(&self, ctx: &ArchiveContext) -> Result<()> {
        debug!("args: {:?}", self);

        let mut archive = ctx.open_archive()?;
        let mut builder = archive.append_symlink(&self.name, &self.target);

        if let Some(created) = self.timestamps.created {
            builder.set_created(created);
        }

        if let Some(changed) = self.timestamps.changed {
            builder.set_changed(changed);
        }

        if let Some(modified) = self.timestamps.modified {
            builder.set_modified(modified);
        }

        builder.build().map_err(Into::into)
    }
}
