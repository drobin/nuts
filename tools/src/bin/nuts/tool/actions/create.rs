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

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

use nuts::container::Container;
use nuts::result::Result;
use nuts::types::{Cipher, DiskType, Options, WrappingKey};

use crate::tool;

pub fn make<'a, 'b>() -> App<'a, 'b> {
    let general_args = tool::actions::general_args();

    let block_size_help = "Set the block-size to SIZE. Default is 512.";
    let cipher_help = "Set the cipher to CIPHER. Can be one of: \
        none, aes128-ctr. Default is aes128-ctr.";
    let disk_type_help = "Set the disk-type to DISK. \
        Can be one of: fat-zero, fat-random, thin-zero, thin-random. \
        Default is fat-random.";
    let iterations_help = "Sets the number of iterations of the PBKDF2 algorithm to N. \
        Default is 65536.";
    let salt_length_help = "Sets the length of the salt used by the PBKDF2 algorithm to N. \
        Default is 16.";
    let overwrite_help = "If set, overwrites an existing container.";

    SubCommand::with_name("create")
        .about("Creates a nuts-volume.")
        .version(crate_version!())
        .arg(Arg::with_name("PATH").required(true).index(1))
        .arg(
            Arg::with_name("SIZE")
                .required(true)
                .index(2)
                .validator(tool::contrib::clap::is_size::<u64>),
        )
        .args(&general_args)
        .arg(
            Arg::with_name("block-size")
                .short("b")
                .long("block-size")
                .value_name("SIZE")
                .validator(tool::contrib::clap::is_size::<u32>)
                .help(&block_size_help),
        )
        .arg(
            Arg::with_name("cipher")
                .short("c")
                .long("cipher")
                .value_name("CIPHER")
                .help(&cipher_help),
        )
        .arg(
            Arg::with_name("disk-type")
                .short("d")
                .long("disk-type")
                .value_name("DISK")
                .help(&disk_type_help),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .value_name("N")
                .help(&iterations_help),
        )
        .arg(
            Arg::with_name("salt-length")
                .short("s")
                .long("salt-length")
                .value_name("N")
                .help(&salt_length_help),
        )
        .arg(
            Arg::with_name("overwrite")
                .long("overwrite")
                .help(&overwrite_help),
        )
}

pub fn run(sub: &ArgMatches) -> Result<()> {
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

    container.set_password_callback(tool::utils::ask_for_password);
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
