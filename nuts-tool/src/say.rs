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

use crate::cli::global::GLOBALS;

pub fn is_quiet() -> bool {
    GLOBALS.with_borrow(|g| g.say.quiet)
}

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

#[derive(Default)]
pub struct Say {
    quiet: bool,
}

impl Say {
    pub fn set_quiet(&mut self, quiet: bool) {
        self.quiet = quiet;
    }

    pub fn say(&self, level: Level, args: Arguments<'_>) {
        if self.quiet {
            return;
        }

        let msg = match args.as_str() {
            Some(s) => Cow::Borrowed(s),
            None => Cow::Owned(args.to_string()),
        };

        match level.into_color() {
            Some(color) => println!("{}", msg.color(color)),
            None => println!("{}", msg),
        }
    }
}

#[macro_export]
macro_rules! say {
    ($level:ident $($arg:tt)*) => {{
        $crate::cli::global::GLOBALS.with_borrow(|g|
            g.say.say($crate::say::Level::$level, format_args!($($arg)*))
        )
    }};

    ($($arg:tt)*) => {
        say!(Normal $($arg)*);
    };
}

#[macro_export]
macro_rules! say_warn {
    ($($arg:tt)*) => {
        say!(Warning $($arg)*);
    };
}

#[macro_export]
macro_rules! say_err {
    ($($arg:tt)*) => {
        say!(Error $($arg)*);
    };
}
