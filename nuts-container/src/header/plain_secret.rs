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

#[cfg(test)]
mod tests;

use nuts_backend::{Backend, Binary};
use openssl::error::ErrorStack;
use std::fmt::{self, Write};

use crate::buffer::{Buffer, BufferError, BufferMut, ToBuffer};
use crate::header::HeaderError;
use crate::migrate::Migrator;
use crate::ossl;
use crate::svec::SecureVec;

fn fmt_key_iv(key: &[u8], iv: &[u8]) -> Result<(String, String), fmt::Error> {
    if cfg!(feature = "debug-plain-keys") {
        let mut key_out = String::with_capacity(2 * key.len());
        let mut iv_out = String::with_capacity(2 * iv.len());

        for n in key.iter() {
            write!(key_out, "{:02x}", n)?;
        }

        for n in iv.iter() {
            write!(iv_out, "{:02x}", n)?;
        }

        Ok((key_out, iv_out))
    } else {
        Ok((
            format!("<{} bytes>", key.len()),
            format!("<{} bytes>", iv.len()),
        ))
    }
}

// ** plain-secret history **
//
// * rev 0
//
// inital version
//
// * rev 1
//
// - userdata field removed
// - top_id field inserted
// - decrease vec lengths from 8 to 1, settings from 8 to 2
//
// * rev 2
//
// - sid inserted

#[derive(Debug, PartialEq)]
pub struct Magics([u32; 2]);

impl Magics {
    fn generate() -> Result<Magics, ErrorStack> {
        ossl::rand_u32().map(|magic| Magics([magic, magic]))
    }

    fn get_and_validate<T: Buffer>(buf: &mut T) -> Result<Magics, HeaderError> {
        let n1 = buf.get_u32()?;
        let n2 = buf.get_u32()?;

        if n1 == n2 {
            Ok(Magics([n1, n2]))
        } else {
            Err(HeaderError::WrongPassword)
        }
    }

    fn put<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        buf.put_u32(self.0[0])?;
        buf.put_u32(self.0[1])?;

        Ok(())
    }
}

impl From<u32> for Magics {
    fn from(value: u32) -> Self {
        Magics([value; 2])
    }
}

pub struct PlainRev0<B: Backend> {
    pub magics: Magics,
    pub key: SecureVec,
    pub iv: SecureVec,
    pub userdata: SecureVec,
    pub settings: B::Settings,
    pub sid: Option<u32>,      // transient, migrated from userdata attribute
    pub top_id: Option<B::Id>, // transient, migrated from userdata attribute
}

impl<B: Backend> PlainRev0<B> {
    pub fn migrate(&mut self, migrator: &Migrator) -> Result<(), HeaderError> {
        if let Some((sid, top_id_bytes)) = migrator.migrate_rev0(&self.userdata)? {
            self.sid = Some(sid);

            match <B::Id as Binary>::from_bytes(&top_id_bytes) {
                Some(id) => self.top_id = Some(id),
                None => return Err(HeaderError::InvalidTopId),
            }
        }

        Ok(())
    }
}

impl<B: Backend> PartialEq for PlainRev0<B> {
    fn eq(&self, other: &PlainRev0<B>) -> bool {
        let lhs_settings_bytes = self.settings.as_bytes();
        let rhs_settings_bytes = other.settings.as_bytes();

        self.magics == other.magics
            && self.key == other.key
            && self.iv == other.iv
            && self.userdata == other.userdata
            && lhs_settings_bytes == rhs_settings_bytes
            && self.sid == other.sid
            && self.top_id == other.top_id
    }
}

impl<B: Backend> fmt::Debug for PlainRev0<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (key, iv) = fmt_key_iv(&self.key, &self.iv)?;

        fmt.debug_struct("PlainRev0")
            .field("magics", &self.magics)
            .field("key", &key)
            .field("iv", &iv)
            .field("userdata", &self.userdata)
            .field("settings", &self.settings.as_bytes())
            .field("sid", &self.sid)
            .field("top_id", &self.top_id.as_ref().map(ToString::to_string))
            .finish()
    }
}

