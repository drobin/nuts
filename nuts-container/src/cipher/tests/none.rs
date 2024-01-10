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

use crate::cipher::Cipher;

use super::ctx_test;

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

ctx_test!(ctx_decrypt_3_1, None.decrypt, 3, [1, 2, 3] -> [1, 2, 3]);
ctx_test!(ctx_decrypt_3_2, None.decrypt, 2, [1, 2, 3] -> [1, 2]);
ctx_test!(ctx_decrypt_3_3, None.decrypt, 4, [1, 2, 3] -> [1, 2, 3, 0]);
ctx_test!(ctx_decrypt_2_1, None.decrypt, 2, [1, 2] -> [1, 2]);
ctx_test!(ctx_decrypt_2_2, None.decrypt, 1, [1, 2] -> [1]);
ctx_test!(ctx_decrypt_2_3, None.decrypt, 3, [1, 2] -> [1, 2, 0]);
ctx_test!(ctx_decrypt_1_1, None.decrypt, 1, [1] -> [1]);
ctx_test!(ctx_decrypt_1_2, None.decrypt, 0, [1] -> []);
ctx_test!(ctx_decrypt_1_3, None.decrypt, 2, [1] -> [1, 0]);
ctx_test!(ctx_decrypt_0_1, None.decrypt, 0, [] -> []);
ctx_test!(ctx_decrypt_0_2, None.decrypt, 1, [] -> [0]);

ctx_test!(ctx_encrypt_3_1, None.encrypt, 3, [1, 2, 3] -> [1, 2, 3]);
ctx_test!(ctx_encrypt_3_2, None.encrypt, 2, [1, 2, 3] -> [1, 2]);
ctx_test!(ctx_encrypt_3_3, None.encrypt, 4, [1, 2, 3] -> [1, 2, 3, 0]);
ctx_test!(ctx_encrypt_2_1, None.encrypt, 2, [1, 2] -> [1, 2]);
ctx_test!(ctx_encrypt_2_2, None.encrypt, 1, [1, 2] -> [1]);
ctx_test!(ctx_encrypt_2_3, None.encrypt, 3, [1, 2] -> [1, 2, 0]);
ctx_test!(ctx_encrypt_1_1, None.encrypt, 1, [1] -> [1]);
ctx_test!(ctx_encrypt_1_2, None.encrypt, 0, [1] -> []);
ctx_test!(ctx_encrypt_1_3, None.encrypt, 2, [1] -> [1, 0]);
ctx_test!(ctx_encrypt_0_1, None.encrypt, 0, [] -> []);
ctx_test!(ctx_encrypt_0_2, None.encrypt, 1, [] -> [0]);
