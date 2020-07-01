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
extern crate log;
extern crate nuts;

use clap::{crate_name, crate_version};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use log::LevelFilter;

use nuts::container::Container;
use nuts::result::Result;
use nuts::types::{Cipher, DiskType, Options, WrappingKey};

struct NutsLogger;

impl log::Log for NutsLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

static LOGGER: NutsLogger = NutsLogger;

pub fn init_logger() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
}

pub fn update_logger(sub: &ArgMatches) {
    if sub.is_present("quiet") {
        log::set_max_level(LevelFilter::Off);
    } else if sub.is_present("verbose") {
        log::set_max_level(LevelFilter::Trace);
    }
}

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
    init_logger();

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
    update_logger(sub);

    let path = sub.value_of("PATH").unwrap();
    let container = Container::open(path)?;
    let digest = container
        .digest()
        .map_or_else(|| String::from("none"), |d| d.to_string());

    say!(sub, "cipher:           {}", container.cipher());
    say!(sub, "digest:           {}", digest);
    say!(sub, "disk type:        {}", container.dtype());
    say!(sub, "block size:       {}", container.bsize());
    say!(sub, "blocks:           {}", container.blocks());
    say!(sub, "allocated blocks: {}", container.ablocks());

    Ok(())
}

fn create(sub: &ArgMatches) -> Result<()> {
    update_logger(sub);

    let cipher = if let Some(cipher) = sub.value_of("cipher") {
        Cipher::from_string(cipher)?
    } else {
        Cipher::Aes128Ctr
    };

    let path = sub.value_of("PATH").unwrap();
    let mut options = Options::default_with_cipher(cipher);

    if let Some(dtype) = sub.value_of("disk-type") {
        options.dtype = DiskType::from_string(dtype)?;
    }

    if let Some(iterations) = sub.value_of("iterations") {
        match options.wkey {
            Some(WrappingKey::Pbkdf2 {
                iterations: _,
                salt_len,
            }) => {
                let iterations = iterations.parse::<u32>().unwrap();
                options.wkey = Some(WrappingKey::Pbkdf2 {
                    iterations,
                    salt_len,
                });
            }
            None => {
                panic!("unexpected wrapping key");
            }
        }
    }

    if let Some(salt_len) = sub.value_of("salt-length") {
        match options.wkey {
            Some(WrappingKey::Pbkdf2 {
                iterations,
                salt_len: _,
            }) => {
                let salt_len = salt_len.parse::<u32>().unwrap();
                options.wkey = Some(WrappingKey::Pbkdf2 {
                    iterations,
                    salt_len,
                });
            }
            None => {
                panic!("unexpected wrapping key");
            }
        }
    }

    let container = Container::create(path, &options)?;
    let digest = container
        .digest()
        .map_or_else(|| String::from("none"), |d| d.to_string());

    say!(sub, "cipher:           {}", container.cipher());
    say!(sub, "digest:           {}", digest);
    say!(sub, "disk type:        {}", container.dtype());
    say!(sub, "block size:       {}", container.bsize());
    say!(sub, "blocks:           {}", container.blocks());
    say!(sub, "allocated blocks: {}", container.ablocks());

    Ok(())
}
