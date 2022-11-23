// MIT License
//
// Copyright (c) 2022 Robin Doer
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
use nuts::backend::Backend;
use nuts::directory::DirectoryBackend;
use std::cmp;

use crate::tool::actions::{container_arg, is_valid, open_container};
use crate::tool::convert::Convert;
use crate::tool::format::{Format, Output};
use crate::tool::size::Size;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Reads a block from the container.")
        .arg(
            Arg::with_name("ID")
                .required(true)
                .index(1)
                .validator(is_valid::<<DirectoryBackend as Backend>::Id>)
                .help("Range of block-ids to read."),
        )
        .arg(container_arg())
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("FORMAT")
                .default_value("raw")
                .validator(is_valid::<Format>)
                .help(
                    "Specifies the format of the dump. \
                     Can be one of raw, hex.",
                ),
        )
        .arg(
            Arg::with_name("max-bytes")
                .long("max-bytes")
                .short("m")
                .value_name("SIZE")
                .validator(is_valid::<Size<u32>>)
                .help("Reads up to SIZE bytes. Default is unlimited."),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let mut container = open_container(args)?;
    let block_size = container.backend().block_size();

    let id = args
        .value_of("ID")
        .unwrap()
        .parse::<<DirectoryBackend as Backend>::Id>()
        .unwrap();

    let format = Format::from_str(args.value_of("format").unwrap()).unwrap();
    let max_bytes = args
        .value_of("max-bytes")
        .map_or(block_size, |s| *Size::<u32>::from_str(s).unwrap());
    let max_bytes = cmp::min(max_bytes, block_size);

    debug!("id: {}", id);
    debug!("format: {:?}", format);
    debug!("max_bytes: {}", max_bytes);

    let mut buf = vec![0; max_bytes as usize];
    let mut out = Output::new(format);

    let n = container.read(&id, &mut buf)?;

    out.print(&buf[..n]);
    out.flush();

    Ok(())
}
