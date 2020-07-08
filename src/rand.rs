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

use crate::error::Error;
use crate::result::Result;

#[cfg(not(test))]
pub fn random(target: &mut [u8]) -> Result<()> {
    ::openssl::rand::rand_bytes(target)
        .map(|_| ())
        .or_else(|err| {
            let msg = format!("{}", err);
            Err(Error::Rand(msg))
        })
}

#[cfg(test)]
pub const RND: [u8; 16] = [
    33, 155, 95, 60, 65, 96, 253, 183, 93, 150, 39, 110, 253, 132, 24, 187,
];

#[cfg(test)]
pub fn random(target: &mut [u8]) -> Result<()> {
    assert!(target.len() <= RND.len());
    target.clone_from_slice(&RND[..target.len()]);
    Ok(())
}
