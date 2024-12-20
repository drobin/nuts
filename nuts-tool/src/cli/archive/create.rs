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
use clap::{ArgAction, Args};
use log::debug;
use nuts_archive::ArchiveFactory;
use nuts_container::Container;
use std::path::PathBuf;

use crate::archive::append_recursive;
use crate::cli::ctx::ArchiveContext;

#[derive(Args, Debug)]
pub struct ArchiveCreateArgs {
    /// Path to files/directories to be added to the archive. If PATHS contains
    /// a directory all entries in the directory are also appended. If no PATHS
    /// are specified an empty archive is created.
    paths: Vec<PathBuf>,

    /// Force to create the archive even if another service is running in the
    /// container
    #[clap(short, long, action = ArgAction::SetTrue)]
    force: bool,

    #[clap(long, hide = true)]
    migrate: bool,
}

impl ArchiveCreateArgs {
    pub fn run(&self, ctx: &ArchiveContext) -> Result<()> {
        debug!("args: {:?}", self);

        let container = ctx.open_container()?;
        let mut archive = Container::create_service::<ArchiveFactory>(container)?;

        for path in self.paths.iter() {
            append_recursive(ctx, &mut archive, path)?;
        }

        Ok(())
    }
}
