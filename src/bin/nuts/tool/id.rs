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

use nuts::container::Container;
use std::cmp;

use crate::tool::result::Result;

#[derive(Debug)]
pub struct IdRange {
    start: Option<u64>,
    end: Option<u64>,
    resolved: bool,
}

impl IdRange {
    pub fn new(start: Option<u64>, end: Option<u64>) -> IdRange {
        IdRange {
            start,
            end,
            resolved: false,
        }
    }

    pub fn start(&self) -> Option<u64> {
        self.start
    }

    pub fn end(&self) -> Option<u64> {
        self.end
    }

    pub fn resolve(&mut self, container: &Container) -> Result<&Self> {
        let max_end = container.blocks()? - 1;

        let mut start = match self.start {
            Some(n) => n,
            None => 0,
        };

        let mut end = match self.end {
            Some(n) => n,
            None => max_end,
        };

        start = cmp::min(start, max_end);
        end = cmp::min(end, max_end);

        if start > end {
            end = start;
        }

        self.start = Some(start);
        self.end = Some(end);
        self.resolved = true;

        Ok(self)
    }

    pub fn iter(&self) -> IdRangeIterator {
        assert!(self.resolved, "calling iter() on an unresolved IdRange");
        IdRangeIterator::new(self.start.unwrap(), self.end.unwrap())
    }
}

pub struct IdRangeIterator {
    cur: u64,
    end: u64,
}

impl IdRangeIterator {
    fn new(start: u64, end: u64) -> IdRangeIterator {
        IdRangeIterator { cur: start, end }
    }
}

impl Iterator for IdRangeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur <= self.end {
            let val = self.cur;
            self.cur += 1;
            Some(val)
        } else {
            None
        }
    }
}
