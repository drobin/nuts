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

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use log::debug;
use nuts_archive::{Archive, DirectoryBuilder, FileBuilder};
use nutsbackend_directory::DirectoryBackend;
use std::fs::File;
use std::io::{self};

use crate::tool::actions::archive::{
    add_recursive, container_arg, into_pattern_vec, open_container, EXCLUDE_HELP, INCLUDE_HELP,
    PATTERN_HELP,
};

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Appends an entry to a nuts archive.")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(container_arg())
        .arg(
            Arg::with_name("files")
                .required(true)
                .multiple(true)
                .value_name("PATTERN")
                .help(PATTERN_HELP),
        )
        .arg(
            Arg::with_name("exclude")
                .long("exclude")
                .multiple(true)
                .value_name("PATTERN")
                .help(EXCLUDE_HELP),
        )
        .arg(
            Arg::with_name("include")
                .long("include")
                .multiple(true)
                .value_name("PATTERN")
                .help(INCLUDE_HELP),
        )
        .subcommand(
            SubCommand::with_name("file")
                .about("Appends a single file to a nuts archive.")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .value_name("NAME")
                        .help("Name of the file."),
                )
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .value_name("PATH")
                        .help("Specifies the path to the file to read for the entry."),
                ),
        )
        .subcommand(
            SubCommand::with_name("directory")
                .about("Appends a single directory to a nuts archive.")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .value_name("NAME")
                        .help("Name of the directory."),
                ),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let container = open_container(args)?;
    let mut archive = Archive::open(container)?;

    match args.subcommand() {
        ("file", Some(matches)) => add_file(matches, &mut archive),
        ("directory", Some(matches)) => add_directory(matches, &mut archive),
        _ => add_multiple(args, &mut archive),
    }
}

fn add_file(args: &ArgMatches, archive: &mut Archive<DirectoryBackend>) -> Result<()> {
    let name = args.value_of("name").unwrap();
    debug!("name: {}", name);

    let reader: Box<dyn io::Read> = match args.value_of("file") {
        Some(path) => {
            let file = File::open(path)?;

            debug!("file: {:?}", file);

            Box::new(file)
        }
        None => {
            debug!("read from stdin");
            Box::new(io::stdin())
        }
    };

    let builder = FileBuilder::new(name.to_string()).set_content(reader);

    archive.add(builder)?;

    println!("a {}", name);

    Ok(())
}

fn add_directory(args: &ArgMatches, archive: &mut Archive<DirectoryBackend>) -> Result<()> {
    let name = args.value_of("name").unwrap();
    debug!("name: {}", name);

    let builder = DirectoryBuilder::new(name.to_string());

    archive.add(builder)?;

    println!("a {}", name);

    Ok(())
}

fn add_multiple(args: &ArgMatches, archive: &mut Archive<DirectoryBackend>) -> Result<()> {
    let includes = into_pattern_vec(args.values_of("include"))?;
    let excludes = into_pattern_vec(args.values_of("exclude"))?;

    debug!("includes: {:?}", includes);
    debug!("excludes: {:?}", excludes);

    if let Some(paths) = args.values_of("files") {
        for path in paths {
            debug!("traverse {}", path);
            add_recursive(path, archive, &includes, &excludes)?;
        }
    }

    Ok(())
}
