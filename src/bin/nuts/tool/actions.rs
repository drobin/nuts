// MIT License
//
// Copyright (c) 2022 Robin Doer
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

pub mod container;

use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};
use log::debug;
use nuts::container::{Container, OpenOptionsBuilder};
use nuts::directory::{DirectoryBackend, DirectoryOpenOptions};
use std::{env, result};

use crate::tool::convert::Convert;

fn is_valid<T: Convert>(s: String) -> result::Result<(), String> {
    T::from_str(&s).map(|_| ())
}

pub fn path_arg<'a, 'b>(idx: u64) -> Arg<'a, 'b> {
    Arg::with_name("PATH")
        .required(true)
        .index(idx)
        .help("The path to the container.")
}

pub fn container_arg<'a, 'b>() -> Arg<'a, 'b> {
    let help = "Specifies the path of the container. Overwrites the path from \
              the NUTS_CONTAINER environment variable.";

    Arg::with_name("container")
        .long("container")
        .value_name("PATH")
        .help(help)
}

fn container_path(args: &ArgMatches) -> Result<String> {
    if let Some(s) = args.value_of("container") {
        debug!("path from cmdline: {}", s);
        Ok(s.to_string())
    } else {
        match env::var("NUTS_CONTAINER") {
            Ok(s) => {
                debug!("path from env: {}", s);
                Ok(s)
            }
            Err(env::VarError::NotPresent) => Err(anyhow!("Cannot obtain path to container.")),
            Err(env::VarError::NotUnicode(err)) => Err(anyhow!(
                "Invalid path to container: {}",
                err.to_string_lossy()
            )),
        }
    }
}

fn open_container(args: &ArgMatches) -> Result<Container<DirectoryBackend>> {
    let path = container_path(args)?;
    let builder = OpenOptionsBuilder::for_backend(DirectoryOpenOptions::for_path(path));
    let options = builder.build()?;

    Ok(Container::open(options)?)
}
