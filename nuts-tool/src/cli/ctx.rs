// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use crate::cli::global::GlobalArgs;

pub struct GlobalContext {
    pub verbose: u8,
    pub quiet: bool,
}

impl GlobalContext {
    pub fn from_args(args: &GlobalArgs) -> GlobalContext {
        GlobalContext {
            verbose: args.verbose,
            quiet: args.quiet,
        }
    }
}

macro_rules! say {
    ($ctx:expr, $level:ident $($arg:tt)*) => {
        if !$ctx.quiet {
            $crate::say::say($crate::say::Level::$level, format_args!($($arg)*))
        }
    };

    ($ctx:expr, $($arg:tt)*) => {
        say!($ctx, Normal $($arg)*);
    };
}

macro_rules! say_warn {
    ($ctx:expr, $($arg:tt)*) => {
        say!($ctx, Warning $($arg)*);
    };
}

macro_rules! say_err {
    ($ctx:expr, $($arg:tt)*) => {
        say!($ctx, Error $($arg)*);
    };
}

pub(crate) use {say, say_err, say_warn};
