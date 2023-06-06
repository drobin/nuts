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

use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use log::debug;
use nuts_archive::{Archive, IterEntry};
use nutsbackend_directory::DirectoryBackend;
use std::io::{self, Read, Write};

use crate::tool::actions::archive::{container_arg, open_container};

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Extracts an entry from a nuts archive.")
        .arg(container_arg())
        .arg(
            Arg::with_name("path")
                .required(true)
                .index(1)
                .value_name("PATH")
                .help("Path to file to be extracted."),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let container = open_container(args)?;
    let mut archive = Archive::open(container)?;

    let path = args.value_of("path").unwrap();

    debug!("path: {}", path);

    match find_entry(&mut archive, path)? {
        Some(entry) => read_entry(entry),
        None => Err(anyhow!("no such file: {}", path)),
    }
}

fn find_entry<'a, 'b>(
    archive: &'a mut Archive<DirectoryBackend>,
    name: &'b str,
) -> Result<Option<IterEntry<'a, DirectoryBackend>>> {
    for result in archive.scan() {
        let entry = result?;

        if entry.name() == name {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

fn read_entry(mut entry: IterEntry<DirectoryBackend>) -> Result<()> {
    let mut buf = [0; 64];

    loop {
        let n = entry.read(&mut buf)?;

        if n > 0 {
            io::stdout().write_all(&buf[..n])?;
        } else {
            break;
        }
    }

    io::stdout().flush()?;

    Ok(())
}
