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

use anyhow::Result;
use clap::{ArgGroup, Args};
use log::debug;
use nuts_container::ModifyOptionsBuilder;
use std::cell::RefCell;
use std::os::fd::RawFd;
use std::path::PathBuf;

use crate::cli::global::PasswordSource;
use crate::cli::open_container;
use crate::cli::password::password_from_source_twice;

thread_local! {
    static SOURCE: RefCell<PasswordSource> = RefCell::new(Default::default());
}

fn password_callback() -> Result<Vec<u8>, String> {
    SOURCE.with_borrow(|src| password_from_source_twice(src, "Enter a new password"))
}

#[derive(Args, Debug)]
#[clap(group(ArgGroup::new("new_password").required(false).multiple(false)))]
pub struct ContainerChangePasswordArgs {
    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    /// Reads the new password from the specified file descriptor <FD>. The
    /// password is the first line until a `\n` is read.
    #[clap(long, group = "new_password", value_name = "FD")]
    new_password_from_fd: Option<RawFd>,

    /// Reads the new password from the specified file <PATH>. The password is
    /// the first line until a `\n` is read.
    #[clap(long, group = "new_password", value_name = "PATH")]
    new_password_from_file: Option<PathBuf>,

    #[clap(from_global)]
    verbose: u8,
}

impl ContainerChangePasswordArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        SOURCE.with_borrow_mut(|src| {
            *src = PasswordSource::new(
                self.new_password_from_fd,
                self.new_password_from_file.clone(),
            )
        });

        let mut container = open_container(&self.container, self.verbose)?;
        let options = ModifyOptionsBuilder::default()
            .change_password(password_callback)
            .build();

        container.modify(options)?;

        Ok(())
    }
}
