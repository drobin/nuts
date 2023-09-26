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

use crate::backend::Backend;
use crate::container::cipher::Cipher;
use crate::container::kdf::Kdf;

/// Information from the container.
#[derive(Debug, PartialEq)]
pub struct Info<B: Backend> {
    /// Information from the lower backend.
    pub backend: B::Info,

    /// The cipher used for encryption.
    pub cipher: Cipher,

    /// The key derivation function.
    pub kdf: Kdf,

    /// The gross block size is the block size specified by the
    /// [backend](Backend::block_size).
    ///
    /// This is the actual size of a block in the backend. Note that the number
    /// of userdata bytes per block can be smaller! This is the net block size
    /// ([`Info::bsize_net`]).
    pub bsize_gross: u32,

    /// The (net) block size of the container.
    ///
    /// The net block size is the number of bytes you can store in a block. It
    /// can be less than the gross block size ([`Info::bsize_gross`]).
    ///
    /// Depending on the selected cipher, you need to store additional data in
    /// a block. I.e. an AE-cipher results into a tag, which needs to be stored
    /// additionally. Such data must be substracted from the gross block size
    /// and results into the net block size.
    pub bsize_net: u32,
}
