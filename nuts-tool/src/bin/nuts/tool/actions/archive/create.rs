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
use clap::{App, Arg, ArgMatches};
use log::debug;
use nuts_archive::Archive;

use crate::tool::actions::archive::{
    add_recursive, container_arg, into_pattern_vec, open_container, EXCLUDE_HELP, INCLUDE_HELP,
    PATTERN_HELP,
};

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Creates a nuts archive.")
        .arg(container_arg())
        .arg(
            Arg::with_name("files")
                .required(false)
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
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let includes = into_pattern_vec(args.values_of("include"))?;
    let excludes = into_pattern_vec(args.values_of("exclude"))?;

    debug!("includes: {:?}", includes);
    debug!("excludes: {:?}", excludes);

    let container = open_container(args)?;
    let mut archive = Archive::create(container)?;

    if let Some(paths) = args.values_of("files") {
        for path in paths {
            debug!("traverse {}", path);
            add_recursive(path, &mut archive, &includes, &excludes)?;
        }
    }

    Ok(())
}
