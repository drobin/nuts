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

pub mod add;
pub mod create;
pub mod extract;
pub mod info;
pub mod list;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use glob::Pattern;
use log::{debug, trace};
use nuts::container::{Container, OpenOptionsBuilder};
use nuts_archive::{Archive, PathBuilder};
use nutsbackend_directory::{DirectoryBackend, DirectoryOpenOptions};
use std::env;
use std::path::Path;

use crate::tool::container_dir_for;
use crate::tool::password::ask_for_password;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("a nuts archive")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(add::command(SubCommand::with_name("add")))
        .subcommand(create::command(SubCommand::with_name("create")))
        .subcommand(extract::command(SubCommand::with_name("extract")))
        .subcommand(info::command(SubCommand::with_name("info")))
        .subcommand(list::command(SubCommand::with_name("list")))
}

pub fn run(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("add", Some(matches)) => add::run(matches),
        ("create", Some(matches)) => create::run(matches),
        ("extract", Some(matches)) => extract::run(matches),
        ("info", Some(matches)) => info::run(matches),
        ("list", Some(matches)) => list::run(matches),
        _ => Err(anyhow!("Missing implementation for subcommand")),
    }
}

const PATTERN_HELP: &str = "Path to files/directories to be added to the \
    archive. If PATTERN contains a directory all entries in the directory are \
    also appended. if no PATTERN are specified an empty archive is created.";
const EXCLUDE_HELP: &str = "Do not process files or directories that match \
    the specified pattern.";
const INCLUDE_HELP: &str = "Process only files or directories that match the \
    specified pattern. Note that exclusions specified with --exclude take \
    precedence over inclusions. If no inclusions are explicitly specified, \
    all entries are processed by default.";

pub fn container_arg<'a, 'b>() -> Arg<'a, 'b> {
    let help = "Specifies the name of the container. Overwrites the name from \
                the NUTS_CONTAINER environment variable.";

    Arg::with_name("container")
        .long("container")
        .value_name("PATH")
        .help(help)
}

fn container_name(args: &ArgMatches) -> Result<String> {
    if let Some(s) = args.value_of("container") {
        debug!("name from cmdline: {}", s);
        Ok(s.to_string())
    } else {
        match env::var("NUTS_CONTAINER") {
            Ok(s) => {
                debug!("name from env: {}", s);
                Ok(s)
            }
            Err(env::VarError::NotPresent) => Err(anyhow!("Cannot obtain name of container.")),
            Err(env::VarError::NotUnicode(err)) => Err(anyhow!(
                "Invalid name of container: {}",
                err.to_string_lossy()
            )),
        }
    }
}

fn open_container(args: &ArgMatches) -> Result<Container<DirectoryBackend>> {
    let name = container_name(args)?;
    let path = container_dir_for(name)?;

    let builder = OpenOptionsBuilder::new(DirectoryOpenOptions::for_path(path))
        .with_password_callback(ask_for_password);
    let options = builder.build()?;

    Ok(Container::open(options)?)
}

pub fn time_format_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("time-format")
        .long("time-format")
        .value_name("FORMAT")
        .possible_values(&["local", "utc"])
        .default_value("local")
        .help("Specifies the format used for timestamps.")
}

fn to_timestamp(args: &ArgMatches, dt: &DateTime<Utc>) -> String {
    match args.value_of("time-format") {
        Some("local") => dt.with_timezone(&Local).to_string(),
        Some("utc") => dt.with_timezone(&Utc).to_string(),
        _ => panic!("invalid time-format"),
    }
}

fn any_match(path: &Path, pattern: &[Pattern]) -> bool {
    pattern.iter().any(|p| p.matches_path(path))
}

fn matches(path: &Path, includes: &[Pattern], excludes: &[Pattern]) -> bool {
    if excludes.is_empty() || !any_match(path, excludes) {
        if includes.is_empty() || any_match(path, includes) {
            trace!("{}: include match - pass", path.display());
            true
        } else {
            trace!("{} include mismatch - reject", path.display());
            false
        }
    } else {
        trace!("{}: exclude match - reject", path.display());
        true
    }
}

fn into_pattern_vec<'a, I: Iterator<Item = &'a str>>(iter: Option<I>) -> Result<Vec<Pattern>> {
    Ok(iter.map_or(Ok(vec![]), |iter| {
        iter.map(|pat| Pattern::new(pat)).collect()
    })?)
}

fn add_recursive<P: AsRef<Path>>(
    path: P,
    archive: &mut Archive<DirectoryBackend>,
    includes: &[Pattern],
    excludes: &[Pattern],
) -> Result<()> {
    let path = path.as_ref();

    if matches(path, includes, excludes) {
        archive.add(PathBuilder::resolve(path)?)?;

        println!("a {}", path.display());

        if path.is_dir() {
            for child in path.read_dir()? {
                add_recursive(child?.path(), archive, includes, excludes)?;
            }
        }
    }

    Ok(())
}
