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

pub mod immut;
pub mod mode;
pub mod r#mut;

use nuts_bytes::Writer;
use nuts_container::backend::Backend;
use serde::{Deserialize, Serialize};

use crate::entry::mode::Mode;
use crate::error::ArchiveResult;
use crate::pager::Pager;

#[cfg(test)]
const HALF: u8 = 53;
#[cfg(test)]
const FULL: u8 = 106;

#[derive(Debug, Deserialize, Serialize)]
struct Inner {
    name: String,
    mode: Mode,
    size: u64,
}

impl Inner {
    fn new(name: String, mode: Mode) -> Inner {
        Inner {
            name,
            mode,
            size: 0,
        }
    }

    fn load<B: Backend>(pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<Inner, B> {
        let mut reader = pager.read_buf(id)?;
        let inner = reader.deserialize()?;

        Ok(inner)
    }

    fn flush<B: Backend>(&self, pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let buf = {
            let mut writer = Writer::new(vec![]);

            writer.serialize(self)?;

            writer.into_target()
        };

        pager.write(id, &buf)?;

        Ok(())
    }
}
