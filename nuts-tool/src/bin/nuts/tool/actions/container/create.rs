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
use log::debug;
use nuts::container::{Cipher, Container, CreateOptionsBuilder, Digest, Kdf};
use nuts::directory::{DirectoryBackend, DirectoryCreateOptions};

use crate::tool::actions::{container_dir_for, is_valid, name_arg};
use crate::tool::convert::Convert;
use crate::tool::kdf::KdfSpec;
use crate::tool::password::ask_for_password;
use crate::tool::size::Size;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let cipher_help = "Set the cipher to CIPHER. Can be one of: none";
    let kdf_help = "Specifies the key derivation function. The default is \
                    pbkdf2.\n\n\
                    There are two ways to specify the KDF. The short form \
                    only specifies the algorithm name. The long form can \
                    customize the algorithm; it starts with the algorithm \
                    name followed by sections separated by a colon. A section \
                    can empty. In this case a default value is taken. The \
                    number of sections and its meaning depends on the \
                    algorithm.\n\n\
                    Algorithm: PBKDF2\n\
                    Value: pbkdf2[:[<DIGEST>]:[<ITERATIONS>]:[<SALT_LENGTH>]] - \
                    Selects PBKDF2 with the given digest (default: sha1), \
                    the given number of iterations (default: 65536) and salt \
                    length (default: 16).";

    app.about("Creates a nuts-container.")
        .arg(name_arg(1))
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
            Arg::with_name("kdf")
                .short("k")
                .long("kdf")
                .value_name("SPEC")
                .validator(is_valid::<KdfSpec>)
                .help(&kdf_help),
        )
        .arg(
            Arg::with_name("overwrite")
                .long("overwrite")
                .help("If set, overwrites an existing container."),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let name = args.value_of("NAME").unwrap();
    let bsize = Size::<u32>::from_str(args.value_of("block-size").unwrap()).unwrap();
    let cipher = Cipher::from_str(args.value_of("cipher").unwrap()).unwrap();
    let overwrite = args.is_present("overwrite");

    let path = container_dir_for(name)?;

    debug!("name: {}", name);
    debug!("path: {}", path.display());
    debug!("bsize: {}", *bsize);
    debug!("cipher: {:?}", cipher);
    debug!("overwrite: {}", overwrite);

    let backend_options = DirectoryCreateOptions::for_path(path)
        .with_bsize(*bsize)
        .with_overwrite(overwrite);
    let mut builder = CreateOptionsBuilder::<DirectoryBackend>::new(backend_options, cipher)
        .with_password_callback(ask_for_password);

    if cipher != Cipher::None {
        let kdf = create_kdf(args)?;
        debug!("kdf: {:?}", kdf);

        builder = builder.with_kdf(kdf);
    }

    let options = builder.build()?;

    Container::create(options)?;

    Ok(())
}

fn create_kdf(args: &ArgMatches) -> Result<Kdf> {
    let default_digest = Digest::Sha1;
    let default_iterations = 65536;
    let default_salt_len = 16;

    let (digest, iterations, salt_len) = match args.value_of("kdf") {
        Some(s) => {
            let spec = KdfSpec::from_str(s).unwrap();
            let digest = spec.digest.unwrap_or(default_digest);
            let iterations = spec.iterations.unwrap_or(default_iterations);
            let salt_len = spec.salt_len.unwrap_or(default_salt_len);

            (digest, iterations, salt_len)
        }
        None => (default_digest, default_iterations, default_salt_len),
    };

    Ok(Kdf::generate_pbkdf2::<DirectoryBackend>(
        digest, iterations, salt_len,
    )?)
}
