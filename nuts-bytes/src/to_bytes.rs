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

#[cfg(test)]
mod tests;

use thiserror::Error;

use crate::put_bytes::{PutBytes, PutBytesError};

/// Error type of the [`ToBytes`] trait.
#[derive(Debug, Error, PartialEq)]
pub enum ToBytesError {
    /// Errors coming from [`PutBytes`].
    #[error(transparent)]
    PutBytes(#[from] PutBytesError),
}

/// Trait that supports writing datatypes into a binary data stream.
///
/// Datatypes that implements this trait can be serialized into a binary data
/// stream.
pub trait ToBytes {
    /// Writes data into the given `target`.
    ///
    /// Serializes this instance into its binary representation and writes the
    /// binary data into `target`. Returns the number of bytes actually
    /// serialized.
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError>;
}

impl ToBytes for bool {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        let val = if *self { 1u8 } else { 0u8 };

        ToBytes::to_bytes(&val, target)
    }
}

macro_rules! impl_to_bytes_for_primitive {
    ($type:ty) => {
        impl ToBytes for $type {
            fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
                let n = target
                    .put_bytes(&self.to_be_bytes())
                    .map(|()| std::mem::size_of::<$type>())?;
                Ok(n)
            }
        }
    };
}

impl_to_bytes_for_primitive!(i8);
impl_to_bytes_for_primitive!(i16);
impl_to_bytes_for_primitive!(i32);
impl_to_bytes_for_primitive!(i64);
impl_to_bytes_for_primitive!(u8);
impl_to_bytes_for_primitive!(u16);
impl_to_bytes_for_primitive!(u32);
impl_to_bytes_for_primitive!(u64);
impl_to_bytes_for_primitive!(f32);
impl_to_bytes_for_primitive!(f64);

impl ToBytes for usize {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        ToBytes::to_bytes(&(*self as u64), target)
    }
}

impl ToBytes for char {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        ToBytes::to_bytes(&(*self as u32), target)
    }
}

impl<TB: ToBytes, const COUNT: usize> ToBytes for [TB; COUNT] {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        let mut n = 0;

        for i in 0..COUNT {
            n += ToBytes::to_bytes(&self[i], target)?;
        }

        Ok(n)
    }
}

impl<TB: ToBytes> ToBytes for &[TB] {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        let mut n = self.len().to_bytes(target)?;

        for i in 0..self.len() {
            n += self.as_ref()[i].to_bytes(target)?;
        }

        Ok(n)
    }
}

impl<TB: ToBytes> ToBytes for Vec<TB> {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        ToBytes::to_bytes(&self.as_slice(), target)
    }
}

impl ToBytes for &str {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        self.as_bytes().to_bytes(target)
    }
}

impl<T: ToBytes> ToBytes for Option<T> {
    fn to_bytes<PB: PutBytes>(&self, target: &mut PB) -> Result<usize, ToBytesError> {
        match self {
            Some(val) => Ok(ToBytes::to_bytes(&1u8, target)? + ToBytes::to_bytes(val, target)?),
            None => ToBytes::to_bytes(&0u8, target),
        }
    }
}

impl ToBytes for () {
    fn to_bytes<PB: PutBytes>(&self, _target: &mut PB) -> Result<usize, ToBytesError> {
        Ok(0)
    }
}
