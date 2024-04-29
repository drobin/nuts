// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use nuts_backend::{Backend, Binary, IdSize};
use nuts_bytes::{FromBytes, PutBytes, TakeBytes, ToBytes};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid id bytes")]
struct InvalidIdBytes;

#[derive(Debug, Error)]
#[error("invalid id size")]
struct InvalidIdSize;

pub struct Id<B: Backend>(B::Id);

impl<B: Backend> Id<B> {
    pub fn new(id: B::Id) -> Id<B> {
        Id(id)
    }
}

impl<B: Backend> Binary for Id<B> {
    fn from_bytes(bytes: &[u8]) -> Option<Id<B>> {
        <B::Id as Binary>::from_bytes(bytes).map(|id| Id(id))
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }
}

impl<B: Backend> FromBytes for Id<B> {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Id<B>, nuts_bytes::Error> {
        let size = <B::Id as IdSize>::size();
        let mut buf = vec![0; size];

        source.take_bytes(&mut buf)?;

        match <B::Id as Binary>::from_bytes(&buf) {
            Some(id) => Ok(Id(id)),
            None => Err(nuts_bytes::Error::Custom(Box::new(InvalidIdBytes))),
        }
    }
}

impl<B: Backend> ToBytes for Id<B> {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, nuts_bytes::Error> {
        let size = <B::Id as IdSize>::size();
        let bytes = self.0.as_bytes();

        if bytes.len() == size {
            target.put_bytes(&bytes)?;

            Ok(size)
        } else {
            Err(nuts_bytes::Error::Custom(Box::new(InvalidIdSize)))
        }
    }
}

impl<B: Backend> AsRef<B::Id> for Id<B> {
    fn as_ref(&self) -> &B::Id {
        &self.0
    }
}

impl<B: Backend> Clone for Id<B> {
    fn clone(&self) -> Id<B> {
        Id(self.0.clone())
    }
}

impl<B: Backend> FromStr for Id<B>
where
    B::Id: FromStr,
{
    type Err = <<B as Backend>::Id as FromStr>::Err;

    fn from_str(s: &str) -> Result<Id<B>, Self::Err> {
        s.parse().map(|id| Id(id))
    }
}

impl<B: Backend> PartialEq for Id<B> {
    fn eq(&self, other: &Id<B>) -> bool {
        self.0 == other.0
    }
}

impl<B: Backend> fmt::Debug for Id<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl<B: Backend> fmt::Display for Id<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}
