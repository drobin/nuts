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

pub mod backends;
pub mod config;
pub mod container;

use anyhow::Result;
use clap::{Arg, ArgMatches};
use log::debug;
use nuts::container::{Container, OpenOptionsBuilder};
use nuts_backend::plugin::locate_backend;
use std::result;

use crate::tool::backend::ProxyBackend;
use crate::tool::backend::ProxyOpenOptions;
use crate::tool::config::{Config, ContainerConfig};
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

fn open_container(args: &ArgMatches) -> Result<Container<ProxyBackend>> {
    let name = args.value_of("NAME").unwrap();

    let config = Config::parse()?;
    let backend = ContainerConfig::get_backend(name)?;

    debug!("backend for container {}: {}", name, backend);

    let loader = locate_backend(backend, &config.search_path)?;
    let path = container_dir_for(name)?;

    let builder = OpenOptionsBuilder::new(ProxyOpenOptions::new(loader, path))
        .with_password_callback(ask_for_password);
    let options = builder.build()?;

    Ok(Container::open(options)?)
}
