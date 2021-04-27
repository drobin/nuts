// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

use clap::ArgMatches;
use log::debug;
use nuts::container::Container;
use std::cmp;

use crate::tool;
use crate::tool::convert::Convert;
use crate::tool::format::Format;
use crate::tool::id::IdRange;
use crate::tool::logger;
use crate::tool::output::Output;
use crate::tool::result::Result;

pub fn run(sub: &ArgMatches) -> Result<()> {
    logger::update(sub);

    let path = sub.value_of("PATH").unwrap();
    let mut range = IdRange::from_str(sub.value_of("RANGE").unwrap())?;
    let mut container = Container::new();

    let format = match sub.value_of("format") {
        Some(format) => Format::from_str(format)?,
        None => Format::Raw,
    };

    let max_bytes = match sub.value_of("max-bytes") {
        Some(s) => u64::from_str(s)?,
        None => u64::MAX,
    };

    container.set_password_callback(tool::utils::ask_for_password);
    container.open(path, None)?;

    range.resolve(&container)?;
    debug!("range: {:?}", range);

    read(sub, &mut container, format, &range, max_bytes)
}

fn read(
    sub: &ArgMatches,
    container: &mut Container,
    format: Format,
    range: &IdRange,
    max_bytes: u64,
) -> Result<()> {
    let mut output = Output::new(format);

    let mut remaining_bytes = max_bytes;
    let mut buf = vec![0; container.bsize()? as usize];

    for id in range.iter() {
        let read_bytes = container.read(id, &mut buf)?;
        let num_bytes = cmp::min(read_bytes as usize, remaining_bytes as usize);

        remaining_bytes = remaining_bytes - num_bytes as u64;

        if !sub.is_present("quiet") {
            output.push(&buf[..num_bytes]).print();
        }

        if remaining_bytes == 0 {
            break;
        }
    }

    if !sub.is_present("quiet") {
        output.flush();
    }

    Ok(())
}
