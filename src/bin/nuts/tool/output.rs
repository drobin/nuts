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

use std::cmp;
use std::io::{self, Write};

use crate::tool::format::Format;

macro_rules! print_offset {
    ($offs:expr) => {
        print!("{:>08x} ", $offs);
    };
}

macro_rules! print_hex {
    ($n:expr) => {
        print!("{:02x} ", $n);
    };
}
pub struct Output {
    fmt: Format,
    buf: Vec<u8>,
    offset: usize,
}

impl Output {
    pub fn new(fmt: Format) -> Output {
        Output {
            fmt,
            buf: Vec::new(),
            offset: 0,
        }
    }

    pub fn push(&mut self, data: &[u8]) -> &mut Self {
        self.buf.extend(data.iter());
        self
    }

    pub fn print(&mut self) {
        match self.fmt {
            Format::Raw => self.print_raw(),
            Format::Hex => self.print_hex(),
        }
    }

    pub fn flush(&mut self) {
        match self.fmt {
            Format::Raw => self.flush_raw(),
            Format::Hex => self.flush_hex(),
        }
    }

    fn print_hex(&mut self) {
        while self.buf.len() > 0 {
            let offs_line = self.offset % 16;
            let remaining = 16 - offs_line;
            let nbytes = cmp::min(remaining, self.buf.len());

            if offs_line == 0 {
                print_offset!(self.offset);
            }

            for n in self.buf.drain(..nbytes) {
                print_hex!(n);
            }

            self.offset += nbytes;

            if self.offset % 16 == 0 {
                println!();
            }
        }
    }

    fn flush_hex(&mut self) {
        // print remaining data
        self.print_hex();

        if self.offset == 0 {
            // Nothing was printed, print at least the offset-marker.
            print_offset!(self.offset);
        }

        if self.offset == 0 || self.offset % 16 != 0 {
            // Appen newline, if not present.
            println!();
        }
    }

    fn print_raw(&mut self) {
        match io::stdout().write_all(&self.buf) {
            Ok(()) => {
                self.offset += self.buf.len();
                self.buf.clear();
            }
            Err(error) => {
                eprintln!("failed to print to stderr: {}", error);
            }
        }
    }

    fn flush_raw(&mut self) {
        match io::stdout().flush() {
            Ok(()) => (),
            Err(error) => {
                eprintln!("failed t flush stdout: {}", error);
            }
        }
    }
}
