// MIT License
//
// Copyright (c) 2020 Robin Doer
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

/// Details about an [`Error::InvalHeader`] error.
///
/// [`Error::InvalHeader`]: enum.Error.html#variant.InvalHeader
#[derive(PartialEq, Debug)]
pub enum InvalHeaderKind {
    /// Invalid magic.
    ///
    /// The first few bytes encodes a magic string, which is incorrect.
    InvalMagic,

    /// Invalid revision.
    InvalRevision,

    /// Invalid cipher.
    InvalCipher,

    /// Invalid digest.
    InvalDigest,

    /// Invalid wrapping key.
    InvalWrappingKey,
}

/// Collection of error-codes.
#[derive(Debug)]
pub enum Error {
    /// An invalid argument was passed to a function.
    ///
    /// It has a message, that describes the failure.
    InvalArg(String),

    /// An invalid header of the container was detected.
    ///
    /// The value contains some details about the error.
    InvalHeader(InvalHeaderKind),

    /// Not enough data available to read from a source.
    NoData,

    /// Not enough space available to write into a target.
    NoSpace,
}