pub struct PlainRev1<B: Backend> {
    pub magics: Magics,
    pub key: SecureVec,
    pub iv: SecureVec,
    pub top_id: Option<B::Id>,
    pub settings: B::Settings,
}

impl<B: Backend> PartialEq for PlainRev1<B> {
    fn eq(&self, other: &PlainRev1<B>) -> bool {
        let lhs_settings_bytes = self.settings.as_bytes();
        let rhs_settings_bytes = other.settings.as_bytes();

        self.magics == other.magics
            && self.key == other.key
            && self.iv == other.iv
            && self.top_id == other.top_id
            && lhs_settings_bytes == rhs_settings_bytes
    }
}

impl<B: Backend> fmt::Debug for PlainRev1<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (key, iv) = fmt_key_iv(&self.key, &self.iv)?;

        fmt.debug_struct("PlainRev1")
            .field("magics", &self.magics)
            .field("key", &key)
            .field("iv", &iv)
            .field("top_id", &self.top_id.as_ref().map(ToString::to_string))
            .field("settings", &self.settings.as_bytes())
            .finish()
    }
}

pub struct PlainRev2<B: Backend> {
    pub magics: Magics,
    pub key: SecureVec,
    pub iv: SecureVec,
    pub sid: Option<u32>,
    pub top_id: Option<B::Id>,
    pub settings: B::Settings,
}

impl<B: Backend> PartialEq for PlainRev2<B> {
    fn eq(&self, other: &PlainRev2<B>) -> bool {
        let lhs_settings_bytes = self.settings.as_bytes();
        let rhs_settings_bytes = other.settings.as_bytes();

        self.magics == other.magics
            && self.key == other.key
            && self.iv == other.iv
            && self.sid == other.sid
            && self.top_id == other.top_id
            && lhs_settings_bytes == rhs_settings_bytes
    }
}

impl<B: Backend> fmt::Debug for PlainRev2<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (key, iv) = fmt_key_iv(&self.key, &self.iv)?;

        fmt.debug_struct("PlainRev1")
            .field("magics", &self.magics)
            .field("key", &key)
            .field("iv", &iv)
            .field("sid", &self.sid)
            .field("top_id", &self.top_id.as_ref().map(ToString::to_string))
            .field("settings", &self.settings.as_bytes())
            .finish()
    }
}

#[derive(PartialEq)]
pub enum PlainSecret<B: Backend> {
    Rev0(PlainRev0<B>),
    Rev1(PlainRev1<B>),
    Rev2(PlainRev2<B>),
}

impl<B: Backend> PlainSecret<B> {
    pub fn from_buffer_rev0<T: Buffer>(buf: &mut T) -> Result<PlainSecret<B>, HeaderError> {
        let magics = Magics::get_and_validate(buf)?;
        let key = buf.get_vec::<8>()?.into();
        let iv = buf.get_vec::<8>()?.into();
        let userdata = buf.get_vec::<8>()?.into();
        let settings_bytes: SecureVec = buf.get_vec::<8>()?.into();

        let settings = Binary::from_bytes(&settings_bytes).ok_or(HeaderError::InvalidSettings)?;

        Ok(PlainSecret::Rev0(PlainRev0 {
            magics,
            key,
            iv,
            userdata,
            settings,
            sid: None,
            top_id: None,
        }))
    }

