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

#[macro_use]
extern crate lazy_static;

mod tool;

use clap::{crate_name, crate_version};
use clap::{App, AppSettings, Arg, SubCommand};
use nuts::types::{Cipher, DiskType, Options, WrappingKey};

use crate::tool::actions::general_args;
use crate::tool::contrib::clap::is_valid;
use crate::tool::convert::{Convert, Size};
use crate::tool::format::Format;
use crate::tool::id::IdRange;
use crate::tool::result::{Error, Result};

fn main() {
    std::process::exit(match run_tool() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err.get_message());
            1
        }
    })
}

fn run_tool() -> Result<()> {
    tool::logger::init();

    let options = Options::default()?; // for defaults

    let cipher_list = [Cipher::Aes128Ctr, Cipher::None]
        .iter()
        .map(|c| c.to_str())
        .collect::<Vec<String>>();
    let dtype_list = [
        DiskType::FatRandom,
        DiskType::FatZero,
        DiskType::ThinRandom,
        DiskType::ThinZero,
    ]
    .iter()
    .map(|d| d.to_str())
    .collect::<Vec<String>>();
    let format_list = [Format::String, Format::Hex]
        .iter()
        .map(|f| f.to_str())
        .collect::<Vec<String>>();

    let block_size_help = format!(
        "Set the block-size to SIZE. Default is {}.",
        options.bsize()
    );
    let cipher_help = format!(
        "Set the cipher to CIPHER. Can be one of: {}. Default is {}.",
        cipher_list.join(", "),
        options.cipher.to_str()
    );
    let disk_type_help = format!(
        "Set the disk-type to DISK. Can be one of: {}. Default is {}.",
        dtype_list.join(", "),
        options.dtype.to_str()
    );
    let format_info_help = format!(
        "Specifies the format of the userdata dump. Can be one of {}. Default is {}.",
        format_list.join(", "),
        Format::default().to_str()
    );
    let format_read_help = format!(
        "Specifies the format of the dump. Can be one of {}. Default is {}.",
        format_list.join(", "),
        Format::default().to_str()
    );
    let iterations_help = {
        let iterations = match options.wkey {
            Some(WrappingKey::Pbkdf2 {
                iterations,
                salt: _,
            }) => iterations,
            None => 0,
        };
        format!(
            "Sets the number of iterations of the PBKDF2 algorithm to N. Default is {}.",
            iterations
        )
    };
    let max_bytes_read_help = "Reads up to SIZE bytes. Default is unlimited.";
    let max_bytes_write_help = "Writes up to SIZE bytes. Default is unlimited.";
    let overwrite_help = "If set, overwrites an existing container.";
    let path_help = "The path to the container.";
    let range_read_help = "Range of block ids to read.";
    let range_write_help = "Range of block ids to write.";
    let salt_length_help = {
        let salt_len = match options.wkey {
            Some(WrappingKey::Pbkdf2 {
                iterations: _,
                salt,
            }) => salt.len(),
            None => 0,
        };
        format!(
            "Sets the length of the salt used by the PBKDF2 algorithm to N. Default is {}.",
            salt_len
        )
    };
    let userdata_help = "If set, dumps the userdata stored in the header.";

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("info")
                .about("Shows information about a nuts-volume.")
                .version(crate_version!())
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .index(1)
                        .help(path_help),
                )
                .args(&general_args())
                .arg(
                    Arg::with_name("userdata")
                        .long("userdata")
                        .help(userdata_help),
                )
                .arg(
                    Arg::with_name("format")
                        .long("format")
                        .short("f")
                        .value_name("FORMAT")
                        .validator(is_valid::<Format>)
                        .help(&format_info_help),
                ),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a nuts-volume.")
                .version(crate_version!())
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .index(1)
                        .help(path_help),
                )
                .arg(
                    Arg::with_name("SIZE")
                        .required(true)
                        .index(2)
                        .validator(is_valid::<Size<u64>>),
                )
                .args(&general_args())
                .arg(
                    Arg::with_name("block-size")
                        .short("b")
                        .long("block-size")
                        .value_name("SIZE")
                        .validator(is_valid::<Size<u32>>)
                        .help(&block_size_help),
                )
                .arg(
                    Arg::with_name("cipher")
                        .short("c")
                        .long("cipher")
                        .value_name("CIPHER")
                        .validator(is_valid::<Cipher>)
                        .help(&cipher_help),
                )
                .arg(
                    Arg::with_name("disk-type")
                        .short("d")
                        .long("disk-type")
                        .value_name("DISK")
                        .validator(is_valid::<DiskType>)
                        .help(&disk_type_help),
                )
                .arg(
                    Arg::with_name("iterations")
                        .short("i")
                        .long("iterations")
                        .value_name("N")
                        .validator(is_valid::<u32>)
                        .help(&iterations_help),
                )
                .arg(
                    Arg::with_name("salt-length")
                        .short("s")
                        .long("salt-length")
                        .value_name("N")
                        .validator(is_valid::<u32>)
                        .help(&salt_length_help),
                )
                .arg(
                    Arg::with_name("overwrite")
                        .long("overwrite")
                        .help(&overwrite_help),
                ),
        )
        .subcommand(
            SubCommand::with_name("read")
                .about("Reads data from a nuts-volume.")
                .version(crate_version!())
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .index(1)
                        .help(path_help),
                )
                .arg(
                    Arg::with_name("RANGE")
                        .required(true)
                        .index(2)
                        .validator(is_valid::<IdRange>)
                        .help(range_read_help),
                )
                .args(&general_args())
                .arg(
                    Arg::with_name("format")
                        .long("format")
                        .short("f")
                        .value_name("FORMAT")
                        .validator(is_valid::<Format>)
                        .help(&format_read_help),
                )
                .arg(
                    Arg::with_name("max-bytes")
                        .long("max-bytes")
                        .short("m")
                        .value_name("SIZE")
                        .validator(is_valid::<Size<u64>>)
                        .help(max_bytes_read_help),
                ),
        )
        .subcommand(
            SubCommand::with_name("write")
                .about("Writes data info a nuts-volume.")
                .version(crate_version!())
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .index(1)
                        .help(path_help),
                )
                .arg(
                    Arg::with_name("RANGE")
                        .required(true)
                        .index(2)
                        .validator(is_valid::<IdRange>)
                        .help(range_write_help),
                )
                .args(&general_args())
                .arg(
                    Arg::with_name("max-bytes")
                        .long("max-bytes")
                        .short("m")
                        .value_name("SIZE")
                        .validator(is_valid::<Size<u64>>)
                        .help(max_bytes_write_help),
                ),
        )
        .get_matches();

    if let Some(sub) = matches.subcommand_matches("info") {
        tool::actions::info::run(sub)
    } else if let Some(sub) = matches.subcommand_matches("create") {
        tool::actions::create::run(sub)
    } else if let Some(sub) = matches.subcommand_matches("read") {
        tool::actions::read::run(sub)
    } else if let Some(sub) = matches.subcommand_matches("write") {
        tool::actions::write::run(sub)
    } else {
        let msg = String::from("Missing implementation for subcommand");
        Err(Error::new(&msg))
    }
}
