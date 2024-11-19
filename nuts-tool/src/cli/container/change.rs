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

pub mod kdf;
pub mod password;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::cli::container::change::kdf::ContainerChangeKdfArgs;
use crate::cli::container::change::password::ContainerChangePasswordArgs;

#[derive(Args, Debug)]
pub struct ContainerChangeArgs {
    #[clap(subcommand)]
    command: ContainerChangeCommand,
}

impl ContainerChangeArgs {
    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
pub enum ContainerChangeCommand {
    /// Changes the key derivation function of the container
    Kdf(ContainerChangeKdfArgs),

    /// Changes the password of the container
    Password(ContainerChangePasswordArgs),
}

impl ContainerChangeCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Kdf(args) => args.run(),
            Self::Password(args) => args.run(),
        }
    }
}
