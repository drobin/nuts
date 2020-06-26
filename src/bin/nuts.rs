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
use nuts::types::{Cipher, DiskType, Options, WrappingKey};

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
    let mut options = Options::default();

    if let Some(dtype) = sub.value_of("disk-type") {
        options.dtype = DiskType::from_string(dtype)?;
    }

    if let Some(iterations) = sub.value_of("iterations") {
        let WrappingKey::Pbkdf2 {
            iterations: _,
            salt_len,
        } = options.wkey;
        let iterations = iterations.parse::<u32>().unwrap();
        options.wkey = WrappingKey::Pbkdf2 {
            iterations,
            salt_len,
        };
    }

    if let Some(salt_len) = sub.value_of("salt-length") {
        let WrappingKey::Pbkdf2 {
            iterations,
            salt_len: _,
        } = options.wkey;
        let salt_len = salt_len.parse::<u32>().unwrap();
        options.wkey = WrappingKey::Pbkdf2 {
            iterations,
            salt_len,
        };
    }

    if let Some(cipher) = sub.value_of("cipher") {
        options.cipher = Cipher::from_string(cipher)?;
    }

    let container = Container::create(path, &options)?;

    say!(sub, "cipher:           {}", container.cipher());
    say!(sub, "digest:           {}", container.digest());
    say!(sub, "disk type:        {}", container.dtype());
    say!(sub, "block size:       {}", container.bsize());
    say!(sub, "blocks:           {}", container.blocks());
    say!(sub, "allocated blocks: {}", container.ablocks());

    Ok(())
}
