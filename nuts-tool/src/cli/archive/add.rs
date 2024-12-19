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

pub mod dir;
pub mod file;
pub mod symlink;

use anyhow::Result;
use chrono::offset::LocalResult;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use clap::builder::{TypedValueParser, ValueParserFactory};
use clap::{value_parser, Arg, ArgAction, Args, Command, Subcommand};
use log::debug;
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::archive::append_recursive;
use crate::cli::archive::add::dir::ArchiveAddDirectoryArgs;
use crate::cli::archive::add::file::ArchiveAddFileArgs;
use crate::cli::archive::add::symlink::ArchiveAddSymlinkArgs;
use crate::cli::ctx::ArchiveContext;

const TSTAMP_HELP: &str = "\x1B[1m\x1B[4mTimestamps:\x1B[0m

A <TIMESTAMP> argument is of the form \"YYYY-MM-DDThh:mm:ss[tz]\" where the letters represent the following:

    \x1B[4mYYYY\x1B[0m  Four decimal digits representing the year.
    \x1B[4mMM\x1B[0m    The month of the year, from 01 to 12.
    \x1B[4mDD\x1B[0m    the day of the month, from 01 to 31.
    \x1B[4mhh\x1B[0m    The hour of the day, from 00 to 23.
    \x1B[4mmm\x1B[0m    The minute of the hour, from 00 to 59.
    \x1B[4mss\x1B[0m    The second of the minute, from 00 to 60.
    \x1B[4mtz\x1B[0m    An optional letter Z indicating the time is in UTC. Otherwise, the time is assumed to be in local time.";

fn tstamp_error(cmd: &Command, arg: Option<&Arg>, value: &OsStr) -> clap::Error {
    use clap::error::{ContextKind, ContextValue, ErrorKind};

    let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);

    if let Some(a) = arg {
        err.insert(ContextKind::InvalidArg, ContextValue::String(a.to_string()));
    }

    err.insert(
        ContextKind::InvalidValue,
        ContextValue::String(value.to_string_lossy().to_string()),
    );

    err
}

#[derive(Clone, Debug)]
struct Timestamp;

impl TypedValueParser for Timestamp {
    type Value = DateTime<Utc>;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<DateTime<Utc>, clap::Error> {
        let str_value = value.to_string_lossy();

        let (str_value, utc) = match str_value.strip_suffix('Z') {
            Some(s) => (s, true),
            None => (str_value.as_ref(), false),
        };

        let dt = NaiveDateTime::parse_from_str(str_value, "%Y-%m-%dT%H:%M:%S")
            .map_err(|_| tstamp_error(cmd, arg, value))?;

        let dt_utc = if utc {
            Utc.from_local_datetime(&dt)
        } else {
            Local.from_local_datetime(&dt).map(Into::into)
        };

        match dt_utc {
            LocalResult::Single(dt) => Ok(dt),
            LocalResult::Ambiguous(_earliest, latest) => Ok(latest),
            LocalResult::None => Err(tstamp_error(cmd, arg, value)),
        }
    }
}

impl ValueParserFactory for Timestamp {
    type Parser = Self;

    fn value_parser() -> Self {
        Timestamp
    }
}

#[derive(Args, Debug)]
struct TimestampArgs {
    /// Change the creation time to the specified date time instead of the
    /// current time of day
    #[clap(short = 'r', long, value_parser = value_parser!(Timestamp), value_name = "TIMESTAMP")]
    created: Option<DateTime<Utc>>,

    /// Change the changed time to the specified date time instead of the
    /// current time of day
    #[clap(short = 'n', long, value_parser = value_parser!(Timestamp), value_name = "TIMESTAMP")]
    changed: Option<DateTime<Utc>>,

    /// Change the modified time to the specified date time instead of
    /// he current time of day
    #[clap(short = 'm', long, value_parser = value_parser!(Timestamp), value_name = "TIMESTAMP")]
    modified: Option<DateTime<Utc>>,
}

#[derive(Args, Debug)]
// #[clap(group(ArgGroup::new("input").required(true).multiple(false)))]
#[clap(args_conflicts_with_subcommands = true)]
pub struct ArchiveAddArgs {
    #[clap(subcommand)]
    command: Option<ArchiveAddCommand>,

    /// Path to files/directories to be added to the archive. If PATHS contains
    /// a directory all entries in the directory are also appended. If no PATHS
    /// are specified an empty archive is created.
    paths: Vec<PathBuf>,

    /// Starts the migration when the container/archive is opened
    #[clap(long, action = ArgAction::SetTrue)]
    pub migrate: bool,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ArchiveAddArgs {
    pub fn run(&self, ctx: &ArchiveContext) -> Result<()> {
        if let Some(command) = self.command.as_ref() {
            return command.run(ctx);
        }

        debug!("args: {:?}", self);

        let mut archive = ctx.open_archive(&self.container, self.migrate)?;

        for path in self.paths.iter() {
            append_recursive(ctx, &mut archive, path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum ArchiveAddCommand {
    /// Appends a custom file to the archive.
    File(ArchiveAddFileArgs),

    /// Appends a custom directory to the archive.
    Directory(ArchiveAddDirectoryArgs),

    /// Appends a custom symlink to the archive.
    Symlink(ArchiveAddSymlinkArgs),
}

impl ArchiveAddCommand {
    pub fn run(&self, ctx: &ArchiveContext) -> Result<()> {
        match self {
            Self::File(args) => args.run(ctx),
            Self::Directory(args) => args.run(ctx),
            Self::Symlink(args) => args.run(ctx),
        }
    }
}
