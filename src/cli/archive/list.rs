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
use clap::{ArgAction, Args};
use log::debug;
use nuts_archive::{Archive, Entry};
use nuts_directory::DirectoryBackend;
use std::fmt::{self, Write};
use std::path::PathBuf;

use crate::cli::open_container;

enum Type {
    File,
    Directory,
    Symlink,
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::File => fmt.write_char('-'),
            Type::Directory => fmt.write_char('d'),
            Type::Symlink => fmt.write_char('l'),
        }
    }
}

impl<'a> From<&'a Entry<'a, DirectoryBackend<PathBuf>>> for Type {
    fn from(entry: &'a Entry<'a, DirectoryBackend<PathBuf>>) -> Self {
        match entry {
            Entry::File(_) => Type::File,
            Entry::Directory(_) => Type::Directory,
            Entry::Symlink(_) => Type::Symlink,
        }
    }
}

struct ListEntry {
    name: String,
    size: u64,
    r#type: Type,
}

impl<'a> From<&'a Entry<'a, DirectoryBackend<PathBuf>>> for ListEntry {
    fn from(entry: &Entry<DirectoryBackend<PathBuf>>) -> Self {
        ListEntry {
            name: entry.name().to_string(),
            size: entry.size(),
            r#type: entry.into(),
        }
    }
}

fn collect_entries<'a>(
    archive: &'a mut Archive<DirectoryBackend<PathBuf>>,
) -> Result<Vec<ListEntry>> {
    let mut vec = vec![];
    let mut entry_opt = archive.first();

    loop {
        match entry_opt {
            Some(Ok(entry)) => {
                vec.push((&entry).into());
                entry_opt = entry.next();
            }
            Some(Err(err)) => return Err(err.into()),
            None => break,
        }
    }

    Ok(vec)
}

fn print_short(entries: Vec<ListEntry>) {
    for entry in entries {
        println!("{}", entry.name);
    }
}

fn print_long(entries: Vec<ListEntry>) {
    let max_size = entries.iter().max_by_key(|e| e.size).map_or(0, |e| e.size);
    let max_n = max_size.checked_ilog10().unwrap_or(0) as usize + 1;

    for entry in entries {
        println!("{} {:>max_n$} {}", entry.r#type, entry.size, entry.name);
    }
}

#[derive(Args, Debug)]
pub struct ArchiveListArgs {
    /// Lists the content in the long format
    #[clap(short, long, action = ArgAction::SetTrue)]
    long: bool,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ArchiveListArgs {
    pub fn run(&self) -> Result<()> {
        debug!("container: {}", self.container);
        debug!("long: {}", self.long);

        let container = open_container(&self.container)?;
        let mut archive = Archive::open(container)?;

        let entries = collect_entries(&mut archive)?;

        if self.long {
            print_long(entries);
        } else {
            print_short(entries);
        }

        Ok(())
    }
}
