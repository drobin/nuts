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

use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use log::debug;
use nuts::container::{Cipher, Container, CreateOptionsBuilder, Digest, Kdf};
use nuts_backend::plugin::locate_backend;

use crate::tool::actions::{container_dir_for, is_valid, name_arg};
use crate::tool::backend::{ProxyBackend, ProxyCreateOptions};
use crate::tool::config::{Config, ContainerConfig};
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
            Arg::with_name("backend")
                .long("backend")
                .value_name("BACKEND")
                .help("Selects BACKEND as backend."),
        )
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
    let backend = args
        .value_of("backend")
        .ok_or_else(|| anyhow!("Missing value for --backend"))?;
    let name = args.value_of("NAME").unwrap();
    let bsize = Size::<u32>::from_str(args.value_of("block-size").unwrap()).unwrap();
    let cipher = Cipher::from_str(args.value_of("cipher").unwrap()).unwrap();
    let overwrite = args.is_present("overwrite");

    let path = container_dir_for(name)?;

    debug!("backend: {}", backend);
    debug!("name: {}", name);
    debug!("path: {}", path.display());
    debug!("bsize: {}", *bsize);
    debug!("cipher: {:?}", cipher);
    debug!("overwrite: {}", overwrite);

    let config = Config::parse()?;
    let loader = locate_backend(backend, &config.search_path)?;

    let backend_options = ProxyCreateOptions::new(loader, path);
    let mut builder = CreateOptionsBuilder::<ProxyBackend>::new(backend_options, cipher)
        .with_password_callback(ask_for_password);

    if cipher != Cipher::None {
        let kdf = create_kdf(args)?;
        debug!("kdf: {:?}", kdf);

        builder = builder.with_kdf(kdf);
    }

    let options = builder.build()?;

    Container::create(options)?;
    ContainerConfig::put_backend(name, backend)?;

    Ok(())
}

fn create_pbkdf2(
    digest: Option<Digest>,
    iterations: Option<u32>,
    salt_len: Option<u32>,
) -> Result<Kdf> {
    const DEFAULT_DIGEST: Digest = Digest::Sha1;
    const DEFAULT_ITERATIONS: u32 = 65536;
    const DEFAULT_SALT_LEN: u32 = 16;

    let digest = digest.unwrap_or(DEFAULT_DIGEST);
    let iterations = iterations.unwrap_or(DEFAULT_ITERATIONS);
    let salt_len = salt_len.unwrap_or(DEFAULT_SALT_LEN);

    Ok(Kdf::generate_pbkdf2::<ProxyBackend>(
        digest, iterations, salt_len,
    )?)
}

fn create_kdf(args: &ArgMatches) -> Result<Kdf> {
    let kdf = match args.value_of("kdf") {
        Some(s) => {
            let spec = KdfSpec::from_str(s).unwrap();

            match spec {
                KdfSpec::None => todo!(),
                KdfSpec::Pbkdf2 {
                    digest,
                    iterations,
                    salt_len,
                } => create_pbkdf2(digest, iterations, salt_len)?,
            }
        }
        None => create_pbkdf2(None, None, None)?,
    };

    Ok(kdf)
}
