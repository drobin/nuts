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

use std::fmt;

use crate::types::WrappingKey;

#[derive(PartialEq)]
pub struct WrappingKeyData {
    pub wkey: WrappingKey,
    pub pbkdf2: Option<Vec<u8>>,
}

impl WrappingKeyData {
    pub fn pbkdf2(iterations: u32, salt: &Vec<u8>) -> WrappingKeyData {
        let wkey = WrappingKey::Pbkdf2 {
            iterations,
            salt_len: salt.len() as u32,
        };
        WrappingKeyData {
            wkey,
            pbkdf2: Some(salt.to_vec()),
        }
    }
}

impl fmt::Debug for WrappingKeyData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let pbkdf2 = self.pbkdf2.as_ref().map_or_else(
            || None,
            |p| {
                let s = format!("<{} bytes>", p.len());
                Some(s)
            },
        );

        fmt.debug_struct("WrappingKeyData")
            .field("wkey", &self.wkey)
            .field("pbkdf2", &pbkdf2)
            .finish()
    }
}
