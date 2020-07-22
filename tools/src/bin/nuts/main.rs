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

mod tool;

use clap::{crate_name, crate_version};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use rpassword::read_password_from_tty;

use nuts::container::Container;
use nuts::result::Result;
use nuts::types::{Cipher, DiskType, Options, WrappingKey};

fn ask_for_password() -> Result<Vec<u8>> {
    let password = read_password_from_tty(Some("Enter a password:"))?;
    Ok(password.as_bytes().to_vec())
}

fn main() -> Result<()> {
    tool::logger::init();

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
    tool::logger::update(sub);

    let path = sub.value_of("PATH").unwrap();
    let mut container = Container::new();

    container.set_password_callback(ask_for_password);
    container.open(path, None)?;

    let digest = container
        .digest()?
        .map_or_else(|| String::from("none"), |d| d.to_string());

    say!(sub, "cipher:           {}", container.cipher()?);
    say!(sub, "digest:           {}", digest);
    say!(sub, "disk type:        {}", container.dtype()?);
    say!(sub, "block size:       {}", container.bsize()?);
    say!(sub, "blocks:           {}", container.blocks()?);
    say!(sub, "allocated blocks: {}", container.ablocks()?);

    Ok(())
}

fn create(sub: &ArgMatches) -> Result<()> {
    tool::logger::update(sub);

    let cipher = if let Some(cipher) = sub.value_of("cipher") {
        Cipher::from_string(cipher)?
    } else {
        Cipher::Aes128Ctr
    };

    let path = sub.value_of("PATH").unwrap();
    let size = tool::utils::to_size::<u64>(sub.value_of("SIZE").unwrap()).unwrap();
    let mut options = Options::default_with_cipher(cipher);

    let bsize = match sub.value_of("block-size") {
        Some(bsize) => tool::utils::to_size::<u32>(bsize).unwrap(),
        None => options.bsize(),
    };
    let blocks = size / bsize as u64;

    options.update_sizes(bsize, blocks)?;

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

    let mut container = Container::new();

    container.set_password_callback(ask_for_password);
    container.create(path, &options)?;

    let digest = container
        .digest()?
        .map_or_else(|| String::from("none"), |d| d.to_string());

    say!(sub, "cipher:           {}", container.cipher()?);
    say!(sub, "digest:           {}", digest);
    say!(sub, "disk type:        {}", container.dtype()?);
    say!(sub, "block size:       {}", container.bsize()?);
    say!(sub, "blocks:           {}", container.blocks()?);
    say!(sub, "allocated blocks: {}", container.ablocks()?);

    Ok(())
}