    pub fn from_buffer_rev1<T: Buffer>(buf: &mut T) -> Result<PlainSecret<B>, HeaderError> {
        let magics = Magics::get_and_validate(buf)?;
        let key = buf.get_vec::<1>()?.into();
        let iv = buf.get_vec::<1>()?.into();
        let top_id_bytes: SecureVec = buf.get_vec::<1>()?.into();
        let settings_bytes: SecureVec = buf.get_vec::<2>()?.into();

        let top_id = if !top_id_bytes.is_empty() {
            Some(Binary::from_bytes(&top_id_bytes).ok_or(HeaderError::InvalidTopId)?)
        } else {
            None
        };

        let settings = Binary::from_bytes(&settings_bytes).ok_or(HeaderError::InvalidSettings)?;

        Ok(PlainSecret::Rev1(PlainRev1 {
            magics,
            key,
            iv,
            top_id,
            settings,
        }))
    }

    pub fn from_buffer_rev2<T: Buffer>(buf: &mut T) -> Result<PlainSecret<B>, HeaderError> {
        let magics = Magics::get_and_validate(buf)?;
        let key = buf.get_vec::<1>()?.into();
        let iv = buf.get_vec::<1>()?.into();
        let sid_raw = buf.get_u32()?;
        let top_id_bytes: SecureVec = buf.get_vec::<1>()?.into();
        let settings_bytes: SecureVec = buf.get_vec::<2>()?.into();

        let sid = if sid_raw > 0 { Some(sid_raw) } else { None };

        let top_id = if !top_id_bytes.is_empty() {
            Some(Binary::from_bytes(&top_id_bytes).ok_or(HeaderError::InvalidTopId)?)
        } else {
            None
        };

        let settings = Binary::from_bytes(&settings_bytes).ok_or(HeaderError::InvalidSettings)?;

        Ok(PlainSecret::Rev2(PlainRev2 {
            magics,
            key,
            iv,
            sid,
            top_id,
            settings,
        }))
    }

    pub fn create_latest(
        key: SecureVec,
        iv: SecureVec,
        settings: B::Settings,
    ) -> Result<(u32, PlainSecret<B>), ErrorStack> {
        let rev = Self::Rev2(PlainRev2 {
            magics: Magics::generate()?,
            key,
            iv,
            sid: None,
            top_id: None,
            settings,
        });

        Ok((2, rev))
    }
}

impl<B: Backend> ToBuffer for PlainSecret<B> {
    fn to_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        match self {
            PlainSecret::Rev0(rev0) => {
                rev0.magics.put(buf)?;
                buf.put_vec::<8>(&rev0.key)?;
                buf.put_vec::<8>(&rev0.iv)?;
                buf.put_vec::<8>(&rev0.userdata)?;
                buf.put_vec::<8>(&rev0.settings.as_bytes())?;
            }
            PlainSecret::Rev1(rev1) => {
                rev1.magics.put(buf)?;
                buf.put_vec::<1>(&rev1.key)?;
                buf.put_vec::<1>(&rev1.iv)?;

                match rev1.top_id.as_ref() {
                    Some(id) => buf.put_vec::<1>(&id.as_bytes())?,
                    None => buf.put_vec::<1>(&[])?,
                }

                buf.put_vec::<2>(&rev1.settings.as_bytes())?;
            }
            PlainSecret::Rev2(rev2) => {
                rev2.magics.put(buf)?;
                buf.put_vec::<1>(&rev2.key)?;
                buf.put_vec::<1>(&rev2.iv)?;

                match rev2.sid {
                    Some(n) => buf.put_u32(n)?,
                    None => buf.put_u32(0)?,
                }

                match rev2.top_id.as_ref() {
                    Some(id) => buf.put_vec::<1>(&id.as_bytes())?,
                    None => buf.put_vec::<1>(&[])?,
                }

                buf.put_vec::<2>(&rev2.settings.as_bytes())?;
            }
        }

        Ok(())
    }
}

impl<B: Backend> fmt::Debug for PlainSecret<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Rev0(rev0) => fmt.debug_tuple("Rev0").field(rev0).finish(),
            Self::Rev1(rev1) => fmt.debug_tuple("Rev1").field(rev1).finish(),
            Self::Rev2(rev2) => fmt.debug_tuple("Rev2").field(rev2).finish(),
        }
    }
}
