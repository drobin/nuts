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

use crate::secret::Secret;
use crate::types::{DiskType, BLOCK_MIN_SIZE};

fn ok_secret() -> Secret {
    Secret {
        dtype: DiskType::FatRandom,
        bsize: BLOCK_MIN_SIZE,
        blocks: 4711,
        master_key: vec![1],
        master_iv: vec![2, 3],
        hmac_key: vec![4, 5, 6],
        userdata: vec![7, 8, 9, 10],
    }
}

#[test]
fn ok() {
    let mut secret = ok_secret();
    secret.zero();

    assert_eq!(secret.master_key, [0]);
    assert_eq!(secret.master_iv, [0, 0]);
    assert_eq!(secret.hmac_key, [0, 0, 0]);
    assert_eq!(secret.userdata, [7, 8, 9, 10]);
}
