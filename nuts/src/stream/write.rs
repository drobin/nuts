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

use log::trace;
use std::io::{self, Write};

use nuts_backend::Backend;

use crate::stream::Stream;

impl<'a, B: 'static + Backend> Write for Stream<'a, B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        trace!("next write attempt");

        if buf.len() == 0 {
            return Ok(0);
        }

        if self.current_id().is_none() {
            match self.last_block() {
                Some(Ok(_)) => {
                    let len = self.current_payload().map_or(0, |buf| buf.len());
                    self.offs = len;
                }
                Some(Err(err)) => return Err(err.into()),
                None => {
                    if let Err(err) = self.insert(None) {
                        return Err(err.into());
                    }
                }
            }
        }

        if self.available_payload() == 0 {
            self.insert(None).unwrap();
        }

        let len = self.append_payload(buf);

        if self.available_payload() == 0 {
            self.flush_current_block()?;
        }

        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(self.flush_current_block()?)
    }
}
