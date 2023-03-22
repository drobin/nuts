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

use std::{error, fmt, result};

use nuts_backend::Backend;

use crate::bytes;
use crate::container::ContainerError;

pub enum StreamError<B: Backend> {
    Bytes(bytes::Error),
    Container(ContainerError<B>),
    InvalidMagic,
}

pub type StreamResult<T, B> = result::Result<T, StreamError<B>>;

impl<B: Backend> fmt::Debug for StreamError<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt::Debug::fmt(cause, fmt),
            Self::Container(cause) => fmt::Debug::fmt(cause, fmt),
            Self::InvalidMagic => write!(fmt, "StreamError::InvalidMagic"),
        }
    }
}

impl<B: Backend> fmt::Display for StreamError<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Self::Container(cause) => fmt::Display::fmt(cause, fmt),
            Self::InvalidMagic => write!(fmt, "invalid magic"),
        }
    }
}

impl<B: Backend + 'static> error::Error for StreamError<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            StreamError::Bytes(cause) => Some(cause),
            StreamError::Container(cause) => Some(cause),
            StreamError::InvalidMagic => None,
        }
    }
}

impl<B: Backend> From<bytes::Error> for StreamError<B> {
    fn from(cause: bytes::Error) -> Self {
        StreamError::Bytes(cause)
    }
}

impl<B: Backend> From<ContainerError<B>> for StreamError<B> {
    fn from(cause: ContainerError<B>) -> Self {
        StreamError::Container(cause)
    }
}
