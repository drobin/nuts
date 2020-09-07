// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use nuts::container::Container;

use crate::tool;
use crate::tool::convert::Convert;
use crate::tool::format::Format;
use crate::tool::output::Output;

pub fn run(sub: &ArgMatches) -> tool::result::Result<()> {
    tool::logger::update(sub);

    let path = sub.value_of("PATH").unwrap();
    let mut container = Container::new();
    let mut userdata = Vec::new();

    container.set_password_callback(tool::utils::ask_for_password);
    container.open(path, Some(&mut userdata))?;

    if sub.is_present("userdata") {
        print_userdata(sub, &userdata)
    } else {
        print_info(sub, &container)
    }
}

fn print_info(sub: &ArgMatches, container: &Container) -> tool::result::Result<()> {
    let digest = container
        .digest()?
        .map_or_else(|| String::from("none"), |d| d.to_string());

    if !sub.is_present("quiet") {
        println!("cipher:           {}", container.cipher()?);
        println!("digest:           {}", digest);
        println!("disk type:        {}", container.dtype()?);
        println!("block size:       {}", container.bsize()?);
        println!("blocks:           {}", container.blocks()?);
        println!("allocated blocks: {}", container.ablocks()?);
    }

    Ok(())
}

fn print_userdata(sub: &ArgMatches, userdata: &Vec<u8>) -> tool::result::Result<()> {
    let format = match sub.value_of("format") {
        Some(format) => Format::from_str(format)?,
        None => Format::default(),
    };

    if !sub.is_present("quiet") {
        let mut output = Output::new(format);
        output.push(userdata).flush();
    }

    Ok(())
}
