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

use clap::ArgMatches;

use nuts::container::Container;
use nuts::types::{Cipher, DiskType, Options, WrappingKey};

use crate::tool;

pub fn run(sub: &ArgMatches) -> tool::result::Result<()> {
    tool::logger::update(sub);

    let cipher = if let Some(cipher) = sub.value_of("cipher") {
        Cipher::from_string(cipher)?
    } else {
        Cipher::Aes128Ctr
    };

    let path = sub.value_of("PATH").unwrap();
    let size = tool::utils::to_size::<u64>(sub.value_of("SIZE").unwrap())?;
    let mut options = Options::default_with_cipher(cipher)?;

    let bsize = match sub.value_of("block-size") {
        Some(bsize) => tool::utils::to_size::<u32>(bsize)?,
        None => options.bsize(),
    };
    let blocks = size / bsize as u64;

    options.update_sizes(bsize, blocks)?;

    if let Some(dtype) = sub.value_of("disk-type") {
        options.dtype = DiskType::from_string(dtype)?;
    }

    if let Some(wkey_data) = options.wkey.as_ref() {
        let WrappingKey::Pbkdf2 {
            iterations: default_iterations,
            salt: default_salt,
        } = wkey_data;

        let iterations = match sub.value_of("iterations") {
            Some(s) => s.parse::<u32>()?,
            None => *default_iterations,
        };

        let salt_len = match sub.value_of("salt-length") {
            Some(s) => s.parse::<u32>()?,
            None => default_salt.len() as u32,
        };

        options.wkey = Some(WrappingKey::generate_pbkdf2(iterations, salt_len)?);
    };

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
