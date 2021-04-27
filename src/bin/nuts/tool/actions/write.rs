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
use std::io::{stdin, Read};

use crate::tool::convert::Convert;
use crate::tool::id::IdRange;
use crate::tool::logger;
use crate::tool::result::Result;
use crate::tool::utils;

pub fn run(sub: &ArgMatches) -> Result<()> {
    logger::update(sub);

    let path = sub.value_of("PATH").unwrap();
    let mut range = IdRange::from_str(sub.value_of("RANGE").unwrap())?;
    let mut container = Container::new();

    let max_bytes = match sub.value_of("max-bytes") {
        Some(s) => u64::from_str(s)?,
        None => u64::MAX,
    };

    container.set_password_callback(utils::ask_for_password);
    container.open(path, None)?;

    range.resolve(&container)?;
    debug!("range: {:?}", range);

    write(sub, &mut container, &range, max_bytes as usize)
}

fn write(
    sub: &ArgMatches,
    container: &mut Container,
    range: &IdRange,
    max_bytes: usize,
) -> Result<()> {
    let mut nbytes = 0;
    let mut blocks = 0;
    let mut iter = range.iter();

    let mut buf = vec![0; container.bsize()? as usize];

    while nbytes < max_bytes {
        let n = cmp::min(buf.len(), max_bytes - nbytes);
        let nread = stdin().read(&mut buf[..n])?;

        if nread == 0 {
            break;
        }

        nbytes += nread;

        match iter.next() {
            Some(id) => {
                container.write(id, &buf[..nread])?;
                blocks += 1;
            }
            None => break,
        }
    }

    if !sub.is_present("quiet") {
        println!("{} bytes written to {} block(s).", nbytes, blocks);
    }

    Ok(())
}
