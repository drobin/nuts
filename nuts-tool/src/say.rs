// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use colored::{Color, Colorize};
use std::borrow::Cow;
use std::fmt::Arguments;

#[derive(Clone, Copy)]
pub enum Level {
    Normal,
    Warning,
    Error,
}

impl Level {
    fn into_color(self) -> Option<Color> {
        match self {
            Self::Normal => None,
            Self::Warning => Some(Color::Yellow),
            Self::Error => Some(Color::Red),
        }
    }
}

pub fn say_args(level: Level, args: Arguments<'_>) {
    let msg = match args.as_str() {
        Some(s) => Cow::Borrowed(s),
        None => Cow::Owned(args.to_string()),
    };

    match level.into_color() {
        Some(color) => println!("{}", msg.color(color)),
        None => println!("{}", msg),
    }
}

macro_rules! say {
    ($ctx:expr, $($arg:tt)*) => {
        if !$ctx.quiet {
            $crate::say::say_args($crate::say::Level::Normal, format_args!($($arg)*))
        }
    };
}

macro_rules! say_warn {
    ($ctx:expr, $($arg:tt)*) => {
        if !$ctx.quiet {
            $crate::say::say_args($crate::say::Level::Warning, format_args!($($arg)*))
        }
    };
}

macro_rules! say_err {
    ($ctx:expr, $($arg:tt)*) => {
        if !$ctx.quiet {
            $crate::say::say_args($crate::say::Level::Error, format_args!($($arg)*))
        }
    };
}

pub(crate) use {say, say_err, say_warn};
