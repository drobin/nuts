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
use log::{debug, trace};
use nuts::backend::Backend;
use nuts::directory::DirectoryBackend;
use std::cmp;
use std::io::{self, Read};

use crate::tool::actions::{container_arg, is_valid, open_container};
use crate::tool::convert::Convert;
use crate::tool::size::Size;

fn fill_buf(buf: &mut [u8]) -> Result<usize> {
    let mut nread = 0;

    while nread < buf.len() {
        let n = io::stdin().read(&mut buf[nread..])?;
        trace!("read {} bytes: nread: {}, max: {}", n, nread, buf.len());

        if n > 0 {
            nread += n;
        } else {
            break;
        }
    }

    Ok(nread)
}

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Writes a block into the container.")
        .arg(
            Arg::with_name("ID")
                .required(false)
                .index(1)
                .validator(is_valid::<<DirectoryBackend as Backend>::Id>)
                .help("The range of block-ids to write."),
        )
        .arg(container_arg())
        .arg(
            Arg::with_name("max-bytes")
                .long("max-bytes")
                .short("b")
                .value_name("SIZE")
                .validator(is_valid::<Size<u32>>)
                .help("Specifies the maximum number of bytes to write."),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let mut container = open_container(args)?;
    let block_size = container.backend().block_size();

    let id = match args.value_of("ID") {
        Some(s) => s.parse::<<DirectoryBackend as Backend>::Id>()?,
        None => container.aquire()?,
    };

    let max_bytes = args
        .value_of("max-bytes")
        .map_or(block_size, |s| *Size::<u32>::from_str(s).unwrap());
    let max_bytes = cmp::min(max_bytes, block_size);

    debug!("id: {}", id);
    debug!("max_bytes: {:?}", max_bytes);
    debug!("block_size: {}", block_size);

    let mut buf = vec![0; max_bytes as usize];
    let n = fill_buf(&mut buf)?;

    debug!("{} bytes read from stdin", n);

    container.write(&id, &buf[..n])?;

    println!("{} bytes written into {}.", n, id);

    Ok(())
}
