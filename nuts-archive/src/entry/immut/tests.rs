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

mod file_read_all;
mod file_read_vec;
mod inner_first;
mod inner_next;
mod inner_read;
mod symlink;

use nuts_memory::MemoryBackend;

use crate::entry::FULL;
use crate::tests::setup_archive_with_bsize;
use crate::Archive;

fn setup_archive(num: u8) -> Archive<MemoryBackend> {
    let mut archive = setup_archive_with_bsize(FULL as u32);
    let mut entry = archive.append_file("f1").build().unwrap();

    if num > 0 {
        entry.write_all(&(0..num).collect::<Vec<u8>>()).unwrap();
    }

    archive
}
