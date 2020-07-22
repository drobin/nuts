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

use std::convert::TryFrom;

use rpassword::read_password_from_tty;

use crate::tool;

pub fn to_size<T>(s: &str) -> tool::result::Result<T>
where
    T: TryFrom<u64>,
{
    let s = s.trim_matches(char::is_whitespace).to_lowercase();

    let (s, factor) = if s.ends_with("kb") {
        (&s[..s.len() - 2], 1024)
    } else if s.ends_with("mb") {
        (&s[..s.len() - 2], 1024 * 1024)
    } else if s.ends_with("gb") {
        (&s[..s.len() - 2], 1024 * 1024 * 1024)
    } else {
        (&s[..], 1)
    };

    let size = s.parse::<u64>()?;

    match T::try_from(factor * size) {
        Ok(n) => Ok(n),
        Err(_) => {
            let msg = format!("size {} is too large", size);
            Err(tool::result::Error::new(&msg))
        }
    }
}

pub fn ask_for_password() -> nuts::result::Result<Vec<u8>> {
    let password = read_password_from_tty(Some("Enter a password:"))?;
    Ok(password.as_bytes().to_vec())
}
