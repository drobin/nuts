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

mod directory;
mod symlink;
mod write;
mod write_all;

use nuts_memory::MemoryBackend;

use crate::entry::mode::Mode;
use crate::entry::r#mut::{DirectoryBuilder, FileBuilder, InnerBuilder, SymlinkBuilder};
use crate::id::Id;
use crate::Archive;

fn lookup(archive: &mut Archive<MemoryBackend>, idx: usize) -> Option<&Id<MemoryBackend>> {
    match archive.tree.lookup(&mut archive.pager, idx) {
        Some(result) => Some(result.unwrap()),
        None => None,
    }
}

fn setup_inner_builder<'a>(
    archive: &'a mut Archive<MemoryBackend>,
) -> InnerBuilder<'a, MemoryBackend> {
    InnerBuilder::new(
        &mut archive.pager,
        &archive.header_id,
        &mut archive.header,
        &mut archive.tree,
        "foo".to_string(),
        Mode::file(),
    )
}

fn setup_file_builder<'a>(
    archive: &'a mut Archive<MemoryBackend>,
) -> FileBuilder<'a, MemoryBackend> {
    FileBuilder::new(
        &mut archive.pager,
        &archive.header_id,
        &mut archive.header,
        &mut archive.tree,
        "foo".to_string(),
    )
}

fn setup_directory_builder<'a>(
    archive: &'a mut Archive<MemoryBackend>,
) -> DirectoryBuilder<'a, MemoryBackend> {
    DirectoryBuilder::new(
        &mut archive.pager,
        &archive.header_id,
        &mut archive.header,
        &mut archive.tree,
        "foo".to_string(),
    )
}

fn setup_symlink_builder<'a>(
    archive: &'a mut Archive<MemoryBackend>,
) -> SymlinkBuilder<'a, MemoryBackend> {
    SymlinkBuilder::new(
        &mut archive.pager,
        &archive.header_id,
        &mut archive.header,
        &mut archive.tree,
        "foo".to_string(),
        "bar".to_string(),
    )
}
