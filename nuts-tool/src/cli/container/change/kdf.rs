// MIT License
//
// Copyright (c) 2024 Robin Doer
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
use clap::Args;
use log::debug;
use nuts_container::{Kdf, ModifyOptionsBuilder};

use crate::cli::open_container;

#[derive(Args, Debug)]
pub struct ContainerChangeKdfArgs {
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
    /// Selects PBKDF2 with the given digest (default: sha256),
    /// the given number of iterations (default: 65536) and salt
    /// length (default: 16).
    kdf: Kdf,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ContainerChangeKdfArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let mut container = open_container(&self.container)?;
        let options = ModifyOptionsBuilder::default()
            .change_kdf(self.kdf.clone())
            .build();

        container.modify(options)?;

        Ok(())
    }
}
