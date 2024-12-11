// MIT License
//
// Copyright (c) 2024 Robin Doer
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
use clap::{ArgAction, Args};
use log::debug;
use nuts_container::LATEST_REVISION;
use std::cmp::Ordering;

use crate::cli::archive::open_archive;
use crate::cli::error::ExitOnly;
use crate::cli::prompt_yes_no;
use crate::say;

#[derive(Args, Debug)]
pub struct ArchiveMigrateArgs {
    /// Instead of executing the migration, simply check whether one is
    /// necessary. The exit code is 1 if a migration can be carried out,
    /// otherwise 0
    #[clap(long, action = ArgAction::SetTrue)]
    verify: bool,

    /// Say yes, don't prompt for migration
    #[clap(short, long, action = ArgAction::SetTrue)]
    yes: bool,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ArchiveMigrateArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let archive = open_archive(&self.container, false)?;
        let info = archive.as_ref().info()?;

        match info.revision.cmp(&LATEST_REVISION) {
            Ordering::Equal => {
                say!(
                    "container revision: {}, no migration necessary",
                    info.revision
                );

                Ok(())
            }
            Ordering::Less => {
                say!(
                    "container revision: {}, migration required to revision {}",
                    info.revision,
                    LATEST_REVISION
                );

                if self.verify {
                    Err(ExitOnly::new(1).into())
                } else if prompt_yes_no("Do you really want to start the migration?", self.yes)? {
                    open_archive(&self.container, true).map(|_| ())
                } else {
                    say!("aborted");
                    Ok(())
                }
            }
            Ordering::Greater => Err(anyhow!(
                "invalid container revision {}, cannot be greater than {}",
                info.revision,
                LATEST_REVISION
            )),
        }
    }
}
