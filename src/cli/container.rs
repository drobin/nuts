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

pub mod create;
pub mod delete;
pub mod info;
pub mod list;
pub mod read;
pub mod write;

use anyhow::Result;
use clap::{Args, PossibleValue, Subcommand, ValueEnum};
use nuts_container::container::Cipher;
use std::ops::Deref;

use crate::cli::container::create::ContainerCreateArgs;
use crate::cli::container::delete::ContainerDeleteArgs;
use crate::cli::container::info::ContainerInfoArgs;
use crate::cli::container::list::ContainerListArgs;
use crate::cli::container::read::ContainerReadArgs;
use crate::cli::container::write::ContainerWriteArgs;

const AES128_GCM: &str = "aes128-gcm";
const AES128_CTR: &str = "aes128-ctr";
const NONE: &str = "none";

#[derive(Clone, Debug)]
pub struct CliCipher(Cipher);

impl PartialEq<Cipher> for CliCipher {
    fn eq(&self, other: &Cipher) -> bool {
        self.0 == *other
    }
}

impl Deref for CliCipher {
    type Target = Cipher;

    fn deref(&self) -> &Cipher {
        &self.0
    }
}

impl ValueEnum for CliCipher {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CliCipher(Cipher::Aes128Gcm),
            CliCipher(Cipher::Aes128Ctr),
            CliCipher(Cipher::None),
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        let value = match self.0 {
            Cipher::None => NONE,
            Cipher::Aes128Ctr => AES128_CTR,
            Cipher::Aes128Gcm => AES128_GCM,
        };

        Some(PossibleValue::new(value))
    }
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true, subcommand_required = true)]
pub struct ContainerArgs {
    #[clap(subcommand)]
    command: Option<ContainerCommand>,
}

impl ContainerArgs {
    pub fn run(&self) -> Result<()> {
        self.command
            .as_ref()
            .map_or(Ok(()), |command| command.run())
    }
}

#[derive(Debug, Subcommand)]
pub enum ContainerCommand {
    /// Creates a nuts-container
    Create(ContainerCreateArgs),

    /// Removes a container again
    Delete(ContainerDeleteArgs),

    /// Prints general information about the container
    Info(ContainerInfoArgs),

    /// Lists all available container
    List(ContainerListArgs),

    /// Reads a block from the container
    Read(ContainerReadArgs),

    /// Writes a block into the container
    Write(ContainerWriteArgs),
}

impl ContainerCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Create(args) => args.run(),
            Self::Delete(args) => args.run(),
            Self::Info(args) => args.run(),
            Self::List(args) => args.run(),
            Self::Read(args) => args.run(),
            Self::Write(args) => args.run(),
        }
    }
}
