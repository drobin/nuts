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

pub mod create;
pub mod delete;
pub mod info;
pub mod list;
pub mod read;
pub mod write;

use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use nuts::container::{Container, OpenOptionsBuilder};
use nutsbackend_directory::{DirectoryBackend, DirectoryOpenOptions};

use crate::tool::{container_dir_for, password::ask_for_password};

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("General container tasks")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(create::command(SubCommand::with_name("create")))
        .subcommand(delete::command(SubCommand::with_name("delete")))
        .subcommand(info::command(SubCommand::with_name("info")))
        .subcommand(list::command(SubCommand::with_name("list")))
        .subcommand(read::command(SubCommand::with_name("read")))
        .subcommand(write::command(SubCommand::with_name("write")))
}

pub fn run(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("create", Some(matches)) => create::run(matches),
        ("delete", Some(matches)) => delete::run(matches),
        ("info", Some(matches)) => info::run(matches),
        ("list", Some(matches)) => list::run(matches),
        ("read", Some(matches)) => read::run(matches),
        ("write", Some(matches)) => write::run(matches),
        _ => Err(anyhow!("Missing implementation for subcommand")),
    }
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
