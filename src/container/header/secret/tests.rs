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

mod plain_secret;
mod secret;

use crate::container::header::secret::{Magics, PlainSecret};
use crate::memory::{MemSettings, MemoryBackend};

// key: AE 18 FF 41 77 79 0F 07 AB 11 E2 F1 8C 87 AD 9A
// iv: 01010101010101010101010101010101
const SECRET: [u8; 34] = [
    0x5c, 0x68, 0x30, 0x8f, 0x47, 0x19, 0xf4, 0x76, 0xf2, 0x72, 0xbc, 0x06, 0x1c, 0xf3, 0x58, 0xca,
    0x54, 0x2c, 0xca, 0xf8, 0xe6, 0x7d, 0xe1, 0xfb, 0xb4, 0xe1, 0x1c, 0xbe, 0xb7, 0x82, 0x54, 0x3b,
    0xee, 0x16,
];

const PLAIN_SECRET: [u8; 34] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    0, 0, 0, 0, 0, 0, 0, 2, 1, 2, // key
    0, 0, 0, 0, 0, 0, 0, 3, 3, 4, 5, // iv
    0x01, 0x00, 0x00, 0x02, 0x9A, // Some(top-id)
          // settings (empty)
];

fn plain_secret() -> PlainSecret<MemoryBackend> {
    PlainSecret {
        magics: Magics([4711, 4711]),
        key: vec![1, 2].into(),
        iv: vec![3, 4, 5].into(),
        top_id: Some("666".parse().unwrap()),
        settings: MemSettings(),
    }
}
