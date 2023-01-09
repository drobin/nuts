// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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
use std::io::{self, Read};

use crate::{backend::Backend, stream::Stream};

macro_rules! eval_stream_op {
    ($expr:expr) => {
        match $expr {
            Some(Ok(_)) => {}
            Some(Err(err)) => return Err(err.into()),
            None => return Ok(0),
        }
    };
}

impl<'a, B: 'static + Backend> Read for Stream<'a, B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!("next read attempt");

        if buf.len() == 0 {
            return Ok(0);
        }

        if self.cur.is_none() {
            trace!("switch to first block");
            eval_stream_op!(self.first_block());
        };

        loop {
            let len = self.copy_remaining_payload(buf);

            if len == 0 {
                trace!("no remaining bytes, switch to next block");
                eval_stream_op!(self.next_block());
            } else {
                trace!("copied {} bytes to buf", len);
                return Ok(len);
            }
        }
    }
}
