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

//! Transformation from/to binary streams.

mod error;
mod options;
mod reader;
mod writer;

pub use error::{Error, IntType, Result};
pub use options::Options;
pub use reader::Reader;
pub use writer::Writer;

#[cfg(test)]
macro_rules! assert_error {
    ($err:expr, $type:ident :: $memb:ident) => {
        match $err {
            $type::$memb => {}
            _ => panic!("invalid error"),
        }
    };

    ($err:expr, $type:ident :: $memb:ident ( $(| $arg:ident | $assert:expr),+ ) ) => {
        match $err {
            $type::$memb($($arg),*) => {
                $(
                    assert!($assert);
                )*
            }
            _ => panic!("invalid error"),
        }
    };

    ($err:expr, $type:ident :: $memb:ident { $(| $arg:ident | $assert:expr),+ } ) => {
        match $err {
            $type::$memb{$($arg),*} => {
                $(
                    assert!($assert);
                )*
            }
            _ => panic!("invalid error"),
        }
    };
}

#[cfg(test)]
macro_rules! assert_error_eq {
    ($err:expr, $type:ident :: $memb:ident ( $(| $arg:ident | $val:expr),+ ) ) => {
        match $err {
            $type::$memb($($arg),*) => {
                $(
                    assert_eq!($arg, $val);
                )*
            }
            _ => panic!("invalid error"),
        }
    };

    ($err:expr, $type:ident :: $memb:ident { $(| $arg:ident | $val:expr),+ } ) => {
        match $err {
            $type::$memb{$($arg),*} => {
                $(
                    assert_eq!($arg, $val);
                )*
            }
            _ => panic!("invalid error"),
        }
    };
}

#[cfg(test)]
pub(crate) use {assert_error, assert_error_eq};
