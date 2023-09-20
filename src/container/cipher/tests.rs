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

mod aes128_ctr;
mod aes128_gcm;
mod none;
mod serde;
mod string;

const KEY: [u8; 16] = [b'x'; 16];
const IV: [u8; 16] = [b'y'; 16];

macro_rules! cipher_test {
    ($name:ident, $cipher:ident . $method:ident, [ $($input:literal),* ], $num:literal, [ $($expected:literal),* ]) => {
        #[test]
        fn $name() {
            use crate::container::cipher::tests::{KEY, IV};

            let input = [$($input),*];
            let mut output = Vec::new();

            let n = Cipher::$cipher.$method(&input, &mut output, &KEY, &IV).unwrap();

            assert_eq!(n, $num);
            assert_eq!(output, [$($expected),*]);
        }
    };
}

pub(crate) use cipher_test;
