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
use nuts_container::ModifyOptionsBuilder;
use rpassword::prompt_password;

use crate::cli::open_container;

pub fn ask_for_new_password() -> Result<Vec<u8>, String> {
    let pass1 = prompt_password("Enter a new password: ").map_err(|err| err.to_string())?;
    let pass2 =
        prompt_password("Enter a new password (repeat): ").map_err(|err| err.to_string())?;

    if pass1 == pass2 {
        Ok(pass1.as_bytes().to_vec())
    } else {
        Err("The passwords do not match".to_string())
    }
}

#[derive(Args, Debug)]
pub struct ContainerChangePasswordArgs {
    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ContainerChangePasswordArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let mut container = open_container(&self.container, self.verbose)?;
        let options = ModifyOptionsBuilder::default()
            .change_password(ask_for_new_password)
            .build();

        container.modify(options)?;

        Ok(())
    }
}
