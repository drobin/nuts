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

mod aes128_ctr;
mod aes128_gcm;
mod bytes;
mod none;
mod string;

const KEY: [u8; 16] = [b'x'; 16];
const IV: [u8; 16] = [b'y'; 16];

macro_rules! ctx_test {
    ($name:ident, $cipher:ident . $method:ident, $num:literal, [ $($input:literal),* ] -> [ $($expected:literal),* ]) => {
        #[test]
        fn $name() {
            use crate::cipher::tests::{IV, KEY};
            use crate::cipher::CipherContext;

            let input = [$($input),*];
            let expected = [$($expected),*];

            let mut ctx = CipherContext::new(Cipher::$cipher);

            ctx.copy_from_slice($num, &input);

            let output = ctx.$method(&KEY, &IV).unwrap();

            assert_eq!(output, expected);
        }
    };
}

pub(crate) use ctx_test;
