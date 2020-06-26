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

extern crate clap;
extern crate nuts;

use clap::{crate_name, crate_version};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use nuts::container::Container;
use nuts::result::Result;
use nuts::types::Options;

macro_rules! say {
    ($sub:expr) => {
        if !$sub.is_present("quiet") {
            println!();
        }
    };
    ($sub:expr, $arg:expr) => {
        if !$sub.is_present("quiet") {
            println!($arg);
        }
    };
    ($sub:expr $(,$arg:expr)*) => {
        if !$sub.is_present("quiet") {
            println!($($arg,)*);
        }
    };
}

fn main() -> Result<()> {
    let info_command = include!("info.sub");
    let create_command = include!("create.sub");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(info_command)
        .subcommand(create_command)
        .get_matches();

    if let Some(sub) = matches.subcommand_matches("info") {
        info(sub)
    } else if let Some(sub) = matches.subcommand_matches("create") {
        create(sub)
    } else {
        Ok(())
    }
}

fn info(sub: &ArgMatches) -> Result<()> {
    let path = sub.value_of("PATH").unwrap();
    let container = Container::open(path)?;

    say!(sub, "cipher:           {}", container.cipher());
    say!(sub, "digest:           {}", container.digest());
    say!(sub, "disk type:        {}", container.dtype());
    say!(sub, "block size:       {}", container.bsize());
    say!(sub, "blocks:           {}", container.blocks());
    say!(sub, "allocated blocks: {}", container.ablocks());

    Ok(())
}

fn create(sub: &ArgMatches) -> Result<()> {
    let path = sub.value_of("PATH").unwrap();
    let options = Options::default();
    let container = Container::create(path, &options)?;

    say!(sub, "cipher:           {}", container.cipher());
    say!(sub, "digest:           {}", container.digest());
    say!(sub, "disk type:        {}", container.dtype());
    say!(sub, "block size:       {}", container.bsize());
    say!(sub, "blocks:           {}", container.blocks());
    say!(sub, "allocated blocks: {}", container.ablocks());

    Ok(())
}
