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
use crate::tool::convert::{Convert, Size, WrappingKeySpec};

pub fn run(sub: &ArgMatches) -> tool::result::Result<()> {
    tool::logger::update(sub);

    let cipher = if let Some(cipher) = sub.value_of("cipher") {
        Cipher::from_str(cipher)?
    } else {
        Cipher::Aes128Ctr
    };

    let path = sub.value_of("PATH").unwrap();
    let mut options = Options::default_with_cipher(cipher)?;

    if let Some(bsize) = sub.value_of("block-size") {
        options.set_bsize(Size::<u32>::from_str(bsize)?.nbytes)?;
    }

    let size = Size::<u64>::from_str(sub.value_of("SIZE").unwrap())?.nbytes;
    options.set_size(size);

    if let Some(dtype) = sub.value_of("disk-type") {
        options.set_dtype(DiskType::from_str(dtype)?);
    }

    if let Some(wkey_data) = options.wkey() {
        if let Some(wkey_spec) = sub.value_of("wrapping-key") {
            let wkey_spec = WrappingKeySpec::from_str(wkey_spec)?;

            let WrappingKey::Pbkdf2 {
                iterations: default_iterations,
                salt: default_salt,
            } = wkey_data;

            let iterations = match wkey_spec.iterations {
                Some(n) => n,
                None => *default_iterations,
            };

            let salt_len = match wkey_spec.salt_len {
                Some(n) => n,
                None => default_salt.len() as u32,
            };

            let wkey = WrappingKey::generate_pbkdf2(iterations, salt_len)?;
            options.set_wkey(wkey);
        }
    };

    let mut container = Container::new();

    container.set_password_callback(tool::utils::ask_for_password);
    container.create(path, &options)?;

    if !sub.is_present("quiet") {
        println!("cipher:           {}", container.cipher()?.to_str());
        println!("digest:           {}", container.digest()?.to_str());
        match container.wrapping_key()? {
            Some(wkey) => {
                let spec = WrappingKeySpec::from_wrapping_key(&wkey);
                println!("wrapping key:     {}", spec.to_str());
            }
            None => println!("wrapping key:     none"),
        };
        println!("disk type:        {}", container.dtype()?.to_str());
        println!("block size:       {}", container.bsize()?);
        println!("blocks:           {}", container.blocks()?);
        println!("allocated blocks: {}", container.ablocks()?);
    }

    Ok(())
}
