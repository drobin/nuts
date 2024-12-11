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

use clap::{ArgAction, ArgGroup, Args};
use log::debug;
use std::cell::RefCell;
use std::os::fd::RawFd;
use std::path::PathBuf;

use crate::say::Say;

thread_local! {
    pub static GLOBALS: RefCell<GlobalValues> = RefCell::new(Default::default());
}

#[derive(Default)]
pub struct GlobalValues {
    pub verbose: u8,
    pub say: Say,
    pub password_source: PasswordSource,
}

impl GlobalValues {
    fn init_password_source(&mut self, args: &GlobalArgs) {
        if let Some(fd) = args.password_from_fd {
            debug!("read password from fd {fd}");
            self.password_source = PasswordSource::Fd(fd);
        } else if let Some(path) = args.password_from_file.as_ref() {
            debug!("read password from path {}", path.display());
            self.password_source = PasswordSource::Path(path.clone());
        } else {
            debug!("read password from console");
            self.password_source = PasswordSource::Console;
        }
    }
}

pub enum PasswordSource {
    Fd(RawFd),
    Path(PathBuf),
    Console,
}

impl PasswordSource {
    pub fn new(fd: Option<RawFd>, path: Option<PathBuf>) -> PasswordSource {
        if let Some(fd) = fd {
            Self::Fd(fd)
        } else if let Some(path) = path {
            Self::Path(path)
        } else {
            Self::Console
        }
    }
}

impl Default for PasswordSource {
    fn default() -> Self {
        Self::Console
    }
}

#[derive(Args, Clone, Debug)]
#[clap(group(ArgGroup::new("password").required(false).multiple(false)))]
pub struct GlobalArgs {
    /// Enable verbose output. Can be called multiple times
    #[clap(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Be quiet. Don't produce any output
    #[clap(short, long, action = ArgAction::SetTrue, global = true)]
    pub quiet: bool,

    /// Reads the password from the specified file descriptor <FD>. The
    /// password is the first line until a `\n` is read.
    #[clap(long, group = "password", global = true, value_name = "FD")]
    pub password_from_fd: Option<RawFd>,

    /// Reads the password from the specified file <PATH>. The password is the
    /// first line until a `\n` is read.
    #[clap(long, group = "password", global = true, value_name = "PATH")]
    pub password_from_file: Option<PathBuf>,
}

impl GlobalArgs {
    pub fn init(&self) {
        GLOBALS.with_borrow_mut(|g| {
            g.verbose = self.verbose;
            g.say.set_quiet(self.quiet);
            g.init_password_source(self);
        });
    }
}
