// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use anyhow::Result;
use clap::{Arg, ArgMatches};
use nuts::container::{Container, OpenOptionsBuilder};
use nutsbackend_directory::{DirectoryBackend, DirectoryOpenOptions};
use std::result;

use crate::tool::container_dir_for;
use crate::tool::convert::Convert;
use crate::tool::password::ask_for_password;

fn is_valid<T: Convert>(s: String) -> result::Result<(), String> {
    T::from_str(&s).map(|_| ())
}

pub fn name_arg<'a, 'b>(idx: u64) -> Arg<'a, 'b> {
    Arg::with_name("NAME")
        .required(true)
        .index(idx)
        .help("The name of the container.")
}

fn open_container(args: &ArgMatches) -> Result<Container<DirectoryBackend>> {
    let name = args.value_of("NAME").unwrap();
    let path = container_dir_for(name)?;

    let builder = OpenOptionsBuilder::new(DirectoryOpenOptions::for_path(path))
        .with_password_callback(ask_for_password);
    let options = builder.build()?;

    Ok(Container::open(options)?)
}
