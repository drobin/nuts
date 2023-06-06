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
use clap::{App, ArgMatches};
use log::debug;
use nuts_archive::{self, Archive, Group, IterEntry, Type};
use nuts_backend::Backend;
use nutsbackend_directory::DirectoryBackend;
use std::result;

use crate::tool::actions::archive::{container_arg, open_container, time_format_arg, to_timestamp};

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Lists the content of an archive.")
        .arg(container_arg())
        .arg(time_format_arg())
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let container = open_container(args)?;
    let mut archive = Archive::open(container)?;

    let entries =
        archive.scan().collect::<result::Result<
            Vec<IterEntry<DirectoryBackend>>,
            nuts_archive::Error<DirectoryBackend>,
        >>()?;

    let max_size = entries
        .iter()
        .max_by(|lhs, rhs| lhs.size().cmp(&rhs.size()))
        .map_or_else(|| 0, |entry| entry.size());
    let max_size_len = max_size.to_string().len();
    debug!("max size: {} ({})", max_size, max_size_len);

    for entry in entries {
        println!(
            "{} {:>size_width$} {} {}",
            as_modestr(&entry),
            entry.size(),
            to_timestamp(args, &entry.mtime()),
            entry.name(),
            size_width = max_size_len
        );
    }

    Ok(())
}

macro_rules! to_char {
    ($entry:expr, $question:ident ( $group:path ) ? $true:literal : $false:literal) => {
        if $entry.$question($group) {
            $true
        } else {
            $false
        }
    };
}

fn as_modestr<B: Backend>(entry: &IterEntry<B>) -> String {
    let mut s = String::with_capacity(10);

    s.push(match entry.ftype() {
        Type::File => '-',
        Type::Directory => 'd',
        Type::Symlink => 'l',
    });

    s.push(to_char!(entry, is_readable(Group::User) ? 'r' : '-'));
    s.push(to_char!(entry, is_writable(Group::User) ? 'w' : '-'));
    s.push(to_char!(entry, is_executable(Group::User) ? 'x' : '-'));
    s.push(to_char!(entry, is_readable(Group::Group) ? 'r' : '-'));
    s.push(to_char!(entry, is_writable(Group::Group) ? 'w' : '-'));
    s.push(to_char!(entry, is_executable(Group::Group) ? 'x' : '-'));
    s.push(to_char!(entry, is_readable(Group::Other) ? 'r' : '-'));
    s.push(to_char!(entry, is_writable(Group::Other) ? 'w' : '-'));
    s.push(to_char!(entry, is_executable(Group::Other) ? 'x' : '-'));

    s
}
