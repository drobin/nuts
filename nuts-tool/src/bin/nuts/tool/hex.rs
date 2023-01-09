// MIT License
//
// Copyright (c) 2022 Robin Doer
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

use log::{trace, warn};
use std::cmp;

const WIDTH: usize = 16;

#[derive(Debug)]
pub struct HexWriter {
    buf: Vec<u8>,
    offset: usize,
}

impl HexWriter {
    pub fn new() -> HexWriter {
        HexWriter {
            buf: vec![],
            offset: 0,
        }
    }
    pub fn fill<T: AsRef<[u8]>>(&mut self, buf: T) {
        self.buf.extend_from_slice(buf.as_ref());
    }

    pub fn print(&mut self) {
        while self.buf.len() >= WIDTH {
            self.print_line(false);
        }
    }

    pub fn flush(&mut self) {
        self.print();
        self.print_line(true);
    }

    pub fn print_line(&mut self, force: bool) {
        if self.buf.is_empty() {
            return;
        }

        let width = if force {
            cmp::min(self.buf.len(), WIDTH)
        } else {
            WIDTH
        };

        trace!(
            "print_line: width = {}, avail = {}, force = {}",
            width,
            self.buf.len(),
            force
        );

        if self.buf.len() < width {
            warn!(
                "insufficient data available, need {}, got {} (force: {})",
                width,
                self.buf.len(),
                force
            );
            return;
        }

        let (hex, ascii) = self.buf.drain(..width).enumerate().fold(
            (String::new(), String::new()),
            |(mut hex, mut ascii), (idx, n)| {
                hex += &format!("{:02x} ", n);

                if idx % 4 == 3 {
                    hex.push(' ');
                }

                if n.is_ascii() && !n.is_ascii_control() {
                    ascii.push(n.into());
                } else {
                    ascii.push('.');
                }

                (hex, ascii)
            },
        );

        println!("{:>04x}:   {:<52} {}", self.offset, hex, ascii);

        self.offset += WIDTH;
    }
}
