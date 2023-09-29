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

use anyhow::{anyhow, Result};
use clap::{Args, PossibleValue, Subcommand, ValueEnum};
use log::debug;
use nuts_container::container::{Cipher, Container, Digest, Kdf, OpenOptionsBuilder};
use nuts_directory::{DirectoryBackend, OpenOptions};
use rpassword::prompt_password;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;

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

#[derive(Clone, Debug)]
pub struct CliKdf {
    _algorithm: String,
    digest: Option<Digest>,
    iterations: Option<u32>,
    salt_len: Option<u32>,
}

impl CliKdf {
    fn new(algorithm: &str) -> CliKdf {
        CliKdf {
            _algorithm: algorithm.to_string(),
            digest: None,
            iterations: None,
            salt_len: None,
        }
    }

    fn to_kdf(&self) -> Result<Kdf> {
        let digest = self.digest.unwrap_or(Digest::Sha1);
        let iterations = self.iterations.unwrap_or(65536);
        let salt_len = self.salt_len.unwrap_or(16);

        Ok(Kdf::generate_pbkdf2(digest, iterations, salt_len)?)
    }

    fn parse_pbkdf2(v: &[&str]) -> Result<CliKdf, String> {
        if v.len() == 1 {
            Ok(CliKdf::new(v[0]))
        } else if v.len() == 4 {
            let mut kdf = CliKdf::new(v[0]);

            if !v[1].is_empty() {
                kdf.digest = Some(
                    v[1].parse()
                        .map_err(|()| format!("invalid digest: {}", v[1]))?,
                );
            }

            if !v[2].is_empty() {
                kdf.iterations = Some(
                    v[2].parse()
                        .map_err(|_| format!("invalid iterations: {}", v[2]))?,
                );
            }

            if !v[3].is_empty() {
                kdf.salt_len = Some(
                    v[3].parse()
                        .map_err(|_| format!("invalid salt length: {}", v[3]))?,
                );
            }

            Ok(kdf)
        } else {
            Err(String::from("invalid KDF specification"))
        }
    }
}

impl FromStr for CliKdf {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let v: Vec<&str> = s
            .split(':')
            .map(|s| s.trim_matches(char::is_whitespace))
            .collect();

        if v.is_empty() {
            return Err("empty KDF specification".to_string());
        }

        match v[0] {
            "pbkdf2" => Self::parse_pbkdf2(&v),
            _ => Err("unknown algorithm".to_string()),
        }
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

fn tool_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => {
            let tool_dir = dir.join(".nuts");

            debug!("tool_dir: {}", tool_dir.display());

            if !tool_dir.is_dir() {
                debug!("creating tool dir {}", tool_dir.display());
                fs::create_dir(&tool_dir)?;
            }

            Ok(tool_dir)
        }
        None => Err(anyhow!("unable to locate home-directory")),
    }
}

fn open_container(name: &str) -> Result<Container<DirectoryBackend>> {
    // let name = container_name(args)?;
    let path = container_dir_for(name)?;

    let builder = OpenOptionsBuilder::new().with_password_callback(ask_for_password);
    let options = builder.build::<DirectoryBackend>()?;

    Ok(Container::open(OpenOptions::for_path(path), options)?)
}

fn container_dir() -> Result<PathBuf> {
    let parent = tool_dir()?;
    let dir = parent.join("container.d");

    debug!("container_dir: {}", dir.display());

    if !dir.is_dir() {
        debug!("creating container dir {}", dir.display());
        fs::create_dir(&dir)?;
    }

    Ok(dir)
}

fn container_dir_for<S: AsRef<str>>(name: S) -> Result<PathBuf> {
    let parent = container_dir()?;
    let dir = parent.join(name.as_ref());

    debug!("container_dir for {}: {}", name.as_ref(), dir.display());

    Ok(dir)
}

pub fn ask_for_password() -> Result<Vec<u8>, String> {
    let password = prompt_password("Enter a password: ").map_err(|err| err.to_string())?;
    Ok(password.as_bytes().to_vec())
}
