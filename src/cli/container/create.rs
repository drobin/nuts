// MIT License
//
// Copyright (c) 2023 Robin Doer
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
use clap::{value_parser, Args};
use log::debug;
use nuts_container::container::{Cipher, Container, CreateOptionsBuilder};
use nuts_directory::{CreateOptions, DirectoryBackend};

use crate::cli::container::{ask_for_password, container_dir_for, CliCipher, CliKdf, AES128_GCM};

#[derive(Args, Debug)]
pub struct ContainerCreateArgs {
    /// The  name of the new container
    name: String,

    /// Set the block-size to SIZE
    #[clap(short, long, id = "SIZE", default_value = "512")]
    block_size: u32,

    /// Sets the cipher to CIPHER.
    #[clap(short, long, value_parser = value_parser!(CliCipher), default_value = AES128_GCM)]
    cipher: CliCipher,

    /// Specifies the key derivation function.
    ///
    /// There are two ways to specify the KDF. The short form
    /// only specifies the algorithm name. The long form can
    /// customize the algorithm; it starts with the algorithm
    /// name followed by sections separated by a colon. A section
    /// can empty. In this case a default value is taken. The
    /// number of sections and its meaning depends on the
    /// algorithm.
    ///
    /// For PBKDF2: pbkdf2[:[<DIGEST>]:[<ITERATIONS>]:[<SALT_LENGTH>]]
    ///
    /// Selects PBKDF2 with the given digest (default: sha1),
    /// the given number of iterations (default: 65536) and salt
    /// length (default: 16).
    #[clap(short, long, value_parser)]
    kdf: Option<CliKdf>,

    /// If set, overwrites an existing container
    #[clap(short, long, value_parser)]
    overwrite: bool,
}

impl ContainerCreateArgs {
    pub fn run(&self) -> Result<()> {
        let path = container_dir_for(&self.name)?;

        debug!("name: {}", self.name);
        debug!("path: {}", path.display());
        debug!("bsize: {}", self.block_size);
        debug!("cipher: {:?}", *self.cipher);
        debug!("overwrite: {}", self.overwrite);

        let backend_options = CreateOptions::for_path(path)
            .with_bsize(self.block_size)
            .with_overwrite(self.overwrite);
        let mut builder =
            CreateOptionsBuilder::new(*self.cipher).with_password_callback(ask_for_password);

        if self.cipher != Cipher::None {
            if let Some(kdf) = self.kdf.as_ref() {
                debug!("kdf: {:?}", kdf);
                builder = builder.with_kdf(kdf.to_kdf()?);
            }
        }

        let options = builder.build::<DirectoryBackend>()?;
        Container::<DirectoryBackend>::create(backend_options, options)?;

        Ok(())
    }
}
