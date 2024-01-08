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

use colored::{Color, Colorize};
use std::{borrow::Cow, cell::RefCell, fmt::Arguments};

thread_local! {
    pub static SAY: RefCell<Say> = RefCell::new(Say::new());
}

pub fn is_quiet() -> bool {
    SAY.with(|say| say.borrow().quiet)
}

pub fn set_quiet(quiet: bool) {
    SAY.with(|say| say.borrow_mut().quiet = quiet)
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

pub struct Say {
    quiet: bool,
}

impl Say {
    fn new() -> Say {
        Say { quiet: false }
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

macro_rules! say {
    ($level:ident $($arg:tt)*) => {{
        crate::say::SAY.with(|say|
            say.borrow().say(crate::say::Level::$level, format_args!($($arg)*))
        );
    }};

    ($($arg:tt)*) => {
        say!(Normal $($arg)*);
    };
}

#[allow(unused_macros)]
macro_rules! say_warn {
    ($($arg:tt)*) => {
        say!(Warning $($arg)*);
    };
}

macro_rules! say_err {
    ($($arg:tt)*) => {
        say!(Error $($arg)*);
    };
}

#[allow(unused_imports)]
pub(crate) use {say, say_err, say_warn};
