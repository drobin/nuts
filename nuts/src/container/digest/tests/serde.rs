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

use nuts_bytes::{BufferSource, Reader, VecTarget, Writer};
use serde::{Deserialize, Serialize};

use crate::container::Digest;

#[test]
fn de_sha1() {
    let mut reader = Reader::new(BufferSource::new(&[0x00, 0x00, 0x00, 0x00]));
    assert_eq!(Digest::deserialize(&mut reader).unwrap(), Digest::Sha1);
}

#[test]
fn ser_sha1() {
    let mut writer = Writer::new(VecTarget::new(vec![]));
    assert_eq!(Digest::Sha1.serialize(&mut writer).unwrap(), 4);
    assert_eq!(writer.into_target().into_vec(), [0x00, 0x00, 0x00, 0x00]);
}
