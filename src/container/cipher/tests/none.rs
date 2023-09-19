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

use crate::container::cipher::Cipher;

use super::cipher_test;

#[test]
fn block_size() {
    assert_eq!(Cipher::None.block_size(), 1);
}

#[test]
fn key_len() {
    assert_eq!(Cipher::None.key_len(), 0);
}

#[test]
fn iv_len() {
    assert_eq!(Cipher::None.iv_len(), 0);
}

#[test]
fn tag_size() {
    assert_eq!(Cipher::None.tag_size(), 0);
}

cipher_test!(decrypt_1, None.decrypt, [1, 2, 3], 3, [1, 2, 3]);
cipher_test!(decrypt_2, None.decrypt, [1, 2], 2, [1, 2]);
cipher_test!(decrypt_3, None.decrypt, [1], 1, [1]);
cipher_test!(decrypt_4, None.decrypt, [], 0, []);

cipher_test!(encrypt_1, None.encrypt, [1, 2, 3], 3, [1, 2, 3]);
cipher_test!(encrypt_2, None.encrypt, [1, 2], 2, [1, 2]);
cipher_test!(encrypt_3, None.encrypt, [1], 1, [1]);
cipher_test!(encrypt_4, None.encrypt, [], 0, []);
