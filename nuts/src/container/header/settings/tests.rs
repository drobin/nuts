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

use crate::bytes::Options;
use crate::container::header::settings::Settings;
use crate::memory::{MemSettings, MemoryBackend};

#[test]
fn from_backend() {
    let settings = Settings::from_backend::<MemoryBackend>(&MemSettings()).unwrap();
    assert_eq!(settings, []);
}

#[test]
fn into_backend() {
    let _settings = Settings(vec![].into())
        .into_backend::<MemoryBackend>()
        .unwrap();
}

#[test]
fn ser_empty() {
    let settings = Settings(vec![].into());
    let vec = Options::new().to_vec(&settings).unwrap();
    assert_eq!(vec, [0]);
}

#[test]
fn ser() {
    let settings = Settings(vec![1, 2, 3].into());
    let vec = Options::new().to_vec(&settings).unwrap();
    assert_eq!(vec, [3, 1, 2, 3]);
}

#[test]
fn de_empty() {
    let settings = Options::new().from_bytes::<Settings>(&[0]).unwrap();
    assert_eq!(settings, []);
}

#[test]
fn de() {
    let settings = Options::new()
        .from_bytes::<Settings>(&[3, 1, 2, 3])
        .unwrap();
    assert_eq!(settings, [1, 2, 3]);
}
