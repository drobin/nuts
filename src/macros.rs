// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

macro_rules! secure_vec {
    () => {
        crate::utils::SecureVec::new(vec![])
    };
    ($elem:expr; $n:expr) => {
        crate::utils::SecureVec::new(vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => (
        crate::utils::SecureVec::new(vec![$($x),+])
    );
}

#[cfg(test)]
macro_rules! assert_error {
    ($kind:expr, $elem:expr) => {
        let err = $elem.unwrap_err();
        assert_eq!(format!("{:?}", err), format!("{:?}", $kind));
    };
}

#[cfg(test)]
macro_rules! assert_io_error {
    ($kind:expr, $elem:expr) => {
        if let crate::error::Error::IoError(err) = $elem.unwrap_err() {
            assert_eq!(err.kind(), $kind);
        } else {
            panic!("invalid error");
        }
    };
}

#[cfg(test)]
macro_rules! assert_inval_header {
    ($name:expr, $elem:expr) => {
        let err = $elem.unwrap_err();
        match err {
            crate::error::Error::IoError(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
                assert_eq!(format!("{}", err), format!("invalid {} detected", $name));
            }
            _ => panic!("unexpected error: {:?}", err),
        }
    };
}
