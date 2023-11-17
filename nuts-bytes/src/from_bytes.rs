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

use std::{mem, string::FromUtf8Error};

use crate::take_bytes::{TakeBytes, TakeBytesError};

/// Trait that supports reading datatypes from a binary data stream.
///
/// Datatypes that implements this trait can be read from a binary data stream.
pub trait FromBytes<E: TakeBytesError>
where
    Self: Sized,
{
    /// Reads data from the given `source`.
    ///
    /// Reads as much as necessary from `source`. The method deserializes the
    /// instance and returns it.
    ///
    /// # Errors
    ///
    /// If not enough data are available in `source`, the
    /// [`TakeBytes::take_bytes()`] call returns a [`TakeBytesError::eof()`]
    /// error, which should be simply forwarded.
    ///
    /// Custom errors can be defined in the `E` generic. See the
    /// [`TakeBytes` implementation in `String`](#impl-FromBytes%3CE%3E-for-String)
    /// for an example.
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E>;
}

impl<E: TakeBytesError> FromBytes<E> for bool {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let val: u8 = FromBytes::from_bytes(source)?;

        Ok(val != 0)
    }
}

macro_rules! impl_from_bytes_for_primitive {
    ($type:ty) => {
        impl<E: TakeBytesError> FromBytes<E> for $type {
            fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
                let mut buf = [0; mem::size_of::<$type>()];

                source.take_bytes::<E>(&mut buf)?;

                Ok(<$type>::from_be_bytes(buf))
            }
        }
    };
}

impl_from_bytes_for_primitive!(i8);
impl_from_bytes_for_primitive!(i16);
impl_from_bytes_for_primitive!(i32);
impl_from_bytes_for_primitive!(i64);
impl_from_bytes_for_primitive!(u8);
impl_from_bytes_for_primitive!(u16);
impl_from_bytes_for_primitive!(u32);
impl_from_bytes_for_primitive!(u64);
impl_from_bytes_for_primitive!(f32);
impl_from_bytes_for_primitive!(f64);

impl<E: TakeBytesError> FromBytes<E> for usize {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let mut buf = [0; mem::size_of::<u64>()];

        source.take_bytes::<E>(&mut buf)?;

        Ok(u64::from_be_bytes(buf) as usize)
    }
}

/// Trait describes an error when
/// [converting into a `char`](trait.FromBytes.html#impl-FromBytes%3CE%3E-for-char).
pub trait TakeCharError: TakeBytesError {
    /// Deserialized an invalid character.
    ///
    /// There is no character which corresponds the given number.
    fn invalid_char(n: u32) -> Self;
}

impl<E: TakeCharError> FromBytes<E> for char {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let n: u32 = FromBytes::from_bytes(source)?;

        char::from_u32(n).ok_or_else(|| E::invalid_char(n))
    }
}

impl<E: TakeBytesError, FB: Copy + Default + FromBytes<E>, const COUNT: usize> FromBytes<E>
    for [FB; COUNT]
{
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let mut target = [Default::default(); COUNT];

        for i in 0..COUNT {
            target[i] = FromBytes::from_bytes(source)?;
        }

        Ok(target)
    }
}

impl<E: TakeBytesError, FB: FromBytes<E>> FromBytes<E> for Vec<FB> {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let len = usize::from_bytes(source)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(FromBytes::from_bytes(source)?);
        }

        Ok(vec)
    }
}

/// Trait describes an error when
/// [converting into a `String`](trait.FromBytes.html#impl-FromBytes%3CE%3E-for-String).
pub trait TakeStringError: TakeBytesError {
    /// Creates an error, where the byte data is not valid UTF-8.
    fn invalid_string(err: FromUtf8Error) -> Self;
}

/// Converts a byte stream into a string.
///
/// The assigned error type is derived from [`TakeStringError`], so the string
/// conversion can be invalid as well.
impl<E: TakeStringError> FromBytes<E> for String {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let len = usize::from_bytes(source)?;

        let mut vec = vec![0; len];
        source.take_bytes(&mut vec)?;

        String::from_utf8(vec).map_err(|err| E::invalid_string(err))
    }
}

impl<E: TakeBytesError, T: FromBytes<E>> FromBytes<E> for Option<T> {
    fn from_bytes<TB: TakeBytes>(source: &mut TB) -> Result<Self, E> {
        let n: u8 = FromBytes::from_bytes(source)?;

        if n == 0 {
            Ok(None)
        } else {
            Ok(Some(FromBytes::from_bytes(source)?))
        }
    }
}
