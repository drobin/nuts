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
use log::debug;
use nuts::container::{Cipher, Container, CreateOptionsBuilder};
use nuts::directory::{DirectoryBackend, DirectoryCreateOptions};

use crate::tool::actions::{is_valid, path_arg};
use crate::tool::convert::Convert;
use crate::tool::size::Size;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let cipher_help = "Set the cipher to CIPHER. Can be one of: none";

    app.about("Creates a nuts-container.")
        .arg(path_arg(1))
        .arg(
            Arg::with_name("block-size")
                .short("b")
                .long("block-size")
                .value_name("SIZE")
                .default_value("512")
                .validator(is_valid::<Size<u32>>)
                .help("Set the block-size to SIZE."),
        )
        .arg(
            Arg::with_name("cipher")
                .short("c")
                .long("cipher")
                .value_name("CIPHER")
                .default_value("none")
                .validator(is_valid::<Cipher>)
                .help(cipher_help),
        )
        .arg(
            Arg::with_name("overwrite")
                .long("overwrite")
                .help("If set, overwrites an existing container."),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let path = args.value_of("PATH").unwrap();
    let bsize = Size::<u32>::from_str(args.value_of("block-size").unwrap()).unwrap();
    let cipher = Cipher::from_str(args.value_of("cipher").unwrap()).unwrap();
    let overwrite = args.is_present("overwrite");

    debug!("path: {}", path);
    debug!("bsize: {}", *bsize);
    debug!("cipher: {:?}", cipher);
    debug!("overwrite: {}", overwrite);

    let backend_options = DirectoryCreateOptions::for_path(path)
        .with_bsize(*bsize)
        .with_overwrite(overwrite);
    let builder =
        CreateOptionsBuilder::<DirectoryBackend>::for_backend(backend_options).with_cipher(cipher);
    let options = builder.build()?;

    Container::create(options)?;

    Ok(())
}
