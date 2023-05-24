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
use log::{debug, trace};
use nuts::container::Container;
use nuts::stream::Stream;
use nuts_backend::Backend;
use nutsbackend_directory::{DirectoryBackend, DirectoryId};
use std::cmp;
use std::io::{self, Read};

use crate::tool::actions::container::{name_arg, open_container};
use crate::tool::actions::is_valid;
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

fn write_all(stream: &mut Stream<DirectoryBackend>, buf: &[u8]) -> Result<()> {
    let mut nbytes = 0;

    while nbytes < buf.len() {
        let n = stream.write(&buf[nbytes..])?;
        assert!(n > 0);
        nbytes += n;
    }

    Ok(())
}

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Writes a block into the container.")
        .arg(name_arg(1))
        .arg(
            Arg::with_name("ID")
                .required(false)
                .index(2)
                .help("The range of block-ids to write."),
        )
        .arg(
            Arg::with_name("stream")
                .long("--stream")
                .short("s")
                .help("Enables streaming mode."),
        )
        .arg(
            Arg::with_name("max-bytes")
                .long("max-bytes")
                .short("b")
                .value_name("SIZE")
                .validator(is_valid::<Size<u64>>)
                .help("Specifies the maximum number of bytes to write."),
        )
}

fn write_block(
    mut container: Container<DirectoryBackend>,
    id: Option<DirectoryId>,
    max_bytes: u64,
) -> Result<()> {
    let block_size = container.backend().block_size();
    let max_bytes = cmp::min(max_bytes as usize, block_size as usize);

    let id = match id {
        Some(id) => {
            debug!("use id from cmdline: {}", id);
            id
        }
        None => {
            let id = container.aquire()?;
            debug!("aquire new id: {}", id);
            id
        }
    };

    let mut buf = vec![0; max_bytes];
    let n = fill_buf(&mut buf)?;

    debug!("{} bytes read from stdin", n);

    container.write(&id, &buf[..n])?;

    println!("{} bytes written into {}", n, id);

    Ok(())
}

fn write_stream(
    mut container: Container<DirectoryBackend>,
    id: Option<DirectoryId>,
    max_bytes: u64,
) -> Result<()> {
    let id = id.map_or_else(|| container.aquire(), |id| Ok(id))?;
    let mut stream = Stream::create(container, id)?;
    let mut buf = [0; 1024];
    let mut num_bytes: usize = 0;

    while num_bytes < max_bytes as usize {
        let num_read = fill_buf(&mut buf)?;

        if num_read > 0 {
            debug!("{} bytes read from stdin", num_read);
            num_bytes += num_read;

            write_all(&mut stream, &buf[..num_read])?;
        } else {
            debug!("eof reached");
            stream.flush()?;
            break;
        }
    }

    println!(
        "{} bytes written into stream starting at {}",
        num_bytes,
        stream.first()
    );

    Ok(())
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let container = open_container(args)?;

    let id = match args.value_of("ID") {
        Some(s) => Some(s.parse()?),
        None => None,
    };

    let streaming = args.is_present("stream");
    let max_bytes = args
        .value_of("max-bytes")
        .map_or(u64::MAX, |s| *s.parse::<Size<u64>>().unwrap());

    debug!("id: {:?}", id);
    debug!("streaming: {}", streaming);
    debug!("max_bytes: {:?}", max_bytes);

    if streaming {
        write_stream(container, id, max_bytes)
    } else {
        write_block(container, id, max_bytes)
    }
}
