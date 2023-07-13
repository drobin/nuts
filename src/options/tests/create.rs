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

use crate::error::Error;
use crate::options::CreateOptions;

#[test]
fn valid_default() {
    let options = CreateOptions::for_path("foo");

    options.validate().unwrap();

    assert_eq!(options.path.to_str().unwrap(), "foo");
    assert_eq!(options.bsize, 512);
}

#[test]
fn invalid_bsize() {
    for n in [0, 1, 511] {
        let options = CreateOptions::for_path("foo").with_bsize(n);
        let err = options.validate().unwrap_err();
        assert!(matches!(err, Error::InvalidBlockSize(_n)));
    }
}

#[test]
fn valid_bsize() {
    for n in [512, 513, 514, 1024] {
        let options = CreateOptions::for_path("foo").with_bsize(n);

        options.validate().unwrap();

        assert_eq!(options.path.to_str().unwrap(), "foo");
        assert_eq!(options.bsize, n);
    }
}
