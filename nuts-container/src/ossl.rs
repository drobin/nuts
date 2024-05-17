// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

use openssl::error::ErrorStack;

#[cfg(test)]
pub use test::rand_bytes;

#[cfg(not(test))]
pub use production::rand_bytes;

pub fn rand_u32() -> Result<u32, ErrorStack> {
    let mut bytes = [0; 4];

    rand_bytes(&mut bytes)?;

    Ok(u32::from_be_bytes(bytes))
}

#[cfg(not(test))]
mod production {
    use openssl::error::ErrorStack;

    pub fn rand_bytes(buf: &mut [u8]) -> Result<(), ErrorStack> {
        openssl::rand::rand_bytes(buf)
    }
}

#[cfg(test)]
mod test {
    use openssl::error::ErrorStack;

    use crate::tests::RND;

    pub fn rand_bytes(buf: &mut [u8]) -> Result<(), ErrorStack> {
        assert!(buf.len() <= RND.len());
        buf.clone_from_slice(&RND[..buf.len()]);
        Ok(())
    }
}
