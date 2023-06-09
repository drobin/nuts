// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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
use nuts::container::Container;
use nuts::stream::Stream;
use nutsbackend_directory::{DirectoryBackend, DirectoryId};
use std::cmp;

use crate::tool::actions::container::{name_arg, open_container};
use crate::tool::actions::is_valid;
use crate::tool::format::{Format, Output};
use crate::tool::size::Size;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Reads a block from the container.")
        .arg(name_arg(1))
        .arg(
            Arg::with_name("ID")
                .required(true)
                .index(2)
                .help("Range of block-ids to read."),
        )
        .arg(
            Arg::with_name("stream")
                .long("--stream")
                .short("s")
                .help("Enables streaming mode."),
        )
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
                .validator(is_valid::<Size<u64>>)
                .help("Reads up to SIZE bytes. Default is unlimited."),
        )
}

fn read_block(
    mut container: Container<DirectoryBackend>,
    id: DirectoryId,
    format: Format,
    max_bytes: u64,
) -> Result<()> {
    let block_size = container.backend().block_size();
    let max_bytes = cmp::min(max_bytes, block_size as u64);

    let mut buf = vec![0; max_bytes as usize];
    let mut out = Output::new(format);

    let n = container.read(&id, &mut buf)?;

    out.print(&buf[..n]);
    out.flush();

    Ok(())
}

fn read_stream(
    container: Container<DirectoryBackend>,
    id: DirectoryId,
    format: Format,
    max_bytes: u64,
) -> Result<()> {
    let mut stream = Stream::open(container, id)?;
    let mut buf = [0; 1024];
    let mut out = Output::new(format);
    let mut remaining_bytes = max_bytes as usize;

    loop {
        let len = cmp::min(buf.len(), remaining_bytes);
        let nread = stream.read(&mut buf[..len])?;

        if nread > 0 {
            out.print(&buf[..nread]);
            remaining_bytes -= nread;

            debug!("{} bytes read, remaining: {}", nread, remaining_bytes);
        } else {
            out.flush();

            debug!("end of stream reached");
            break;
        }
    }

    out.flush();

    Ok(())
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let container = open_container(args)?;
    let id = args.value_of("ID").unwrap().parse()?;

    let streaming = args.is_present("stream");
    let format = args.value_of("format").unwrap().parse::<Format>().unwrap();
    let max_bytes = args
        .value_of("max-bytes")
        .map_or(u64::MAX, |s| *s.parse::<Size<u64>>().unwrap());

    debug!("id: {}", id);
    debug!("streaming: {}", streaming);
    debug!("format: {:?}", format);
    debug!("max_bytes: {}", max_bytes);

    if streaming {
        read_stream(container, id, format, max_bytes)
    } else {
        read_block(container, id, format, max_bytes)
    }
}
