// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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
use nuts_archive::{Entry, Group};
use std::cmp;
use std::fmt::{self, Write};

use crate::backend::PluginBackend;
use crate::cli::archive::open_archive;
use crate::say;
use crate::time::TimeFormat;

const SIZE_WIDTH: usize = 9;

struct Type<'a>(&'a Entry<'a, PluginBackend>);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Entry::File(_) => fmt.write_char('-'),
            Entry::Directory(_) => fmt.write_char('d'),
            Entry::Symlink(_) => fmt.write_char('l'),
        }
    }
}

struct Permission<'a>(&'a Entry<'a, PluginBackend>, Group);

impl<'a> fmt::Display for Permission<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.0.can_read(self.1) {
            fmt.write_char('r')?;
        } else {
            fmt.write_char('-')?;
        };

        if self.0.can_write(self.1) {
            fmt.write_char('w')?;
        } else {
            fmt.write_char('-')?;
        };

        if self.0.can_execute(self.1) {
            fmt.write_char('x')?;
        } else {
            fmt.write_char('-')?;
        };

        Ok(())
    }
}

struct Name<'a>(&'a Entry<'a, PluginBackend>);

impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(symlink) = self.0.as_symlink() {
            write!(fmt, "{} -> {}", self.0.name(), symlink.target())
        } else {
            write!(fmt, "{}", self.0.name())
        }
    }
}

#[derive(Debug)]
struct PrintLongContext {
    size_width: usize,
}

impl Default for PrintLongContext {
    fn default() -> Self {
        Self {
            size_width: SIZE_WIDTH,
        }
    }
}

#[derive(Args, Debug)]
pub struct ArchiveListArgs {
    /// Lists the content in the long format
    #[clap(short, long, action = ArgAction::SetTrue)]
    long: bool,

    /// Use appended time instead of last modification for long printing
    #[clap(short, long, action = ArgAction::SetTrue)]
    appended: bool,

    /// Use creation time instead of last modification for long printing
    #[clap(short = 'r', long, action = ArgAction::SetTrue)]
    created: bool,

    /// Use change time instead of last modification for long printing
    #[clap(short = 'n', long, action = ArgAction::SetTrue)]
    changed: bool,

    /// Specifies the format used for timestamps
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FORMAT",
        default_value = "local"
    )]
    time_format: TimeFormat,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveListArgs {
    fn print_short(&self, entry: &Entry<PluginBackend>) {
        say!("{}", entry.name());
    }

    fn print_long(&self, entry: &Entry<PluginBackend>, ctx: &mut PrintLongContext) {
        let tstamp = if self.appended {
            entry.appended()
        } else if self.created {
            entry.created()
        } else if self.changed {
            entry.changed()
        } else {
            entry.modified()
        };

        let size = entry.size().to_string();

        ctx.size_width = cmp::max(ctx.size_width, size.len());

        say!(
            "{}{}{}{} {:>size_width$} {} {}",
            Type(entry),
            Permission(entry, Group::User),
            Permission(entry, Group::Group),
            Permission(entry, Group::Other),
            size,
            self.time_format.format(tstamp, "%d %b %H:%M"),
            Name(entry),
            size_width = ctx.size_width,
        );
    }

    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let mut archive = open_archive(&self.container, self.verbose)?;

        let mut entry_opt = archive.first();
        let mut ctx_opt = None;

        loop {
            match entry_opt {
                Some(Ok(entry)) => {
                    if self.long {
                        let ctx = ctx_opt.get_or_insert_with(Default::default);
                        self.print_long(&entry, ctx);
                    } else {
                        self.print_short(&entry);
                    }

                    entry_opt = entry.next();
                }
                Some(Err(err)) => return Err(err.into()),
                None => break,
            }
        }

        Ok(())
    }
}
