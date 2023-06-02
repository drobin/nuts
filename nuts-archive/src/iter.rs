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

use std::cmp;
use std::io::{self, Read};
use std::ops::Deref;

use log::debug;
use nuts::stream::Position;
use nuts_backend::Backend;
use nuts_bytes::Options;
use serde::Deserialize;

use crate::entry::Entry;
use crate::error::Error;
use crate::header::Header;
use crate::rc::StreamRc;
#[cfg(doc)]
use crate::Archive;

/// An iterator entry yielded by [`Iter`].
///
/// It wraps an [`Entry`] my implementing the [`Deref`] trait.
///
/// It also implements [`Read`], so you can read the content of the entry from
/// the archive.
pub struct IterEntry<'a, B: Backend> {
    stream: StreamRc<B>,
    entry: Entry<'a>,
    nread: usize,
}

impl<'a, B: Backend> IterEntry<'a, B> {
    fn new(entry: Entry, stream: StreamRc<B>) -> IterEntry<B> {
        IterEntry {
            entry,
            stream,
            nread: 0,
        }
    }
}

impl<'a, B: 'static + Backend> Read for IterEntry<'a, B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let remaining = self.entry.size() as usize - self.nread;
        let len = cmp::min(remaining, buf.len());

        let n = self
            .stream
            .borrow_mut()
            .read(&mut buf[..len])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.nread += n;

        Ok(n)
    }
}

impl<'a, B: Backend> Deref for IterEntry<'a, B> {
    type Target = Entry<'a>;

    fn deref(&self) -> &Entry<'a> {
        &self.entry
    }
}

/// Iterator returned by [`Archive::scan()`] scans the whole archive for
/// entries.
///
/// The iterator yields instances of [`IterEntry`], which wraps an [`Entry`].
/// Using [`IterEntry`] you can [read](IterEntry::read) the content of an [`Entry`].
pub struct Iter<B: Backend> {
    stream: StreamRc<B>,
    first: bool,
    skip: i64,
}

impl<B: 'static + Backend> Iter<B> {
    pub(crate) fn new(stream: StreamRc<B>) -> Iter<B> {
        Iter {
            stream,
            first: true,
            skip: 0,
        }
    }

    fn initialize(&mut self) -> Result<(), Error<B>> {
        if self.first {
            self.move_to_front()?;
            self.first = false;
        }

        Ok(())
    }

    fn move_to_front(&mut self) -> Result<(), Error<B>> {
        self.stream.borrow_mut().seek(Position::Start(0))?;

        // Skip the header by reading it.
        let mut reader = Options::new().build_reader(&mut self.stream);
        Header::deserialize(&mut reader)?;

        Ok(())
    }
}

impl<B: 'static + Backend> Iterator for Iter<B> {
    type Item = Result<IterEntry<'static, B>, Error<B>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(err) = self.initialize() {
            return Some(Err(err));
        }

        debug!("skipping {} bytes", self.skip);
        if let Err(err) = self.stream.borrow_mut().seek(Position::Current(self.skip)) {
            return Some(Err(err.into()));
        }

        let mut reader = Options::new().build_reader(&mut self.stream);

        match Entry::deserialize(&mut reader) {
            Ok(entry) => {
                self.skip = entry.size() as i64;
                Some(Ok(IterEntry::new(entry, StreamRc::clone(&self.stream))))
            }
            Err(nuts_bytes::Error::Eof(_)) => None,
            Err(err) => Some(Err(err.into())),
        }
    }
}
